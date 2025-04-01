mod util;
mod cmd;

#[tokio::main]
async fn main(){
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_line_number(false)
        .with_target(false)
        .with_level(false)
        .with_ansi(false)
        .without_time()
        .init();

    util::console::help();

    'main_loop: loop {
        let mut cmd = String::new();
        match std::io::stdin().read_line(&mut cmd){
            Ok(_) => {
                match cmd.trim(){
                    "quit" => {util::console::clear(); break 'main_loop;},
                    "sync" => {util::sync::sync().await},
                    "path" => {cmd::path::set_path()},
                    "help" => {util::console::help()},
                    _ => {util::console::help()}
                }
            }
            Err(e) => {println!("{}", e)}
        }
    }
}
