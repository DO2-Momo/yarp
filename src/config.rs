use crate::sysinfo::Device;

pub struct User {
  pub name: String,
  pub password: String,
  pub cpassword: String
}

pub struct UserData<'a> {
  pub hostname: String,
  pub user: User,
  pub device: &'a Device,
}

pub struct PartData<'a> {
  pub fs: &'a str,
  pub args: Vec<&'a str>
}
