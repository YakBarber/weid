#![allow(unused_variables)]

use std::fmt::Debug;
use std::collections::HashMap;
use std::io::{stdout, stderr, Write};
use std::process::Command;

use super::util::Result;

impl Debug for Box<dyn Outcome> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        println!("Outcome");
        Ok(())
    }
}

#[derive(Debug,PartialEq,Clone,Copy)]
pub enum OutcomeResult {
    NextQuery(u16),
    Success,
    Failure,
}

pub trait Outcome {
    fn handler(&self, display: &str) -> OutcomeResult;
}

pub struct CmdOutcome {
    cmdargs: [String],
}

// go to a different Query based on id
pub struct GotoQueryOutcome {
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

