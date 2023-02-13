

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

//use atty;
//use clap::Parser;
use std::env;
use std::fmt::{Debug};
use std::collections::HashMap;
use std::io::{stdout, stderr, Write};
use std::process::Command;
use std::string::String;

use termimad::{MadSkin};
use termimad as t;


use weid::util::Result;
use weid::pbin;

impl Debug for Box<dyn Outcome> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        println!("Outcome");
        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Answer {
    id: u16,
    outcomes: Vec<Box<dyn Outcome>>,
    display: String,
}

#[derive(Debug)]
struct Query {
    id: u16,
    text: String,
    answers: Vec<Answer>,
}

#[derive(Debug,PartialEq)]
enum OutcomeResult {
    NextQuery(u16),
    Success,
    Failure,
}

trait Outcome {
    fn handler(&self, display: &str) -> OutcomeResult;

}


// go to a different Query based on id
struct GotoQueryOutcome {
    goto_ids: HashMap<String, u16>,
}

impl Outcome for GotoQueryOutcome {
    fn handler(&self, display: &str) -> OutcomeResult {
        match self.goto_ids.get(&display[..]) {
            Some(id) => OutcomeResult::NextQuery(*id),
            None     => OutcomeResult::Failure,
        }
    }
}


struct StderrOutcome {}


impl Outcome for StderrOutcome {
    fn handler(&self, display: &str) -> OutcomeResult {
        let bytes = display.as_bytes();
        match stderr().write_all(bytes) {
            Ok(_) => OutcomeResult::Success,
            Err(_) => OutcomeResult::Failure,
        }
    }
}

struct StdoutOutcome {}

impl Outcome for StdoutOutcome {
    fn handler(&self, display: &str) -> OutcomeResult {
        let bytes = display.as_bytes();
        match stdout().write_all(bytes) {
            Ok(_) => OutcomeResult::Success,
            Err(_) => OutcomeResult::Failure,
        }
    }
}

struct CmdOutcome {
    cmdargs: [String],
}

impl CmdOutcome {
    fn run_cmd(&self) -> Result<String>{
        let mut builder = Command::new(&self.cmdargs[0]);
        let _ = &builder.args(&self.cmdargs[1..]);
        
        let out = builder.output()?;
        
        Ok(String::from_utf8(out.stdout)?)
    }
}

impl Outcome for CmdOutcome {
    fn handler(&self, display: &str) -> OutcomeResult {
        match &self.run_cmd() {
            Ok(out) =>
                OutcomeResult::Success,
            Err(_) =>
                OutcomeResult::Failure,
        }
    }
}

fn prepare_question_text(post: &pbin::PinboardPost) -> String {
    let mut tags = "".to_string();
    if !post.tag.is_empty() {
        tags = format!("**{}**", post.tag.join("**, **"));
    }
    format!(
        "**{0}**\n\
        \n\
        {1}\n\
        \n\
        *{2}*\n\
        Tags: {3}
        ",
        post.description, post.extended, post.href, tags
    )
}

fn make_skin() -> MadSkin {
    let skin = MadSkin::default();
    skin
}

fn answers_to_asks(answers: &Vec<Answer>) -> HashMap<String, &Answer> {
    let mut out = HashMap::new();
    for a in answers.iter() {
        out.insert(a.id.to_string(), a);
    };
    out
}


fn do_query<'a>(query: &'a Query) -> Option<&'a Answer> {
    let mut q = t::Question::new(&query.text);
    for a in &query.answers {
        q.add_answer(a.id.to_string(), &a.display);
    };
    let ans_map = answers_to_asks(&query.answers);
   
    let skin = make_skin();
    let ans = q.ask(&skin).ok()?;

    println!("{:?}", ans);

    ans_map.get(&ans).copied()
        
}

fn make_answer(id: u16, display: String) -> Answer{
    let ans = Answer { 
        id: id,
        display: display,
        outcomes: Vec::new(),

    };
    ans
}

fn get_default_outcome() -> Box<dyn Outcome> {
    Box::new(StderrOutcome{})
}

fn execute_outcomes(answer: &Answer) -> Vec<OutcomeResult> {
    let mut output = Vec::new();

    if (&answer.outcomes).is_empty() {
        let default = get_default_outcome();
        let out = default.handler(&answer.display[..]);
        output.push(out);
    }
    else {
        for outcome in (&answer.outcomes).iter() {
            let out = outcome.handler(&answer.display[..]);
            output.push(out);
        };
    };
    output
}

fn create_pinboard_queries(posts: &Vec<pbin::PinboardPost>) -> Vec<Query> {
    let mut queries = Vec::new();


    for (i,post) in posts.iter().enumerate() {
        let mut answers = Vec::new();
        answers.push(make_answer(1, "add tags".to_string()));
        answers.push(make_answer(2, "skip".to_string()));
        answers.push(make_answer(3, "edit description".to_string()));

        let query: Query = Query {
            id: i as u16,
            text: prepare_question_text(&post),
            answers: answers,
        };
        queries.push(query);
    };

    queries
}

fn main() {
    let auth: String = env::var("PINBOARD_API_TOKEN").unwrap();
    let p = pbin::PinboardClient::new(auth);
    //println!("{:?}", p.get_all_posts(vec!(("tag".to_string(), "collapse".to_string()))));
    let last = p.get_posts_recent(5).unwrap();

    let queries = create_pinboard_queries(&last.posts);
    println!("{:?}", queries);

    for query in queries {
        let chosen = do_query(&query);

        println!("{:?}", chosen);
        
        let from_outcomes = match chosen {
            Some(ans) => {
                execute_outcomes(ans)
            },
            None => {
                println!("{:?}", chosen);
                Vec::new()
            },
        };
        println!("{:?}", from_outcomes);
    };

}

#[cfg(test)]
mod test {

    use super::*;

    struct SuccessOutcome {}
    struct FailureOutcome {}
    struct NextQueryOutcome {}

    impl Outcome for SuccessOutcome {
        fn handler(&self, display: &str) -> OutcomeResult {
            OutcomeResult::Success
        }
    }

    impl Outcome for FailureOutcome {
        fn handler(&self, display: &str) -> OutcomeResult {
            OutcomeResult::Failure
        }
    }

    impl Outcome for NextQueryOutcome {
        fn handler(&self, display: &str) -> OutcomeResult {
            OutcomeResult::NextQuery(2)
        }
    }

    fn fake_pin() -> pbin::PinboardPost {
        pbin::PinboardPost {
            href: "http://butts.poop".to_string(),
            description: "this is the description".to_string(),
            extended: "this is the extended".to_string(),
            hash: "abcde123456789".to_string(),
            time: "this is the timestamp".to_string(),
            others: "".to_string(),
            tag: vec!["tag1".to_string(), "tag2".to_string()],
        }
    }

    fn fake_answer() -> Answer {
        let outcomes:Vec<Box<dyn Outcome>> = vec![
            Box::new(SuccessOutcome {}),
            Box::new(FailureOutcome {}),
            Box::new(NextQueryOutcome {}),
        ];
        Answer {
            id: 1,
            outcomes: outcomes,
            display: "answer happened".to_string(),
        }
    }

    #[test]
    fn test_execute_outcomes() -> () {
        let ans = fake_answer();
        let out = execute_outcomes(&ans);
        let right = vec![
            OutcomeResult::Success,
            OutcomeResult::Failure,
            OutcomeResult::NextQuery(2),
        ];
        assert_eq!(out, right);
    }
}



//in the future, use subprocesses to make it so piping and keyboard can both work
//
//fn get_piped() -> Option<Vec<String>> {
//    if atty::isnt(atty::Stream::Stdin) {
//        let lines = io::stdin().lines();
//        lines.collect::<Result<Vec<String>, io::Error>>().ok()
//    }
//    else {
//        None
//    }
//}
//
//fn questions_from_file<T: io::BufRead>(fhandle: T) -> Option<Vec<String>> {
//    let lines = fhandle.lines();
//    lines.collect::<Result<Vec<String>, io::Error>>().ok()
//}

