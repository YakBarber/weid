#![allow(unused_variables)]

use std::process::Command;
use std::rc::Rc;
use std::fmt;

use anyhow::Result;

use crate::qa::*;
use crate::querylist::*;



#[derive(Clone)]
pub enum Outcome<'a> {
    Modify(Rc<dyn FnOnce(&mut QueryList) -> Result<()> + 'a>),
    Command(String),
    Closure(Rc<dyn Fn() -> Result<String> + 'a>),
}

impl<'a> fmt::Debug for Outcome<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Outcome")
    }
}

impl<'a> Outcome<'a> {
    pub fn new_cmd(cmd: String) -> Self {
        todo!();
    }

    pub fn new_closure<F>(fun: F) -> Self
    where 
        F: Fn() -> Result<String> + 'a
    {
        Outcome::Closure(Rc::new(fun))
        
    }

    pub fn execute(&self) -> Result<String> {
        match self {
            Outcome::Modify(_) => todo!(),
            Outcome::Closure(f) => {
                f()

            },
            Outcome::Command(cmd) => {
                run_external_cmd(cmd.clone())
            },
        }
    }
}

pub fn run_external_cmd(cmd: String) -> Result<String> {
    let args = cmd.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
    let mut builder = Command::new(&args[0]);
    let _ = &builder.args(&args[1..]);
    
    let out = builder.output()?;
    
    Ok(String::from_utf8(out.stdout)?)
}

