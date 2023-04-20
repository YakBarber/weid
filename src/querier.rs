#![allow(dead_code, unused_variables)]

use std::collections::hash_map::HashMap;
use std::fmt::Debug;
use std::io::{stdout, Write};

use termimad::MadSkin;
use termimad as t;
use anyhow::{Context, Result};

use super::qa::*;
use super::querylist::*;
use super::outcome::*;

type OutcomeResult = String;

pub struct Querier<'a> {
    ql: QueryList<'a>,
    next: Option<QueryId>,
    visited: Vec<QueryId>,

}

impl<'a> Querier<'a> {
    pub fn new(qlist: QueryList) -> Querier {
        Querier {
            ql: qlist,
            next: None,
            visited: Vec::new(),
        }
    }

    pub fn pick_next_query(&self) -> Option<QueryId> {
        for key in self.ql.peek_queries().keys() {
            if !self.visited.contains(key) {
                return Some(key.clone());
            };
        };
        None
    }

    pub fn get_next_query(&self) -> Option<Query> {
        match self.next {
            None => {
                let qid = self.pick_next_query()?;
                self.get_query(qid)
            },
            Some(qid) => {
                self.get_query(qid)
            },
        }
    }

    pub fn get_query(&self, qid: QueryId) -> Option<Query> {
        self.ql.get_query(qid)
    }

    pub fn mark_visited(&mut self, qid: QueryId) {
        self.visited.push(qid)
    }

    pub fn execute_query(&self, query: &Query<'a>) -> Result<Answer<'a>> {

        let text = query.display();

        // set up Termimad question engine
        let mut q = t::Question::new(text);
        let skin = MadSkin::default();

        // add answers to engine, making a new map to keep track of ids
        let answers = query.answers();
        for (i,a) in answers.iter().enumerate() {
            q.add_answer(i+1, a.display());
        };
       
        //actually prompt the user with the question, get resulting "key"
        let ans = q.ask(&skin)?.parse::<usize>()?;
        
        //return Answer
        Ok(answers.get(ans-1).unwrap().to_owned())
    }
    
    pub fn execute_outcome(&mut self, outcome: Outcome<'a>) -> Result<String> {
        let out = outcome.execute()?;
        stdout().write_all(out.as_bytes())?;
        Ok(out)
    }

}
