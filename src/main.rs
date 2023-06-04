#[rustfmt::skip::macros(view)]
use leptos::{
    component, leptos_dom::mount_to_body, log, on_cleanup, provide_context, view,
    IntoView, Scope
};
use leptos_router::{Route, RouteProps, Router, RouterProps, Routes, RoutesProps};
use tracing::debug;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct ExampleContext(i32);

#[component]
pub fn Navigation(cx: Scope) -> impl IntoView {
    debug!("Rendering website.");

    provide_context(cx, ExampleContext(0));

    return view! { cx,
        <div>
            <Router>
                <nav class="flex justify-evenly text-center align-middle items-center text-[3vmin] h-[5vmin] dark:invert">
                    <a class="inline-block h-[5vmin] rounded-[2vmin] text-black text-center align-middle items-center font-bold no-underline" exact=true href="/" title="Home"> "Home" </a>
                    <a class="inline-block h-[5vmin] rounded-[2vmin] text-black text-center align-middle items-center font-bold no-underline" href="about" title="About"> "About" </a>
                    // <a class="inline-block h-[5vmin] rounded-[2vmin] text-black text-center align-middle items-center font-bold no-underline" href="projects" title="Projects"> "Projects" </a>
                    // <a class="inline-block h-[5vmin] rounded-[2vmin] text-black text-center align-middle items-center font-bold no-underline" href="art" title="Art"> "Art" </a>
                </nav>
                <main>
                    <Routes>
                        <Route path="" view=move |cx| view! { cx,  <Home/> }/>
                        <Route path="about" view=move |cx| view! { cx,  <About/> }/>
                        // <Route path="projects" view=move |_| view! { cx,  <Projects/> }/>
                        // <Route path="art" view=move |cx| view! { cx,  <Art/> }/>
                    </Routes>
                </main>
            </Router>
        </div>
    };
}

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    debug!("rendering <Home/>");

    on_cleanup(cx, || {
        log!("cleaning up <Home/>");
    });

    return view! { cx,
        <div class="flex flex-col justify-center items-center p-[2vmin] gap-[2vmin]">
            <h1 class="inline-block pb-[50px] dark:invert text-center align-middle text-[10vmin]"> "Welcome to my website." </h1>
            <h2 class="inline-block pb-[50px] dark:invert text-center align-middle text-[5vmin]"> "Built from sratch." </h2>
            <div class="flex h-1/2 justify-center items-center">
                <img class="inline dark:invert w-auto h-[15vmin]" src="https://www.rust-lang.org/logos/rust-logo-blk.svg"/>
                <img class="hidden dark:inline w-auto h-[15vmin]" src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_pref_dark_RGB.svg"/>
                <img class="inline dark:hidden w-auto h-[15vmin]" src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg"/>
                <img class="inline w-auto h-[10vmin]" src="https://tailwindcss.com/_next/static/media/tailwindcss-mark.3c5441fc7a190fb1800d4a5c7f07ba4b1345a9c8.svg"/>
                <img class="inline w-auto h-[10vmin] px-[5vmin]" src="https://raw.githubusercontent.com/carlosbaraza/web-assembly-logo/f0f411529c1dafffa233be1bd95b80b79144b675/dist/icon/web-assembly-icon.svg"/>
            </div>
        </div>
        <div class="flex justify-center relative bottom pt-[10vmin] gap-[1vmin]">
            <a href="https://github.com/austinlake04/website/blob/main/LICENSE-APACHE" title="Apache-2.0 License"> <img src="https://img.shields.io/badge/license-Apache--2.0-blue"/> </a>
            <a href="https://github.com/austinlake04/website/blob/main/LICENSE-MIT" title="MIT License"> <img src="https://img.shields.io/badge/license-MIT-blue"/> </a>
            // <a title="docs"> <img src="https://img.shields.io/badge/docs-passing-green"/> </a>
            <a href="https://img.shields.io/github/actions/workflow/status/austinlake04/website/CI.yaml?style=flat-square&branch=main" title="Rust CI"> <img src="https://github.com/austinlake04/website/actions/workflows/CI/badge.svg"/> </a>
        </div>
    };
}


#[component]
pub fn About(cx: Scope) -> impl IntoView {
    debug!("rendering <About/>");

    on_cleanup(cx, || {
        log!("cleaning up <About/>");
    });

    return view! { cx,
        <div class="flex justify-center items-center px-[2.5vmin] gap-[2vmin]">
            <img class="w-[80vmin] rounded-[2vmin] self-start hidden xl:block" src="./assets/portraits/dr_seuss.jpg" draggable="false"/>
            <div class="inline-block w-[80vin] p-[5px]">
                <h1 class="flex flex-col pb-[50px] text-center align-middle text-[10vmin]">
                    <span class="text-center text-black dark:invert"> "My name is " </span>
                    <div class="inline-block">
                        <span class="inline-block bg-amber-500 text-transparent bg-clip-text animate-color_change_reverse hover:animate-color_change"> "Austin Lake" </span>
                        <span class="text-center text-black dark:invert"> "." </span>
                    </div>
                </h1>
                <p class="inline-block text-[5vmin]">
                    <span class="text-black dark:invert">
                        "I am a physics and astrophysics major at the "
                    </span>
                    <span class="bg-amber-500 text-transparent bg-clip-text animate-color_change_reverse hover:animate-color_change">
                        "University of California, Berkeley"
                    </span>
                    <span class="text-black dark:invert">
                        ". My primary research interests are active galactic nuclei, galaxy formation, large scale structure and cosmology.
                        To that end, I would like to develop "
                    </span>
                    <span class="bg-amber-500 text-transparent bg-clip-text animate-color_change_reverse hover:animate-color_change">
                        "instrumentation"
                    </span>
                    <span class="text-black dark:invert">
                        " for space-based telescopes to study the early formation of structure in the universe."
                    </span>
                </p>
            </div>
        </div>
        <div class="p-[20vmin]">
            <h2 class="pb-[20px] text-center text-black dark:invert text-[6.5vmin]"> "Current Affiliations:" </h2>
            <div class="flex flex-wrap justify-center items-center gap-[5vmin]">
                <img class="h-[20vmin]" src="https://upload.wikimedia.org/wikipedia/commons/a/a1/Seal_of_University_of_California%2C_Berkeley.svg" draggable="false"/>
                <img class="h-[20vmin]" src="https://cdn.worldvectorlogo.com/logos/nasa-6.svg" draggable="false"/>
            </div>
        </div>
        <SocialMedia/>
    };
}

#[component]
pub fn Projects(cx: Scope) -> impl IntoView {
    debug!("rendering <Projects/>");

    on_cleanup(cx, || {
        log!("cleaning up <Projects/>");
    });

    return view! { cx,
        <div class="justify-center">
            <h1 class="text-center text-[10vmin]">
                <span class="text-black dark:invert"> "Technical Projects" </span>
            </h1>
            <div class="flex bg-white justify-center h-[100vmin] w-auto p-[5vmin]">
                <div class="w-[400px] h-[600px] bg-red-500 justify-center">
                    <div class="inline-block h-1/6 justify-center text-center align-middle"> "Project Title" </div>
                    <div class="flex bg-yellow-500 gap-[10px] p-[5vmin]">
                        <div class="inline-block rounded-full text-white text-center align-middle bg-green-500 w-[100px] h-[25px]"> "Python" </div>
                        <div class="inline-block rounded-full text-white text-center align-middle bg-green-500 w-[100px] h-[25px]"> "Pandas" </div>
                        <div class="inline-block rounded-full text-white text-center align-middle bg-green-500 w-[100px] h-[25px]"> "NumPy" </div>
                        <div class="inline-block rounded-full text-white text-center align-middle bg-green-500 w-[100px] h-[25px]"> "PyTorch" </div>
                    </div>
                    <div class="flex justify-center bg-blue-500">
                        <p class="inline-block"> "Project description" </p>
                    </div>
                    <div class="flex justify-center items-center h-1/6">
                        <a class="inline-block w-4/5 rounded-full bg-indigo-700 text-white text-center font-bold no-underline" href="https://github.com/austinlake04/desiforecast" title="Home"> "Source Code" </a>
                    </div>
                </div>
            </div>
        </div>
    };
}

#[component]
pub fn Art(cx: Scope) -> impl IntoView {
    debug!("rendering <Art/>");

    on_cleanup(cx, || {
        log!("cleaning up <Art/>");
    });

    // let (mouse_down_at, set_mouse_down_at) = create_signal(cx, 0);
    // let (percentage, set_percentage) = create_signal(cx, 0);
    // let (prev_percentage, set_prev_percentage) = create_signal(cx, 0);
    // let mouse_event = ev::MouseEvent::new("client_x").expect("failed to create mouse event");

    view! { cx,
        <h1 class="text-center text-[10vmin]">
            <span class="text-black dark:invert"> "Art Portfolio" </span>
        </h1>
        <div class="flex absolute transform translate-x-50 gap-[4vmin] left-[50%]">
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

    return view! { cx,
        <div class="left-0 right-0 m-auto relative bottom-[2vmin] flex flex-col justify-center">
            <h2 class="pb-[20px] text-center text-black dark:invert text-[6.5vmin]"> "Social Media:" </h2>
            <div class="flex flex-wrap justify-center items-center align-middle gap-[5vmin]">
                <a href="https://www.facebook.com/austinlake04" title="Facebook">
                    <img class="h-[5vmin] dark:invert" src="./assets/social_media/facebook_black.svg"/>
                </a>
                <a href="https://www.twitter.com/austinlake04" title="Twitter">
                    <img class="h-[5vmin] dark:invert" src="./assets/social_media/twitter_black.svg"/>
                </a>
                <a href="https://www.instagram.com/austinlake04" title="Instagram">
                    <img class="h-[5vmin] dark:invert" src="./assets/social_media/instagram_black.svg"/>
                </a>
                <a href="https://www.linkedin.com/in/austinlake04" title="LinkedIn">
                    <img class="h-[5vmin] dark:invert" src="./assets/social_media/linkedin_black.svg"/>
                </a>
                <a href="https://www.github.com/austinlake04" title="GitHub">
                    <img class="h-[5vmin] dark:invert" src="./assets/social_media/github_black.svg"/>
                </a>
            </div>
        </div>
    };
}

pub fn main() {
    tracing_subscriber::fmt::init();
    mount_to_body(|cx| view! { cx, <Navigation/> });
}
