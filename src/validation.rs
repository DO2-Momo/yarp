

use crate::config::UserData;
use crate::install::install;
use crate::install::get_packages;

pub fn validate_config<'a>(data: &UserData) {
    
    if data.user.password != data.user.cpassword {
        println!("Passwords do not match");  
        return;
    }

    // brute DEBUG Safety for my build ;)
    if data.device.name == "nvme0n1" {
      return; // TODO; REMOVE THIS
    }; 
    
    install(data);

    return;
}
