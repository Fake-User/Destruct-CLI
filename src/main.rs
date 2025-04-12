use s3sync::types::token::create_pipeline_cancellation_token;
use s3sync::config::args::parse_from_args;
use serde::{Deserialize, Serialize};
use terminal_size::terminal_size;
use s3sync::pipeline::Pipeline;
use s3sync::config::Config;

#[derive(Serialize, Deserialize, Debug)]
struct Creds {
    access_key_id: String,
    secret_access_key: String,
    session_token: String
}

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

    let path: String = dirs::audio_dir()
        .expect("ERROR - audio directory not found")
        .join("Destruct")
        .into_os_string()
        .into_string()
        .expect("ERROR - could not convert path to string");

    init(path.clone());

    'main_loop: loop {
        let mut cmd = String::new();
        match std::io::stdin().read_line(&mut cmd){
            Ok(_) => {
                match cmd.trim(){
                    "quit" => {clear(); break 'main_loop;},
                    "sync" => {sync(path.clone()).await},
                    _ => {}
                }
            }
            Err(e) => {println!("{}", e)}
        }
    }
}

async fn sync(path: String){
    println!("connecting to destruct-server");

    let res = reqwest::get("https://destruct-server.rcdis.co/creds")
        .await.expect("ERROR - could not connect to server")
        .text()
        .await.expect("ERROR - could not read server response");
    let res_creds = res.as_str();
    let creds: Creds = serde_json::from_str(&res_creds).expect("ERROR - server credentials invalid");

    let r2_endpoint = "https://38b60100935d30d769c3198e265d1167.r2.cloudflarestorage.com";
    let r2_secret_key = creds.secret_access_key;
    let r2_session_token = creds.session_token;
    let r2_key = creds.access_key_id;

    let args = vec![
        "DESTRUCT",
        "--source-region", "auto",
        "--source-access-key", r2_key.as_str(),
        "--source-secret-access-key", r2_secret_key.as_str(),
        "--source-session-token", r2_session_token.as_str(),
        "--source-endpoint-url", r2_endpoint,
        "--source-force-path-style", "s3://destruct-data/wav/",
        path.as_str()
    ];
    let config = Config::try_from(
        parse_from_args(args)
        .expect("ERROR - could not parse args")
    ).expect("ERROR - could not create config struct");
    let cancellation_token = create_pipeline_cancellation_token();
    let mut pipeline = Pipeline::new(config.clone(), cancellation_token).await;
    pipeline.close_stats_sender();
    pipeline.run().await;
    assert!(!pipeline.has_error());
    heading(format!("COMPLETED SYNC TO - {}", path).as_str());
}

fn center(text: &str) -> String {
    let term_width = terminal_size().unwrap().0.0;
    let length = text.len() as u16;
    if length >= term_width {return text.to_string()};
    let padding = std::cmp::max((term_width - length) / 2, 0);
    format!("{:>width$}{}{:width$}", "", text, "", width = padding as usize)
}

fn heading(text: &str){
    print!("\n\x1b[41m\x1b[30m{}\x1b[0m\n\n", center(text));
}

fn clear(){
    if cfg!(target_os = "windows"){std::process::Command::new("cmd").args(["/c", "cls"]).status().unwrap();}
    else{std::process::Command::new("clear").status().unwrap();};
}

fn init(path: String){
    clear();
    println!("");
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
    for line in logo_array {println!("\x1b[31m{}\x1b[0m", center(line));};

    heading(format!("PATH: {}", path).as_str());

    let cmd_array: Vec<&str> = vec![
        "sync ---------------- sync library",
        "quit ----------- quit destruct-cli"
    ];
    for line in cmd_array {println!("{}", center(line));};
    println!("");
}
