#[rustfmt::skip::macros(view)]
use leptos::*;
use tracing::{debug, info};
use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
struct Project {
    name: String,
    description: String,
    html_url: String,
}

#[component]
pub fn ProjectsPage() -> impl IntoView {
    debug!("Rendering <Projects/>");

    on_cleanup(|| {
        info!("Cleaning up <Projects/>");
    });
    
    let projects: Vec<String> = vec!["Astrosight", "PhysECS", "Quaternion"]
        .into_iter()
        .map(|project| { String::from(project) })
        .collect();

    view! {
        <div class="justify-center">
            <h1 class="text-center text-[50px] align-middle">
                "Technical Projects"
            </h1>
            <div class="flex justify-center
                     p-[20px] gap-[10px]">
                <For
                    each= move || { projects.clone() }
                    key=|project| { project.clone() }
                    children=move |project: String| {
                        view! {
                            <ProjectCard
                                project=project
                            />
                        }
                    }
                />
            </div>
        </div>
    }
}

#[component]
fn ProjectCard(
    project: String,
) -> impl IntoView {
    let (my_project, _) = create_signal(project.clone());

    view! {
        <Await
            future=move || async move {
                let mut opts = RequestInit::new();
                opts.method("GET");
                opts.mode(RequestMode::Cors);

                let url = format!("https://api.github.com/repos/austinlucaslake/{}", my_project.get());
            
                let request = Request::new_with_str_and_init(&url, &opts).unwrap();
                request
                    .headers()
                    .set("Accept", "application/vnd.github.v3+json").unwrap();

                let window = web_sys::window().unwrap();
                let resp_value = JsFuture::from(window.fetch_with_request(&request))
                    .await.unwrap();

                assert!(resp_value.is_instance_of::<Response>());
                let resp: Response = resp_value.dyn_into().unwrap();
                let json = JsFuture::from(resp.json().unwrap()).await.unwrap();
                let result: Project = serde_wasm_bindgen::from_value(json).unwrap();
                result 
            }
            let:data
        >
            <div
                class="border justify-center text-center animate-fade w-[400px]">
                <div class="inline-block text-center h-[150px]">
                    { project.clone() }
                </div>
                <div class="flex justify-center text-center w-4/5 h-[200px]">
                    <p class="inline-block">
                        { data.description.clone() } 
                    </p>
                </div>
                <div
                    class="flex justify-center items-center h-[50px]">
                    <a
                        class="inline-block w-1/2 rounded-full bg-current
                            text-center font-bold no-underline"
                        href=data.html_url.clone()>
                        <span class="invert"> "Learn more" </span>
                    </a> 
                </div>
            </div>
        </Await>
    }
}
