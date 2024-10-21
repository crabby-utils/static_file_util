# `static_file_util`

`static_file_util` is a utility crate for generating and managing static files in Rust applications. It simplifies the
process of embedding static assets into your project and automatically provides hashes for cache-busting, making it
ideal for web applications and similar use cases.

Check the [axum example](https://github.com/crabby-utils/static_file_util/blob/main/examples/axum/src/main.rs) for
details of how to use this in a web application to serve static files, like images or CSS.

## Features

- Define static files with ease using a single macro.
- Automatically computes content hashes, allowing for effective cache control.
- Provides a convenient interface to access static files at runtime.

## Installation

Add `static_file_util` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
static_file_util = "0.1.0"
lazy_static = "1.4"   # Required dependency for lazy static initialization
mime = "0.3"          # For handling MIME types

[build-dependencies]
static_file_util = "0.1.0"
```

## Usage

This usage example demonstrates the steps required to make use of `static_file_util`.

1. The file hashes are calculated using a `build.rs` script.
2. The files are defined using the `static_files!` macro.
3. The `.name` property is used to access the generated file names (including the hash).
4. The StaticFile struct is utilised to resolve the files and serve up the content with the correct MIME type.

### Environment Variable for Hashing

In order to generate unique names for each file, a `build.rs` script can be used to generate content hashes during the
build process. These hashes are passed as environment variables to the main code.

Here's an example of a `build.rs` script that works with `static_file_util`:

```rust
use static_file_util::process_file;

fn main() {
    process_file("images/logo.svg", "logo_svg_HASH");
    process_file("css/styles.css", "styles_css_HASH");
}
```

The `process_file` function reads the file contents, generates a hash, and sets it as an environment variable that the
macro uses during compilation.

It is recommended to add this `build.rs` script to your project in order to automatically generate the
environment variables required to define static files.

### Define Static Files

Use the `static_files!` macro to define your static assets. This macro allows you to embed files directly into your
binary, providing easy access to their contents and metadata.

Here's an example of how to use `static_file_util` in your project:

```rust
use static_file_util::static_files;

static_files!(
    (logo_svg, "../images/logo.svg", mime::IMAGE_SVG),
    (styles_css, "../css/styles.css", mime::TEXT_CSS),
);

fn main() {
    // Example of using the generated file names in an HTML template
    let styles = format!("/static/{}", styles_css.name);
    let logo_src = format!("/static/{}", logo_svg.name);

    let html_template = format!(r#"
<!doctype html>
<html lang="en">
<head>
    <link rel="stylesheet" href="{}">
</head>
<body>
<div id="main">
    <img src="{}" />
</div>
</body>
</html>
"#, styles, logo_src);

    println!("{}", html_template);
}
```

### Axum Example

Here's an example of using `static_file_util` with the Axum web framework to serve static files:

```rust
use axum::body::Body;
use axum::extract::Path;
use axum::http::{header, HeaderValue, Response, StatusCode};
use axum::response::IntoResponse;
// StaticFile definition is created by the static_files! macro
// include it here to use within an Axum handler
use web_assets::StaticFile;

pub async fn static_path(Path(path): Path<String>) -> impl IntoResponse {
    let path = path.trim_start_matches('/');

    if let Some(data) = StaticFile::get(path) {
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
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap()
    }
}
```

### How It Works

The original idea for this crate was inspired by
the [Assets and Cache Busting](https://rust-on-nails.com/docs/full-stack-web/cache-busting/)
section of the [Rust on Nails](https://rust-on-nails.com/) guide. That made use of
the [Ructe](https://docs.rs/ructe/latest/ructe/)
HTML template system for Rust. This solution is a more minimal implementation. It is just focused on embedding static
assets and cache
busting using generated hashes.

The `static_files!` macro generates:

- A `StaticFile` struct for each file, containing:
    - `content`: The raw content of the file as a byte slice.
    - `name`: The name of the file with a unique hash appended for cache-busting.
    - `mime`: The MIME type of the file.
- A `STATICS` vector that holds references to all the defined static files.
- A utility function `StaticFile::get(path)` is provided for easy lookup of a file based on the path.

### Example Use Cases

- **Web Applications**: Embed images, stylesheets, and other static resources directly into your binary.
- **Caching**: The unique hash in each static file's name ensures that assets can be cached effectively by browsers
  while ensuring cache invalidation when content changes.

## Documentation

The complete documentation for `static_file_util` is available on [docs.rs](https://docs.rs/static_file_util).

## License

This project is licensed under either the [MIT license](LICENSE-MIT) or the [Apache License 2.0](LICENSE-APACHE), at
your option.




