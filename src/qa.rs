#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::fmt::Debug;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::iter::IntoIterator;

use nanoid::nanoid;
use anyhow::Result;

use super::outcome::*;

#[derive(Clone, Debug)]
pub struct Answer<'a> {
    display: String,
    outcomes: Vec<Outcome<'a>>,
}

impl<'a> Answer<'a> {
    pub fn from_text(display: String) -> Answer<'a> {
        Answer {
            display,
            outcomes: Vec::new(),
        }
    }

    pub fn display(&self) -> String {
        self.display.clone()
    }

    pub fn add_outcome(&mut self, outcome: Outcome<'a>) {
        self.outcomes.push(outcome);
    }

    pub fn outcomes(&self) -> Vec<Outcome<'a>> {
        self.outcomes.clone()
    }
}

impl<'a> PartialEq for Answer<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.display == other.display
    }
}


#[derive(Clone, Debug)]
pub struct Query<'a> { 
    display: String,
    answers: Vec<Answer<'a>>,
}

impl<'a> Query<'a> {
    pub fn from_text(display: String) -> Query<'a> {
        Query {
            display,
            answers: Vec::new(),
        }
    }

    pub fn display(&self) -> &String {
        &self.display
    }

    pub fn add_answer(&mut self, answer: Answer<'a>) {
        self.answers.push(answer);
    }

    pub fn add_answers<I>(&mut self, iter: I) 
    where
        I: IntoIterator<Item = Answer<'a>>,
    {
        for i in iter.into_iter() {
            self.add_answer(i);
        };
    }

    pub fn answers(&self) -> Vec<Answer<'a>> {
        self.answers.clone()
    }
}

impl<'a> PartialEq for Query<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.display == other.display
    }
}

