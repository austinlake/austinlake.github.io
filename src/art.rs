use core::cmp::{max, min};
use js_sys::Object;
use leptos::*;
use tracing::{debug, info};
use web_sys::{Animation, Element, FillMode, KeyframeAnimationOptions, MouseEvent};
use wasm_bindgen::JsValue;

#[component]
pub fn ArtPage() -> impl IntoView {
    debug!("rendering <Art/>");

    on_cleanup(|| {
        info!("cleaning up <Art/>");
    });

    let (mouse_position, set_mouse_position) = create_signal(0);
    let (percentage, set_percentage) = create_signal(0);
    let (prev_percentage, set_prev_percentage) = create_signal(0);

    view! {
        <div
            class="flex-block flex-col select-none">
            <h1
                class="block text-center text-[5vmin]">
                "Art"
            </h1>
            <Gallery
                mouse_position=mouse_position
                set_mouse_position=set_mouse_position
                percentage=percentage
                set_percentage=set_percentage
                prev_percentage=prev_percentage
                set_prev_percentage=set_prev_percentage
            />
        </div>
    }
}

#[component]
fn Gallery(
    mouse_position: ReadSignal<i32>,
    set_mouse_position: WriteSignal<i32>,
    percentage: ReadSignal<i32>,
    set_percentage: WriteSignal<i32>,
    prev_percentage: ReadSignal<i32>,
    set_prev_percentage: WriteSignal<i32>
) -> impl IntoView {
    let mouse_down = move |e: MouseEvent| {
        set_mouse_position.update(|x: &mut i32| *x = e.client_x());
    };
    
    let mouse_up = move |_| {
        set_mouse_position.update(|x: &mut i32| *x = 0);
        set_prev_percentage.update(|x: &mut i32| *x = percentage.get());
    };
   
    let mouse_move = move |e: MouseEvent| {
        if mouse_position.get() == 0 { return; }
        let delta: i32 = mouse_position.get() - e.client_x();
        let window = web_sys::window().expect("no global `window` exists");
        let max_delta: i32 = window.inner_width().unwrap().as_f64().unwrap() as i32 / 2;
        let next_percentage: i32 = prev_percentage.get() + (-100 * delta / max_delta);
        set_percentage.update(|x: &mut i32| *x = max(min(next_percentage, 0), -100));
        
        let gallery = web_sys::window().expect("No window present.")
            .document().expect("No document present")
            .get_elements_by_class_name("gallery")
            .get_with_index(0).expect("No gallery class present");
        
        let mut animation_options = KeyframeAnimationOptions::new();
        animation_options    
            .duration(&JsValue::from_str("1200"))
            .fill(FillMode::Forwards);

        gallery.animate_with_keyframe_animation_options(
            Object::try_from(&JsValue::from_str(
                format!("{{ transform: 'translate({}%, -50%)' }}", percentage.get()).as_str()
            )),
            &animation_options 
        );

        for i in 0..gallery.get_elements_by_tag_name("img").length() {
            gallery.get_elements_by_tag_name("img")
                .get_with_index(i).expect("No img present")
                .animate_with_keyframe_animation_options(
                    Object::try_from(&JsValue::from_str(
                        format!("{{ objectPosition: '{}% center' }}", percentage.get()+100).as_str()
                    )),
                    &animation_options 
                );
        }
    };

    let gallery_items: Vec<String> = vec![
            "ivc_commencement.jpg",
            "high_school_side_profile.jpg",
            "golden_bear_welcome.jpg",
            "sunset_at_tvt.jpg",
            "dr_seuss.jpg"]
        .into_iter()
        .map(|i| String::from(i))
        .collect();


    view! {
        <div on:mousedown=mouse_down
             on:mouseup=mouse_up
             on:mousemove=mouse_move
             class="gallery flex gap-[20px] translate-y-[50%]
                 overflow-hidden items-center absolute left-50 top-50">
            <For
                each=move || { gallery_items.clone() }
                key=|file| { file.clone() }
                children=move |file: String| {
                    view! {
                        <GalleryItem
                            file=file
                        />
                    }
                }
            />
        </div>
    }
}

#[component]
fn GalleryItem(
    file: String
) -> impl IntoView {
    view! {
        <img
            class="gallery_item w-[20vmin] h-[40vmin] object-cover object-[100%_center]"
            src=move || { format!("portraits/{file}") }
            draggable="false"
        />
    }
}
