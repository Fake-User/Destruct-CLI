use ansi_term::Style;
use ansi_term::Colour::RGB;
use ansi_escapes::{CursorPrevLine, EraseLine};

#[path ="./path.rs"] mod path;

pub fn heading(text: &str){
    let center: String = center(text);
    println!("");
    println!("{}", Style::new().on(RGB(255, 0, 51)).fg(RGB(0, 0, 0)).paint(center));
    println!("");
}

pub fn clear_previous_line(){
    print!("{}", CursorPrevLine);
    print!("{}", EraseLine);
}

fn center(text: &str) -> String{
    let characters: Vec<char> = text.chars().collect();
    let length = text.len();
    let padding = (96 - length) / 2;
    let mut chars: Vec<char> = vec![];
    for c in 0..96 {
        if c < padding {chars.push(' ')}
        else if c < padding + length {chars.push(characters[c - padding])}
        else {chars.push(' ')};
    }
    let fin: String = chars.iter().collect();
    return fin;
}

pub fn reg(text: &str){
    println!("{}", Style::new().fg(RGB(255, 255, 255)).paint(text));
}

pub fn clear(){
    print!("\x1B[2J\x1B[1;1H");
}

pub fn logo(){
    let logo_array: Vec<&str> = vec![
        "   *%@@@@@@@@@@@  @@@@@@@@@@@%*   ",
        " *@@@@@@@@@@@@@@  @@@@@@@@@@@@@@* ",
        "*@@@@#+#@@@@@@@+  +@@@@@@@#+#@@@@*",
        "@@@@@@@\\.*#@@@@*  *@@@@#*./@@@@@@@",
        "@@@@@@@%:  *@@@-  -@@@*  :%@@@@@@@",
        "@@@@@@@@@\\   ..    ..   /@@@@@@@@@",
        "@@@@@@@@@@=            =@@@@@@@@@@",
        "@@@@@@@@@@#.          .#@@@@@@@@@@",
        "@@:-..                      ..-:@@",
        "@@@@@@@@@@#.          .#@@@@@@@@@@",
        "@@@@@@@@@@=            =@@@@@@@@@@",
        "@@@@@@@@@/   ..    ..   \\@@@@@@@@@",
        "@@@@@@@%:  *@@@-  -@@@*  :%@@@@@@@",
        "@@@@@@@/.*#@@@@*  *@@@@#*.\\@@@@@@@",
        "*@@@@#+#@@@@@@@+  +@@@@@@@#+#@@@@*",
        " *@@@@@@@@@@@@@@  @@@@@@@@@@@@@@* ",
        "   *%@@@@@@@@@@@  @@@@@@@@@@@%*   ",
    ];

    println!("");
    for line in logo_array {println!("{}", Style::new().fg(RGB(255, 0, 51)).paint(center(line)))};
}

fn get_path() -> String {
    let config_file_path = format!("{}/Destruct/destruct-cli-config.txt", dirs::config_local_dir().unwrap().into_os_string().into_string().unwrap());
    match std::fs::read_to_string(&config_file_path){
        Ok(s) => {
            if s.len() > 3 {return s}
            else{ return "NOT SET".to_string()};
        },
        Err(e) => "NOT SET".to_string(),
    }
}

pub fn help(){
    let library_path = get_path();
    clear();
    logo();
    heading(format!("DESTRUCT CLI | LIBRARY PATH - {}", library_path).as_str());

    let logo_array: Vec<&str> = vec![
        "              sync - sync library                       help - list cli arguements",
        "              path - set library path                   quit - quit destruct-cli"
    ];

    for line in logo_array {println!("{}", line)};
    println!("");
}
