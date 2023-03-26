
use std::process::{Stdio, Command};

use crate::config::{UserData, PartData};
use crate::components::control::{PackageProfile};
use crate::sysinfo::Device;

use std::path::Path;

use std::str;
use std::fs;

use std::io::prelude::*;

pub mod logic;
pub mod partitions;

///
/// Partition FS Instructions
///
pub fn get_partitions_fs() -> Vec<PartData<'static>> {
  let part_info = vec![
    PartData {
      fs: "mkfs.fat",
      args: vec!["-F", "32"]
    },
    PartData {
      fs: "mkswap",
      args: vec![]
    },
    PartData {
      fs: "mkfs.ext4",
      args: vec!["-F"]
    },
    PartData {
      fs: "mkfs.ext4",
      args: vec!["-F"]
    }
  ];
  return part_info;
}



pub fn write_hostname(hostname: &str) {
  std::fs::write("/mnt/etc/hostname", &hostname)
  .expect("Failed to write file");
}

/// Move installtion chroot script
/// 
pub fn enable_install_script() -> std::io::Result<()> {
 
  let mut chmod = Command::new("chmod")
      .arg("+x")
      .arg("/mnt/install.sh")
      .spawn()
      .expect("FAILED");

  chmod.wait().expect("FAILED");

  Ok(())
}

/// Move the contents of /root in the child's system root (/)
/// 
/// # Returns
///   handler
pub fn copy_root() -> std::io::Result<()> {

  let mut etc_copy = Command::new("cp").arg("-r")
  .args(vec![
    "./root/.",
    "/mnt/"
  ])
  .spawn()
  .expect("FAILED");

  etc_copy.wait().expect("Failed to copy");  
  
  Ok(())
}

/// Remove empty lines in package files, and remove comments
/// 
/// # Arguments
/// 
/// `packages` - Raw list of packages
/// 
/// # Returns
/// 
/// The packages without comments & line breaks
/// 
pub fn filter_packages(packages: &mut Vec<String>) {

  packages.iter_mut().for_each(
    |p| *p = p.trim().to_string()
  );

  packages.retain_mut(|x| {
    x.len() != 0 && x.chars().nth(0).unwrap() != '#'
  });

}

/// Read a package pack file, and add it to the mutable String reference
/// 
/// # Arguments
/// 
///  `content` - A mutable string reference containing the file raw text data
///  `pack_name` - The package pack file name
/// 
pub fn read_packages_bundle(content: &mut String, pack_name: &str) -> String {
  let mut ans = String::new();

  let mut path: String = "./packages/".to_owned();
  path.push_str(pack_name); path.push_str(".x86_64");

  let file_handler = fs::File::open(&path);

  let mut file = match file_handler {
    Ok(file) => file,
    Err(error) => panic!("Package files not found! {:?}", error)
  };

  file.read_to_string(&mut ans)
    .expect("FAILED");
  
  content.push_str(&ans);
  content.push_str("\n");

  return ans;
}

/// Get package names from files
/// 
/// # Arguments
/// 
/// `params` - the configuration of the packages
/// 
/// # Returns
/// 
/// A handler with the package names
pub fn get_packages(params: PackageProfile) -> std::io::Result<Vec<String>> {

  let mut content = String::new();

  read_packages_bundle(&mut content, "base");

  if params.multimedia == true {
    read_packages_bundle(&mut content, "multimedia");
  }

  if params.nightly == true {
    read_packages_bundle(&mut content, "nightly");
  }

  if params.desktop == true {
    read_packages_bundle(&mut content, "desktop");
  }

  if params.utils == true {
    read_packages_bundle(&mut content, "utils");
  }

  if params.amd_gpu == true {
    read_packages_bundle(&mut content, "amd_gpu");
  }

  if params.intel_gpu == true {
    read_packages_bundle(&mut content, "intel_gpu");
  }

  let split = content.split("\n");
  let mut ans: Vec<String> = split.collect::<Vec<&str>>()
    .iter()
    .map(|s| s.to_string())
    .collect();
  
  filter_packages(&mut ans);

  Ok(ans)
}

/// Spawn pacstrap
/// installing packages to mounted device
/// 
/// # Arguments
/// A list of packages
/// 
pub fn pacstrap(packages: Vec<&str>) -> std::io::Result<()>  {

  println!("PACKAGES: ");
  for i in 0..packages.len() {
    print!("{} ", packages[i])
  }

  let mut install_packages = Command::new("pacstrap")
    .arg("-K").arg("/mnt")
    .args(packages)
    .stdout(Stdio::inherit())
    .spawn()
    .unwrap();

  let _res = install_packages.wait().expect("FAILED");

  Ok(())
}

/// Change root to device's root and execute installation script
/// 
/// # Arguments
///  `data` - The form data from the frontend
/// 
pub fn chroot(
  data: &UserData,
) {

  // Enter installed device
  let mut chroot = Command::new("arch-chroot")
    .args(vec!["/mnt", "/install.sh"])
    .arg(&data.user.name).arg(&data.user.password)
    .arg(if is_legacy() { "true" } else { "false" })
    .arg(&partitions::slashdev(&data.device.name, 0))
    .spawn()
    .expect("FAILED");

  chroot.wait().expect("FAILED");
  
  return;
}

/// Function calls Sensitive device manipulations
/// 
/// Goes from nothing, to a fully partitioned device
/// 
/// # Arguments
/// 
///  `data` - The data coming from the frontend
///  `part_ingo` - The static partition configuration
///  `partitions_mb` - list of partitions sizes in bytes 
/// 
pub fn device_manipulation(
  data: &UserData,
  part_info: &Vec<PartData>,
  partitions_mb: &Vec<u128>,
  is_legacy: bool) 
{

  println!("\n --- Legacy: {} ---\n ", if is_legacy { "TRUE" } else { "FALSE" });
  
  // // // // // // // // // // // // //
  // -----  DEVICE MANIPULATION ----- //
  partitions::wipe_fs(&data.device.name);

  if is_legacy {
    partitions::make_mbr(
      partitions_mb,
      &partitions::slashdev(&data.device.name, 0)
    );
  } else {
    partitions::make_uefi(
      partitions_mb,
      &partitions::slashdev(&data.device.name, 0)
    );
  }
  
  partitions::make_fs(part_info, &data.device.name);
  partitions::mount(&data.device.name, data.ratio != 100.0);
  // // // // // // // // // // // // //
}

///
/// Detect if parent system booted in UEFI or Legacy mode
/// 
/// # Returns
///   Whether or not the system was booted in Legacy mode
/// 
pub fn is_legacy() -> bool {
  return !Path::new("/sys/firmware/efi").exists();
}

/// Install process
/// 
/// # Arguments
/// 
///   `data` - The data from the frontend
/// 
pub fn install<'a>(data: &UserData) {

  let partitions_mb: Vec<u128> = logic::calculate_partitions(
    data.device,
    data.swap as u128,
    (data.ratio/100.0) as f64,
    data.ratio != 100.0
  );

  let part_info:Vec<PartData> = get_partitions_fs(); 

  // Device manipulation
  device_manipulation(data, &part_info, &partitions_mb, is_legacy());

  // Get all specified packages
  let packages:Vec<String> = get_packages(data.packages).unwrap();
  
  // Install all packages
  pacstrap(packages.iter().map(|s| s as &str).collect())
    .expect("FAILED");
  
  // Generate fstab file 
  partitions::genfstab();

  copy_root()
    .expect("root copy failed");
  
  write_hostname(&data.hostname);

  // enable chroot script
  enable_install_script()
    .expect("Can't chmod install script");

  // Run chroot script
  chroot(&data);

  partitions::umount(&data.device.name)
    .expect("Couldn't unmount partitions");

  println!("\n--- THE DEVICE SUCCESSFULLY INSTALLED ---");
}
