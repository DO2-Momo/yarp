
use std::process::Command;
use std::str;

use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Partition {
  pub name: String,
  pub rm: bool,
  pub size: String,
  pub ro: bool,
  pub mountpoints: Vec<Option<String>>
}

#[derive(Serialize, Deserialize)]
pub struct Device {
  pub name: String,
  pub rm: bool,
  pub size: String,
  pub ro: bool,
  pub children: Option<Vec<Partition>>
}

#[derive(Serialize, Deserialize)]
pub struct Devices {
  pub blockdevices: Vec<Device>
}

pub fn get_devices() -> Devices {
  let raw_data = Command::new("lsblk")
    .arg("-J")
    .output()
    .expect("Failed to execute lsblk");
  
  let buf:   &[u8] = &raw_data.stdout;
  
  let fdata: &str  = str::from_utf8(&buf).unwrap();

  let data: Devices = serde_json::from_str(
    fdata
  ).unwrap();

  return data;
}
