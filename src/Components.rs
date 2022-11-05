use gtk::{
  Box,
  Orientation,
  Label
};
use gtk::prelude::*;

pub fn DeviceRow(name: &str, size: &str) -> Box {
  let device_row = Box::builder()
  .orientation(Orientation::Horizontal)
  .name("device_button")
  .build();
  
  BoxExt::append(
    &device_row,
    &Label::with_mnemonic(name)
  );

  BoxExt::append(
    &device_row,
    &Label::with_mnemonic(size)
  );

  return device_row;
}