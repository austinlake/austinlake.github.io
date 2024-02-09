#[rustfmt::skip::macros(view)]
use cfg_if::cfg_if;
use leptos::view;
use website::app::App;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        #[actix_web::main]
        async fn main() -> std::io::Result<()> {
            use actix_files::{Files, NamedFile};
            use actix_web::{get, HttpServer, middleware::Compress, Responder};
            use leptos::get_configuration;
            use leptos_actix::{generate_route_list, handle_server_fns, LeptosRoutes};
            
            let conf = get_configuration(None).await.unwrap();
            let addr = conf.leptos_options.site_addr;
            let routes = generate_route_list(|| view! { scope, <App/> });

            #[get("/style.css")]
            async fn css() -> impl Responder {
                NamedFile::open_async("/pkg/website.css").await
            }

            HttpServer::new(move || {
                let leptos_options = &conf.leptos_options;
                let site_root = &leptos_options.site_root;

                actix_web::App::new()
                    .route("/api/{tail:.*}", handle_server_fns())
                    .leptos_routes(
                        leptos_options.to_owned(),
                    routes.to_owned(),
                        || view! { <App/> },
                    )
                    .service(Files::new("/", site_root))
                .wrap(Compress::default())
            })
            .bind(&addr)?
            .run()
            .await
        }
    } else {
        fn main() {
            tracing_subscriber::fmt::init();
            leptos::mount_to_body(|| view! { <App/> })
        }
    }
}
