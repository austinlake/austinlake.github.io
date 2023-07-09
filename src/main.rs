#[rustfmt::skip::macros(view)]
use leptos::*;
use website::app::App;

#[cfg(not(feature = "ssr"))]
fn main() {
    tracing_subscriber::fmt::init();
    mount_to_body(|cx| view! { cx, <App/> })
}

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};

    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(|cx| view! { cx, <App/> });

    #[get("/style.css")]
    async fn css() -> impl Responder {
        actix_files::NamedFile::open_async("/pkg/website.css").await
    }
    // Explicit server function registration is no longer required
    // on the main branch. On 0.3.0 and earlier, uncomment the lines
    // below to register the server functions.
    // _ = GetPost::register();
    // _ = ListPostMetadata::register();

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        App::new()
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            .leptos_routes(
                leptos_options.to_owned(),
                routes.to_owned(),
                |cx| view! { cx, <App/> },
            )
            .service(Files::new("/", site_root))
        //.wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}
