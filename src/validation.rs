

use crate::config::UserData;
use crate::install::install;

pub fn validate_config<'a>(data: &UserData) -> bool {
  
    if data.user.password != data.user.cpassword {
      println!("Passwords do not match");  
      return false;
    }

    // brute DEBUG Safety for my build ;)
    if data.device.name == "nvme0n1" {
      return false; // TODO; REMOVE THIS
    }; 
    
    install(data);

    return true;
}
