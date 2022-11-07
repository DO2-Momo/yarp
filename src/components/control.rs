use gtk::{EntryBuffer, Box};

pub struct FormData {
  pub username: EntryBuffer,
  pub hostname: EntryBuffer,
  pub password: EntryBuffer,
  pub cpassword: EntryBuffer,
} 

pub struct Form {
  pub widget: Box,
  pub data: FormData
}
