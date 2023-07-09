#[rustfmt::skip::macros(view)]
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use styled::style;
use tracing::debug;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct ExampleContext(i32);

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    debug!("Rendering website.");

    // provide_meta_context(cx);
    provide_meta_context(cx);

    let _styles = style!(
        body {
            background-color: rgb(255, 255, 255);
        }
    
        @media (prefers-color-scheme: dark) {
            body {
                background-color: rgb(0, 0, 0);
            }
        }
    );

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/website.css" />
        <Title text="Austin Lucas Lake" />
        <div>
            <Router>
                <nav class="flex justify-evenly text-center align-middle items-center text-[3vmin] py-[5vmin] h-[5vmin] animate-fade_in dark:invert">
                    <a class="inline-block h-[5vmin] rounded-[2vmin] text-center align-middle items-center font-bold no-underline group transition-all duration-300 ease-in-out" exact=true href="/" title="Home">
                        <span class="bg-left-bottom bg-gradient-to-r from-black to-black bg-[length:0%_2px] bg-no-repeat group-hover:bg-[length:100%_2px] transition-all duration-500 ease-out">
                            "Home"
                        </span>
                    </a>
                    <a class="inline-block h-[5vmin] rounded-[2vmin] text-center align-middle items-center font-bold no-underline group transition-all duration-300 ease-in-out" href="about" title="About">
                        <span class="bg-left-bottom bg-gradient-to-r from-black to-black bg-[length:0%_2px] bg-no-repeat group-hover:bg-[length:100%_2px] transition-all duration-500 ease-out">  
                            "About"
                        </span>
                    </a>
                    // <a class="inline-block h-[5vmin] rounded-[2vmin] text-center align-middle items-center font-bold no-underline group transition-all duration-300 ease-in-out" href="projects" title="Projects">
                    //     <span class="bg-left-bottom bg-gradient-to-r from-black to-black bg-[length:0%_2px] bg-no-repeat group-hover:bg-[length:100%_2px] transition-all duration-500 ease-out">  
                    //         "Projects"
                    //     </span>
                    // </a>
                    // <a class="inline-block h-[5vmin] rounded-[2vmin] text-center align-middle items-center font-bold no-underline group transition-all duration-300 ease-in-out" href="Art" title="Art">
                    //     <span class="bg-left-bottom bg-gradient-to-r from-black to-black bg-[length:0%_2px] bg-no-repeat group-hover:bg-[length:100%_2px] transition-all duration-500 ease-out">  
                    //         "Art"
                    //     </span>
                    // </a>
                </nav>
                <main class="flex flex-col justify-center items-center pt-[5vmin] gap-y-[50px]">
                    <Routes>
                        <Route path="" view=move |cx| view! { cx,  <Home/> }/>
                        <Route path="about" view=move |cx| view! { cx,  <About/> }/>
                        // <Route path="projects" view=move |_| view! { cx,  <Projects/> }/>
                        // <Route path="art" view=move |cx| view! { cx,  <Art/> }/>
                    </Routes>
                    <Footers/>
                </main>
            </Router>
        </div>
    }
}

#[component]
fn Home(cx: Scope) -> impl IntoView {
    debug!("rendering <Home/>");

    on_cleanup(cx, || {
        log!("cleaning up <Home/>");
    });

    view! { cx,
        <div class="flex flex-col justify-center items-center gap-[2vmin]">
            <h1 class="block overflow-hidden pb-[50px] align-middle text-[5vmin] animate-falling dark:invert">
                <span class="text-center"> "My name is " </span>
                <span class="text-transparent bg-clip-text bg-black animate-falling hover:animate-highlight hover:after:animate-highlight_rev dark:hover:animate-dark_highlight dark:hover:after:animate-dark_highlight_rev">
                    "Austin Lucas Lake"
                </span>
                <span class="text-center"> "." </span>
            </h1>
            <h2 class="block pb-[10vmin] animate-fade_in text-center align-middle text-[3vmin] dark:invert"> "Welcome to my website. Built from scratch." </h2>
            // <WebTools/>
        </div>
        // <Badges/>
    }
}

#[component]
fn Orbit(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class="relative h-[100px] w-[100px]">
            <div class="absolute rounded-full animate-spin1">
                <div class="absolute rounded-full h-[5px] w-[5px]">
                
                </div>
            </div>
        </div>
    }
}

#[component]
fn WebTools(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class="flex h-1/2 justify-center items-center gap-x-[5vmin] animate-fade_in">
            <img title="Rust" class="inline w-auto h-[15vmin] dark:invert" src="https://www.rust-lang.org/logos/rust-logo-blk.svg"/>
            <img title="Leptos" class="inline w-auto h-[10vmin]" src="https://leptos.dev/images/leptos_circle.svg"/>
            <img title="TailwindCSS" class="inline w-auto h-[10vmin]" src="https://tailwindcss.com/_next/static/media/tailwindcss-mark.3c5441fc7a190fb1800d4a5c7f07ba4b1345a9c8.svg"/>
            <img title="WebAsembly" class="inline w-auto h-[10vmin]" src="https://raw.githubusercontent.com/carlosbaraza/web-assembly-logo/f0f411529c1dafffa233be1bd95b80b79144b675/dist/icon/web-assembly-icon.svg"/>
        </div>
    }
}

#[component]
fn Badges(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class="flex justify-center relative bottom pt-[10vmin] gap-[1vmin] animate-fade_in">
            <a href="https://github.com/austinlucaslake/website/releases " title="Latest Release"> <img class="grayscale hover:grayscale-0" src="https://img.shields.io/github/v/release/austinlucaslake/website?logo=github"/> </a>
            <a href="https://github.com/austinlucaslake/website/blob/main/LICENSE" title="License"> <img class="grayscale hover:grayscale-0" src="https://img.shields.io/github/license/austinlucaslake/website"/> </a>
            <a href="https://github.com/austinlucaslake/website/actions" title="CI Status"> <img class="grayscale hover:grayscale-0" src="https://github.com/austinlucaslake/website/actions/workflows/ci.yaml/badge.svg?event=pull_request"/> </a>
        </div>
    }
}

#[component]
fn About(cx: Scope) -> impl IntoView {
    debug!("rendering <About/>");

    on_cleanup(cx, || {
        log!("cleaning up <About/>");
    });

    view! { cx,
        <div class="flex flex-col justify-center items-center align-center px-[5vmin] gap-[5vmin]">
            <div class="flex justify-evenly items-center align-center px-[5vmin] gap-x-[10vmin]">
                <p class="block text-[25px] w-[400px] dark:invert">
                    <span>
                        "Currently, I'm a double major studying "
                    </span>
                    <span class="text-transparent bg-clip-text bg-black hover:animate-highlight hover:after:animate-highlight_rev dark:hover:animate-dark_highlight dark:hover:after:animate-dark_highlight_rev">
                        "physics"
                    </span>
                    <span>
                        " and "
                    </span>
                    <span class="text-transparent bg-clip-text bg-black hover:animate-highlight hover:after:animate-highlight_rev dark:hover:animate-dark_highlight dark:hover:after:animate-dark_highlight_rev">
                        "astrophysics"
                    </span>
                    <span>
                        " at the "
                    </span>
                    <span class="text-transparent bg-clip-text bg-black hover:animate-highlight hover:after:animate-highlight_rev dark:hover:animate-dark_highlight dark:hover:after:animate-dark_highlight_rev">
                        "University of California, Berkeley"
                    </span>
                    <span>
                        ". My primary research interests are related to high resolution and direct imaging of "
                    </span>
                    <span class="text-transparent bg-clip-text bg-black hover:animate-highlight hover:after:animate-highlight_rev dark:hover:animate-dark_highlight dark:hover:after:animate-dark_highlight_rev">
                        "exoplanets"
                    </span>
                    <span>
                        " for ground-based telescopes and space missions. To that end, I am eager to get involved in "
                    </span>
                    <span class="text-transparent bg-clip-text bg-black hover:animate-highlight hover:after:animate-highlight_rev dark:hover:animate-dark_highlight dark:hover:after:animate-dark_highlight_rev">
                        "instrumentation"
                    </span>
                    <span>
                        " research involving lens design, adaptive optics, wavefront sensing, and detectors."
                    </span>
                </p>
                <img class="w-[200px] [clip-path:ellipse(50%_50%_at_50%_50%)] hidden lg:block" src="portraits/ivc_commencement.jpg" draggable="false"/>
            </div>
            
            <div class="flex flex-wrap justify-center items-center gap-[5vmin]">
                <img class="h-[100px]" src="https://upload.wikimedia.org/wikipedia/commons/a/a1/Seal_of_University_of_California%2C_Berkeley.svg" draggable="false"/>
                <img class="h-[100px]" src="https://cdn.worldvectorlogo.com/logos/nasa-6.svg" draggable="false"/>
            </div>
        </div>
    }
}



#[component]
fn SocialMediaFooter(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex flex-col justify-left align-middle gap-y-[10px] h-[5vmin]">
            <h3 class="inline-block dark:invert"> "Social Media" </h3>
            <a class="flex flex-row flex-nowrap gap-x-[10px] justify-left align-center text-gray-500 fill-gray-500 hover:text-black hover:fill-black dark:hover:invert" href="https://www.facebook.com/austinlucaslake" title="Facebook">
                <svg viewBox="0 0 14222 14222" class="inline h-[25px] w-[25px]">
                    <defs>
                        <mask id="Facebook">
                            <circle cx="7111" cy="7112" r="7111" fill="#fff"/>
                            <path d="M9879 9168l315-2056H8222V5778c0-562 275-1111 1159-1111h897V2917s-814-139-1592-139c-1624 0-2686 984-2686 2767v1567H4194v2056h1806v4969c362 57 733 86 1111 86s749-30 1111-86V9168z" fill="#000"/>
                        </mask>
                    </defs>
                    <circle cx="7111" cy="7112" r="7111" mask="url(#Facebook)"/>
                </svg>
                <span> "Facebook" </span>
            </a>
            <a class="flex flex-row flex-nowrap gap-x-[10px] justify-left align-center text-gray-500 fill-gray-500 hover:text-black hover:fill-black dark:hover:invert" href="https://www.twitter.com/austinlucaslake" title="Twitter">
                <svg viewBox="0 0 248 204" class="inline h-[25px] w-[25px]">
                    <path d="M221.95,51.29c0.15,2.17,0.15,4.34,0.15,6.53c0,66.73-50.8,143.69-143.69,143.69v-0.04 C50.97,201.51,24.1,193.65,1,178.83c3.99,0.48,8,0.72,12.02,0.73c22.74,0.02,44.83-7.61,62.72-21.66 c-21.61-0.41-40.56-14.5-47.18-35.07c7.57,1.46,15.37,1.16,22.8-0.87C27.8,117.2,10.85,96.5,10.85,72.46c0-0.22,0-0.43,0-0.64 c7.02,3.91,14.88,6.08,22.92,6.32C11.58,63.31,4.74,33.79,18.14,10.71c25.64,31.55,63.47,50.73,104.08,52.76 c-4.07-17.54,1.49-35.92,14.61-48.25c20.34-19.12,52.33-18.14,71.45,2.19c11.31-2.23,22.15-6.38,32.07-12.26 c-3.77,11.69-11.66,21.62-22.2,27.93c10.01-1.18,19.79-3.86,29-7.95C240.37,35.29,231.83,44.14,221.95,51.29z"/>                    
                </svg>
                <span> "Twitter" </span>
            </a>
            <a class="flex flex-row flex-nowrap gap-x-[10px] justify-left align-center text-gray-500 fill-gray-500 hover:text-black hover:fill-black dark:hover:invert" href="https://www.instagram.com/austinlucaslake" title="Instagram">
                <svg viewBox="0 0 1000 1000" class="inline h-[25px] w-[25px]">
                    <path d="M295.42,6c-53.2,2.51-89.53,11-121.29,23.48-32.87,12.81-60.73,30-88.45,57.82S40.89,143,28.17,175.92c-12.31,31.83-20.65,68.19-23,121.42S2.3,367.68,2.56,503.46,3.42,656.26,6,709.6c2.54,53.19,11,89.51,23.48,121.28,12.83,32.87,30,60.72,57.83,88.45S143,964.09,176,976.83c31.8,12.29,68.17,20.67,121.39,23s70.35,2.87,206.09,2.61,152.83-.86,206.16-3.39S799.1,988,830.88,975.58c32.87-12.86,60.74-30,88.45-57.84S964.1,862,976.81,829.06c12.32-31.8,20.69-68.17,23-121.35,2.33-53.37,2.88-70.41,2.62-206.17s-.87-152.78-3.4-206.1-11-89.53-23.47-121.32c-12.85-32.87-30-60.7-57.82-88.45S862,40.87,829.07,28.19c-31.82-12.31-68.17-20.7-121.39-23S637.33,2.3,501.54,2.56,348.75,3.4,295.42,6m5.84,903.88c-48.75-2.12-75.22-10.22-92.86-17-23.36-9-40-19.88-57.58-37.29s-28.38-34.11-37.5-57.42c-6.85-17.64-15.1-44.08-17.38-92.83-2.48-52.69-3-68.51-3.29-202s.22-149.29,2.53-202c2.08-48.71,10.23-75.21,17-92.84,9-23.39,19.84-40,37.29-57.57s34.1-28.39,57.43-37.51c17.62-6.88,44.06-15.06,92.79-17.38,52.73-2.5,68.53-3,202-3.29s149.31.21,202.06,2.53c48.71,2.12,75.22,10.19,92.83,17,23.37,9,40,19.81,57.57,37.29s28.4,34.07,37.52,57.45c6.89,17.57,15.07,44,17.37,92.76,2.51,52.73,3.08,68.54,3.32,202s-.23,149.31-2.54,202c-2.13,48.75-10.21,75.23-17,92.89-9,23.35-19.85,40-37.31,57.56s-34.09,28.38-57.43,37.5c-17.6,6.87-44.07,15.07-92.76,17.39-52.73,2.48-68.53,3-202.05,3.29s-149.27-.25-202-2.53m407.6-674.61a60,60,0,1,0,59.88-60.1,60,60,0,0,0-59.88,60.1M245.77,503c.28,141.8,115.44,256.49,257.21,256.22S759.52,643.8,759.25,502,643.79,245.48,502,245.76,245.5,361.22,245.77,503m90.06-.18a166.67,166.67,0,1,1,167,166.34,166.65,166.65,0,0,1-167-166.34" transform="translate(-2.5 -2.5)"/>
                </svg>
                <span> "Instagram" </span>
            </a>
            <a class="flex flex-row flex-nowrap gap-x-[10px] justify-left align-center text-gray-500 fill-gray-500 hover:text-black hover:fill-black dark:hover:invert" href="https://www.instagram.com/austinlucaslake" href="https://www.linkedin.com/in/austinlucaslake" title="LinkedIn">
                <svg viewBox="0 0 256 256" class="inline h-[25px] w-[25px]">
                    <defs>
                        <mask id="LinkedIn">
                            <path d="M 0 6.447 C 0 2.887 2.978 0 6.651 0 h 76.698 C 87.022 0 90 2.887 90 6.447 v 77.106 C 90 87.114 87.022 90 83.349 90 H 6.651 C 2.978 90 0 87.114 0 83.553 V 6.447 z" fill="#fff" transform=" matrix(1 0 0 1 0 0) " stroke-linecap="round"/>   <path d="M 20.485 29.151 c 4.74 0 7.691 -3.121 7.691 -7.021 c -0.088 -3.988 -2.95 -7.022 -7.601 -7.022 c -4.65 0 -7.69 3.034 -7.69 7.022 c 0 3.9 2.95 7.021 7.512 7.021 H 20.485 L 20.485 29.151 z" transform=" matrix(1 0 0 1 0 0) " stroke-linecap="round" fill="#000"/>
                            <path d="M 27.282 75.339 v -40.64 H 13.688 v 40.64 H 27.282 z" transform=" matrix(1 0 0 1 0 0) " stroke-linecap="round" fill="#000"/>
                            <path d="M 34.804 75.339 h 13.594 V 52.644 c 0 -1.215 0.088 -2.428 0.447 -3.296 c 0.983 -2.427 3.219 -4.94 6.975 -4.94 c 4.919 0 6.887 3.727 6.887 9.19 v 21.741 h 13.592 V 52.037 c 0 -12.483 -6.706 -18.291 -15.65 -18.291 c -7.333 0 -10.553 4.073 -12.342 6.847 h 0.091 v -5.894 H 34.804 C 34.982 38.513 34.804 75.339 34.804 75.339 L 34.804 75.339 z" transform=" matrix(1 0 0 1 0 0) " stroke-linecap="round" fill="#000"/>
                    
                        </mask>
                    </defs>
                    <g mask="url(#LinkedIn)" transform="translate(1.4065934065934016 1.4065934065934016) scale(2.81 2.81)">
                        <path d="M 0 6.447 C 0 2.887 2.978 0 6.651 0 h 76.698 C 87.022 0 90 2.887 90 6.447 v 77.106 C 90 87.114 87.022 90 83.349 90 H 6.651 C 2.978 90 0 87.114 0 83.553 V 6.447 z" transform=" matrix(1 0 0 1 0 0) " stroke-linecap="round" />
                    </g>
                </svg>
                <span> "LinkedIn" </span>
            </a>
            <a class="flex flex-row flex-nowrap gap-x-[10px] justify-left align-center text-gray-500 fill-gray-500 hover:text-black hover:fill-black dark:hover:invert" href="https://www.github.com/austinlucaslake" title="GitHub">
                <svg viewBox="0 0 98 96" class="inline h-[25px] w-[25px]">
                    <path d="M48.854 0C21.839 0 0 22 0 49.217c0 21.756 13.993 40.172 33.405 46.69 2.427.49 3.316-1.059 3.316-2.362 0-1.141-.08-5.052-.08-9.127-13.59 2.934-16.42-5.867-16.42-5.867-2.184-5.704-5.42-7.17-5.42-7.17-4.448-3.015.324-3.015.324-3.015 4.934.326 7.523 5.052 7.523 5.052 4.367 7.496 11.404 5.378 14.235 4.074.404-3.178 1.699-5.378 3.074-6.6-10.839-1.141-22.243-5.378-22.243-24.283 0-5.378 1.94-9.778 5.014-13.2-.485-1.222-2.184-6.275.486-13.038 0 0 4.125-1.304 13.426 5.052a46.97 46.97 0 0 1 12.214-1.63c4.125 0 8.33.571 12.213 1.63 9.302-6.356 13.427-5.052 13.427-5.052 2.67 6.763.97 11.816.485 13.038 3.155 3.422 5.015 7.822 5.015 13.2 0 18.905-11.404 23.06-22.324 24.283 1.78 1.548 3.316 4.481 3.316 9.126 0 6.6-.08 11.897-.08 13.526 0 1.304.89 2.853 3.316 2.364 19.412-6.52 33.405-24.935 33.405-46.691C97.707 22 75.788 0 48.854 0z"/>
                </svg>
                <span> "GitHub" </span>
            </a>
        </div>
    }
}

#[component]
fn Footers(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class="flex flex-grid justify-center w-[90vmin] pt-[50px] border-t-[2px] animate-fade_in">
            // <ProjectsFooter/>
            // <ArtFooter/>
            <SocialMediaFooter/>
        </div>
    }
}


#[component]
fn Projects(cx: Scope) -> impl IntoView {
    debug!("rendering <Projects/>");

    on_cleanup(cx, || {
        log!("cleaning up <Projects/>");
    });

    view! { cx,
        <div class="justify-center">
            <h1 class="text-center text-[3vmin]">
                <span class="dark:text-white"> "Technical Projects" </span>
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
                        <a class="inline-block w-4/5 rounded-full bg-indigo-700 text-white text-center font-bold no-underline" href="https://github.com/austinlucaslake/desiforecast" title="Home"> "Source Code" </a>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn ProjectsFooter(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class="flex flex-col justify-left align-middle gap-y-[10px] h-[5vmin]">
            <h3 class="inline-block dark:invert"> "Projects" </h3>
            <a class="flex flex-row flex-nowrap gap-x-[10px] justify-left align-center text-gray-500 fill-gray-500 hover:text-black hover:fill-black dark:hover:invert" href="projects/1" title="Project 1">
                <span> "Project 1" </span>
            </a>
            <a class="flex flex-row flex-nowrap gap-x-[10px] justify-left align-center text-gray-500 fill-gray-500 hover:text-black hover:fill-black dark:hover:invert" href="projects/2" title="Project 2">
                <span> "Project 2" </span>
            </a>
            <a class="flex flex-row flex-nowrap gap-x-[10px] justify-left align-center text-gray-500 fill-gray-500 hover:text-black hover:fill-black dark:hover:invert" href="projects/3" title="Project 3">
                <span> "Project 3" </span>
            </a>
            <a class="flex flex-row flex-nowrap gap-x-[10px] justify-left align-center text-gray-500 fill-gray-500 hover:text-black hover:fill-black dark:hover:invert" href="projects/4" title="Project 4">
                <span> "Project 4" </span>
            </a>
        </div>
    }
}

#[component]
fn Art(cx: Scope) -> impl IntoView {
    debug!("rendering <Art/>");

    on_cleanup(cx, || {
        log!("cleaning up <Art/>");
    });

    // let (mouse_down_at, set_mouse_down_at) = create_signal(cx, 0);
    // let (percentage, set_percentage) = create_signal(cx, 0);
    // let (prev_percentage, set_prev_percentage) = create_signal(cx, 0);
    // let mouse_event = ev::MouseEvent::new("client_x").expect("failed to create mouse event");

    view! { cx,
        <h1 class="text-center text-[5vmin]">
            <span class="dark:text-white"> "Art Portfolio" </span>
        </h1>
        <div class="flex absolute transform translate-x-50 gap-[4vmin] left-[50%]">
            <img class="w-[40vmin] h-[50vmin] object-cover object-[100%_50%]" src="assets/portraits/ivc_commencement.jpg" draggable="false"/>
            <img class="w-[40vmin] h-[50vmin] object-cover object-[100%_50%]" src="assets/portraits/high_school_side_profile.jpg" draggable="false"/>
            <img class="w-[40vmin] h-[50vmin] object-cover object-[100%_50%]" src="assets/portraits/golden_bear_welcome.jpg" draggable="false"/>
            <img class="w-[40vmin] h-[50vmin] object-cover object-[100%_50%]" src="assets/portraits/sunset_at_tvt.jpg" draggable="false"/>
            <img class="w-[40vmin] h-[50vmin] object-cover object-[100%_50%]" src="assets/portraits/dr_seuss.jpg" draggable="false"/>
        </div>
    }
}

#[component]
pub fn ArtFooter(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class="flex flex-col justify-left align-middle gap-y-[10px] h-[5vmin]">
            <h3 class="inline-block dark:invert"> "Art" </h3>
            <a class="flex flex-row flex-nowrap gap-x-[10px] justify-left align-center text-gray-500 fill-gray-500 hover:text-black hover:fill-black dark:hover:invert" href="art/1" title="Art 1">
                <span> "Art 1" </span>
            </a>
            <a class="flex flex-row flex-nowrap gap-x-[10px] justify-left align-center text-gray-500 fill-gray-500 hover:text-black hover:fill-black dark:hover:invert" href="art/2" title="Art 2">
                <span> "Art 2" </span>
            </a>
            <a class="flex flex-row flex-nowrap gap-x-[10px] justify-left align-center text-gray-500 fill-gray-500 hover:text-black hover:fill-black dark:hover:invert" href="art/3" title="Art 3">
                <span> "Art 3" </span>
            </a>
            <a class="flex flex-row flex-nowrap gap-x-[10px] justify-left align-center text-gray-500 fill-gray-500 hover:text-black hover:fill-black dark:hover:invert" href="art/4" title="Art 4">
                <span> "Art 4" </span>
            </a>
        </div>
    }
}