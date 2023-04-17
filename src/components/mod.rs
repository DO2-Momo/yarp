use gtk::{
  Box,
  CheckButton,
  Orientation,
  Label,
  Text,
  InputPurpose,
  Justification,
  Align,
  EntryBuffer,
  Scale,
  PositionType,
};
use gtk::prelude::*;

pub mod control;
use control::{Form, FormData, PackageProfile};

/// TODO: FETCH FROM OS
pub fn hardware_specs(
  chk_amd_gpu: &CheckButton,
  chk_intel_gpu: &CheckButton
) -> Box {

  let package_label = Label::builder()
    .label("Hardware:")
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

  package_profile_box.append(&package_label);
  package_profile_box.append(chk_amd_gpu);
  package_profile_box.append(chk_intel_gpu);

  return package_profile_box;
}

pub fn get_package_profile_box(
  chk_desktop: &CheckButton,
  chk_utils: &CheckButton,
  chk_multimedia: &CheckButton,
  chk_nightly: &CheckButton
) -> Box {
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

  package_profile_box.append(&package_label);
  package_profile_box.append(chk_desktop);
  package_profile_box.append(chk_utils);
  package_profile_box.append(chk_multimedia);
  package_profile_box.append(chk_nightly);

  return package_profile_box;
} 


pub fn get_partition_box(
  swap_scale: &Scale,
  part_scale: &Scale,
  root_home_ratio_label: &Label
) -> Box {

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

  swap_scale.set_value(4096.0);
  swap_scale.set_increments(256.0, 1024.0);
  swap_scale.set_range(512.0, 8129.0);
  swap_scale.add_mark(2048.0, PositionType::Bottom,  Some("2048 Mb"));
  swap_scale.add_mark(4096.0, PositionType::Bottom,  Some("4096 Mb"));
  swap_scale.add_mark(8192.0, PositionType::Bottom,  Some("8192 Mb"));
  swap_scale.set_draw_value(true);

  part_scale.set_value(50.0);
  part_scale.set_increments(1.0, 5.0);
  part_scale.set_range(25.0, 100.0);
  part_scale.add_mark(25.0, PositionType::Bottom,  Some("25%"));
  part_scale.add_mark(50.0, PositionType::Bottom,  Some("50%"));

  part_scale.add_mark(75.0, PositionType::Bottom,  Some("75%"));
  part_scale.add_mark(100.0, PositionType::Bottom, Some("100%"));
  part_scale.set_draw_value(true);

  let size_label = Label::builder().label("Root: Home:").build();

  let partition_box = Box::builder()
    .orientation(Orientation::Vertical)
    .hexpand(true)
    .vexpand(true)
    .css_classes(vec![String::from("partition-box")])
    .build();

  partition_box.append(&partition_label);

  partition_box.append(&swap_label);
  partition_box.append(swap_scale);

  partition_box.append(root_home_ratio_label);
  partition_box.append(part_scale);
  partition_box.append(&size_label);


  return partition_box;
}

pub fn form_field(label: &str, buf: &EntryBuffer, hidden: bool ) -> Box {
  let input = Text::builder()
      .css_classes(vec![String::from("form-input")])
      .buffer(buf)
      .hexpand(true)
      .width_request(30)
      .max_length(18)
      .build();

  if hidden == true {
      input.set_input_purpose(InputPurpose::Password);
      input.set_visibility(false);
  }

  let label = Label::builder()
      .label(label)
      .halign(Align::End)
      .css_classes(vec![String::from("form-label")])
      .build();

  label.set_justify(Justification::Left);

  let field = Box::builder()
      .css_classes(vec![String::from("form-field")])

      .halign(Align::Start)
      .build();

  field.append(&label);
  field.append(&input);

  return field;
}

pub fn form(name: &str) -> Form {

  let title = Label::builder()
    .label(name)
    .justify(Justification::Left)
    .xalign(0.06)
    .css_classes(vec![String::from("section-title")])
    .build();

  let ubuf = EntryBuffer::builder().build();
  let hbuf = EntryBuffer::builder().build();
  let pbuf = EntryBuffer::builder().build();
  let cbuf = EntryBuffer::builder().build();

  let form_data = FormData {
    username: ubuf,
    hostname: hbuf,
    password: pbuf,
    cpassword: cbuf,
    packages: PackageProfile {
      desktop:true,
      utils: false,
      multimedia: false,
      nightly: false,
      intel_gpu: true,
      amd_gpu: false
    }
  };

  let hostname = form_field("username ", &form_data.username, false);
  let username = form_field("hostname ", &form_data.hostname, false);
  let password = form_field("password ", &form_data.password,true);
  let cpassword = form_field("confirm password ", &form_data.cpassword, true);

  let container: Box = Box::builder()
      .orientation(Orientation::Vertical)
      .hexpand(false)
      .vexpand(false)
      .css_classes(vec![String::from("form-section")])
      .build();

  let form: Box = Box::builder()
      .orientation(Orientation::Vertical)
      .hexpand(false)
      .vexpand(false)
      .css_classes(vec![String::from("form")])
      .build();

  form.append(&hostname);
  form.append(&username);
  form.append(&password);
  form.append(&cpassword);

  container.append(&title);
  container.append(&form);
  
  return Form {
    widget: container,
    data: form_data
  };
}

pub fn get_label(name: &str, size: &str) -> String {
  let mut buf: Vec<&str> = Vec::<&str>::new();
  buf.push(name);
  buf.push(size);

  return buf.join("     ");
}


