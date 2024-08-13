use serde::{Deserialize, Serialize};
use s3sync::config::Config;
use s3sync::pipeline::Pipeline;
use s3sync::config::args::parse_from_args;
use s3sync::types::token::create_pipeline_cancellation_token;

use crate::path;

#[derive(Serialize, Deserialize, Debug)]
struct Creds {
    access_key_id: String,
    secret_access_key: String,
    session_token: String
}

#[path = "./console.rs"] mod console;

pub async fn sync(){
    console::clear_previous_line();
    loop {
        let local_path = path::get_path();
        if local_path != "LIBRARY PATH NOT SET".to_string(){
            println!("syncing library to - {}", local_path);
            let res = reqwest::get("https://destruct.rcdis.co/creds")
                .await.unwrap()
                .text()
                .await.unwrap();

            let res_creds = res.as_str();

            let creds: Creds = serde_json::from_str(&res_creds).unwrap();

            let r2_key = creds.access_key_id;
            let r2_session_token = creds.session_token;
            let r2_secret_key = creds.secret_access_key;
            let r2_endpoint = "https://38b60100935d30d769c3198e265d1167.r2.cloudflarestorage.com";

            let args = vec![
                "DESTRUCT",
                "--source-region", "auto",
                "--source-access-key", r2_key.as_str(),
                "--source-secret-access-key", r2_secret_key.as_str(),
                "--source-session-token", r2_session_token.as_str(),
                "--source-endpoint-url", r2_endpoint,
                "--source-force-path-style", "s3://destruct-data/wav/",
                local_path.as_str()
            ];
            let config = Config::try_from(parse_from_args(args).unwrap()).unwrap();
            let cancellation_token = create_pipeline_cancellation_token();
            let mut pipeline = Pipeline::new(config.clone(), cancellation_token).await;
            pipeline.close_stats_sender();
            pipeline.run().await;
            assert!(!pipeline.has_error());
            console::heading(format!("COMPLETED SYNC TO - {}", local_path).as_str());
            break;
        }
        else{
            println!("library path not set\n");
            path::set_path();
        }
    }
}
