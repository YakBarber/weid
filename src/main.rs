

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
use weid::querylist::*;
use weid::querier::*;

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


fn create_pinboard_queries<'a>(posts: &'a Vec<pbin::PinboardPost>, client: &'a pbin::PinboardClient) -> QueryList<'a> {

    let mut ql = QueryList::new();

    for (i,post) in posts.iter().enumerate() {

        let query = Query::from_text(&prepare_question_text(post));
        let qid = ql.insert_query(query).unwrap();

        let a1 = Answer::from_text("add_tags");
        let a1id = ql.insert_answer(a1).unwrap();
        ql.link_answer_to_query(&a1id, &qid);

        let a2 = Answer::from_text("skip");
        let a2id = ql.insert_answer(a2).unwrap();
        ql.link_answer_to_query(&a2id, &qid);

        let a3 = Answer::from_text("edit description");
        let a3id = ql.insert_answer(a3).unwrap();
        ql.link_answer_to_query(&a3id, &qid);

        let mut a4 = Answer::from_text("mark read");
        let o4 = Outcome::new(|| pbin::set_read(client.clone(), post.clone()));
        a4.add_outcome(o4.id());
        ql.insert_outcome(o4);
        let a4id = ql.insert_answer(a4).unwrap();
        ql.link_answer_to_query(&a4id, &qid);

        let mut a5 = Answer::from_text("mark unread");
        let o5 = Outcome::new(|| pbin::set_unread(client.clone(), post.clone()));
        a5.add_outcome(o5.id());
        ql.insert_outcome(o5);
        let a5id = ql.insert_answer(a5).unwrap();
        ql.link_answer_to_query(&a5id, &qid);
    };

    ql
}

fn do_output(outs: Vec<&Answer>) -> Result<()> {
    for ans in outs.iter() {
        let bytes = ans.display.as_bytes();
        stdout().write_all(bytes)?
    };
    Ok(())
}


fn output_query_results(anss: Vec<String>) {
    for ans in anss.iter() {
        stdout().write_all(&ans.as_bytes()).unwrap();
        stdout().write_all("\n".as_bytes()).unwrap();
    };
}

fn do_weid(ql: &mut QueryList) -> Result<()> {
    let mut querier = Querier::new(ql.clone());
    loop {
        if let Ok(result) = &querier.do_next_query() {
            todo!();
        }
        else {
            break;
        };
    };

    Ok(())
}


fn main() {
    let auth: String = env::var("PINBOARD_API_TOKEN").unwrap();
    let p = pbin::PinboardClient::new(auth);

    let last = p.get_posts_recent(5).unwrap();

    let mut ql = create_pinboard_queries(&last.posts, &p);

    let out = do_weid(&mut ql);

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

