use leptos::ev::KeyboardEvent;
use leptos::{leptos_dom::logging::console_log, *};
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

use crate::content::{ME, WHOAMI};

#[derive(Serialize, Deserialize, Debug)]
struct Content {
    whoami: String,
    skills: String,
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/portfolio.css"/>
        <Title text="Welcome to Leptos"/>
        <Router>
            <main class="bg-ct-base my-0 mx-auto h-screen w-screen text-ct-pink">
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/minimal" view=Minimal/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let on_enter = move |key: KeyboardEvent| console_log(format!("key {:?}", key.key()).as_str());
    view! {
        <h1 class="text-ct-red">{WHOAMI}</h1>
        <h1 class="">{ME.whoami}</h1>
        <h1 class="">{ME.skills.desc}</h1>
        <input on:keydown=on_enter type="text" class="appearance-none border border-none rounded-md py-2 px-4 focus:outline-none focus:border-none bg-ct-base caret-ct-red"/>
    }
}

#[component]
fn Minimal() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);
    view! {
        <h1 class="text-ct-red">"Welcome to minimal view"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Foundfsfesf"</h1>
    }
}
