use lazy_static::lazy_static;

pub const WHOAMI: &str = "whoami";
pub const SKILLS: &str = "skills";
pub const EXPERIENCE: &str = "experience";
pub const RESUME: &str = "resume";
pub const MINIMAL: &str = "minimal";
pub const HELP: &str = "help";

pub const WHOAMI_REPLY_1: &str = "A Passionate Full-Stack Developer";
pub const WHOAMI_REPLY_2: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.";
pub const SKILLS_REPLY_LANGS: &str = "Rust, Go, Typescript, Python, C";
pub const SKILLS_REPLY_FRONT: &str =
    "NextJS, React, React Native, Flutter, Leptos, Redux, TailwindCSS, Material-UI, HTML, CSS";
pub const SKILLS_REPLY_BACK: &str = "Node, Express, Gin, GraphQL, Prisma, JWT, MongoDB, PostgreSQL";
pub const SKILLS_REPLY_TOOLS: &str = "Vim, Linux, Docker, Git/GitHub, Postman";
pub const EXPERIENCE_REPLY: &str = "whoami";
pub const RESUME_REPLY: &str = "resume";
pub const MINIMAL_REPLY: &str = "minimal";
pub const HELP_REPLY: &str = "whoami";

lazy_static! {
    pub static ref ME: Me = Me::new(
        "Sahil",
        Skills {
            desc: "here are my skills",
            langs: "rust",
            frontend: "f",
            backend: "b",
            tools: "t",
        }
    );
}

pub struct Me {
    pub whoami: &'static str,
    pub skills: Skills,
}

pub struct Skills {
    pub desc: &'static str,
    pub langs: &'static str,
    pub frontend: &'static str,
    pub backend: &'static str,
    pub tools: &'static str,
}

impl Me {
    pub fn new(name: &'static str, skills: Skills) -> Self {
        Me {
            whoami: name,
            skills,
        }
    }
}
