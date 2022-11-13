use crate::config::{UserData, PartData};
use crate::components::control::{PackageProfile};
use std::process::{Stdio,Command};
use crate::sysinfo::Device;
use ctrlc;
use std::str;
use std::fs;

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

pub fn slashdev(name: &str, id: u16) -> String {
  let mut devname = String::from("/dev/");
  devname.push_str(name);
  if id != 0 {
    devname.push_str(&id.to_string());
  }
  return devname;
}

pub fn getPartFsInfo() -> Vec<PartData<'static>> {
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

pub fn sum(arr: &Vec<u64>) -> u64 {
  let mut sum:u64 = 0;
  for i in 0..arr.len() {
    sum += arr[i];
  }
  return sum;
}

pub fn calculate_partitions(
  device: &Device,
  root: f32,
  home: f32,
  has_home:bool
) -> Vec<u64> {
  let mut sizes = Vec::<u64>::new();
  let size: u64 = toMB(device.size);

  sizes.push(0);
  sizes.push(300);
  sizes.push(2048 + sizes[sizes.len()-1]);
  if !has_home {
    sizes.push(size-2350);
    return sizes;
  }
  
  sizes.push((root * (size - 2350) as f32) as u64 + sizes[sizes.len()-1]);
  sizes.push((home * (size - 2350) as f32) as u64 + sizes[sizes.len()-1]);

  return sizes;
}

pub fn make_partitions(partitions_mb: Vec<u64>, devname: &str) {
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

pub fn make_filesystem(part_info: Vec<PartData>, device_name: &str) {
  // Make file systems according to part_info
  for i in 0..part_info.len()-1 {
    let mkfs = Command::new(part_info[i].fs)
    .args(&part_info[i].args)
    .arg(&slashdev!(device_name, i as u16 +1))  
    .spawn();

    mkfs.expect("FAILED").wait();
  }
}

pub fn wipe_fs(devname: &str) {
  let umount = Command::new("umount")
    .arg("-Rf").arg("/mnt")
    .spawn();

  umount.expect("FAILED").wait();
  // Launch
  let wipefs = Command::new("wipefs")
    .args(vec!["-af", devname])  
    .spawn();

  // Wait
  wipefs.expect("FAILED").wait();

  println!("--- CLEARED DRIVE, FORMATING PARTITIONGS... ---")
}

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

  let mut os_release = Command::new("cp").arg("-r")
  .args(vec![
    "./root/etc/os-release",
    "/mnt/etc/os-release"
  ])
  .spawn();

  let out = os_release.expect("failed").wait();
  

  let mut sudoers = Command::new("cp").arg("-r")
  .args(vec![
    "./root/etc/sudoers",
    "/mnt/etc/sudoers"
  ])
  .spawn();

  let out = sudoers.expect("failed").wait();

  

  let mut cp_skel = Command::new("cp").arg("-r")
  .args(vec![
    "./root/etc/skel",
    "/mnt/etc/"
  ])
  .spawn();

  let out = cp_skel.expect("failed").wait();

  

  let mut cp_system_wide = Command::new("cp").arg("-r")
  .args(vec![
    "./root/etc/skel/.config",
    "/mnt/etc/*"
  ])
  .spawn();

  let out = cp_system_wide.expect("failed").wait();

  let mut cp_lightdm = Command::new("cp").arg("-r")
  .args(vec![
    "./root/etc/lightdm",
    "/mnt/etc/"
  ])
  .spawn();

  let out = cp_lightdm.expect("failed").wait();

  
  let mut rm_backgrounds = Command::new("rm").arg("-rf")
  .arg(
    "/mnt/usr/share/backgrounds/xfce"
  )
  .spawn();

  let out = rm_backgrounds.expect("failed").wait();

  let mut cp_lightdm = Command::new("cp").arg("-r")
  .args(vec![
    "./root/usr/share/backgrounds/xfce",
    "/mnt/usr/share/backgrounds/"
  ])
  .spawn();

  let out = cp_lightdm.expect("failed").wait();

  
  Ok(())
}

pub fn filterPackages(mut packages: Vec<String>) -> Vec<String>{
  for i in 0..packages.len() {
    if packages[i].len() == 0 {
      packages.remove(i);
    }
  }

  return packages;
}

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



pub fn pacstrap(packages: Vec<&str>) {

  for i in 0..packages.len() {
    println!("PACKAGES: {}", packages[i])
  }

  println!(" --- PACSTRAP ---");

  let install_packages = Command::new("pacstrap")
    .arg("/mnt")
    .args(packages)
    .stdout(Stdio::inherit())
    .spawn();

  install_packages.expect("failed").wait();

  println!(" --- PACSTRAP END ---");

  return;
}

pub fn grub_install(is_removable: bool) -> String {

  let mut optional:Vec<&str> = vec!["--recheck"];
  if is_removable { optional.push("--removable") };

  let mut grub_install_cmd = 
      String::from(
        "grub-install --target=x86_64-efi --efi-directory=/boot/efi --bootloader-id=ARCH"
      );

  for option in optional {
      grub_install_cmd.push(' ');
      grub_install_cmd.push_str(option);
  }

  return grub_install_cmd;
}

pub fn chroot(
  data: &UserData,
  is_removable: bool
) {

  // Disable quitting
  ctrlc::set_handler(move || {
      println!("Can't abort install");
  })
  .expect("failed to set ctrlc handler");

  let chroot = Command::new("arch-chroot")
    .args(vec!["/mnt", "/install"])
    .arg(&data.user.name).arg(&data.user.password).arg(&data.hostname)
    .spawn();

  chroot.expect("failed").wait();
  
  return;
}

pub fn install<'a>(data: &UserData) {

  let partitions_mb: Vec<u64> = calculate_partitions(data.device, 0.7, 0.3, false);
  let part_info:Vec<PartData> = getPartFsInfo(); 
  let devname:&str = &slashdev!(&data.device.name); // Ex: /dev/sdX

  // --- DEVICE MANIPULATION ---
  wipe_fs(devname);
  make_partitions(partitions_mb, devname);
  make_filesystem(part_info, &data.device.name);
  mount_part(&data.device.name, false);

  move_script();
  // // // // // // // // // // // 

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
  
  // Run chroot script
  chroot(&data, true);


  // Transfer /etc/skel
  let cp_skel = Command::new("cp").arg("-r")
  .args(vec![
    "./root/etc/skel/",
    "/mnt/etc/*"
  ])
  .spawn();
  cp_skel.expect("failed").wait();

  // umount device
  let umount = Command::new("umount")
    .arg("-R").arg("/mnt")
    .spawn();

  umount.expect("FAILED").wait();


  println!("--- THE DEVICE SUCCESSFULLY INSTALLED ---");
}
