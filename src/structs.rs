use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CV {
    _name: String,
    _phone: String,
    _email: String,
    _site: String,
    _github: String,
    _edu: Vec<Education>,
    _wrk: Vec<Work>,
    _projs: Vec<Proj>,
    _small_projs: Vec<SmallProj>,
    _hobby_projs: Vec<HobbyProj>,
}

#[derive(Debug, Deserialize)]
pub struct Education {
    _start_date: Date,
    _end_date: Date,
    _institute: String,
    _location: String,
    _degree: String,
    _major: String,
    _desc: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Work {
    _start_date: Date,
    _end_date: Date,
    _title: String,
    _company: String,
    _desc: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Date {
    _year: usize,
    _month: usize,
    _day: usize,
}

#[derive(Debug, Deserialize)]
pub struct Proj {
    _title: String,
    _url: String,
    _stack: String,
    _desc: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct SmallProj {
    _title: String,
    _url: String,
    _desc: Vec<String>,
}

type HobbyProj = SmallProj;
