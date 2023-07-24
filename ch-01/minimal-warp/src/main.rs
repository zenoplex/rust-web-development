use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::get().map(|| "Hello, world!");

    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}
