mod sysinfo;
mod components;
mod validation;
mod install;
mod config;

use config::{User, UserData};
use crate::components::prompt_user;

use std::rc::Rc;
use std::cell::RefCell;
use std::{str};
use gtk::gdk::Display;
use gtk::prelude::*;
use gtk::{
  DropDown,
  Button,
  Application,
  ApplicationWindow,
  Box, 
  Orientation,
  Label,
  Text,
  StyleContext,
  CssProvider
};

const APP_ID: &str = "org.gtk_rs.yarp";

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

fn main() {
  // Create a new application
  let app = Application::builder().application_id(APP_ID).build();

  // Connect to "activate" signal of `app`
  app.connect_startup(|_| load_css());
  app.connect_activate(build_ui);

  // Run the application
  app.run();
}


fn build_ui(app: &Application) {


  // Get devices data
  let device_data: sysinfo::Devices = sysinfo::get_devices();

  // Create a button with label and margins
  let button = Button::builder()
    .label("Confirm")
    .build();

  let main_box = Box::builder()
    .orientation(Orientation::Vertical)
    .css_classes(vec![String::from("box")])
    .build();


  // Create a window
  let window = ApplicationWindow::builder()
    .application(app)
    .default_width(800).default_height(600)
    .title("SYSTEM: yarp installer")
    .child(&main_box)
    .build();

  let mut device_names: Vec<String> = Vec::<String>::new();
  // Format to "name  size"
  for i in 0..device_data.blockdevices.len() {
    // Convert to GB
    let size_gb:f32 = (( device_data.blockdevices[i].size / 10000000 ) as f32 ) / 100 as f32;
    // Push label
    device_names.push(components::getLabel(
      &device_data.blockdevices[i].name,
      &size_gb.to_string()
    ));
  }
  // // to &str vector
  let device_labels: Vec<&str> = device_names.iter().map(|s| s as &str).collect();
  // Create dropdown menu
  let device_menu = DropDown::from_strings(&device_labels);

  let device_box = Box::builder()
      .css_classes(vec![String::from("dropdown")])
      .build();

  device_box.append(&device_menu);

  let form = components::form();

  // Connect to "clicked" signal of `button`
  button.connect_clicked(move |button| {

    prompt_user(&form.data);

    // Format all form data to installation data
    let name: String = form.data.username.text();  
    let password: String = form.data.password.text();
    let cpassword: String = form.data.cpassword.text();
    let hostname: String = form.data.hostname.text();

    let device: sysinfo::Device =
     device_data.blockdevices[device_menu.selected() as usize].clone();


    let user = User {
      name, password, cpassword
    };

    let userData = UserData {
      user, hostname, 
      device: &device
    };

    // Validate configuration & Start installation if is valid
    validation::validate_config(&userData);

  });


  // Add to main continer 
  main_box.append(&device_box);
  main_box.append(&form.widget);
  main_box.append(&button);

  // Present window
  window.present();
}
