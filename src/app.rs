use leptos::ev::Event;
use leptos::{leptos_dom::logging::console_log, *};
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

use crate::content::EXPERIENCE;
use crate::content::HELP;
use crate::content::SKILLS;
use crate::content::WHOAMI;

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
    view! {
        <Prompt />
    }
}

fn check_command(c: &str) -> bool {
    let commands = [WHOAMI, SKILLS, EXPERIENCE, HELP];
    for i in commands {
        if c == i {
            console_log("exist");
            return true;
        }
    }
    false
}

#[component]
fn Prompt() -> impl IntoView {
    let (inp, set_inp) = create_signal("".to_string());
    let (entered_command, set_entered_command) = create_signal("".to_string());

    let on_enter = move |key: Event| {
        let entered = event_target_value(&key);
        if check_command(&entered) {
            set_entered_command.set(entered);
            set_inp.set("".to_string());
            console_log(format!("{:?}", inp.get()).as_str());
            console_log(format!("{:?}", entered_command.get()).as_str());
        } else {
            console_log("not found");
            set_inp.set("".to_string());
        }
    };

    view! {
    <h1 class="text-ct-red">portfolio</h1>
            <div class="flex flex-row w-full">
    <div class="text-ct-pink h-8">">"</div>
        <input on:change=on_enter type="text" class="appearance-none border border-none rounded-md focus:outline-none focus:border-none bg-ct-base caret-ct-red pb-2 mx-2 w-full" autofocus prop:value=inp/>
            </div>
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
