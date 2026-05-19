use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CV {
    name: String,
    phone: String,
    email: String,
    site: String,
    github: String,
    linkedin: String,
    institutes: Option<Vec<Education>>,
    workplaces: Option<Vec<Work>>,
    projects: Option<Vec<Proj>>,
    small_projects: Option<Vec<SmallProj>>,
    hobby_projects: Option<Vec<HobbyProj>>,
    skills: Option<Skills>,
}

impl CV {
    pub fn anonymise(&mut self) {
        self.name = "John Doe".to_string();
        self.phone = "+1 234 56789".to_string();
        self.email = "john.doe@gmail.com".to_string();
        self.github = "github.com".to_string();
        self.site = "example.com".to_string();
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Education {
    start_date: Date,
    end_date: Date,
    institute: String,
    location: String,
    degree: String,
    major: String,
    desc: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Work {
    start_date: Date,
    end_date: Date,
    title: String,
    company: String,
    desc: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Date {
    year: usize,
    month: usize,
    day: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Proj {
    title: String,
    url: String,
    stack: String,
    desc: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmallProj {
    title: String,
    url: String,
    desc: Vec<String>,
}

type HobbyProj = SmallProj;

#[derive(Debug, Serialize, Deserialize)]
pub struct Skills {
    langs: String,
    operating_systems: String,
    dev_tools: String,
    libraries: String,
}
