#[rustfmt::skip::macros(view)]
// use leptos::{component, IntoView, Stylesheet, Title, view};
use leptos::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{Route, Router, Routes};
use tracing::debug;
use crate::{about::AboutPage, art::ArtPage, dark_mode::DarkModeToggle, home::HomePage, projects::ProjectsPage};

#[component]
pub fn App() -> impl IntoView {
    debug!("Rendering website.");

    provide_meta_context();

    view! {
        <Stylesheet
            id="stylesheet"
            href="/pkg/website.css"
        />
        <Title
            text="Austin Lucas Lake"
        />
        <div>
            <Header/>
            <main
                class="justify-center items-center p-y-[100px]
                    gap-y-[50px] h-[90vmin]">
                <Router>
                    <Routes>
                        <Route
                            path=""
                            view=HomePage
                        />
                        <Route
                            path="about"
                            view=AboutPage
                        />
                        <Route
                            path="projects"
                            view=ProjectsPage
                        />
                        <Route
                            path="art"
                            view=ArtPage
                        />
                    </Routes>
                </Router>
            </main>
            <Footer/>
        </div>
    }
}


#[component]
fn Header() -> impl IntoView {
    view! {
        <div class="flex h-[100px] py-[50px] text-[30px] text-center items-center align-middle justify-evenly">
            <nav
                class="flex justify-evenly align-middle items-center w-[500px]">
                <NavItem
                    title="Home"
                    href="/"
                />
                <NavItem
                    title="About"
                    href="about"
                />
                <NavItem
                    title="Projects"
                    href="projects"
                />
                <NavItem
                    title="Art"
                    href="art"
                />
            </nav>
            <div>
                <DarkModeToggle/>
            </div>
        </div>
    }
}

#[component]
fn NavItem(
    title: &'static str,
    href: &'static str
) -> impl IntoView {
    view! {
        <a
            class="inline-block h-[5vmin] rounded-[2vmin]
                text-center align-middle items-center
                font-bold no-underline group transition-all
                duration-300 ease-in-out"
            exact=true
            href=href
            title=title>
            <span
                class="bg-left-bottom bg-gradient-to-r from-current
                    to-current bg-[length:0%_2px] bg-no-repeat
                    group-hover:bg-[length:100%_2px]
                    transition-all duration-500 ease-out select-none">
                { title }
            </span>
        </a>
    }
}

#[component]
fn FooterCard(
    title: &'static str,
    href: &'static str
) -> impl IntoView {
    view! {
        <a
            class="flex flex-row flex-nowrap gap-x-[10px] justify-left
                align-center text-gray-500 fill-gray-500 hover:text-black"
            href=href
            title=title>
            <span>
                { title }
            </span>
        </a>
    }
}

#[component]
fn SocialMedia() -> impl IntoView {
    view! {
        <div
            class="flex flex-col justify-left align-middle gap-y-[10px]
                h-[5vmin]">
            <h3
                class="inline-block"> "Social Media" </h3>
                <FooterCard
                    title="Facebook"
                    href="https://www.facebook.com/austinlucaslake"
                />
                <FooterCard
                    title="Twitter"
                    href="https://www.twitter.com/austinlucaslake"
                />
                <FooterCard
                    title="Instragram"
                    href="https://www.instagram.com/austinlucaslake"
                />
                <FooterCard
                    title="LinkedIn"
                    href="https://www.linkedin.com/austinlucaslake"
                />
                <FooterCard
                    title="Facebook"
                    href="https://www.github.com/austinlucaslake"
                />
        </div>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <div
            clas="justify-center pt-[50px] h-[50px]">
            <div
                class="flex justify-center border-t-[2px] gap-y-[25px] gap-x-[50px]">
                // <SocialMedia/>
                <span
                    class="block align-center">
                    "Austin Lucas Lake © 2023 - Present"
                </span>
            </div>
        </div>
    }
}
