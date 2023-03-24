#![allow(dead_code, unused_variables)]

use std::collections::hash_map::HashMap;
use std::fmt::Debug;

use termimad::MadSkin;
use termimad as t;
use anyhow::{Context, Result};

use super::qa::*;
use super::querylist::*;


use super::outcome::OutcomeId;

#[derive(Debug)]
pub struct OutcomeResult {
    outcome: OutcomeId,
    output: String,
}

pub struct Querier<'a> {
    ql: QueryList<'a>,
    visited: HashMap<QueryId, bool>,
    next: Option<QueryId>,

}

impl<'a> Querier<'a> {
    pub fn new(qlist: QueryList) -> Querier {
        Querier {
            visited: {
                let mut hm = HashMap::new();
                for qid in &qlist.get_query_ids() {
                    hm.insert(qid.to_owned(), false);
                };
                hm
            },
            ql: qlist,
            next: None,
        }
    }

    fn get_next_unvisited_query_id(&self) -> Option<QueryId> {
        for (qid, visited) in self.visited.iter() {
            if !visited {
                return Some(qid.clone());
            };
        };
        None
    }

    fn get_next_query(&self) -> Option<Query> {
        let qid = self.next.clone().or(self.get_next_unvisited_query_id())?;
        self.ql.get_query(&qid).cloned()
    }

    // TODO make a better return type with an enum/struct later
    pub fn do_next_query(&mut self) -> Result<Option<Vec<OutcomeResult>>> {
        let next = &self.get_next_query();
        match next {
            Some(n) => {
                let q_id = n.id().clone();
                let ans_id = self.execute_query(n)?;

                let ans = self.ql.get_answer(&ans_id).context("can't find answer")?;
                let a_id = ans.id().clone();
                let mut ors = Vec::new();

                for o_id in ans.outcomes() {
                    let r = self.execute_outcome(o_id, q_id, a_id).context("outcome.execute failed")?;
                    ors.push(r);
                };
                self.visited.insert(q_id,true);
                Ok(Some(ors))
            },
            None => Ok(None)
        }
    }

    fn execute_outcome(&self, oid: &OutcomeId, qid: QueryId, aid: AnswerId) -> Result<OutcomeResult> {
        let outcome = self.ql.get_outcome(oid).context("can't find outcome")?;
        let query = self.ql.get_query(&qid).context("can't find query")?;
        let answer = self.ql.get_answer(&aid).context("can't find answer")?;
        let out = outcome.execute(query, answer)?;
        Ok(OutcomeResult{
            outcome: oid.clone(),
            output: out,
        })

    }

    fn execute_query(&self, query: &Query) -> Result<AnswerId> {

        let text = &self.get_query_text(query)?;

        // set up Termimad question engine
        let mut q = t::Question::new(text);
        let skin = MadSkin::default();

        // add answers to engine, making a new map to keep track of ids
        let mut ans_map = HashMap::new();
        for (i,a) in query.answers().iter().enumerate() {
            let ans = self.ql.get_answer(a).context("can't find answer")?;
            q.add_answer(i+1, ans.display());
            ans_map.insert((i+1).to_string(),a);
        };
       
        //actually prompt the user with the question, get resulting "key"
        let ans = q.ask(&skin)?;

        // get the AnswerId associated with the key
        let out = ans_map.get(&ans).unwrap();
        
        //return AnswerId
        Ok(out.to_string())
    }

    fn get_query_text(&'a self, query: &'a Query) -> Result<String> {
        match &query.get_seed() {
            QuerySeed::FromOutcome(o) => {
                let outcome = &self.ql.get_outcome(o).context("can't find outcome")?;
                outcome.execute()
            },
            QuerySeed::Text(t) => {
                Ok(t.to_owned())
            },
        }
    }
}
