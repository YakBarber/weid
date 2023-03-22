#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::fmt::Debug;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::marker::PhantomData;

use nanoid::nanoid;
use anyhow::Result;

use super::outcome::*;

pub type AnswerId = String;
pub type QueryId = String;

#[derive(Clone, Hash, PartialEq, Debug)]
pub struct Answer {
    pub display: String,
    _id: AnswerId,
    next_query: Option<QueryId>,
    outcomes: Vec<OutcomeId>,
}

impl Answer {
    pub fn from_text(display: &str) -> Answer {
        Answer {
            display: display.to_owned(),
            _id: nanoid!(),
            next_query: None,
            outcomes: Vec::new(),
        }
    }

    pub fn id(&self) -> &AnswerId {
        &self._id
    }

    pub fn outcomes(&self) -> &Vec<OutcomeId> {
        &self.outcomes
    }

    pub fn display(&self) -> String {
        self.display.clone()
    }

    pub fn set_next_query(&mut self, qid: &QueryId) {
        self.next_query = Some(qid.clone());
    }

    pub fn add_outcome(&mut self, oid: &OutcomeId) {
        self.outcomes.push(oid.clone());
    }
}

#[derive(Clone, Debug)]
pub enum QuerySeed {
    Text(String),
    FromOutcome(OutcomeId),
}

#[derive(Clone, Debug)]
pub struct Query { 
    _id: QueryId,
    seed: QuerySeed,
    answers: Vec<AnswerId>,
}

impl Query {
    pub fn from_text(display: &str) -> Query {
        Query {
            seed: QuerySeed::Text(display.to_owned()),
            _id: nanoid!(),
            answers: Vec::new(),
        }
    }

    pub fn from_outcome(outcome: OutcomeId) -> Query {
        Query {
            seed: QuerySeed::FromOutcome(outcome),
            _id: nanoid!(),
            answers: Vec::new(),
        }
    }

    pub fn id(&self) -> &QueryId {
        &self._id
    }

    pub fn add_answer(&mut self, ans: &AnswerId) {
        self.answers.push(ans.clone());
    }

    pub fn answers(&self) -> &Vec<AnswerId> {
        &self.answers
    }

    pub fn get_seed(&self) -> &QuerySeed {
        &self.seed
    }
}

