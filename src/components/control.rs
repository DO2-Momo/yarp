use gtk::{EntryBuffer, Box};

pub struct FormData {
  pub username: EntryBuffer,
  pub hostname: EntryBuffer,
  pub password: EntryBuffer,
  pub cpassword: EntryBuffer,
  pub packages: PackageProfile
} 

pub struct Form {
  pub widget: Box,
  pub data: FormData
}


#[derive(Copy, Clone)]
pub struct PackageProfile {
  pub desktop: bool,
  pub multimedia: bool,
  pub utils: bool,
  pub nightly: bool,
  pub amd_gpu: bool,
  pub intel_gpu: bool
}

pub enum PackageBundle {
  BASE, DESKTOP, UTILS, NIGHTLY, MULTIMEDIA
}