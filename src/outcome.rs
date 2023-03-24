#![allow(unused_variables)]

use std::process::Command;
use std::rc::Rc;
use std::fmt;

use nanoid::nanoid;
use anyhow::Result;

use crate::qa::*;

pub type OutcomeId = String;

#[derive(Clone)]
pub struct Outcome<'a> {
    _id: String,
    closure: Rc<dyn Fn(&Query, &Answer) -> Result<String> + 'a>,
}

impl<'a> fmt::Debug for Outcome<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Outcome #{:?}", self._id)
    }
}

impl<'a> Outcome<'a> {
    pub fn id(&self) -> &OutcomeId {
        &self._id
    }

    pub fn execute(&self, query: &Query, answer: &Answer) -> Result<String> {
        (&self.closure)(query, answer)
    }

    pub fn new<F>(cloj: F) -> Outcome<'a>
    where
        F: Fn(&Query, &Answer) -> Result<String> + 'a,
    {
        Outcome {
            _id: nanoid!(),
            closure: Rc::new(cloj),
        }
    }


    pub fn new_cmd(cmd: &'a [&str]) -> Outcome<'a> {
        Outcome {
            _id: nanoid!(),
            closure: Rc::new(|q, a| run_external_cmd(cmd)),
        }
    }
}

pub fn run_external_cmd(args: &[&str]) -> Result<String>{
    let mut builder = Command::new(args[0]);
    let _ = &builder.args(&args[1..]);
    
    let out = builder.output()?;
    
    Ok(String::from_utf8(out.stdout)?)
}

