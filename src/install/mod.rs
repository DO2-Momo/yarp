use std::process::{Stdio,Command};
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

// sda -> /dev/sda Macro
macro_rules! slashdev {
  ($a: expr) => {
    slashdev($a, 0)
  };

  ($a: expr, $b: expr) => {
    slashdev($a, $b)
  };
}

/**
 * Used for slashdev! macro
 */
pub fn slashdev(name: &str, id: u16) -> String {
  let mut devname = String::from("/dev/");
  devname.push_str(name);
  if id != 0 {
    devname.push_str(&id.to_string());
  }
  return devname;
}

/**
 *  Partition FS Instructions
 */
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

pub fn toMB(size: u128) -> u64 {
  return (size as u64) / 1000000; 
}

pub fn space_as_string(size: u64, unit: &str ) -> String {
  let mut ans = String::new();
  ans.push_str(&size.to_string());
  ans.push_str(unit);
  return ans;
}

/**
 * Calculate
 */
pub fn sum(arr: &Vec<u64>) -> u64 {
  let mut sum:u64 = 0;
  for i in 0..arr.len() {
    sum += arr[i];
  }
  return sum;
}

/**
 * Calculate partition sizes
 *
 * TODO: Refactor
 */
pub fn calculate_partitions(
  device: &Device,
  swap: u64,
  root: f32,
  home: f32,
  has_home:bool
) -> Vec<u64> {
  let mut sizes = Vec::<u64>::new();
  let size: u64 = toMB(device.size);
  let efi: u64 = 100;

  sizes.push(0);
  sizes.push(efi);
  sizes.push(swap + sizes[sizes.len()-1]);
  if !has_home {
    sizes.push(size - (swap + efi));
    return sizes;
  }
  
  sizes.push((root * (size - (swap + efi)) as f32) as u64 + sizes[sizes.len()-1]);
  sizes.push((home * (size - (swap + efi)) as f32) as u64 + sizes[sizes.len()-1]);

  return sizes;
}

pub fn make_partitions(partitions_mb: &Vec<u64>, devname: &str) {
  // Set GPT label
  let mut parted_label = Command::new("parted")
  .arg(devname)
  .arg("mklabel")
  .arg("gpt")
  .spawn()
  .unwrap();

  // Wait
  parted_label.wait();

  // Make partitions
  for i in 0..(partitions_mb.len()-1) {
    // Launch parted
    let mut parted = Command::new("parted")
      .args(vec![
        "-s",
        "-a",
        "optimal",
        devname,
        "mkpart",
        "primary",
        &space_as_string(partitions_mb[i]+1, "MB"),
        &space_as_string(partitions_mb[i+1]+1, "MB")
      ])  
      .spawn();
      
      // Wait
      parted.expect("FAILED").wait();
  }
}

pub fn make_filesystem(part_info: &Vec<PartData>, device_name: &str) {
  // Make file systems according to part_info
  for i in 0..part_info.len()-1 {
    let mkfs = Command::new(part_info[i].fs)
    .args(&part_info[i].args)
    .arg(&slashdev!(device_name, i as u16 +1))  
    .spawn();

    mkfs.expect("FAILED").wait();
  }
}

pub fn wipe_fs(name: &str) {
  let umount = Command::new("umount")
    .arg("-Rf").arg("/mnt")
    .spawn();

  umount.expect("FAILED").wait();
  
  let umount = Command::new("swapoff")
    .arg(&slashdev!(name, 2))
    .spawn();

  umount.expect("FAILED").wait();
  // Launch
  let wipefs = Command::new("wipefs")
    .args(vec!["--all", "--force", &slashdev!(name)])  
    .spawn();

  // Wait
  wipefs.expect("FAILED").wait();

  println!("--- CLEARED DRIVE, FORMATING PARTITIONGS... ---")
}

/**
 *
 */
pub fn mount_part(devname: &str, has_home:bool) {

  let mount_root = Command::new("mount")
    .arg(slashdev!(devname, 3))
    .arg("/mnt")
    .spawn();

  mount_root.expect("FAILED").wait();

  let swapon = Command::new("swapon")
  .arg(slashdev!(devname, 2))
  .spawn();

  swapon.expect("FAILED").wait();

  let mount_boot = Command::new("mount")
  .arg("--mkdir").arg(slashdev!(devname, 1))
  .arg("/mnt/boot/efi")
  .spawn();

  mount_boot.expect("FAILED").wait();

  if has_home  {

    let mount_home = Command::new("mount")
    .arg("--mkdir").arg(slashdev!(devname, 4))
    .arg("/mnt/home")
    .spawn();

    mount_home.expect("FAILED").wait();
  }

}

pub fn genfstab() {
  // Mount home directory
  let genfstab_cmd = Command::new("genfstab")
      .arg("-U")    
      .arg("-p")
      .arg("/mnt")  
      .output()
      .expect("failed to generate fstab file");

  let output = String::from_utf8(genfstab_cmd.stdout).unwrap();
  std::fs::write("/mnt/etc/fstab", &output).expect("Failed to write file");
} 

pub fn move_script() -> std::io::Result<()> {
  let mut cp = Command::new("cp")
  .args(vec![
    "./root/install.sh",
    "/mnt/install"
  ])
  .spawn();

  let out = cp.expect("failed").wait();

  let mut chmod = Command::new("chmod")
      .arg("+x")
      .arg("/mnt/install")
      .spawn();

  chmod.expect("failed").wait();

  Ok(())
}

pub fn move_user_config() -> std::io::Result<()> {

  let mut etc_copy = Command::new("cp").arg("-r")
  .args(vec![
    "./root/etc/*",
    "/mnt/etc/"
  ])
  .spawn()
  .expect("failed");

  let out = etc_copy.wait().expect("Failed to copy");  

  let mut cp_system_wide_config = Command::new("cp").arg("-r")
  .args(vec![
    "./root/etc/skel/.config",
    "/mnt/etc/*"
  ])
  .spawn()
  .expect("failed to execute");

  cp_system_wide_config.wait().expect("Failed to copy");
  
  Ok(())
}

/**
 * Remove empty lines in package files,
 * and remove comments
 */
pub fn filterPackages(mut packages: Vec<String>) -> Vec<String>{
  let mut filtered_packages: Vec<String> = Vec::<String>::new();
  for i in 0..packages.len() {
    if (packages[i].len() == 0 ||
       packages[i].chars().nth(0).unwrap() == '#') {
      continue;
    }
    filtered_packages.push(String::from(&packages[i]));
  }

  return filtered_packages;
}

/**
 * Get package names from files
 */
pub fn get_packages(params: PackageProfile) -> std::io::Result<Vec<String>> {

  let mut ans: Vec<String> = Vec::<String>::new();
  let mut content = String::new();
  let mut tmp_content = String::new();

  tmp_content = String::new();
  let mut file = fs::File::open("./packages/base.x86_64")?;
  file.read_to_string(&mut tmp_content)?;
  content.push_str(&tmp_content);
  content.push_str("\n");

  if params.multimedia == true {
    tmp_content = String::new();
    file = fs::File::open("./packages/multimedia.x86_64")?;
    file.read_to_string(&mut tmp_content)?;
    content.push_str(&tmp_content);
    content.push_str("\n");
  }

  if params.nightly == true {
    tmp_content = String::new();
    file = fs::File::open("./packages/nightly.x86_64")?;
    file.read_to_string(&mut tmp_content)?;
    content.push_str(&tmp_content);
    content.push_str("\n");
  }

  if params.desktop == true {
    tmp_content = String::new();
    file = fs::File::open("./packages/desktop.x86_64")?;
    file.read_to_string(&mut tmp_content)?;
    content.push_str(&tmp_content);
    content.push_str("\n");
  }

  if params.utils == true {
    tmp_content = String::new();
    file = fs::File::open("./packages/utils.x86_64")?;
    file.read_to_string(&mut tmp_content)?;
    content.push_str(&tmp_content);
    content.push_str("\n");
  }

  let mut split = content.split("\n");
  ans = split.collect::<Vec<&str>>()
    .iter()
    .map(|s| s.to_string())
    .collect();
  
  Ok(filterPackages(ans))
}

/**
 * Spawn pacstrap
 * installing packages to mounted device
 */
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
    .expect("Failed to spawn pacstrap");

  install_packages.wait().expect("Failed to complete package installation");

  println!(" --- PACSTRAP COMPLETED ---");

  return;
}

/**
 * Change root to device's root and execute installation script
 */
pub fn chroot(
  data: &UserData,
  is_removable: bool
) {

  // Enter installed device
  let chroot = Command::new("arch-chroot")
    .args(vec!["/mnt", "/install"])
    .arg(&data.user.name).arg(&data.user.password).arg(&data.hostname)
    .spawn();

  chroot.expect("failed").wait();
  
  return;
}

/**
 * Copy files from skel to user's home
 */
pub fn init_home() {
  // Transfer /etc/skel
  let mut cp_skel = Command::new("cp")
  .arg("-r")
  .args(vec![
    "./root/etc/skel/",
    "/mnt/etc/*"
  ])
  .spawn()
  .expect("Cant execute cp");

  cp_skel.wait().expect("Cant copy files");
}

pub fn exit_handler(critical: bool) {
  if critical {
    println!("Can't exit during critical procedure");
  } else {
    process::exit(0x00);
  }
}

pub fn device_manipulation(data: &UserData, part_info: &Vec<PartData>, partitions_mb: &Vec<u64>) {
  // --- DEVICE MANIPULATION ---
  wipe_fs(&data.device.name);
  make_partitions(partitions_mb, &slashdev!(&data.device.name));
  make_filesystem(part_info, &data.device.name);
  mount_part(&data.device.name, data.ratio != 100.0);

  move_script();
  // // // // // // // // // // // 
}

/**
 * Install process
 */
pub fn install<'a>(data: &UserData) {

  let partitions_mb: Vec<u64> = calculate_partitions(
    data.device,
    data.swap as u64,
    (data.ratio/100.0) as f32, 
    ((100.0-data.ratio)/100.0) as f32,
    data.ratio != 100.0
  );
  let part_info:Vec<PartData> = get_partitions_fs(); 

  // Safe guard for exiting during device manipulation
  ctrlc::set_handler(move || println!("Can't exit during device manipulation"))
      .expect("Could not send signal on channel.");

  device_manipulation(data, &part_info, &partitions_mb);

  // Get all specified packages
  let packages:Vec<String> = get_packages(data.packages).unwrap();
  let pack:Vec<&str> = 
    packages.iter().map(|s| s as &str).collect();
  
  // Install all packages
  pacstrap(pack);
  
  // Generate fstab file 
  genfstab();

  // Move user config
  move_user_config();

  let mut critical = true;


  
  // Run chroot script
  chroot(&data, true);

  // Initiate files in home
  init_home();

  // unmount all device partitions
  let mut umount = Command::new("umount")
    .arg("-R").arg("/mnt")
    .spawn()
    .expect("FAILED");

  umount.wait().expect("Can't mount");

  println!("--- THE DEVICE SUCCESSFULLY INSTALLED ---");
}
