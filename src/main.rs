use warp::Filter;
use serde::{Deserialize, Serialize};
use serde_json::value::{to_raw_value, RawValue};
use tokio::io::{AsyncWriteExt, AsyncBufReadExt, BufReader};
use tokio::fs::{OpenOptions, create_dir};
use chrono::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct SinceQuery {
    since: Option<usize>, //Unix timestamp
}

#[derive(Serialize, Deserialize, Debug)]
struct BoardAction {
    action: String,
    payload: Box<RawValue>
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    match create_dir("./build/static/data/").await {
        Ok(_) => println!("Made directory"),
        Err(_) => println!("Error making directory")
    }

    let get_message = warp::get()
        .and(warp::path("boards"))
        .and(warp::path::param())
        .and(warp::query::<SinceQuery>())
        .and_then(|board_id: String, since: SinceQuery| async move {
            match OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(format!("./build/static/data/{}.ba", board_id)).await {
                Ok(f) => {
                    let since_c = match since.since {
                        Some(s) => s,
                        None => 0
                    };
                        
                    let mut skip = false;                    
                    let mut output: Vec<Box<RawValue>> = vec![];
                    let mut lines = BufReader::new(f).lines();
                    while let Some(line) = lines.next_line().await.unwrap() {
                        if skip {
                            skip = !skip;
                            continue;
                        }
                        let timestamp = line.parse::<usize>().unwrap();

                        if timestamp > since_c {
                            let json_str = lines.next_line().await.unwrap().unwrap();
                            output.push(to_raw_value(&json_str).unwrap());
                        } else {
                            skip = !skip;
                        }
                    }
                    Ok(warp::reply::json(&output))
                }
                Err(_a) => Err(warp::reject())
            }
        });

    let post_message = warp::post()
        .and(warp::path("boards"))
        .and(warp::path::param())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and_then(|board_id: String, action: BoardAction| async move {
            let action = format!("{}\n{}\n", Utc::now().timestamp(), serde_json::to_string(&action).unwrap());

            match OpenOptions::new()
                .append(true)
                .create(true)
                .open(format!("./build/static/data/{}.ba", board_id))
                .await {
                Ok(mut f) => match f.write(&action.into_bytes()).await {
                        Ok(n) => Ok(format!("Success, wrote: {}", n)),
                        Err(_a) => Err(warp::reject())
                },
                Err(_a) => Err(warp::reject())
            }
        });

    let index = warp::path::end()
        .and(warp::fs::file("./build/index.html"));

    let static_files = warp::path("static")
        .and(warp::fs::dir("./build/static"));

    let routes = index
        .or(static_files)
        .or(get_message)
        .or(post_message);
        

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}