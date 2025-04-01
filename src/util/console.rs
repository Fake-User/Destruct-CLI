use terminal_size::terminal_size;
use std::io::{self, Write};

pub fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    String::from(io::stdin().lines().next().unwrap().unwrap())
}

fn center(text: &str) -> String {
    let term_width = terminal_size().unwrap().0.0;
    let length = text.len() as u16;
    if length >= term_width {return text.to_string()};
    let padding = std::cmp::max((term_width - length) / 2, 0);
    format!("{:>width$}{}{:width$}", "", text, "", width = padding as usize)
}

pub fn heading(text: &str){
    print!("\n\x1b[41m\x1b[30m{}\x1b[0m\n\n", center(text));
}

pub fn clear_previous_line(){
    print!("\x1b[F\x1b[2K\n");
}

pub fn clear(){
    if cfg!(target_os = "windows"){std::process::Command::new("cmd").args(["/c", "cls"]).status().unwrap();}
    else{std::process::Command::new("clear").status().unwrap();};
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
        "   *%@@@@@@@@@@@  @@@@@@@@@@@%*   "
    ];
    println!("");
    for line in logo_array {println!("\x1b[31m{}\x1b[0m", center(line));};
}

pub fn help(){
    clear();
    logo();
    heading("DESTRUCT ADMIN");

    let logo_array: Vec<&str> = vec![
        "sync ------------- upload library to r2       path --------- download library from r2",
        "help -------------- list cli arguements       quit ---------------- quit destruct-cli"
    ];

    for line in logo_array {println!("{}", center(line))};
}
