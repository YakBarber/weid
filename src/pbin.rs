
use std::vec::Vec;
use std::fmt::Debug;
use reqwest;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use anyhow::Result;

pub type PinboardUrl = String;

#[derive(Debug,Deserialize)]
pub struct PinboardResult {
    pub result_code: String,
}

#[derive(Debug,Deserialize,Clone)]
pub struct PinboardPost {

    pub href: String,
    pub description: String,
    pub extended: String,
    pub hash: String,
    pub time: String,
    pub toread: String,

    #[serde(default)]
    pub others: String,

    #[serde(default)]
    pub tag: Vec<String>,
}

#[derive(Debug,Deserialize)]
pub struct PinboardPosts {
    pub date: String,
    pub user: String,
    pub posts: Vec<PinboardPost>,
}

//this is ugly but at least it deserializes.
//should make another public type to make this more usable
pub type PinboardSuggested = Vec<HashMap<String,Vec<String>>>;

#[derive(Debug,Deserialize)]
#[allow(dead_code)]
pub struct PinboardTag {
    count: u32,
    tag: String,
}

pub type PinboardTagList = HashMap<String,u32>;

#[derive(Clone)]
pub struct PinboardClient {
    auth_token: String, // user:1234567890ABCDEABCDE
    format: String, // only "json" right now
}

#[allow(dead_code)]
impl PinboardClient {
    pub fn new(auth: String) -> PinboardClient {
        PinboardClient {
            auth_token: auth,
            format: "json".to_string(),
        }
    }

    fn make_api_url(&self, method: &str, args: &Vec<(String, String)>) -> PinboardUrl {
        let mut url = String::from(&format!("https://api.pinboard.in/v1/{method}?"));
        url.push_str(&format!("auth_token={}&", self.auth_token)[..]);
        url.push_str(&format!("format={}&", self.format)[..]);
        if !args.is_empty() {
            for (k,v) in args {
                url.push_str(&format!("{k}={v}&")[..]);
            };
        };
        url
    }

    fn api_get<T: DeserializeOwned>(&self, meth: &str, args: &Vec<(String, String)> ) -> Result<T> {
        let url = self.make_api_url(meth, args);
        let r = reqwest::blocking::get(url)?.text()?;
        Ok(serde_json::from_str(&r[..])?)
    }

    pub fn get_posts_recent(&self, count: u32) -> Result<PinboardPosts> {
        let args = vec!(("count".to_string(), count.to_string()));
        self.api_get::<PinboardPosts>("posts/recent", &args)
    }

    pub fn get_suggested_tags(&self, url: &PinboardUrl) -> Result<PinboardSuggested> {
        let args = vec!(("url".to_string(), url.clone()));
        self.api_get::<PinboardSuggested>("posts/suggest", &args)
    }

    pub fn get_all_tags(&self) -> Result<PinboardTagList> {
        self.api_get::<PinboardTagList>("tags/get", &Vec::new())
    }

    // this one has a once per 5 min rate limit!
    pub fn get_all_posts(&self, args: Vec<(String,String)>) -> Result<Vec<PinboardPost>> {
        self.api_get::<Vec<PinboardPost>>("posts/all", &args)
    }

    pub fn update_post(&self, post: PinboardPost) -> Result<()> {
        let args = vec![
            ("url".to_string(), post.href), 
            ("description".to_string(), post.description), 
            ("extended".to_string(), post.extended), 
            ("tags".to_string(), post.tag.join(" ")), 
            ("dt".to_string(), post.time), 
            ("replace".to_string(), "yes".to_string()), 
            ("shared".to_string(), "no".to_string()), 
            ("toread".to_string(), post.toread), 
        ];
        self.api_get::<PinboardResult>("posts/add", &args)?;
        Ok(())
    }
}

pub fn set_unread(client: PinboardClient, post: PinboardPost) -> Result<String> {
    let mut new = post.clone();
    new.toread = "yes".to_string();
    client.update_post(new)?;
    Ok("".to_string())
}

pub fn set_read(client: PinboardClient, post: PinboardPost) -> Result<String> {
    let mut new = post.clone();
    new.toread = "no".to_string();
    client.update_post(new)?;
    Ok("".to_string())
}



