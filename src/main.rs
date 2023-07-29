use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::get()
        .and(warp::path("hello"))
        .map(|| "Hello, world!");

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
