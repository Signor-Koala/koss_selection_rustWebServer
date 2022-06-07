use warp::Filter;

#[tokio::main]
async fn main() {

    let filter = warp::get()
        .and(warp::fs::dir("webpages"));

    warp::serve(filter)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

