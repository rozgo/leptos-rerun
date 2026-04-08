use cfg_if::cfg_if;
use http::status::StatusCode;
use leptos::prelude::*;

#[cfg(feature = "ssr")]
use leptos_axum::ResponseOptions;

#[derive(Clone, Debug)]
pub enum AppError {
    NotFound,
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound => write!(f, "Not Found"),
        }
    }
}

impl std::error::Error for AppError {}

impl AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
        }
    }
}

#[component]
pub fn ErrorTemplate(
    #[prop(optional)] outside_errors: Option<Errors>,
    #[prop(optional)] errors: Option<RwSignal<Errors>>,
) -> impl IntoView {
    let errors = match outside_errors {
        Some(errors) => RwSignal::new(errors),
        None => errors.expect("expected a reactive error list"),
    };

    let errors = errors.get();
    let errors: Vec<AppError> = errors
        .into_iter()
        .filter_map(|(_key, error)| error.downcast_ref::<AppError>().cloned())
        .collect();

    cfg_if! { if #[cfg(feature = "ssr")] {
        let response = use_context::<ResponseOptions>();
        if let Some(response) = response
            && let Some(first_error) = errors.first()
        {
            response.set_status(first_error.status_code());
        }
    }}

    view! {
        <section style="padding: 2rem; color: #f2f4ff;">
            <h1>{if errors.len() > 1 { "Errors" } else { "Error" }}</h1>
            <For
                each=move || errors.clone().into_iter().enumerate()
                key=|(index, _)| *index
                children=move |(_, error)| {
                    let message = error.to_string();
                    let status = error.status_code();
                    view! {
                        <article>
                            <h2>{status.to_string()}</h2>
                            <p>{message}</p>
                        </article>
                    }
                }
            />
        </section>
    }
}
