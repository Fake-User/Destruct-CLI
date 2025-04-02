use crate::util::console;
use std::io::prelude::*;
extern crate dirs;

pub fn get_path() -> String {
    let config_dir = dirs::config_local_dir().expect("ERROR - config directory not found");
    let config_file_path = config_dir.join("Destruct").join("destruct-cli-config.txt");

    match std::fs::read_to_string(&config_file_path) {
        Ok(s) if s.len() > 3 => s,
        _ => "LIBRARY PATH NOT SET".to_string(),
    }
}

fn write_config(config_path: &std::path::PathBuf, library_path: &std::path::PathBuf) {
    std::fs::create_dir_all(&config_path).expect("ERROR - create config directory failed");
    let config_file = config_path.join("destruct-cli-config.txt");
    let mut file = std::fs::File::create(&config_file).expect("ERROR - create config file failed");
    write!(file, "{}", library_path.display()).expect("ERROR - write config file failed");
    console::heading(&format!("SET LIBRARY PATH TO - {}", library_path.display()));
}

pub fn set_path(){
    let home_dir = dirs::home_dir().expect("ERROR - home directory not found");
    let default_path = dirs::audio_dir().expect("ERROR - audio directory not found").join("Destruct");
    let config_path = dirs::config_local_dir().expect("ERROR - config directory not found").join("Destruct");
    console::clear_previous_line();
    loop {
        println!("enter path to sync library or use default - {}", default_path.display());
        let mut user_input = String::new();

        match std::io::stdin().read_line(&mut user_input){
            Ok(_) => {
                let user_input = user_input.trim();

                match user_input{
                    "" => {
                        console::clear_previous_line();
                        println!("using default path - {}", default_path.display());
                        write_config(&config_path, &default_path);
                        break;
                    }
                    _ => {
                        let user_path = if user_input.starts_with('~'){
                            if user_input == "~" || user_input == "~/"{home_dir.clone()}
                            else{home_dir.join(&user_input[2..])}
                        }
                        else{
                            std::path::PathBuf::from(user_input)
                        };

                        if user_path.starts_with(&home_dir){
                            write_config(&config_path, &user_path);
                            break;
                        }
                        else{
                            println!("ERROR - path must be relative to your home folder");
                        }
                    }
                }
            }
            Err(e) => println!("{}", e)
        }
    }
}
