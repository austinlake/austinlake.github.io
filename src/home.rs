use leptos::*;
use tracing::{debug, info};
use rand::{thread_rng, Rng};

#[component]
pub fn HomePage() -> impl IntoView {
    debug!("rendering <Home/>");

    on_cleanup(|| {
        info!("cleaning up <Home/>");
    });

    view! {
        <div
            class="flex flex-col justify-center items-center
                gap-[2vmin] absolute">
            <Background/>
        </div>
    }
}

#[component]
fn Background() -> impl IntoView {
    view! {
        <div class="bg-black animate-twinkle absolute h-1/2">
            <StaryNight/>
            <StaryNight/>
            <StaryNight/>
            <StaryNight/>
            <StaryNight/>
        </div>
    }
}

#[component]
fn StaryNight() -> impl IntoView {
    let mut rng = thread_rng();
    let mut stars = [[0u8; 3]; 20];
    for i in 0..20 {
        stars[i][0] = rng.gen_range(0..=100); // x coordinate
        stars[i][1] = rng.gen_range(0..=100); // y coordinate
        stars[i][2] = rng.gen_range(0..=5); // pixel radius
    }
    view! {
        <div class="relative">
            <For
                each= move || { stars }
                key=|star| { *star }
                children=move |star: [u8; 3]| {
                    view! {
                        <Star
                            x=star[0]
                            y=star[1]
                            radius=star[2] 
                        />
                    }
                }
            />
        </div>
    }
}

#[component]
fn Star(
    x: u8,
    y: u8,
    radius: u8
) -> impl IntoView {
    view! {
        <div class=format!("w-[{radius}px] border-radius")>
        </div>
    }
}
