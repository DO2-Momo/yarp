mod sysinfo;
mod components;
mod validation;
mod install;
mod config;

use crate::config::{User, UserData};
use crate::sysinfo::Devices;
use crate::components::prompt_user;
use crate::components::control::{PackageProfile};
use std::{str};
use gtk::gdk::Display;
use gtk::prelude::*;
use gtk::gio::SimpleAction;
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
use std::rc::Rc;

use std::cell::Cell;

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

fn get_device_labels(device_data: &Devices) -> Vec<String> {
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

  return device_names;
}

fn get_device_sizes(devices: &sysinfo::Devices) -> Vec<u128> {
  let mut ans = Vec::<u128>::new();

  for i in 0..devices.blockdevices.len() {
    ans.push(devices.blockdevices[i].size + 0);
  }

  return ans;
}

fn build_ui(app: &Application) {
  // Get devices data
  let device_data: sysinfo::Devices = sysinfo::get_devices();
  let device_data_sizes: Vec<u128> = get_device_sizes(&device_data);
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

  let form = components::form();

  let flex = Box::builder()
    .orientation(Orientation::Horizontal)
    .hexpand(true)
    .vexpand(true)
    .css_classes(vec![String::from("flex-div")])
    .build();
  
  let right_box = Box::builder()
    .orientation(Orientation::Vertical)
    .hexpand(true)
    .vexpand(true)
    .css_classes(vec![String::from("right-box")])
    .width_request(400)
    .build();

  let partition_label = Label::builder()
    .label("Partitions:")
    .justify(Justification::Left)
    .xalign(0.0)
    .css_classes(vec![String::from("section-title")])
    .build();
  
  let swap_label = Label::builder()
    .label("Swap")
    .justify(Justification::Left)
    .xalign(0.0)
    .css_classes(vec![String::from("section-title-2")])
    .build();
      
  let root_home_ratio_label = Label::builder()
    .label("home/root ratio")
    .justify(Justification::Left)
    .xalign(0.0)
    .css_classes(vec![String::from("section-title-2")])
    .build();

  let package_label = Label::builder()
    .label("Packages:")
    .justify(Justification::Left)
    .xalign(0.0)
    .css_classes(vec![String::from("section-title")])
    .build();

  let package_profile_box = Box::builder()
    .orientation(Orientation::Vertical)
    .halign(Align::Start)
    .valign(Align::Start)
    .css_classes(vec![String::from("package-box")])
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

  let chk_desktop:CheckButton    = CheckButton::with_label("Desktop Environement");
  let chk_utils:CheckButton      = CheckButton::with_label("Desktop Features");
  let chk_multimedia:CheckButton = CheckButton::with_label("Multimedia Utils");
  let chk_nightly:CheckButton    = CheckButton::with_label("Nightly pack");

  package_profile_box.append(&package_label);
  package_profile_box.append(&chk_desktop);
  package_profile_box.append(&chk_utils);
  package_profile_box.append(&chk_multimedia);
  package_profile_box.append(&chk_nightly);

  let scale_swap = Scale::builder()
    .orientation(Orientation::Horizontal)
    .width_request(275)
    .height_request(80)
    .value_pos(PositionType::Top)
    .has_origin(true)
    .can_target(true)
    .fill_level(16384.0)
    .vexpand(false)
    .build();

  scale_swap.set_value(4096.0);
  scale_swap.set_increments(256.0, 1024.0);
  scale_swap.set_range(0.0, 16384.0);
  scale_swap.add_mark(0.0, PositionType::Bottom,  Some("0 Mb"));
  scale_swap.add_mark(8192.0, PositionType::Bottom,  Some("8192 Mb"));
  scale_swap.add_mark(16384.0, PositionType::Bottom, Some("16384 Mb"));
  scale_swap.set_draw_value(true);

  let scale_part_ratio = Scale::builder()
    .orientation(Orientation::Horizontal)
    .width_request(275)
    .height_request(80)
    .value_pos(PositionType::Top)
    .has_origin(true)
    .can_target(true)
    .fill_level(100.0)
    .vexpand(false)
    .build();

  scale_part_ratio.set_value(50.0);
  scale_part_ratio.set_increments(1.0, 5.0);
  scale_part_ratio.set_range(0.0, 100.0);
  scale_part_ratio.add_mark(0.0, PositionType::Bottom,  Some("25%"));
  scale_part_ratio.add_mark(50.0, PositionType::Bottom,  Some("50%"));
  scale_part_ratio.add_mark(100.0, PositionType::Bottom, Some("100%"));
  scale_part_ratio.set_draw_value(true);

  // let slider_wrap = Box::builder()
  //   .css_classes(vec![String::from("slider-wrap")])
  //   .hexpand(true).vexpand(true)
  //   .build();
    
  // slider_wrap.append(&scale_part_ratio);

  let partition_box = Box::builder()
    .orientation(Orientation::Vertical)
    .hexpand(true)
    .vexpand(true)
    .css_classes(vec![String::from("partition-box")])
    .build();

  let partition_slider_box = Box::builder()
    .orientation(Orientation::Horizontal)
    .hexpand(true)
    .vexpand(true)
    .css_classes(vec![String::from("partition-box")])
    .build();


  let swap_size_input = Entry::builder()
    .input_purpose(InputPurpose::Digits)
    .css_classes(vec![String::from("swap-field")])
    .build();


  partition_box.append(&partition_label);


  partition_box.append(&swap_label);
  partition_box.append(&scale_swap);

  partition_box.append(&root_home_ratio_label);
  partition_box.append(&scale_part_ratio);

  right_box.append(&partition_box);
  right_box.append(&package_profile_box);



  // Create dropdown menu with the labels
  let device_menu = DropDown::from_strings(&device_labels);

  // device_menu.connect_selected_item_notify(move |device_menu| {
  //   // Activate "win.count" and pass "1" as parameter
  //   let param = (device_data_sizes[device_menu.selected() as usize]/1000000) as i32;
  //   device_menu
  //     .activate_action("win.set_root_size", Some(&param.to_variant()))
  //     .expect("The action does not exist.");
  // });



  let device_box = Box::builder()
    .css_classes(vec![String::from("dropdown")])
    .build();

  device_box.append(&device_menu);

  let original_state:i32 = 0;
  let set_root_size = SimpleAction::new_stateful(
    "set_root_size",
    Some(&i32::static_variant_type()),
    &original_state.to_variant(),
  );

  // scale_part_ratio.connect_value_changed( move |scale_part_ratio| {
  //   // Activate "win.count" and pass "1" as parameter
  //   let param = scale_part_ratio.value() as i32 * device_data_sizes[device_menu.selected() as usize] as i32;
  //   scale_part_ratio
  //     .activate_action("win.set_root_size", Some(&param.to_variant()))
  //     .expect("The action does not exist.");
  // });



  // set_root_size.connect_activate(clone!(@weak root_size_label => move |action, param| {
  //   let mut state = action
  //     .state()
  //     .expect("Could not get state.")
  //     .get::<i32>()
  //     .expect("The variant needs to be of type `i32`.");

  //   // Get parameter
  //   let param = param
  //     .expect("Could not get parameter.")
  //     .get::<i32>()
  //     .expect("The variant needs to be of type `i32`.");

  //   state = param;

  //   root_size_label.set_label(&format!("{state} Mb"));
  // }));

  // Connect to "clicked" signal of `confirm_button`
  confirm_button.connect_clicked(move |confirm_button| {

    prompt_user(&form.data);

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

    let userData = UserData {
      user, hostname, 
      device: device,
      packages: packages
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

