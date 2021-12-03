#![deny(warnings)]
use serde::{Deserialize, Serialize};

use warp::Filter;

#[derive(Deserialize, Serialize)]
struct Employee {
    name: String,
    rate: u32,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // POST /employees/:rate  {"name":"Sean","rate":2}
    let promote = warp::post()
        .and(warp::path("employees"))
        .and(warp::path::param::<u32>())
        // Only accept bodies smaller than 16kb...
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::form())
        .map(|rate, mut employee: Employee| {
            employee.rate = rate;
            warp::reply::json(&employee)
        });

    let index = warp::get()
        .and(warp::path::end())
        .map(|| {
            warp::reply::html("<form action='/employees/100' method='post'><input name='name'><input name='rate'><input type='submit'></form>")
        });

    warp::serve(promote.or(index)).run(([127, 0, 0, 1], 3030)).await
}