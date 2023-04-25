#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
#[rustfmt::skip::macros(view)]
use anyhow::{Error, Result};
use leptos::{
    component, leptos_dom::mount_to_body, log, on_cleanup, provide_context, tracing, view,
    IntoView, Scope
};
use leptos_meta::{Stylesheet, StylesheetProps};
use leptos_router::{Route, RouteProps, Router, RouterProps, Routes, RoutesProps};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct ExampleContext(i32);

#[component]
pub fn Navigation(cx: Scope) -> impl IntoView {
    log::debug!("Rendering website.");

    provide_context(cx, ExampleContext(0));

    view! { cx,
        <div>
            <Router>
                <nav class="flex justify-evenly items-center align-middle gap-10 bg-gray-500 h-[50px]">
                    <a class="inline-block w-[100px] h-[80%] rounded-[10px] bg-gray-500 hover:bg-gray-700 text-white text-center align-middle font-bold no-underline"
                    exact=true href="/" title="About"> "About" </a>
                    <a class="inline-block w-[100px] h-[80%] rounded-[10px] bg-gray-500 hover:bg-gray-700 text-white text-center align-middle font-bold no-underline"
                    href="projects" title="Projects"> "Projects" </a>
                    <a class="inline-block w-[100px] h-[80%] rounded-[10px] bg-gray-500 hover:bg-gray-700 text-white text-center align-middle font-bold no-underline"
                    href="art" title="Art"> "Art" </a>
                </nav>
                <main>
                    <Routes>
                        <Route path="" view=move |cx| view! { cx,  <About/> }/>
                        <Route path="projects" view=move |_| view! { cx,  <Projects/> }/>
                        <Route path="art" view=move |cx| view! { cx,  <Art/> }/>
                    </Routes>
                </main>
            </Router>
            <SocialMedia/>
        </div>
    }
}

#[component]
pub fn About(cx: Scope) -> impl IntoView {
    log::debug!("rendering <About/>");

    on_cleanup(cx, || {
        log!("cleaning up <About/>");
    });

    view! { cx,
        <div>
            <h1 class="flex overflow-hidden tracking-[2px] box-border text-white">
                { "Hello, and welcome to my personal website! I am a physics and astrophysics major at
                the University of California, Berkeley. My primary research interests are cosmology and the 
                study large scale structure." }
            </h1>
        </div>
    }
}

#[component]
pub fn Projects(cx: Scope) -> impl IntoView {
    log::debug!("rendering <Projects/>");

    on_cleanup(cx, || {
        log!("cleaning up <Projects/>");
    });

    view! { cx,
        <div>
            <h1 class="inline-block overflow-hidden tracking-[2px] whitespace-nowrap box-border text-white">
                { "My technical projects..." }
            </h1>
        </div>
    }
}

#[component]
pub fn Art(cx: Scope) -> impl IntoView {
    log::debug!("rendering <Art/>");

    on_cleanup(cx, || {
        log!("cleaning up <Art/>");
    });

    view! { cx,
        <div>
            <h1 class="inline-block overflow-hidden tracking-[2px] whitespace-nowrap box-border text-white">
                { "My art portfolio..." }
            </h1>
            <div class="flex gap-[4vmin] left-[50%] absolute">
                <img class="w-[40vmin] h-[50vmin] object-cover object-[100%_50%]" src="./assets/portraits/ivc_commencement.jpg" draggable="false"/>
                <img class="w-[40vmin] h-[50vmin] object-cover object-[100%_50%]" src="./assets/portraits/high_school_side_profile.jpg" draggable="false"/>
                <img class="w-[40vmin] h-[50vmin] object-cover object-[100%_50%]" src="./assets/portraits/golden_bear_welcome.jpg" draggable="false"/>
                <img class="w-[40vmin] h-[50vmin] object-cover object-[100%_50%]" src="./assets/portraits/sunset_at_tvt.jpg" draggable="false"/>
                <img class="w-[40vmin] h-[50vmin] object-cover object-[100%_50%]" src="./assets/portraits/dr_seuss.jpg" draggable="false"/>
            </div>
        </div>
    }
}

#[component]
pub fn SocialMedia(cx: Scope) -> impl IntoView {
    log::debug!("rendering <SocialMedia/>");

    on_cleanup(cx, || {
        log!("cleaning up <SocialMedia/>");
    });

    view! { cx,
        <footer class="flex justify-center align-middle p-[25px] gap-[10px]">
            <a href="https://www.facebook.com/austinlake04/" title="Facebook">
                <img class="w-[50px] h-[50px] grayscale hover:grayscale-0" src="./assets/logos/facebook_logo.svg"/>
            </a>
            <a href="https://twitter.com/austinlake04" title="Twitter">
                <img class="w-[50px] h-[50px] grayscale hover:grayscale-0" src="./assets/logos/twitter_logo.svg"/>
            </a>
            <a href="https://www.instagram.com/austinlake04/" title="Instagram">
                <img class="w-[50px] h-[50px] grayscale hover:grayscale-0" src="./assets/logos/instagram_logo.svg"/>
            </a>
            <a href="https://www.linkedin.com/in/austinlake04/" title="LinkedIn">
                <img class="w-[50px] h-[50px] grayscale hover:grayscale-0" src="./assets/logos/linkedin_logo.svg"/>
            </a>
            <a href="https://github.com/austinlake04" title="GitHub">
                <img class="w-[50px] h-[50px] grayscale hover:grayscale-0" src="./assets/logos/github_logo.svg"/>
            </a>
        </footer>
    }
}

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|cx| view! { cx, <Navigation/> });
}
