use lazy_static::lazy_static;

pub const WHOAMI: &str = "whoami";
pub const SKILLS: &str = "skills";
pub const EXPERIENCE: &str = "experience";
pub const HELP: &str = "help";

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
