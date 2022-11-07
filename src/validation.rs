

use crate::config::UserData;
use crate::components::prompt_user;

use crate::install::install;

pub fn validate_config(data: &UserData) {
    
    if data.user.password != data.user.cpassword {
        println!("Passwords do not match");  
        return;
    }

    prompt_user();
    install(data);

    return;
}
