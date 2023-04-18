use stylist::Style;
use yew::{ prelude::*,
        html,
        Callback, 
        TargetCast,
};
use yew_router::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{console, HtmlElement, MouseEvent, Window, Document, Element};
use std::cmp;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/art")]
    Art,
    #[at("/projects")]
    Projects,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(SocialMedia)]
fn social_media() -> Html {
    html! {
        <footer>
                <div class={classes!("social_media")}>
                    <a href="https://www.facebook.com/austinlake04/" title="Facebook">
                        <img class={classes!("logo")} src="assets/logos/facebook_logo.svg"/></a>
                    <a href="https://twitter.com/austinlake04" title="Twitter">
                        <img class={classes!("logo")} src="assets/logos/twitter_logo.svg"/></a>
                    <a href="https://www.instagram.com/austinlake04/" title="Instagram">
                        <img class={classes!("logo")} src="assets/logos/instagram_logo.svg"/></a>
                    <a href="https://www.linkedin.com/in/austinlake04/" title="LinkedIn">
                        <img class={classes!("logo")} src="assets/logos/linkedin_logo.svg"/></a>
                    <a href="https://github.com/austinlake04" title="GitHub">
                        <img class={classes!("logo")} src="assets/logos/github_logo.svg"/></a>
                </div>
        </footer>
    }
}


#[function_component(NavigationBar)]
fn navigation_bar() -> Html {
    let navigator = use_navigator().unwrap();
    let art_navigator = navigator.clone();
    let projects_navigator = navigator.clone();

    html! {
        <header>
            <button onclick={ move |_| navigator.push(&Route::Home) }>{ "Home" }</button>
            <button onclick={ move |_| art_navigator.push(&Route::Art) }>{ "Art" }</button>
            <button onclick ={ move |_| projects_navigator.push(&Route::Projects) }>{ "Projects" }</button>
        </header>
    }
}


fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <h1>{ "My personal portfolio website, built from scratch in Rust using the Yew framework" }</h1>
        },
        Route::Art => html! { 
            <div id="track">
                <img class={classes!("image")} src="assets/portraits/ivc_commencement.jpg" draggable="false"/>
                <img class={classes!("image")} src="assets/portraits/high_school_side_profile.jpg" draggable="false"/>
                <img class={classes!("image")} src="assets/portraits/golden_bear_welcome.jpg" draggable="false"/>
                <img class={classes!("image")} src="assets/portraits/sunset_at_tvt.jpg" draggable="false"/>
                <img class={classes!("image")} src="assets/portraits/dr_seuss.jpg" draggable="false"/>
            </div>    
        },
        Route::Projects => html! { <h1> { "TODO: Projects"} </h1> },
        Route::NotFound => html! { <h1> { "This page does not exist." } </h1> },
    }
}

#[function_component]
fn App() -> Html {

    const STYLESHEET: &str = include_str!("../style.css");

    let stylesheet = Style::new(STYLESHEET).unwrap();

    html! {
        <div class={stylesheet}>
            <BrowserRouter>
                <NavigationBar/>
                <Switch<Route> render={switch} />
                <SocialMedia/>
            </BrowserRouter>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}