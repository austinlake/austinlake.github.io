use leptos::*;
use leptos_meta::Meta;
use leptos_router::ActionForm;

#[server(ToggleDarkMode, "/api")]
pub async fn toggle_dark_mode(prefers_dark: bool) -> Result<bool, ServerFnError> {
    use actix_web::http::header::{HeaderMap, HeaderValue, SET_COOKIE};
    use leptos_actix::{ResponseOptions, ResponseParts};

    let response =
        use_context::<ResponseOptions>().expect("to have leptos_actix::ResponseOptions provided");
    let mut response_parts = ResponseParts::default();
    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        HeaderValue::from_str(&format!("darkmode={prefers_dark}; Path=/"))
            .expect("to create header value"),
    );
    response_parts.headers = headers;

    std::thread::sleep(std::time::Duration::from_millis(250));

    response.overwrite(response_parts);
    Ok(prefers_dark)
}

#[cfg(not(feature = "ssr"))]
fn initial_prefers_dark() -> bool {
    use wasm_bindgen::JsCast;
    use web_sys::HtmlDocument;

    let doc = document().unchecked_into::<HtmlDocument>();
    let cookie = doc.cookie().unwrap_or_default();
    cookie.contains("darkmode=true")
}

#[cfg(feature = "ssr")]
fn initial_prefers_dark() -> bool {
    use_context::<actix_web::HttpRequest>()
        .and_then(|req| {
            req.cookies()
                .map(|cookies| {
                    cookies
                        .iter()
                        .any(|cookie| cookie.name() == "darkmode" && cookie.value() == "true")
                })
                .ok()
        })
        .unwrap_or(false)
}

#[component]
pub fn DarkModeToggle() -> impl IntoView {
    let initial = initial_prefers_dark();

    let toggle_dark_mode_action = create_server_action::<ToggleDarkMode>();
    let input = toggle_dark_mode_action.input();
    let value = toggle_dark_mode_action.value();

    let prefers_dark = move || {
        match (input.get(), value.get()) {
            (Some(input), _) => input.prefers_dark,
            (_, Some(Ok(value))) => value,
            _ => initial,
        }
    };

    let color_scheme = move || {
        if prefers_dark() {
            "dark".to_string()
        } else {
            "light".to_string()
        }
    };

    view! {
        <Meta
            name="color-scheme"
            content=color_scheme
        />
        <ActionForm action=toggle_dark_mode_action>
            <input
                type="hidden"
                name="prefers_dark"
                value=move || (!prefers_dark()).to_string()
            />
            <input
                type="image"
                width=50
                height=50
                class=move || {
                    if prefers_dark() {
                        "invert"
                    } else {
                        ""
                    }
                }
                src=move || {
                    if prefers_dark() {
                        "https://unpkg.com/ionicons@7.1.0/dist/svg/sunny.svg"
                    } else {
                        "https://unpkg.com/ionicons@7.1.0/dist/svg/moon.svg"
                    }
                }
            />
        </ActionForm>
    }
}