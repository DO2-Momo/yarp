use gtk::prelude::*;
use gtk::{
  DropDown,
  Button,
  Application,
  ApplicationWindow,
  Box, 
  Orientation,
  Label,
  StyleContext,
  CssProvider
};
use gtk::gdk::Display;
use std::str;
mod Sysinfo;
mod Components;

const APP_ID: &str = "org.gtk_rs.yarp";

fn getLabel(name: &str, size: &str) -> String {
  let mut buf: Vec<&str> = Vec::<&str>::new();
  buf.push(name);
  buf.push(size);

  return buf.join("  ");
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

fn build_ui(app: &Application) {

  // Get devices data
  let device_data: Sysinfo::Devices = Sysinfo::get_devices();
  
  // Format to "name  size"
  let mut device_names: Vec<String> = Vec::<String>::new();
  for i in 0..device_data.blockdevices.len() {
    device_names.push(getLabel(
      &device_data.blockdevices[i].name,
      &device_data.blockdevices[i].size
    ));
  }

  // to &str vector
  let device_labels: Vec<&str> = device_names.iter().map(|s| s as &str).collect();

  // Create dropdown menu
  let device_menu = DropDown::from_strings(&device_labels);

  // Create a button with label and margins
  let button = Button::builder()
    .label("Confirm")
    .margin_top(12)
    .margin_bottom(12)
    .margin_start(12)
    .margin_end(12)
    .build();

  // Connect to "clicked" signal of `button`
  button.connect_clicked(move |button| {
    // Set the label to "Hello World!" after the button has been clicked on
    button.set_label("Hello World!");
  });

  let main_box = Box::builder()
    .orientation(Orientation::Vertical)
    .margin_top(12)
    .margin_bottom(12)
    .margin_start(12)
    .margin_end(12)
    .name("box")
    .build();

  BoxExt::append(&main_box, &device_menu);

  BoxExt::append(&main_box, &button);


  // Create a window
  let window = ApplicationWindow::builder()
    .application(app)
    .default_width(800).default_height(600)
    .title("SYSTEM: yarp installer")
    .child(&main_box)
    .build();

  // Present window
  window.present();
}