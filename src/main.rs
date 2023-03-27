

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

use weid::outcome::*;
use weid::qa::*;
use weid::querylist::*;
use weid::querier::*;

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

fn do_weid(ql: &mut QueryList) -> Result<()> {
    let mut querier = Querier::new(ql.clone());
    loop {
        let next = &querier.do_next_query();
        match &next {
            Ok(Some(result)) => {
                println!("{:?}", result);
                continue; 
            },
            Err(e) => {
                println!("oh no {}", e);
                next.as_ref().unwrap();
                break;
            },
            Ok(None) => {
                println!("done!");
                break;
            },
        };
    };

    Ok(())
}


fn main() {
}
