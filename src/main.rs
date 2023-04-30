#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
#[rustfmt::skip::macros(view)]
use color_eyre::eyre::{Error, Result};
use leptos::{
    component, leptos_dom::mount_to_body, log, on_cleanup, provide_context, tracing, view,
    IntoView, Scope
};
use leptos_meta::{Stylesheet, StylesheetProps};
use leptos_router::{Route, RouteProps, Router, RouterProps, Routes, RoutesProps};
use std::process::Command;
use tracing::debug;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct ExampleContext(i32);

#[component]
pub fn Navigation(cx: Scope) -> impl IntoView {
    debug!("Rendering website.");

    provide_context(cx, ExampleContext(0));

    view! { cx,
        <div>
            <Router>
                <nav class="flex justify-evenly p-[20px]">
                    <a class="w-[100px] h-[50px] rounded-[10px] animate-color_change_reverse hover:animate-color_change text-white text-center font-bold no-underline" exact=true href="/" title="Home"> "Home" </a>
                    <a class="w-[100px] h-[50px] rounded-[10px] animate-color_change_reverse hover:animate-color_change text-white text-center font-bold no-underline" href="projects" title="Projects"> "Projects" </a>
                    <a class="w-[100px] h-[50px] rounded-[10px] animate-color_change_reverse hover:animate-color_change text-white text-center font-bold no-underline" href="art" title="Art"> "Art" </a>
                </nav>
                <main>
                    <Routes>
                        <Route path="" view=move |cx| view! { cx,  <Home/> }/>
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
pub fn Home(cx: Scope) -> impl IntoView {
    debug!("rendering <Home/>");

    on_cleanup(cx, || {
        log!("cleaning up <Home/>");
    });

    view! { cx,
        <div>
            <div class="flex justify-center items-center p-[20px] gap-[20px]">
                <img class="w-[308px] md:w-[405px] lg:w-[510px] xl:w-[615px] rounded-[2vmin] align-bottom hidden sm:block" src="./assets/portraits/dr_seuss.jpg" draggable="false"/>
                <div class="inline-block w-[308px] md:w-[405px] lg:w-[510px] xl:w-[615px] p-[5px]">
                    <h1 class="flex flex-col pb-[50px] text-center align-middle text-4xl md:text-4xl lg:text-6xl xl:text-7xl 2xl:text-8xl">
                        <span class="text-center text-black dark:text-white"> "My name is " </span>
                        <div class="inline-block">
                            <span class="inline-block bg-amber-500 text-transparent bg-clip-text animate-color_change_reverse hover:animate-color_change"> "Austin Lake" </span>
                            <span class="text-center text-black dark:text-white"> "." </span>
                        </div>
                    </h1>
                    <p class="inline-block text-xl md:text-2xl lg:text-3xl xl:text-4xl 2xl:text-5xl">
                        <span class="text-black dark:text-white">
                            "I am a physics and astrophysics major at the "
                        </span>
                        <span class="bg-amber-500 text-transparent bg-clip-text animate-color_change_reverse hover:animate-color_change">
                            "University of California, Berkeley"
                        </span>
                        <span class="text-black dark:text-white">
                            ". My primary research interests are active galactic nuclei, galaxy formation, large scale structure and cosmology.
                            To that end, I would like to develop "
                        </span>
                        <span class="bg-amber-500 text-transparent bg-clip-text animate-color_change_reverse hover:animate-color_change">
                            "instrumentation"
                        </span>
                        <span class="text-black dark:text-white">
                            " for space-based telescopes to study the early formation of structure in the universe."
                        </span>
                    </p>
                </div>
            </div>
            <div class="p-[20vmin]">
                <h2 class="pb-[20px] text-center text-black dark:text-white text-xl md:text-2xl lg:text-3xl xl:text-4xl 2xl:text-5xl"> "Current Affiliations:" </h2>
                <div class="flex flex-wrap justify-center items-center">
                    <img class="w-[110px] h-[110px]" src="./assets/logos/uc_berkeley.svg" draggable="false"/>
                    <img class="w-[150px] h-[150px]" src="./assets/logos/nasa.svg" draggable="false"/>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn Projects(cx: Scope) -> impl IntoView {
    debug!("rendering <Projects/>");

    on_cleanup(cx, || {
        log!("cleaning up <Projects/>");
    });

    view! { cx,
        <h1 class="text-center text-4xl md:text-5xl lg:text-6xl xl:text-7xl 2xl:text-8xl">
            <span class="text-black dark:text-white"> "Technical Projects" </span>
        </h1>
    }
}

#[component]
pub fn Art(cx: Scope) -> impl IntoView {
    debug!("rendering <Art/>");

    on_cleanup(cx, || {
        log!("cleaning up <Art/>");
    });

    view! { cx,
        <h1 class="text-center text-4xl md:text-5xl lg:text-6xl xl:text-7xl 2xl:text-8xl">
            <span class="text-black dark:text-white"> "Art Portfolio" </span>
        </h1>
        <div class="grid grid-cols-3 gap-[4vmin] left-[50%]">
            <img class="w-[40vmin] h-[50vmin] object-cover object-[100%_50%]" src="./assets/portraits/ivc_commencement.jpg" draggable="false"/>
            <img class="w-[40vmin] h-[50vmin] object-cover object-[100%_50%]" src="./assets/portraits/high_school_side_profile.jpg" draggable="false"/>
            <img class="w-[40vmin] h-[50vmin] object-cover object-[100%_50%]" src="./assets/portraits/golden_bear_welcome.jpg" draggable="false"/>
            <img class="w-[40vmin] h-[50vmin] object-cover object-[100%_50%]" src="./assets/portraits/sunset_at_tvt.jpg" draggable="false"/>
            <img class="w-[40vmin] h-[50vmin] object-cover object-[100%_50%]" src="./assets/portraits/dr_seuss.jpg" draggable="false"/>
        </div>
    }
}

#[component]
pub fn SocialMedia(cx: Scope) -> impl IntoView {
    debug!("rendering <SocialMedia/>");

    on_cleanup(cx, || {
        log!("cleaning up <SocialMedia/>");
    });

    view! { cx,
        <footer class="left-0 right-0 m-auto relative bottom-[10px] flex justify-center align-middle p-[25px] gap-[25px]">
            <a href="https://www.facebook.com/austinlake04" title="Facebook">
                <img class="w-[50px] h-[50px] dark:hidden" src="./assets/logos/facebook_black.svg"/>
                <img class="w-[50px] h-[50px] hidden dark:inline" src="./assets/logos/facebook_white.svg"/>
            </a>
            <a href="https://www.twitter.com/austinlake04" title="Twitter">
                <img class="w-[50px] h-[50px] dark:hidden" src="./assets/logos/twitter_black.svg"/>
                <img class="w-[50px] h-[50px] hidden dark:inline" src="./assets/logos/twitter_white.svg"/>
            </a>
            <a href="https://www.instagram.com/austinlake04" title="Instagram">
                <img class="w-[50px] h-[50px] dark:hidden" src="./assets/logos/instagram_black.svg"/>
                <img class="w-[50px] h-[50px] hidden dark:inline" src="./assets/logos/instagram_white.svg"/>
            </a>
            <a href="https://www.linkedin.com/in/austinlake04" title="LinkedIn">
                <img class="w-[50px] h-[50px] dark:hidden" src="./assets/logos/linkedin_black.svg"/>
                <img class="w-[50px] h-[50px] hidden dark:inline" src="./assets/logos/linkedin_white.svg"/>
            </a>
            <a href="https://www.github.com/austinlake04" title="GitHub">
                <img class="w-[50px] h-[50px] dark:hidden" src="./assets/logos/github_black.svg"/>
                <img class="w-[50px] h-[50px] hidden dark:inline" src="./assets/logos/github_white.svg"/>
            </a>
        </footer>
    }
}

pub fn main() {
    tracing_subscriber::fmt::init();
    mount_to_body(|cx| view! { cx, <Navigation/> });
}
