use std::io::prelude::*;

use crate::console;
extern crate dirs;

pub fn get_path() -> String {
    let config_file_path = format!("{}/Destruct/destruct-cli-config.txt", dirs::config_local_dir().unwrap().into_os_string().into_string().unwrap());
    match std::fs::read_to_string(&config_file_path){
        Ok(s) => {
            if s.len() > 3 {return s}
            else{ return "NOT SET".to_string()};
        },
        Err(e) => "NOT SET".to_string(),
    }
}

pub fn set_path(){
    let home_path = dirs::home_dir().unwrap().into_os_string().into_string().unwrap();
    let default_path = format!("{}/Destruct", dirs::audio_dir().unwrap().into_os_string().into_string().unwrap());
    let config_path = format!("{}/Destruct", dirs::config_local_dir().unwrap().into_os_string().into_string().unwrap());
    console::clear_previous_line();
    loop{
        println!("{}", format!("enter path to save library or use default: {}", default_path));
        let mut user_path = String::new();
        match std::io::stdin().read_line(&mut user_path){
            Ok(_) => {
                if user_path.trim() == "quit".to_string(){
                    console::clear();
                    std::process::exit(0x0100);
                }
                if user_path.trim() == "help".to_string(){
                    console::help();
                    break;
                }
                if user_path.starts_with(home_path.as_str()) || user_path.starts_with("~") || user_path == "\n".to_string(){
                    if user_path == "\n".to_string(){
                        console::clear_previous_line();
                        println!("using default path - {}", default_path);
                        user_path = default_path;
                    }
                    else if user_path == "~".to_string() || user_path == "~/".to_string(){
                        user_path = home_path;
                    }
                    else if user_path.starts_with("~"){
                        user_path = user_path.trim().to_string();
                        user_path = format!("{}{}", home_path, user_path[1..].to_string())
                    };
                    let absolute_config_path = std::path::absolute(&config_path).unwrap();
                    std::fs::create_dir_all(&absolute_config_path).unwrap();
                    let config_file = format!("{}/destruct-cli-config.txt", &config_path);
                    let absolute_config_file = std::path::absolute(&config_file).unwrap();
                    let mut file = std::fs::File::create(&absolute_config_file).unwrap();
                    write!(file, "{}", user_path).unwrap();
                    console::heading(format!("SET LIBRARY PATH TO - {}", user_path).as_str());
                    break;
                }
                else{
                    println!("ERROR - path must be reltive to your home folder");
                }
            }
            Err(e) => {println!("{}", e)}
        }
    }
}
