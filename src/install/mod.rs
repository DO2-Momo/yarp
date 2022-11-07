use crate::config::{UserData, PartData};
use crate::sysinfo::Device;

use beach;
use std::process::{Stdio,Command};
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
  home: f32
) -> Vec<u64> {
  let mut sizes = Vec::<u64>::new();
  let size: u64 = toMB(device.size);

  sizes.push(0);
  sizes.push(300);
  sizes.push(2048 + sizes[sizes.len()-1]);
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
  // Launch
  let wipefs = Command::new("wipefs")
    .args(vec!["-a", devname])  
    .spawn();

  // Wait
  wipefs.expect("FAILED").wait();
}

pub fn mount_part(devname: &str) {
  let mount_root = Command::new("mount")
    .arg(slashdev!(devname, 3))
    .arg("/mnt")
    .spawn();

  mount_root.expect("FAILED").wait();

  let mkdir_home = Command::new("mkdir")
  .arg("-p")
  .arg("/mnt/home")
  .spawn();

  mkdir_home.expect("FAILED").wait();

  let mkdir_boot = Command::new("mkdir")
  .arg("-p")
  .arg("/mnt/boot/efi")
  .spawn();

  mkdir_boot.expect("FAILED").wait();

  let mount_boot = Command::new("mount")
  .arg(slashdev!(devname, 1))
  .arg("/mnt/boot/efi")
  .spawn();

  mount_boot.expect("FAILED").wait();

  let mount_home = Command::new("mount")
  .arg(slashdev!(devname, 4))
  .arg("/mnt/home")
  .spawn();

  mount_home.expect("FAILED").wait();

  // let swapon = Command::new("swapon")
  // .arg(slashdev!(devname, 2))
  // .spawn();

  // swapon.expect("FAILED").wait();
}


pub fn genfstab() {
  // Mount home directory
  let genfstab_cmd = Command::new("genfstab")
      .arg("-U")    
      .arg("-p")
      .arg("/mnt")  
      .stdout(Stdio::piped())
      .output()
      .expect("failed to generate fstab file");

  let output = String::from_utf8(genfstab_cmd.stdout).unwrap();
  std::fs::write("/mnt/etc/fstab", &output).expect("Failed to write file");
} 


pub fn getPackages() -> std::io::Result<String> {
  // Mount home directory
  let mut file = fs::File::open("./packages/packages.x86_64")?;
  let mut content = String::new();
  file.read_to_string(&mut content)?;

  Ok(content)
}

pub fn pacstrap(packages: String) {
  let mut split = packages.split("\n");
  let packages: Vec<&str> = split.collect();

  for i in 0..packages.len() {
    println!("{}",packages[i]);
  }

  let install_packages = Command::new("pacstrap")
      .arg("/mnt")  
      .args(packages)  
      .spawn()
      .expect("failed to generate fstab file").wait();
}

pub fn grub_install(is_removable: bool) -> String {

  let mut optional:Vec<&str> = vec!["--recheck"];
  if is_removable { optional.push("--removable") };

  let mut grub_install_cmd = 
      String::from("grub-install --target=x86_64-efi --efi-directory=/boot/efi --bootloader-id=ARCH");

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

      .spawn()
      .expect("failed");

  chmod.wait();

  Command::new("arch-chroot")
    .args(vec!["/mnt", "/install"])
    //.arg("\"mount /dev/sda1 /boot/efi; /install\"")
    .arg(&data.user.name).arg(&data.user.password)
    .spawn();

  
  return;
}

pub fn install(data: &UserData) {

  let partitions_mb: Vec<u64> = calculate_partitions(data.device, 0.7, 0.3);
  let part_info:Vec<PartData> = getPartFsInfo(); 
  let devname:&str = &slashdev!(&data.device.name); // Ex: /dev/sdX

  // --- DEVICE MANIPULATION ---
  wipe_fs(devname);
  make_partitions(partitions_mb, devname);
  make_filesystem(part_info, &data.device.name);
  mount_part(&data.device.name);
  // // // // // // // // // // // 

  let packages:String = getPackages().unwrap();
  pacstrap(packages);
  genfstab();
  chroot(&data, true);
}