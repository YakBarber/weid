

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

//use atty;
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

use weid::qa::*;
use weid::querylist::*;
use weid::querier::*;
use weid::cli;

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

fn do_weid() -> Result<()> {
    let ql = cli::get_arg_queries().unwrap();

    let mut querier = Querier::new(ql);

    while let Some(qid) = querier.pick_next_query() {
        querier.mark_visited(qid);
        let query = querier.get_query(qid.clone()).unwrap();
        let answer = querier.execute_query(&query)?;
        for o in answer.outcomes() {
            let out = o.execute()?;
            stdout().write_all(out.as_bytes())?;
        };
    };

    Ok(())
}


fn main() {
    env_logger::init();
    do_weid().unwrap();
}
