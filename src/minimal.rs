use leptos::*;

use crate::content::*;

#[component]
pub fn Minimal() -> impl IntoView {
    view! {
        <div class="w-screen flex flex-col items-center">
        <div class="pt-14 p-4 font-sans sm:w-[80%] md:w-[70%] 2xl:w-[60%] w-full">
        <WhoAmI/>
        <Divider/>
        <Skills/>
        <Divider/>
        <Experience/>
        <Divider/>
        <Projects/>
        </div>
        </div>
    }
}

#[component]
fn Divider() -> impl IntoView {
    view! {
        <div class="w-full h-1 rounded-lg my-6 bg-yellow bg-opacity-90"></div>
    }
}

#[component]
fn WhoAmI() -> impl IntoView {
    view! {
        <div class="text-green text-5xl font-bold">
        <h1 class="mb-2">"Hi, I'm"</h1>
        <h1 class=" bg-slate-700 bg-opacity-80 rounded-md inline-block">"Sahil Yeole"</h1>
        </div>
        <h1 class="my-3 text-lg text-pink">{WHOAMI_REPLY_1}</h1>
        <h1 class="my-6">{WHOAMI_REPLY_2}</h1>
    }
}

#[component]
fn Skills() -> impl IntoView {
    view! {
        <div class="">
        <h1 class="text-sapphire text-4xl my-6 font-bold">Skills</h1>
        <ListItem title= "Programming Languages:" text={SKILLS_REPLY_LANGS}/>
        <ListItem title= "Front-end:" text={SKILLS_REPLY_FRONT}/>
        <ListItem title= "Back-end:" text={SKILLS_REPLY_BACK}/>
        <ListItem title= "Tools:" text={SKILLS_REPLY_TOOLS}/>
        <h1 class="my-6">{WHOAMI_REPLY_2}</h1>
        </div>
    }
}

#[component]
fn ListItem(title: &'static str, text: &'static str) -> impl IntoView {
    view! {
            <text class="flex items-center justify-start lg:py-0 py-1">
            <text class="text-teal text-3xl mr-4 mb-1">"•"
        </text>
            <text class="text-peach text-md mr-4 min-w-[7rem] max-w-[7rem]">{title}</text>
            <text class="text-md">{text}</text>
        </text>
    }
}

#[component]
fn Experience() -> impl IntoView {
    view! {
        <div class="">
        <h1 class="text-sapphire text-4xl my-6 font-bold">Experience</h1>
        <h1 class="text-pink text-xl font-bold">RustDesk</h1>
        <h1 class="text-peach mt-1 mb-4">Software Developer Intern</h1>
        <h1 class="my-6">{WHOAMI_REPLY_2}</h1>
        </div>
    }
}

#[component]
fn Projects() -> impl IntoView {
    view! {
        <div class="">
        <h1 class="text-sapphire text-4xl my-6 font-bold">Projects</h1>
        <h1 class="text-pink text-xl font-bold">DreamStay</h1>
        <h1 class="my-6">{WHOAMI_REPLY_2}</h1>
        </div>
    }
}
