
use std::process::{Stdio, Command};
use std::process;


use crate::config::{UserData, PartData};
use crate::components::control::{PackageProfile};
use crate::sysinfo::Device;

use std::sync::mpsc::channel;
use ctrlc;

use std::str;
use std::fs;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use std::io::prelude::*;

pub mod logic;
pub mod partitions;

///
/// Partition FS Instructions
///
pub fn get_partitions_fs() -> Vec<PartData<'static>> {
  let mut part_info = vec![
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
      .spawn();

  chmod.expect("FAILED").wait();

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

  let out = etc_copy.wait().expect("Failed to copy");  
  
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
pub fn filterPackages(mut packages: Vec<String>) -> Vec<String>{
  let mut filtered_packages: Vec<String> = Vec::<String>::new();
  for i in 0..packages.len() {
    if (packages[i].len() == 0 ||
       packages[i].chars().nth(0).unwrap() == '#') {
      continue;
    }
    filtered_packages.push(String::from(packages[i].trim()));
  }

  return filtered_packages;
}

/// Read a package pack file, and add it to the mutable String reference
/// 
/// # Arguments
/// 
///  `content` - A mutable string reference containing the file raw text data
///  `pack_name` - The package pack file name
/// 
pub fn readPackagesFromFile(content: &mut String, pack_name: &str) -> String {
  let mut ans = String::new();

  let mut path: String = "./packages/".to_owned();
  path.push_str(pack_name); path.push_str(".x86_64");

  let mut file = fs::File::open(&path);
  file.expect("file not found").read_to_string(&mut ans);
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

  readPackagesFromFile(&mut content, "base");

  if params.multimedia == true {
    readPackagesFromFile(&mut content, "multimedia");
  }

  if params.nightly == true {
    readPackagesFromFile(&mut content, "nightly");
  }

  if params.desktop == true {
    readPackagesFromFile(&mut content, "desktop");
  }

  if params.utils == true {
    readPackagesFromFile(&mut content, "utils");
  }

  let mut split = content.split("\n");
  let mut ans: Vec<String> = split.collect::<Vec<&str>>()
    .iter()
    .map(|s| s.to_string())
    .collect();
  
  Ok(filterPackages(ans))
}

/// Spawn pacstrap
/// installing packages to mounted device
/// 
/// # Arguments
/// A list of packages
/// 
pub fn pacstrap(packages: Vec<&str>) {

  println!("PACKAGES: ");
  for i in 0..packages.len() {
    print!("{} ", packages[i])
  }

  let mut install_packages = Command::new("pacstrap")
    .arg("-K").arg("/mnt")
    .args(packages)
    .stdout(Stdio::inherit())
    .spawn()
    .expect("FAILED");

  install_packages.wait().expect("FAILED");

  return;
}

/// Change root to device's root and execute installation script
/// 
/// # Arguments
///  `data` - The form data from the frontend
///  `is_removable` - whether or not the system is removable
/// 
pub fn chroot(
  data: &UserData,
  is_removable: bool
) {

  // Enter installed device
  let chroot = Command::new("arch-chroot")
    .args(vec!["/mnt", "/install.sh"])
    .arg(&data.user.name).arg(&data.user.password).arg(&data.hostname)
    .spawn();

  chroot.expect("FAILED").wait();
  
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
  partitions_mb: &Vec<u64>) {

  // --- DEVICE MANIPULATION ---
  partitions::wipe_fs(&data.device.name);
  partitions::make(partitions_mb, &partitions::slashdev(&data.device.name, 0));
  partitions::make_fs(part_info, &data.device.name);
  partitions::mount(&data.device.name, data.ratio != 100.0);
  // // // // // // // // // // // 
}

/// Install process
/// 
/// # Arguments
/// 
///   `data` - The data from the frontend
/// 
pub fn install<'a>(data: &UserData) {

  let partitions_mb: Vec<u64> = logic::calculate_partitions(
    data.device,
    data.swap as u64,
    (data.ratio/100.0) as f32, 
    ((100.0-data.ratio)/100.0) as f32,
    data.ratio != 100.0
  );
  let part_info:Vec<PartData> = get_partitions_fs(); 

  device_manipulation(data, &part_info, &partitions_mb);

  // Get all specified packages
  let packages:Vec<String> = get_packages(data.packages).unwrap();
  
  // Install all packages
  pacstrap(packages.iter().map(|s| s as &str).collect());
  
  // Generate fstab file 
  partitions::genfstab();

  // Move user config
  copy_root();

  write_hostname(&data.hostname);

  // enable chroot script
  enable_install_script();

  // Run chroot script
  chroot(&data, true);

  partitions::umount(&data.device.name);

  println!("\n--- THE DEVICE SUCCESSFULLY INSTALLED ---");
}
