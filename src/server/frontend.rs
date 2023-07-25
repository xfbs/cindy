use crate::Cindy;
use axum::{
    http::{header::CONTENT_TYPE, HeaderName, HeaderValue, Uri},
    Router,
};
use include_dir::{include_dir, Dir};

/// Frontend files baked-in to the Cindy binary for convenience.
static FRONTEND_FILES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/ui/dist");

/// By default, the index HTML file is served.
const DEFAULT_FILE: &str = "index.html";

/// Serve frontend static file.
async fn static_file(uri: Uri) -> ([(HeaderName, HeaderValue); 1], &'static [u8]) {
    let file = match FRONTEND_FILES.get_file(uri.path().trim_start_matches('/')) {
        Some(file) => file,
        None => FRONTEND_FILES.get_file(DEFAULT_FILE).unwrap(),
    };
    let guess = mime_guess::from_path(file.path());
    let mime = guess
        .first_raw()
        .map(HeaderValue::from_static)
        .unwrap_or_else(|| HeaderValue::from_str(mime::APPLICATION_OCTET_STREAM.as_ref()).unwrap());
    ([(CONTENT_TYPE, mime)], file.contents())
}

pub fn router() -> Router<Cindy> {
    Router::new().fallback(static_file)
}
