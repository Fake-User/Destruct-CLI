use std::io::prelude::*;
extern crate dirs;

pub fn get_path() -> String {
    let config_dir = dirs::config_local_dir().unwrap().into_os_string().into_string().unwrap();
    let config_file_path = format!("{}/Destruct/destruct-cli-config.txt", config_dir);
    match std::fs::read_to_string(&config_file_path){
        Ok(s) => {
            if s.len() > 3 {return s}
            else{ return "NOT SET".to_string()};
        },
        Err(e) => "NOT SET".to_string(),
    }
}

pub fn set_path(){
    let default_path = format!("{}/Destruct/", dirs::audio_dir().unwrap().into_os_string().into_string().unwrap());
    println!("{}", format!("enter path to save library or use default: {}", default_path));
    let mut samples_path = String::new();
    match std::io::stdin().read_line(&mut samples_path){
        Ok(_) => {
            let mut path = &default_path;
            if samples_path.len() < 3 {println!("path not valid")}
            else{path = &samples_path};

            let config_dir = format!("{}/Destruct", dirs::config_local_dir().unwrap().into_os_string().into_string().unwrap());
            let absolute_config_dir = std::path::absolute(&config_dir).unwrap();
            std::fs::create_dir_all(&absolute_config_dir).unwrap();

            let config_file = format!("{}/destruct-cli-config.txt", &config_dir);
            let absolute_config_file = std::path::absolute(&config_file).unwrap();
            let mut file = std::fs::File::create(&absolute_config_file).unwrap();
            write!(file, "{}", path).unwrap();
            println!("{}", format!("set library path to {}", path));
        }
        Err(e) => {println!("{}", e)}
    }


}
