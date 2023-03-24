
use std::vec::Vec;
use std::fmt::Debug;
use std::thread::sleep;
use std::time::Duration;
use reqwest;
use reqwest::StatusCode;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use anyhow::{Result, bail};
use spinoff::{Spinner, spinners, Color};

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
    pub tags: String,
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
    wait_time: Duration, // counter for waiting between too-many-requests responses
}

#[allow(dead_code)]
impl PinboardClient {
    pub fn new(auth: String) -> PinboardClient {
        PinboardClient {
            auth_token: auth,
            format: "json".to_string(),
            wait_time: Duration::from_secs(1),
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

    #[allow(unused_assignments)]
    fn api_get<T: DeserializeOwned>(
        &mut self, 
        meth: &str, 
        args: &Vec<(String, String)>,
        progress: bool
    ) -> Result<T> {
        let url = self.make_api_url(meth, args);
        let mut retry = true;

        let mut spinner: Option<Spinner> = None;
        if progress {
            let s = Spinner::new(spinners::Dots, "Waiting for Pinboard...", Color::White);
            spinner = Some(s);
        };

        while retry {
            retry = false;

            let r = reqwest::blocking::get(&url)?;

            match r.status() {
                StatusCode::OK => {
                    let t = r.text()?;
                    self.wait_time = Duration::from_secs(1);
                    if progress {
                        spinner.unwrap().clear();
                    };
                    return Ok(serde_json::from_str(&t[..])?);
                },
                StatusCode::TOO_MANY_REQUESTS => {
                    sleep((&self.wait_time).clone());
                    self.wait_time = self.wait_time + self.wait_time;
                    retry = true;
                },
                status => {
                    panic!(
                        "Got an unexpected status code from pinboard.in: {}", 
                        status
                    );
                },
            };
        };
        bail!("unreachable")
    }

    pub fn get_posts_recent(&mut self, count: u32, progress: bool) -> Result<PinboardPosts> {
        let args = vec!(("count".to_string(), count.to_string()));
        self.api_get::<PinboardPosts>("posts/recent", &args, progress)
    }

    pub fn get_suggested_tags(&mut self, url: &PinboardUrl, progress: bool) -> Result<PinboardSuggested> {
        let args = vec!(("url".to_string(), url.clone()));
        self.api_get::<PinboardSuggested>("posts/suggest", &args, progress)
    }

    pub fn get_all_tags(&mut self, progress: bool) -> Result<PinboardTagList> {
        self.api_get::<PinboardTagList>("tags/get", &Vec::new(), progress)
    }

    // this one has a once per 5 min rate limit!
    pub fn get_all_posts(&mut self, args: Vec<(String,String)>, progress: bool) -> Result<Vec<PinboardPost>> {
        self.api_get::<Vec<PinboardPost>>("posts/all", &args, progress)
    }

    pub fn update_post(&mut self, post: PinboardPost, progress: bool) -> Result<()> {
        let args = vec![
            ("url".to_string(), post.href), 
            ("description".to_string(), post.description), 
            ("extended".to_string(), post.extended), 
            ("tags".to_string(), post.tags), 
            ("dt".to_string(), post.time), 
            ("replace".to_string(), "yes".to_string()), 
            ("shared".to_string(), "no".to_string()), 
            ("toread".to_string(), post.toread), 
        ];
        self.api_get::<PinboardResult>("posts/add", &args, progress)?;
        Ok(())
    }
}

pub fn set_unread(mut client: PinboardClient, post: PinboardPost, progress: bool) -> Result<String> {
    let mut new = post.clone();
    new.toread = "yes".to_string();
    client.update_post(new, progress)?;
    Ok("".to_string())
}

pub fn set_read(mut client: PinboardClient, post: PinboardPost, progress: bool) -> Result<String> {
    let mut new = post.clone();
    new.toread = "no".to_string();
    client.update_post(new, progress)?;
    Ok("".to_string())
}



