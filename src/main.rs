#![allow(dead_code, unused_variables)]

mod sync;
mod path;
mod console;

#[tokio::main]
async fn main(){
    let args: Vec<String> = std::env::args().collect();
    let path: &String = &args[0];

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_line_number(false)
        .with_ansi(false)
        .without_time()
        .with_target(false)
        .with_level(false)
        .init();

    console::help();

    'main_loop: loop {
        let mut cmd = String::new();
        match std::io::stdin().read_line(&mut cmd){
            Ok(_) => {
                match cmd.trim(){
                    "sync" => {sync::sync().await},
                    "path" => {path::set_path()},
                    "help" => {console::help()},
                    "quit" => {console::clear(); break 'main_loop;},
                    _ => {console::help()}
                }
            }
            Err(e) => {println!("{}", e)}
        }
    }
}
