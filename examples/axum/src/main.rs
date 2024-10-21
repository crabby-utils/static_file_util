use axum::body::Body;
use axum::extract::Path;
use axum::http::{header, HeaderValue, Response, StatusCode};
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;
use static_file_util::static_files;
use tokio::net::TcpListener;

static_files!(
    (crab_svg, "../images/wikimedia-crab.svg", mime::IMAGE_SVG),
    (styles_css, "../css/styles.css", mime::TEXT_CSS),
);

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Listening on http://{}", listener.local_addr().unwrap());

    let app = Router::new()
        .route("/static/*path", get(static_path))
        .route("/", get(handler));

    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> impl IntoResponse {
    let styles = format!("/static/{}", styles_css.name);
    let crab_src = format!("/static/{}", crab_svg.name);
    let content = format!(
        //language=html
        r#"
<!doctype html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Document</title>
    <link rel="stylesheet" href="{styles}">
</head>
<body>
    <img src="{crab_src}" alt="crab svg">
</body>
</html>    
"#
    );

    Html(content)
}

async fn static_path(Path(path): Path<String>) -> impl IntoResponse {
    let path = path.trim_start_matches('/');
    dbg!(&path);

    if let Some(data) = StaticFile::get(path) {
        dbg!("got it");
        Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_str(data.mime.as_ref()).unwrap(),
            )
            .header(
                header::CACHE_CONTROL,
                HeaderValue::from_static("public, max-age=604800"),
            )
            .body(Body::from(data.content))
            .unwrap()
    } else {
        dbg!("missed it");
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap()
    }
}
