

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

//use atty;
//use clap::Parser;
use std::env;
use std::string::String;
use std::collections::HashMap;
use std::io::{stdout, stderr, Write, Read};
use std::process::Command;
use std::fs::File;

use termimad::MadSkin;
use termimad as t;
use nanoid::nanoid;
use anyhow::{Context, Result};
use tempfile::tempdir;
use open;

mod pbin;
use weid::outcome::*;
use weid::qa::*;
use weid::querylist::*;
use weid::querier::*;

fn prepare_question_text(
    post: &pbin::PinboardPost, 
    pbtags: &pbin::PinboardSuggested,
) -> String {
    //tags already on the post
    let mut tags = "".to_string();
    if !post.tags.split(" ").collect::<Vec<&str>>().is_empty() {
        tags = format!("**{}**", post.tags);
    }

    //"suggested" tags are the popular tags for this post
    let mut stags = "".to_string();
    if let Some(sugged) = pbtags[0].get("suggested") {
        if !sugged.is_empty() {
            stags = format!("**{}**", sugged.join("**, **"));
        };
    };

    //"recommended" tags are recommended from the user's own tag list
    let mut rtags = "".to_string();
    if let Some(recced) = pbtags[0].get("recommended") {
        if !recced.is_empty() {
            rtags = format!("**{}**", recced.join("**, **"));
        };
    };

    //println!("{:?}", post);

    // output the markdown
    format!(
        "**{0}**\n\
        \n\
        {1}\n\
        \n\
        *{2}*\n\n\
        Tags: {3}\n\
        Suggested tags: {4}\n\
        Recommended tags: {5}\n\
        ",
        post.description, post.extended, post.href, tags, stags, rtags
    )
}

fn edit_in_editor(start_text: &String) -> Result<String> {
    let editor = env::var("EDITOR").context("no EDITOR defined")?;
    let tmpdir = tempdir()?;
    let tmp_path = tmpdir.path().join(format!("{}.{}",nanoid!(),"sh"));
    let mut tmp = File::create(&tmp_path)?;

    writeln!(tmp, "{}", start_text)?;

    Command::new(&editor).arg(&tmp_path).status().context("editor spawn failed")?;

    let mut change = String::new();
    File::open(tmp_path).context("file read failed")?.read_to_string(&mut change)?;

    Ok(change)
}

fn create_pinboard_query<'a>(
    post: pbin::PinboardPost,
    pbtags: pbin::PinboardSuggested, 
    client: pbin::PinboardClient,
) -> Query<'a> {

    let mut query = Query::from_text(prepare_question_text(&post, &pbtags));

    let a1 = Answer::from_text("skip".to_string());


    let mut a2 = Answer::from_text("update tags".to_string());
    let post2 = post.clone();
    let client2 = client.clone();
    let o2 = Outcome::new_closure(move || {
        let mut p = post2.clone();
        let tags = &post2.tags;
        let new = edit_in_editor(tags)?;
        p.tags = new.to_string();
        client2.clone().update_post(p, true)?;
        Ok(new)
    });
    a2.add_outcome(o2);
    query.add_answer(a2);

    let mut a3 = Answer::from_text("edit extended description".to_string());
    let post3 = post.clone();
    let client3 = client.clone();
    let o3 = Outcome::new_closure(move || {
        let mut p = post3.clone();
        let new = edit_in_editor(&p.extended)?;
        p.extended = new.clone();
        client3.clone().update_post(p, true)?;
        Ok(new)
    });
    a3.add_outcome(o3);
    query.add_answer(a3);

    let mut a4 = Answer::from_text("mark read".to_string());
    let post4 = post.clone();
    let client4 = client.clone();
    let o4 = Outcome::new_closure(move || pbin::set_read(client4.clone(), post4.clone(), true));
    a4.add_outcome(o4);
    query.add_answer(a4);

    let mut a5 = Answer::from_text("mark unread".to_string());
    let post5 = post.clone();
    let client5 = client.clone();
    let o5 = Outcome::new_closure(move || pbin::set_unread(client5.clone(), post5.clone(), true));
    a5.add_outcome(o5);
    query.add_answer(a5);

    let mut a6 = Answer::from_text("view in browser".to_string());
    let post6 = post.clone();
    let o6 = Outcome::new_closure(move || {
        open::that(&post6.href)?;
        Ok("".to_string())
    });
    a6.add_outcome(o6);
    query.add_answer(a6);

    query
}

fn do_output(outs: Vec<&Answer>) -> Result<()> {
    for ans in outs.iter() {
        let disp = ans.display();
        let bytes = disp.as_bytes();
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

    while let Some(qid) = querier.pick_next_query() {
        querier.mark_visited(qid);
        let query = querier.get_query(qid.clone()).unwrap();
        let answer = querier.execute_query(&query)?;
        for o in answer.outcomes() {
            let result = o.execute()?;
        };
    };

    Ok(())
}


fn main() {
    let auth: String = env::var("PINBOARD_API_TOKEN").unwrap();
    let mut p = pbin::PinboardClient::new(auth);

    let dates_raw = p.get_post_dates(true).unwrap();
    let mut dates = dates_raw.dates.keys().collect::<Vec<&String>>();
    dates.sort();
    dates.reverse();
    for date in dates {
        
        let mut ql = QueryList::new();
        let posts = p.get_posts_by_date(date.clone(), true).unwrap();
        for post in posts.posts {
            let pbtags = p.get_suggested_tags(&post.href, true).unwrap();
            let this_q = create_pinboard_query(post.clone(), pbtags.clone(), p.clone());
            ql.insert_query(this_q);
        };

        let out = do_weid(&mut ql);
    };

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

