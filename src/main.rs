use warp::Filter;

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["User-Agent", "Sec-Fetch-Mode", "Referer", "Origin", "Access-Control-Request-Method", "Access-Control-Request-Headers"])
        .allow_methods(vec!["POST", "GET", "OPTIONS"]);

    let response_html = "
<div>
    <h1>This is WARP</h1>
    <p>Hello, WARP!</p>
</div>
";

    let hello = warp::get().and(warp::path("clicked"))
        .map(|| response_html.to_string()).with(cors);

    let index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("./index.html"));

    warp::serve(hello.or(index))
        .run(([127, 0, 0, 1], 3030))
        .await;
}