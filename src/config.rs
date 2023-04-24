use crate::sysinfo::Device;
use crate::components::control::PackageProfile;

pub struct User {
  pub name: String,
  pub password: String,
  pub cpassword: String
}

pub struct UserData<'a> {
  pub hostname: String,
  pub user: User,
  pub device: &'a Device,
  pub packages: PackageProfile,
  pub is_legacy: bool,
  pub swap: u64,
  pub root: u64,
  pub home: u64,
  pub ratio: f64

}

pub struct PartData<'a> {
  pub fs: &'a str,
  pub args: Vec<&'a str>
}
