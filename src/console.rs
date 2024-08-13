use ansi_term::Style;
use ansi_term::Colour::RGB;
use ansi_escapes::{CursorPrevLine, EraseLine};

use crate::path;

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
    let padding = (95 - length) / 2;
    let mut chars: Vec<char> = vec![];
    for c in 0..95 {
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

pub fn help(){
    let library_path = path::get_path();
    clear();
    logo();
    heading(format!("DESTRUCT CLI | LIBRARY PATH - {}", library_path.trim()).as_str());

    let logo_array: Vec<&str> = vec![
        "sync ------- sync library         help ------- list commands",
        "path --- set library path         quit --- quit destruct-cli"

    ];

    for line in logo_array {println!("{}", center(line))};
    println!("");
}
