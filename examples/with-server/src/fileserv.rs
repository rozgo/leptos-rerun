use crate::error_template::ErrorTemplate;
use axum::response::Response as AxumResponse;
use axum::{
    body::Body,
    extract::State,
    http::{Request, Response, StatusCode, Uri},
    response::IntoResponse,
};
use leptos::prelude::{Errors, LeptosOptions, view};
use tower::ServiceExt;
use tower_http::services::ServeDir;

pub async fn file_and_error_handler(
    uri: Uri,
    State(options): State<LeptosOptions>,
    req: Request<Body>,
) -> AxumResponse {
    let root = options.site_root.clone();
    let response = get_static_file(uri.clone(), &root).await.unwrap();

    if response.status() == StatusCode::OK {
        response.into_response()
    } else {
        let errors = Errors::default();
        let handler = leptos_axum::render_app_to_stream(
            move || view! { <ErrorTemplate outside_errors=errors.clone()/> },
        );
        handler(req).await.into_response()
    }
}

async fn get_static_file(uri: Uri, root: &str) -> Result<Response<Body>, (StatusCode, String)> {
    let request = Request::builder()
        .uri(uri)
        .body(Body::empty())
        .expect("failed to build empty request");

    match ServeDir::new(root).oneshot(request).await {
        Ok(response) => Ok(response.into_response()),
        Err(error) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong serving the static file: {error}"),
        )),
    }
}
