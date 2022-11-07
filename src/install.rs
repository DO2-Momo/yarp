use crate::config::UserData;
use crate::sysinfo::Device;

use std::process::Command;
use std::str;

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

  println!("TOTAL Size : {}", device.size);
  let mut sizes = Vec::<u64>::new();
  let size: u64 = toMB(device.size);
  println!("{}",size);

  sizes.push(0);
  sizes.push(300);
  sizes.push(2048 + sizes[sizes.len()-1]);
  sizes.push((root * (size - 2350) as f32) as u64 + sizes[sizes.len()-1]);
  sizes.push((home * (size - 2350) as f32) as u64 + sizes[sizes.len()-1]);

  for i in 0..4 {
    println!("{}", sizes[i].to_string());
  }

  println!("{}", size.to_string());

  println!("{}", sum(&sizes).to_string());

  return sizes;
}


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



pub fn install(data: &UserData) {
  println!(" Username : {}", data.user.name);
  println!(" Hostname : {}", data.hostname);
  println!(" Device : {}", data.device.name);
  println!(" Available Space : {}", data.device.size);

  println!("-- Installing System --");

  // Safety
  if data.device.name == "nvme0n1" {
    return;
  }; 

  let partitions_mb: Vec<u64> = calculate_partitions(data.device, 0.35, 0.65);

  let mut echo = Command::new("echo").arg("hello")
  .spawn().expect("FAILED").wait();

  let devname:&str = &slashdev!(&data.device.name);

  println!("Wiping {} with `wipefs -a {}`", devname, devname);
  let mut wipefs = Command::new("wipefs")
    .args(vec!["-a", devname])  
    .spawn();

  wipefs.expect("FAILED").wait();
    //.expect("Failed to launch hello world");
  // Set GPT label
  let mut gpt_label_cmd = Command::new("parted")
      .arg(devname)
      .arg("mklabel")
      .arg("gpt")
      .spawn()
      .unwrap();

    gpt_label_cmd.wait();
  for i in 0..(partitions_mb.len()-1) {
    println!("building partition {}, {} ", devname, i);
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

      parted.expect("FAILED").wait();
      //.expect("Failed to launch hello world");
  }

  let mut mkfs_boot = Command::new("mkfs.fat").arg("-F")
    .args(vec![
      "-F",
      "32",
      &slashdev!(&data.device.name, 1)
    ])  
    .spawn();

  mkfs_boot.expect("FAILED").wait();

  let mut mkswap = Command::new("mkswap").arg("-F")
    .arg(
      &slashdev!(&data.device.name, 2)
    )  
    .spawn();

    mkswap.expect("FAILED").wait();
  

  let mut mkfs_root = Command::new("mkfs.ext4").arg("-F")
    .arg(
      &slashdev!(&data.device.name, 3)
    )  
    .spawn();

  mkfs_root.expect("FAILED").wait();
  
  let mut mkfs_home = Command::new("mkfs.ext4").arg("-F")
    .arg(
      &slashdev!(&data.device.name, 4)
    )   
    .spawn();

  mkfs_home.expect("FAILED").wait();  

  println!("Done");
}