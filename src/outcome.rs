#![allow(unused_variables)]

use std::fmt::Debug;
use std::process::Command;
use std::rc::Rc;

use nanoid::nanoid;

use super::util::Result;

pub type OutcomeId = String;

#[derive(Clone)]
pub struct Outcome<'a> {
    _id: String,
    closure: Rc<dyn Fn() -> Result<String> + 'a>,
}

impl<'a> Outcome<'a> {
    pub fn id(&self) -> &OutcomeId {
        &self._id
    }

    pub fn execute(&self) {
        (&self.closure)();
    }

    pub fn new<F>(cloj: F) -> Outcome<'a>
    where
        F: Fn() -> Result<String>,
    {
        Outcome {
            _id: nanoid!(),
            closure: Rc::new(cloj),
        }
    }


    pub fn new_cmd(cmd: &'a [&str]) -> Outcome<'a> {
        Outcome {
            _id: nanoid!(),
            closure: Rc::new(|| run_external_cmd(cmd)),
        }
    }
}

pub fn run_external_cmd(args: &[&str]) -> Result<String>{
    let mut builder = Command::new(args[0]);
    let _ = &builder.args(&args[1..]);
    
    let out = builder.output()?;
    
    Ok(String::from_utf8(out.stdout)?)
}

