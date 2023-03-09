
use std::process::{Command};
use crate::config::PartData;

// sda -> /dev/sda Macro
macro_rules! slashdev {
    ($a: expr) => {
        slashdev($a, 0)
    };

    ($a: expr, $b: expr) => {
        slashdev($a, $b)
    };
}

/// Generate an fstab file in /etc/fstab 
/// 
pub fn genfstab() {
    // Mount home directory
    let genfstab_cmd = Command::new("genfstab")
        .arg("-U")    
        .arg("-p")
        .arg("/mnt")  
        .output()
        .expect("failed to generate fstab file");
  
    let output = String::from_utf8(genfstab_cmd.stdout)
                        .unwrap();
  
    // Write command output
    std::fs::write("/mnt/etc/fstab", &output)
          .expect("Failed to write file");
} 

///
/// 
/// 
pub fn make(partitions_mb: &Vec<u64>, devname: &str) {
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

///
/// 
/// 
pub fn make_fs(part_info: &Vec<PartData>, device_name: &str) {
    // Make file systems according to part_info
    for i in 0..part_info.len() {
        println!("{}", &slashdev!(device_name, (i + 1) as u8));

        let mut mkfs = Command::new(part_info[i].fs)
        .args(&part_info[i].args)
        .arg(&slashdev!(device_name, (i + 1) as u8))  
        .spawn()
        .expect("FAILED");

        mkfs.wait().expect("FAILED");
    }
}

/// Wipe all fs signatures on a device
/// 
/// # Arguments
///
///  `name` - The name of the device. ex: (sda, hda, sdb, ...)
///
pub fn wipe_fs(name: &str) {
    let mut umount = Command::new("umount")
      .arg("-l").arg("/mnt")
      .spawn()
      .expect("FAILED");
  
    umount.wait().expect("FAILED");
    
    let mut umount = Command::new("swapoff")
      .arg(&slashdev!(name, 2))
      .spawn()
      .expect("FAILED");
  
    umount.wait().expect("FAILED");
  
    // Launch
    let mut wipefs = Command::new("wipefs")
      .args(vec!["--all", "--force", &slashdev!(name)])  
      .spawn()
      .expect("FAILED");
  
    // Wait
    wipefs.wait().expect("FAILED");
  
    println!("--- CLEARED DRIVE, FORMATING PARTITIONGS... ---")
}

///
/// 
/// 
pub fn umount(devname: &str) -> std::io::Result<()> {
    let mut umount = Command::new("umount")
    .arg("-l").arg("/mnt")
    .spawn()
    .expect("FAILED");

    umount.wait().expect("FAILED");

    // unmount all device partitions
    let mut umount = Command::new("swapoff")
        .arg(&slashdev!(devname, 2))
        .spawn()
        .expect("FAILED");

    umount.wait().expect("Can't unmount");

    // unmount all device partitions
    let mut umount = Command::new("umount")
        .arg("-Rf").arg("/mnt")
        .spawn()
        .expect("FAILED");

    umount.wait().expect("Can't unmount");

    Ok(())
}

/// Mount all paritions
/// 
/// # Arguments
///
/// `devname` - The name of the device. ex: (sda, hda, sdb, ...)
///
/// `has_home` - Whether or not to mount a home directorys
///
pub fn mount(devname: &str, has_home:bool) {

    // Mount root
    let mut mount_root = Command::new("mount")
      .arg(slashdev!(devname, 3))
      .arg("/mnt")
      .spawn()
      .expect("FAILED");
  
    mount_root.wait().expect("FAILED");
  
    // Mount swap 
    let mut swapon = Command::new("swapon")
    .arg(slashdev!(devname, 2))
    .spawn()
    .expect("FAILED");
  
    swapon.wait().expect("FAILED");
  
    // Mount boot
    let mut mount_boot = Command::new("mount")
    .arg("--mkdir").arg(slashdev!(devname, 1))
    .arg("/mnt/boot/efi")
    .spawn()
    .expect("FAILED");
  
    mount_boot.wait().expect("FAILED");
  
    // Mount home parition if exists
    if has_home  {
      let mut mount_home = Command::new("mount")
      .arg("--mkdir").arg(slashdev!(devname, 4))
      .arg("/mnt/home")
      .spawn()
      .expect("FAILED");
  
      mount_home.wait().expect("FAILED");
    }  
}
  


///
/// Used for slashdev! macro
///
pub fn slashdev(name: &str, id: u8) -> String {
    let mut devname = String::from("/dev/");
    devname.push_str(name);
    if id != 0 {
        devname.push_str(&id.to_string());
    }
    return devname;
}


pub fn space_as_string(size: u64, unit: &str ) -> String {
    let mut ans = String::new();
    ans.push_str(&size.to_string());
    ans.push_str(unit);
    return ans;
}