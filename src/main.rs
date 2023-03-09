mod sysinfo;
mod components;
mod validation;
mod install;
mod config;
use crate::config::{User, UserData};
use crate::sysinfo::{Devices, Partition};
use crate::components::prompt_user;
use crate::components::control::{PackageProfile};

use crate::install::partitions::slashdev;

use std::rc::Rc;
use std::cell::Cell;

use std::{str};
use gtk::gdk::Display;
use gtk::prelude::*;
use gtk::gio::{ListModel, SimpleAction};
use gtk::glib;
use glib_macros::clone;
use gtk::{
  DropDown,
  Button,
  Box, 
  Orientation,
  Application,
  ApplicationWindow,
  Label,
  CheckButton,
  StyleContext,
  TextView,
  EntryBuffer,
  CssProvider,
  Align,
  TextBuffer,
  PositionType,
  Scale,
  InputPurpose,
  Justification,
  Entry
};


const APP_ID: &str = "org.gtk_rs.yarp";

/**
 * Load CSS Stylesheet
 */
fn load_css() {
  // Load the CSS file and add it to the provider
  let provider = CssProvider::new();
  provider.load_from_data(include_bytes!("style.css"));

  // Add the provider to the default screen
  StyleContext::add_provider_for_display(
      &Display::default().expect("Could not connect to a display."),
      &provider,
      gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
  );
}

fn get_device_labels(device_data: &Devices) -> Vec<String> {
  let mut device_names: Vec<String> = Vec::<String>::new();
  // Format to "name  size"
  for i in 0..device_data.blockdevices.len() {
    // Convert to GB
    
    // TODO: Clean this math up
    let size_gb:f32 = (( device_data.blockdevices[i].size / 10000000 ) as f32 ) / 100 as f32;

    // Push label
    device_names.push(components::getLabel(
      &device_data.blockdevices[i].name,
      &size_gb.to_string()
    ));
  }

  return device_names;
}

/**
 * Get size of a device
 */
fn get_device_sizes(devices: &sysinfo::Devices) -> Vec<u128> {
  let mut ans = Vec::<u128>::new();

  for i in 0..devices.blockdevices.len() {
    ans.push(devices.blockdevices[i].size + 0);
  }

  return ans;
}

/**
 * Build GTK GUI
 */
fn build_ui(app: &Application) {
  // Get devices data
  let mut device_data: sysinfo::Devices = sysinfo::get_devices();
  let device_data_sizes: Vec<u128> = get_device_sizes(&device_data);

  static mut device_select: usize = 0;

  // Create a button with label and margins
  let confirm_button = Button::builder()
    .label("Confirm")
    .build();

  let main_box = Box::builder()
    .orientation(Orientation::Vertical)
    .hexpand(true)
    .vexpand(true)
    .css_classes(vec![String::from("main"),String::from("box")])
    .build();


  let flex = Box::builder()
    .orientation(Orientation::Horizontal)
    .hexpand(true)
    .vexpand(true)
    .css_classes(vec![String::from("flex-div")])
    .build();
  

  // Create a window
  let window = ApplicationWindow::builder()
    .application(app)
    .default_width(800).default_height(600)
    .title("SYSTEM: yarp installer")
    .child(&main_box)
    .build();

  // Get device labels
  let device_labels_dyn: Vec<String> = get_device_labels(&device_data);
  let device_labels: Vec<&str> = device_labels_dyn.iter().map(|s| s as &str).collect();

  // Package manager bundles
  let chk_desktop:CheckButton    = CheckButton::with_label("Desktop Environement");
  let chk_utils:CheckButton      = CheckButton::with_label("Desktop Features");
  let chk_multimedia:CheckButton = CheckButton::with_label("Multimedia Utils");
  let chk_nightly:CheckButton    = CheckButton::with_label("Nightly pack");

  let package_profile_box = components::get_package_profile_box(
    &chk_desktop, &chk_utils, &chk_multimedia, &chk_nightly
  );

  // Partitions
  let part_scale = Scale::builder()
    .orientation(Orientation::Horizontal)
    .width_request(275)
    .height_request(80)
    .value_pos(PositionType::Top)
    .has_origin(true)
    .can_target(true)
    .fill_level(100.0)
    .vexpand(false)
    .build();

  let swap_scale = Scale::builder()
    .orientation(Orientation::Horizontal)
    .width_request(275)
    .height_request(80)
    .value_pos(PositionType::Top)
    .has_origin(true)
    .can_target(true)
    .fill_level(16384.0)
    .vexpand(false)
    .build();

  let root_home_ratio_label = Label::builder()
    .label("Root / Home")
    .justify(Justification::Left)
    .xalign(0.0)
    .css_classes(vec![String::from("section-title-2")])
    .build();

  let partition_box = components::get_partition_box(
    &swap_scale,
    &part_scale,
    &root_home_ratio_label
  );
  
  // Right Container
  let right_box = Box::builder()
    .orientation(Orientation::Vertical)
    .valign(Align::Start)
    .hexpand(true)
    .vexpand(true)
    .css_classes(vec![String::from("right-box")])
    .width_request(400)
    .build();

  right_box.append(&partition_box);
  right_box.append(&package_profile_box);

  let form = components::form("Host & Login:");

  let device_menu_label = Label::builder()
    .label("Device:")
    .justify(Justification::Left)
    .xalign(0.0)
    .css_classes(vec![String::from("section-title")])
    .build();

  // Create dropdown menu with the labels
  let device_menu = DropDown::from_strings(&device_labels);

  let device_box = Box::builder()
    .css_classes(vec![String::from("dropdown")])
    .build();

  device_box.append(&device_menu_label);
  device_box.append(&device_menu);

  let original_state:i32 = 0;
  let set_root_size = SimpleAction::new_stateful(
    "set_root_size",
    Some(&i32::static_variant_type()),
    &original_state.to_variant(),
  );

  // Connect to "clicked" signal of `confirm_button`
  confirm_button.connect_clicked(move |confirm_button| {

    // TODO: Implement validation prompt 
    // prompt_user(&form.data);

    // Format all form data to installation data
    let name: String = form.data.username.text();  
    let password: String = form.data.password.text();
    let cpassword: String = form.data.cpassword.text();
    let hostname: String = form.data.hostname.text();

    let device: &sysinfo::Device =
    device_data.get(device_menu.selected() as usize);

    let user = User { name, password, cpassword };

    let packages: PackageProfile = PackageProfile {
      desktop:    chk_desktop.is_active(),
      utils:      chk_utils.is_active(),
      multimedia: chk_multimedia.is_active(),
      nightly:    chk_nightly.is_active()
    };

    let space_size:u64 = ((device.size/1000000) as u64)  - swap_scale.value() as u64 - 300;
    println!("space_size {}", space_size);
    let userData = UserData {
      user, hostname, 
      device: device,
      packages: packages,
      swap: swap_scale.value() as u64,
      ratio: part_scale.value(),

      // TODO: Clean this math up
      root: ((part_scale.value()/100.0) * space_size as f64) as u64,
      home: (((100.0 - part_scale.value())/100.0) * space_size as f64) as u64 
    };

    // Validate configuration & Start installation if is valid
    validation::validate_config(&userData);
  });

  // Add to main continer 

  main_box.append(&device_box);
  flex.append(&form.widget);
  flex.append(&right_box);
  main_box.append(&flex);

  main_box.append(&confirm_button);
  window.add_action(&set_root_size);

  // Present window
  window.present();

  return;

}

/**
 * MAIN
 */
fn main() {
  // Create a new application
  let app = Application::builder().application_id(APP_ID).build();

  // Connect to "activate" signal of `app`
  app.connect_startup(|_| load_css());
  app.connect_activate(build_ui);

  // Run the application
  app.run();
}