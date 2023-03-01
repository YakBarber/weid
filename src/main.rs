

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

//use atty;
//use clap::Parser;
use std::env;
use std::string::String;
use std::collections::HashMap;
use std::io::{stdout, stderr, Write};

use termimad::MadSkin;
use termimad as t;

use weid::util::Result;
use weid::pbin;
use weid::outcome::*;
use weid::qa::*;

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

fn do_output(outs: Vec<&Answer>) -> Result<()> {
    for ans in outs.iter() {
        let bytes = ans.display.as_bytes();
        stdout().write_all(bytes)?
    };
    Ok(())
}

fn execute_answer(answer: Option<&Answer>) -> Option<(String, Vec<OutcomeResult>)> {
    answer.map(|ans| {
        let out_result = ans.execute_outcomes();
        match &ans.output {
            None => (ans.display.clone(), out_result),
            Some(s) => (s.clone(), out_result),
        }
    })
}

fn output_query_results(anss: Vec<String>) {
    for ans in anss.iter() {
        stdout().write_all(&ans.as_bytes()).unwrap();
        stdout().write_all("\n".as_bytes()).unwrap();
    };
}

fn do_weid(qlist: QueryList) {

    let mut ans_outs = Vec::new();

    for query in qlist {
        let answer = do_query(&query);
        let (ans_out, _) = execute_answer(answer).unwrap();
        ans_outs.push(ans_out);
    };
    output_query_results(ans_outs);
}

fn do_query<'a>(query: &'a Query) -> Option<&'a Answer> {
    let mut q = t::Question::new(&query.text);
    for a in &query.answers {
        q.add_answer(a.id.to_string(), &a.display);
    };
    let ans_map = answers_to_asks(&query.answers);
   
    let skin = make_skin();
    let ans = q.ask(&skin).ok()?;

    ans_map.get(&ans).copied()
        
}


fn main() {
    let auth: String = env::var("PINBOARD_API_TOKEN").unwrap();
    let p = pbin::PinboardClient::new(auth);

    let last = p.get_posts_recent(5).unwrap();

    let queries = create_pinboard_queries(&last.posts);

    let qlist = QueryList::from_iter(queries);
    let out = do_weid(qlist);

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
            output: Some("answer happened".to_string()),
            display: "choose me!".to_string(),
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

