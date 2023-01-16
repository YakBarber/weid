
use std::vec::{Vec};
use std::fmt::{Debug};
use reqwest;
use serde::{Deserialize};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub type PinboardUrl = String;

#[derive(Debug,Deserialize)]
pub struct PinboardPost {

    href: String,
    description: String,
    extended: String,
    hash: String,
    time: String,

    #[serde(default)]
    others: String,

    #[serde(default)]
    tag: Vec<String>,
}

#[derive(Debug,Deserialize)]
pub struct PinboardPosts {
    date: String,
    user: String,
    posts: Vec<PinboardPost>,
}

//this is ugly but at least it deserializes.
//should make another public type to make this more usable
pub type PinboardSuggested = Vec<HashMap<String,Vec<String>>>;

#[derive(Debug,Deserialize)]
pub struct PinboardTag {
    count: u32,
    tag: String,
}

pub type PinboardTagList = HashMap<String,u32>;

pub struct PinboardClient {
    required_args: Vec<(String, String)>,
}

#[allow(dead_code)]
impl PinboardClient {
    pub fn new(auth: String) -> PinboardClient {
        PinboardClient {
            required_args: 
                vec!(
                    (String::from("auth_token"), auth),
                    (String::from("format"), String::from("json")),
                    ),
        }
    }

    fn make_api_url(&self, method: &str, args: &Vec<(String, String)>) -> PinboardUrl {
        let mut url = String::from(&format!("https://api.pinboard.in/v1/{method}?"));
        for (k,v) in &self.required_args {
            url.push_str(&format!("{k}={v}&")[..]);
        };
        if !args.is_empty() {
            for (k,v) in args {
                url.push_str(&format!("{k}={v}&")[..]);
            };
        };
        url
    }

    fn api_get<T: DeserializeOwned>(&self, method: &str, args: &Vec<(String, String)> ) -> Result<T> {
        let url = self.make_api_url(method, args);
        let r = reqwest::blocking::get(url)?.text()?;
        //println!("{:?}", r);
        serde_json::from_str(&r[..]).map_err(|e| e.into())
    }

    pub fn get_posts_recent(&self, count: u32) -> Result<PinboardPosts> {
        let mut args = self.required_args.clone();
        args.push((String::from("count"), count.to_string()));
        self.api_get::<PinboardPosts>("posts/recent", &args)
    }

    pub fn get_suggested_tags(&self, url: &PinboardUrl) -> Result<PinboardSuggested> {
        let mut args = self.required_args.clone();
        args.push((String::from("url"), url.clone()));
        self.api_get::<PinboardSuggested>("posts/suggest", &Vec::new())
    }

    pub fn get_all_tags(&self) -> Result<PinboardTagList> {
        self.api_get::<PinboardTagList>("tags/get", &Vec::new())
    }

    // this one has a once per 5 min rate limit!
    pub fn get_all_posts(&self, args: Vec<(String,String)>) -> Result<Vec<PinboardPost>> {
        let mut _args = self.required_args.clone();
        for arg in &args {
            _args.push(arg.clone());
        };
        self.api_get::<Vec<PinboardPost>>("posts/all", &_args)
    }
}


