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
  Button
};
use gtk::prelude::*;

pub mod control;
use control::{Form, FormData, PackageProfile};

pub fn package_profile_menu(
  chk_desktop: CheckButton,
  chk_utils: CheckButton,
  chk_multimedia: CheckButton,
  chk_nightly: CheckButton
) -> Box {



  let package_profile_box = Box::builder()
  .orientation(Orientation::Vertical)
  .css_classes(vec![String::from("profile")])
  .build();

  package_profile_box.append(&chk_desktop);
  package_profile_box.append(&chk_utils);
  package_profile_box.append(&chk_multimedia);
  package_profile_box.append(&chk_nightly);

  return package_profile_box;
}

pub fn formField(label: &str, buf: &EntryBuffer, hidden: bool ) -> Box {
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

pub fn form() -> Form {

  let ubuf = EntryBuffer::builder().build();
  let hbuf = EntryBuffer::builder().build();
  let pbuf = EntryBuffer::builder().build();
  let cbuf = EntryBuffer::builder().build();

  let formData = FormData {
    username: ubuf,
    hostname: hbuf,
    password: pbuf,
    cpassword: cbuf,
    packages: PackageProfile {
      desktop:true,
      utils: false,
      multimedia: false,
      nightly: false
    }
  };

  let hostname = formField("username ", &formData.username, false);
  let username = formField("hostname ", &formData.hostname, false);
  let password = formField("password ", &formData.password,true);
  let cpassword = formField("confirm password ", &formData.cpassword, true);


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
  
  return Form {
    widget: form,
    data: formData
  };
}

pub fn DeviceRow(name: &str, size: &str) -> Box {
  let device_row = Box::builder()
  .orientation(Orientation::Horizontal)
  .name("device_button")
  .css_classes(vec![String::from("device-elem")])
  .build();
 
  let check = CheckButton::builder()
      .name(name)
      .css_classes(vec![String::from("device-check")])
      .build();

  device_row.append(&check);

  BoxExt::append(
    &device_row,
    &Label::builder() 
        .label(name)
        .css_classes(vec![String::from("device-label")])
        .build()
  );

  BoxExt::append(
    &device_row,
    &Label::builder() 
        .label(size)
        .css_classes(vec![String::from("device-label")])
        .build()
  );

  return device_row;
}

pub fn getLabel(name: &str, size: &str) -> String {
  let mut buf: Vec<&str> = Vec::<&str>::new();
  buf.push(name);
  buf.push(size);

  return buf.join("     ");
}

pub fn success() -> Box {

  let button = Button::builder()
    .label("Ok").build();

  let label = Label::builder()
    .label("Success").build();

  let success = Box::builder().visible(false).build();

  success.append(&label);
  success.append(&button);

  return success;

}

pub fn prompt_user(data: &FormData)  {

}