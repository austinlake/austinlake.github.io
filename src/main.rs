#[rustfmt::skip::macros(view)]
use leptos::{
    component, leptos_dom::mount_to_body, log, on_cleanup, provide_context, view, IntoView, Scope
};
use leptos_router::{Route, RouteProps, Router, RouterProps, Routes, RoutesProps};
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
                <nav class="flex justify-evenly text-center align-middle items-center text-[3vmin] py-[5vmin] h-[5vmin] dark:invert">
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
    }
}

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    debug!("rendering <Home/>");

    on_cleanup(cx, || {
        log!("cleaning up <Home/>");
    });

    view! { cx,
        <div class="flex flex-col justify-center items-center p-[2vmin] gap-[2vmin]">
            <h1 class="flex flex-col pb-[50px] text-center align-middle text-[10vmin]">
                <span class="text-center text-stone-800 dark:text-stone-200"> "My name is " </span>
                <div class="inline-block">
                    <span class="text-transparent bg-clip-text dark:animate-dark_color_change_reverse dark:hover:animate-dark_color_change animate-color_change_reverse hover:animate-color_change"> "Austin Lake" </span>
                    <span class="text-center text-stone-800 dark:text-stone-200"> "." </span>
                </div>
            </h1>
            <h2 class="inline-block pb-[50px] text-center align-middle text-[5vmin] text-stone-800 dark:invert"> "Welcome to my website. Built from scratch." </h2>
            <div class="flex h-1/2 justify-center items-center">
                // https://www.rust-lang.org/logos/rust-logo-blk.svg
                <svg viewBox="0 0 144 144" class="inline w-auto h-[15vmin] fill-stone-800 dark:fill-stone-200">
                    <path d="m71.05 23.68c-26.06 0-47.27 21.22-47.27 47.27s21.22 47.27 47.27 47.27 47.27-21.22 47.27-47.27-21.22-47.27-47.27-47.27zm-.07 4.2a3.1 3.11 0 0 1 3.02 3.11 3.11 3.11 0 0 1 -6.22 0 3.11 3.11 0 0 1 3.2-3.11zm7.12 5.12a38.27 38.27 0 0 1 26.2 18.66l-3.67 8.28c-.63 1.43.02 3.11 1.44 3.75l7.06 3.13a38.27 38.27 0 0 1 .08 6.64h-3.93c-.39 0-.55.26-.55.64v1.8c0 4.24-2.39 5.17-4.49 5.4-2 .23-4.21-.84-4.49-2.06-1.18-6.63-3.14-8.04-6.24-10.49 3.85-2.44 7.85-6.05 7.85-10.87 0-5.21-3.57-8.49-6-10.1-3.42-2.25-7.2-2.7-8.22-2.7h-40.6a38.27 38.27 0 0 1 21.41-12.08l4.79 5.02c1.08 1.13 2.87 1.18 4 .09zm-44.2 23.02a3.11 3.11 0 0 1 3.02 3.11 3.11 3.11 0 0 1 -6.22 0 3.11 3.11 0 0 1 3.2-3.11zm74.15.14a3.11 3.11 0 0 1 3.02 3.11 3.11 3.11 0 0 1 -6.22 0 3.11 3.11 0 0 1 3.2-3.11zm-68.29.5h5.42v24.44h-10.94a38.27 38.27 0 0 1 -1.24-14.61l6.7-2.98c1.43-.64 2.08-2.31 1.44-3.74zm22.62.26h12.91c.67 0 4.71.77 4.71 3.8 0 2.51-3.1 3.41-5.65 3.41h-11.98zm0 17.56h9.89c.9 0 4.83.26 6.08 5.28.39 1.54 1.26 6.56 1.85 8.17.59 1.8 2.98 5.4 5.53 5.4h16.14a38.27 38.27 0 0 1 -3.54 4.1l-6.57-1.41c-1.53-.33-3.04.65-3.37 2.18l-1.56 7.28a38.27 38.27 0 0 1 -31.91-.15l-1.56-7.28c-.33-1.53-1.83-2.51-3.36-2.18l-6.43 1.38a38.27 38.27 0 0 1 -3.32-3.92h31.27c.35 0 .59-.06.59-.39v-11.06c0-.32-.24-.39-.59-.39h-9.15zm-14.43 25.33a3.11 3.11 0 0 1 3.02 3.11 3.11 3.11 0 0 1 -6.22 0 3.11 3.11 0 0 1 3.2-3.11zm46.05.14a3.11 3.11 0 0 1 3.02 3.11 3.11 3.11 0 0 1 -6.22 0 3.11 3.11 0 0 1 3.2-3.11z"/>
                    <path d="m115.68 70.95a44.63 44.63 0 0 1 -44.63 44.63 44.63 44.63 0 0 1 -44.63-44.63 44.63 44.63 0 0 1 44.63-44.63 44.63 44.63 0 0 1 44.63 44.63zm-.84-4.31 6.96 4.31-6.96 4.31 5.98 5.59-7.66 2.87 4.78 6.65-8.09 1.32 3.4 7.46-8.19-.29 1.88 7.98-7.98-1.88.29 8.19-7.46-3.4-1.32 8.09-6.65-4.78-2.87 7.66-5.59-5.98-4.31 6.96-4.31-6.96-5.59 5.98-2.87-7.66-6.65 4.78-1.32-8.09-7.46 3.4.29-8.19-7.98 1.88 1.88-7.98-8.19.29 3.4-7.46-8.09-1.32 4.78-6.65-7.66-2.87 5.98-5.59-6.96-4.31 6.96-4.31-5.98-5.59 7.66-2.87-4.78-6.65 8.09-1.32-3.4-7.46 8.19.29-1.88-7.98 7.98 1.88-.29-8.19 7.46 3.4 1.32-8.09 6.65 4.78 2.87-7.66 5.59 5.98 4.31-6.96 4.31 6.96 5.59-5.98 2.87 7.66 6.65-4.78 1.32 8.09 7.46-3.4-.29 8.19 7.98-1.88-1.88 7.98 8.19-.29-3.4 7.46 8.09 1.32-4.78 6.65 7.66 2.87z" fill-rule="evenodd"/>
                </svg>
                // https://raw.githubusercontent.com/leptos-rs/leptos/b6d90601524b03e35a808df22e61725910be5fb7/docs/logos/Leptos_logo_RGB.svg
                <svg x="0px" y="0px" viewBox="0 0 437.4294 209.6185" class="h-[15vmin]">
                    <path style="fill:none;" d="M130.0327,79.3931c-11.4854-0.23-22.52,9.3486-24.5034,21.0117l49.1157,0.0293 c-2.1729-10.418-11.1821-21.0449-24.1987-21.0449C130.3081,79.3892,130.1714,79.3907,130.0327,79.3931z"/>
                    <path class="fill-stone-800 dark:fill-stone-200" d="M95.1109,128.1089H58.6797V65.6861c0-1.5234-0.8169-2.4331-2.1855-2.4331h-3.1187 c-1.3159,0-2.2349,1.0005-2.2349,2.4331v67.4297c0,1.4521,0.8145,2.2852,2.2349,2.2852h41.7353c1.4844,0,2.4819-0.9375,2.4819-2.333 v-2.7744C97.5928,128.9253,96.6651,128.1089,95.1109,128.1089z"/>
                    <path class="fill-stone-800 dark:fill-stone-200" d="M146.4561,77.1739c-4.8252-3.001-10.3037-4.5249-16.2837-4.5288c-0.0068,0-0.0137,0-0.0205,0 c-5.7349,0-11.1377,1.4639-16.0566,4.3511c-4.916,2.8853-8.8721,6.8364-11.7593,11.7456 c-2.8975,4.9248-4.3687,10.332-4.3721,16.0713c-0.0034,5.7188,1.4966,11.0654,4.4565,15.8887 c2.9893,4.9209,6.8789,8.7334,11.8887,11.6514c4.8657,2.8633,10.2397,4.3174,15.9717,4.3203c0.0073,0,0.0146,0,0.022,0 c8.123,0,14.7441-2.5869,21.4683-8.3906c0.5493-0.4805,0.8516-1.1201,0.8516-1.8008c0.001-0.6074-0.1743-1.1035-0.5205-1.4756 l-1.3569-1.8428l-0.0732-0.0859c-0.2637-0.2637-0.6929-0.6152-1.3716-0.6152c-0.6421,0-1.2549,0.2217-1.7124,0.6143 c-1.9346,1.585-3.5459,2.8008-4.7969,3.6182c-1.7979,1.208-5.8218,3.2314-12.5986,3.2314c-0.0073,0-0.0142,0-0.021,0 c-0.1357,0.0029-0.269,0.0039-0.4043,0.0039c-12.2642,0-23.4736-10.3262-24.5088-22.4814l53.0127,0.0322c0.0015,0,0.0024,0,0.0034,0 c2.2373,0,3.4697-1.1621,3.4712-3.2715c0.0034-5.2588-1.3574-10.3945-4.0464-15.2705 C155.0015,84.0953,151.2188,80.1363,146.4561,77.1739z M154.6451,100.4341l-49.1157-0.0293 c1.9834-11.6631,13.0181-21.2417,24.5034-21.0117c0.1387-0.0024,0.2754-0.0039,0.4136-0.0039 C143.4629,79.3892,152.4722,90.0162,154.6451,100.4341z"/>
                    <path class="fill-stone-800 dark:fill-stone-200" d="M204.0386,136.6382c5.7319,0,11.1069-1.4502,15.9746-4.3115 c4.938-2.9014,8.75-6.7129,11.6533-11.6533c2.8608-4.8672,4.311-10.2578,4.311-16.0244c0-5.7324-1.4502-11.1064-4.311-15.9746 c-2.9019-4.9385-6.7134-8.75-11.6533-11.6533c-4.8687-2.8618-10.2437-4.3125-15.9746-4.3125 c-9.938,0-19.2021,4.7583-24.3516,12.3174v-9.438c0-0.5946-0.1465-1.0788-0.411-1.4511c-0.3815-0.5369-1.0157-0.834-1.8727-0.834 h-2.6738c-1.4521,0-2.2852,0.833-2.2852,2.2852v5.6964v46.4791v23.9676c0,1.2568,0.7808,2.0371,2.0371,2.0371h3.3667 c0.9209,0,1.6421-0.6992,1.6421-1.5908v-17.098v-10.984C185.0884,131.8892,194.2749,136.6382,204.0386,136.6382z M186.6358,122.5591 c-4.9346-4.9346-7.6831-11.4932-7.542-18.0254c-0.1367-6.3506,2.5439-12.751,7.3545-17.5605 c4.8521-4.8521,11.3037-7.5547,17.7383-7.417c4.3691,0,8.4863,1.1465,12.2314,3.4043c3.7344,2.2979,6.7456,5.4053,8.9492,9.2354 c2.1699,3.9072,3.2695,8.0967,3.2695,12.4697c0.1396,6.4619-2.5967,12.9844-7.5083,17.8955 c-4.7617,4.7617-11.0469,7.3857-17.2544,7.2803C197.6856,129.9712,191.396,127.3208,186.6358,122.5591z"/>
                    <path class="fill-stone-800 dark:fill-stone-200" d="M241.8955,80.3975h7.5669v42.0259c0,6.8174,4.5674,12.1309,11.0825,12.9189 c0.6836,0.1055,1.8379,0.1572,3.5303,0.1572c2.0078,0,3.0273-0.3535,3.0273-2.2842v-2.377c0-1.7891-1.334-2.0371-2.7568-2.0371 c0,0-0.001,0-0.002,0l-1.7871-0.0488c-2.0117-0.0439-3.4883-0.7627-4.3896-2.1367c-0.9697-1.4805-1.4619-3.1738-1.4619-5.0352 V80.3975h10.0928c1.3076,0,2.2852-1.3628,2.2852-2.5815v-1.9312c0-1.3999-0.8359-2.2354-2.2354-2.2354h-10.1426V60.6861 c0-1.4619-0.7969-2.4829-1.9375-2.4829c-0.1865,0-0.4121,0-0.6392,0.0884l-2.6489,0.6865 c-1.2109,0.3682-2.0171,0.9263-2.0171,2.4507v12.2207h-7.5669c-1.4185,0-2.335,0.897-2.335,2.2852v1.8813 C239.5606,79.2393,240.6079,80.3975,241.8955,80.3975z"/>
                    <path class="fill-stone-800 dark:fill-stone-200" d="M379.1182,106.2691c-4.0488-2.9219-8.8545-5.0293-14.291-6.2646 c-6.5049-1.3975-13.4473-5.2129-13.3203-10.3066c0-7.5225,6.6367-10.1914,12.3203-10.1914c5.3574,0,10.2207,3.002,13.001,8.0146 c0.6729,1.2861,1.4785,1.9375,2.3955,1.9375c0.3311,0,0.7061-0.1113,0.9922-0.2832l2.2021-1.1523 c0.5947-0.3408,0.9229-0.9414,0.9229-1.6924c0-0.5205-0.0908-0.9541-0.2617-1.292c-3.6367-8.2466-10.0967-12.4282-19.2021-12.4282 c-11.7305,0-19.6123,6.9263-19.6123,17.2349c0,4.3125,1.8438,7.9746,5.4756,10.8809c3.4482,2.7979,7.9121,4.8623,13.2705,6.1377 c4.5859,1.085,8.3193,2.5654,11.0977,4.4023c1.4159,0.9354,2.4412,2.0535,3.106,3.3672c0.6053,1.1962,0.9135,2.5535,0.9135,4.1005 c0.0742,2.3857-0.79,4.5176-2.5684,6.3389c-3.1445,3.2178-8.4053,4.6689-12.0205,4.6689c-0.0361,0-0.0723,0-0.1074,0 c-3.4268,0-6.4893-0.8438-9.1035-2.5068c-2.5918-1.6484-4.2363-3.8076-5.0293-6.6064c-0.3203-1.0996-0.751-2.1738-2.1553-2.1738 c-0.0742,0-0.2109,0.0146-0.4062,0.0449c-0.1133,0.0166-0.2559,0.0381-0.5088,0.0742l-1.8818,0.4463l-0.1045,0.0332 c-1.0244,0.4082-1.6113,1.1846-1.6113,2.1309c0,0.2285,0.0625,0.6592,0.2178,1.1094c1.9707,8.5801,10.2432,14.3447,20.5732,14.3447 c0.125,0.002,0.249,0.002,0.374,0.002c6.5947,0,12.6748-2.3193,16.7275-6.3945c3.1895-3.208,4.8311-7.2363,4.748-11.6357 c0-2.8187-0.6185-5.3109-1.8062-7.481C382.4437,109.2624,381.0062,107.631,379.1182,106.2691z"/>
                    <path style="fill:#EF3939;" d="M348.9043,45.7325c0-6.3157-3.2826-11.8699-8.2238-15.0756 c-2.811-1.8237-6.1537-2.8947-9.7469-2.8947c-9.9092,0-17.9707,8.0615-17.9707,17.9702c0,4.7659,1.8775,9.0925,4.9157,12.3123 c-3.6619,4.3709-6.6334,9.3336-8.7663,14.7186c-1.5873-0.2422-3.2123-0.3683-4.8662-0.3683 c-17.7158,0-32.1289,14.4131-32.1289,32.1289c0,14.6854,9.9077,27.0922,23.3869,30.9101 c-6.7762,17.3461-23.6572,29.6719-43.3742,29.6719c-16.8195,0-31.583-8.9662-39.7656-22.369 c-2.4778,0.5446-5.0429,0.8519-7.6721,0.9023c9.0226,16.99,26.8969,28.5917,47.4377,28.5917 c23.2646,0,43.1121-14.8788,50.5461-35.6179c0.5204,0.0251,1.0435,0.0398,1.5701,0.0398c17.7158,0,32.1289-14.4131,32.1289-32.1289 c0-13.557-8.4446-25.1712-20.3465-29.8811c1.9001-4.5678,4.5115-8.7646,7.6888-12.4641c0.9996,0.4404,2.0479,0.785,3.1324,1.0384 c1.3144,0.3071,2.6773,0.486,4.0839,0.486C340.8428,63.7032,348.9043,55.6416,348.9043,45.7325z M304.2461,129.5279 c-13.7871,0-25.0039-11.2168-25.0039-25.0039s11.2168-25.0039,25.0039-25.0039S329.25,90.7369,329.25,104.524 S318.0332,129.5279,304.2461,129.5279z M330.9336,34.8872c0.645,0,1.2737,0.0671,1.8881,0.1755 c5.0818,0.8974,8.9576,5.3347,8.9576,10.6697c0,5.9805-4.8652,10.8457-10.8457,10.8457s-10.8457-4.8652-10.8457-10.8457 c0-1.3967,0.2746-2.7282,0.7576-3.9555C322.4306,37.7496,326.35,34.8872,330.9336,34.8872z"/>
                </svg>
                <img class="inline w-auto h-[10vmin]" src="https://tailwindcss.com/_next/static/media/tailwindcss-mark.3c5441fc7a190fb1800d4a5c7f07ba4b1345a9c8.svg"/>
                <img class="inline w-auto h-[10vmin] px-[5vmin]" src="https://raw.githubusercontent.com/carlosbaraza/web-assembly-logo/f0f411529c1dafffa233be1bd95b80b79144b675/dist/icon/web-assembly-icon.svg"/>
            </div>
        </div>
        <div class="flex justify-center relative bottom pt-[10vmin] gap-[1vmin]">
            <a href="https://github.com/austinlake04/website/releases" title="Releases"> <img src="https://img.shields.io/github/v/release/austinlake04/website?logo=github"/> </a>
            <a href="https://github.com/austinlake04/website/blob/main/LICENSE-APACHE" title="Apache-2.0 License"> <img src="https://img.shields.io/github/license/austinlake04/website"/> </a>
            <a href="https://github.com/austinlake04/website/blob/main/LICENSE-MIT" title="MIT License"> <img src="https://img.shields.io/badge/license-MIT-green"/> </a>
            <a href="https://github.com/austinlake04/website/actions" title="Build Status"> <img src="https://github.com/austinlake04/website/actions/workflows/ci.yaml/badge.svg?event=pull_request"/> </a>
        </div>
    }
}

#[component]
pub fn About(cx: Scope) -> impl IntoView {
    debug!("rendering <About/>");

    on_cleanup(cx, || {
        log!("cleaning up <About/>");
    });

    view! { cx,
        <div class="flex justify-center items-center align-center px-[5vmin] gap-[5vmin]">
            <img class="w-[80vmin] rounded-[2vmin] self-start hidden xl:block" src="./assets/portraits/dr_seuss.jpg" draggable="false"/>
            <div class="inline-block w-[80vin] p-[5px]">
                <p class="inline-block text-[5vmin]">
                    <span class="text-stone-800 dark:text-stone-200">
                        "I'm a physics and astrophysics major at the "
                    </span>
                    <span class="text-transparent bg-clip-text dark:animate-dark_color_change_reverse dark:hover:animate-dark_color_change animate-color_change_reverse hover:animate-color_change">
                        "University of California, Berkeley"
                    </span>
                    <span class="text-stone-800 dark:text-stone-200">
                        ". My primary research interests are related to astronomical "
                    </span>
                    <span class="text-transparent bg-clip-text dark:animate-dark_color_change_reverse dark:hover:animate-dark_color_change animate-color_change_reverse hover:animate-color_change">
                        "instrumentation"
                    </span>
                    <span class="text-stone-800 dark:text-stone-200">
                        " for ground-based telescopes and space missions. To that end, I am eager to get involved in research involving lens design, adaptive optics, wavefront sensing, and detectors."
                    </span>
                </p>
            </div>
        </div>
        <div class="p-[20vmin]">
            <h2 class="text-center text-stone-800 dark:text-stone-200 text-[6.5vmin]"> "Current Affiliations:" </h2>
            <div class="flex flex-wrap justify-center items-center gap-[5vmin]">
                <img class="h-[20vmin]" src="https://upload.wikimedia.org/wikipedia/commons/a/a1/Seal_of_University_of_California%2C_Berkeley.svg" draggable="false"/>
                <img class="h-[20vmin]" src="https://cdn.worldvectorlogo.com/logos/nasa-6.svg" draggable="false"/>
            </div>
        </div>
        <SocialMedia/>
    }
}

#[component]
pub fn Projects(cx: Scope) -> impl IntoView {
    debug!("rendering <Projects/>");

    on_cleanup(cx, || {
        log!("cleaning up <Projects/>");
    });

    view! { cx,
        <div class="justify-center">
            <h1 class="text-center text-[10vmin]">
                <span class="text-stone-800 dark:text-stone-200"> "Technical Projects" </span>
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
    }
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
            <span class="text-stone-800 dark:text-stone-200"> "Art Portfolio" </span>
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

    view! { cx,
        <div class="left-0 right-0 m-auto relative bottom-[2vmin] flex flex-col justify-center">
            <h2 class="pb-[20px] text-center text-stone-800 dark:text-stone-200 text-[6.5vmin]"> "Social Media:" </h2>
            <div class="flex flex-wrap justify-center items-center align-middle gap-[5vmin] h-[5vmin] ">
                <a href="https://www.facebook.com/austinlake04" title="Facebook">
                    <svg viewBox="0 0 14222 14222" class="w-[5vmin] h-[5vmin] fill-stone-800 dark:fill-stone-200">
                        <defs>
                            <mask id="facebook">
                                <circle cx="7111" cy="7112" r="7111" fill="#fff"/>
                                <path d="M9879 9168l315-2056H8222V5778c0-562 275-1111 1159-1111h897V2917s-814-139-1592-139c-1624 0-2686 984-2686 2767v1567H4194v2056h1806v4969c362 57 733 86 1111 86s749-30 1111-86V9168z" fill="#000"/>
                            </mask>
                        </defs>
                        <circle cx="7111" cy="7112" r="7111" mask="url(#facebook)"/>
                    </svg>
                </a>
                <a href="https://www.twitter.com/austinlake04" title="Twitter">
                    <svg viewBox="0 0 248 204" class="w-[5vmin] h-[5vmin] fill-stone-800 dark:fill-stone-200">
                        <path d="M221.95,51.29c0.15,2.17,0.15,4.34,0.15,6.53c0,66.73-50.8,143.69-143.69,143.69v-0.04 C50.97,201.51,24.1,193.65,1,178.83c3.99,0.48,8,0.72,12.02,0.73c22.74,0.02,44.83-7.61,62.72-21.66 c-21.61-0.41-40.56-14.5-47.18-35.07c7.57,1.46,15.37,1.16,22.8-0.87C27.8,117.2,10.85,96.5,10.85,72.46c0-0.22,0-0.43,0-0.64 c7.02,3.91,14.88,6.08,22.92,6.32C11.58,63.31,4.74,33.79,18.14,10.71c25.64,31.55,63.47,50.73,104.08,52.76 c-4.07-17.54,1.49-35.92,14.61-48.25c20.34-19.12,52.33-18.14,71.45,2.19c11.31-2.23,22.15-6.38,32.07-12.26 c-3.77,11.69-11.66,21.62-22.2,27.93c10.01-1.18,19.79-3.86,29-7.95C240.37,35.29,231.83,44.14,221.95,51.29z"/>                    
                    </svg>
                </a>
                <a href="https://www.instagram.com/austinlake04" title="Instagram">
                    <svg viewBox="0 0 1000 1000" class="w-[5vmin] h-[5vmin] fill-stone-800 dark:fill-stone-200">
                        <path d="M295.42,6c-53.2,2.51-89.53,11-121.29,23.48-32.87,12.81-60.73,30-88.45,57.82S40.89,143,28.17,175.92c-12.31,31.83-20.65,68.19-23,121.42S2.3,367.68,2.56,503.46,3.42,656.26,6,709.6c2.54,53.19,11,89.51,23.48,121.28,12.83,32.87,30,60.72,57.83,88.45S143,964.09,176,976.83c31.8,12.29,68.17,20.67,121.39,23s70.35,2.87,206.09,2.61,152.83-.86,206.16-3.39S799.1,988,830.88,975.58c32.87-12.86,60.74-30,88.45-57.84S964.1,862,976.81,829.06c12.32-31.8,20.69-68.17,23-121.35,2.33-53.37,2.88-70.41,2.62-206.17s-.87-152.78-3.4-206.1-11-89.53-23.47-121.32c-12.85-32.87-30-60.7-57.82-88.45S862,40.87,829.07,28.19c-31.82-12.31-68.17-20.7-121.39-23S637.33,2.3,501.54,2.56,348.75,3.4,295.42,6m5.84,903.88c-48.75-2.12-75.22-10.22-92.86-17-23.36-9-40-19.88-57.58-37.29s-28.38-34.11-37.5-57.42c-6.85-17.64-15.1-44.08-17.38-92.83-2.48-52.69-3-68.51-3.29-202s.22-149.29,2.53-202c2.08-48.71,10.23-75.21,17-92.84,9-23.39,19.84-40,37.29-57.57s34.1-28.39,57.43-37.51c17.62-6.88,44.06-15.06,92.79-17.38,52.73-2.5,68.53-3,202-3.29s149.31.21,202.06,2.53c48.71,2.12,75.22,10.19,92.83,17,23.37,9,40,19.81,57.57,37.29s28.4,34.07,37.52,57.45c6.89,17.57,15.07,44,17.37,92.76,2.51,52.73,3.08,68.54,3.32,202s-.23,149.31-2.54,202c-2.13,48.75-10.21,75.23-17,92.89-9,23.35-19.85,40-37.31,57.56s-34.09,28.38-57.43,37.5c-17.6,6.87-44.07,15.07-92.76,17.39-52.73,2.48-68.53,3-202.05,3.29s-149.27-.25-202-2.53m407.6-674.61a60,60,0,1,0,59.88-60.1,60,60,0,0,0-59.88,60.1M245.77,503c.28,141.8,115.44,256.49,257.21,256.22S759.52,643.8,759.25,502,643.79,245.48,502,245.76,245.5,361.22,245.77,503m90.06-.18a166.67,166.67,0,1,1,167,166.34,166.65,166.65,0,0,1-167-166.34" transform="translate(-2.5 -2.5)"/>
                    </svg>
                </a>
                <a href="https://www.linkedin.com/in/austinlake04" title="LinkedIn">
                    <svg viewBox="0 0 256 256" class="w-[5vmin] h-[5vmin] fill-stone-800 dark:fill-stone-200">
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
                </a>
                <a href="https://www.github.com/austinlake04" title="GitHub">
                    <svg viewBox="0 0 98 96" class="h-[5vmin] fill-stone-800 dark:fill-stone-200">
                        <path d="M48.854 0C21.839 0 0 22 0 49.217c0 21.756 13.993 40.172 33.405 46.69 2.427.49 3.316-1.059 3.316-2.362 0-1.141-.08-5.052-.08-9.127-13.59 2.934-16.42-5.867-16.42-5.867-2.184-5.704-5.42-7.17-5.42-7.17-4.448-3.015.324-3.015.324-3.015 4.934.326 7.523 5.052 7.523 5.052 4.367 7.496 11.404 5.378 14.235 4.074.404-3.178 1.699-5.378 3.074-6.6-10.839-1.141-22.243-5.378-22.243-24.283 0-5.378 1.94-9.778 5.014-13.2-.485-1.222-2.184-6.275.486-13.038 0 0 4.125-1.304 13.426 5.052a46.97 46.97 0 0 1 12.214-1.63c4.125 0 8.33.571 12.213 1.63 9.302-6.356 13.427-5.052 13.427-5.052 2.67 6.763.97 11.816.485 13.038 3.155 3.422 5.015 7.822 5.015 13.2 0 18.905-11.404 23.06-22.324 24.283 1.78 1.548 3.316 4.481 3.316 9.126 0 6.6-.08 11.897-.08 13.526 0 1.304.89 2.853 3.316 2.364 19.412-6.52 33.405-24.935 33.405-46.691C97.707 22 75.788 0 48.854 0z"/>
                    </svg>
                </a>
            </div>
        </div>
    }
}

#[component]
pub fn Footer(cx: Scope) -> impl IntoView {
    debug!("rendering <Footer/>");

    on_cleanup(cx, || {
        log!("cleaning up <Footer/>");
    });

    view! {cx,
    
    }
}

pub fn main() {
    tracing_subscriber::fmt::init();
    mount_to_body(|cx| view! { cx, <Navigation/> });
}
