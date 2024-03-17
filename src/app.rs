use leptos::ev::Event;
use leptos::{leptos_dom::logging::console_log, *};
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

use crate::content::EXPERIENCE;
use crate::content::HELP;
use crate::content::SKILLS;
use crate::content::WHOAMI;
use crate::minimal::Minimal;

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
        <Title text="Sahil's portfolio"/>
        <Router>
            <main class="bg-base my-0 mx-auto min-h-screen h-full w-screen text-default">
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
    let (cc, set_cc) = create_signal(0);

    view! {
        // <For
        // each = cc
        // key = |cc|
        //
        // />
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
    <h1 class="text-red">portfolio</h1>
            <div class="flex flex-row w-full">
    <div class="text-pink h-8">">"</div>
        <input on:change=on_enter type="text" class="appearance-none border border-none rounded-md focus:outline-none focus:border-none bg-base caret-red pb-2 mx-2 w-full" autofocus prop:value=inp/>
            </div>
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
        <h1>"404: Not Found"</h1>
    }
}
