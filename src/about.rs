use leptos::*;
use tracing::{debug, info};

#[component]
pub fn AboutPage() -> impl IntoView {
    debug!("rendering <About/>");

    on_cleanup(|| {
        info!("cleaning up <About/>");
    });

    view! {
        <div class="flex flex-col justify-center items-center align-center
                    px-[5vmin] gap-[5vmin]">
            <div class="flex justify-evenly items-center align-center
                        px-[5vmin] gap-x-[10vmin]">
                <p class="block text-[25px] w-[400px]">
                    "I am currently a physics major at the University of
                    California, Berkeley. My primary research interests are in
                    high resolution and direct imaging of exoplanets for
                    ground-based telescopes and space missions. To that end, am
                    egear to get contribute to instrumentation research
                    involving coronagraph development, lens design, adaptive
                    optics, wavefront sensing, and detectors."
                </p>
                <img
                    class="w-[200px] h-[400px] object-cover"
                    src="portraits/ivc_commencement.jpg"
                />
            </div>
        </div>
    }
}
