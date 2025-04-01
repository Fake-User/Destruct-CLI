use terminal_size::terminal_size;
use crate::util::path;

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
    let library_path = path::get_path();
    clear();
    logo();
    heading(format!("DESTRUCT CLI | PATH - {}", library_path).as_str());

    let logo_array: Vec<&str> = vec![
        "sync ------- sync library        help ------- list commands",
        "path --- set library path        quit --- quit destruct-cli"
    ];

    for line in logo_array {println!("{}", center(line))};
}
