#![allow(dead_code)]

use std::collections::hash_map::HashMap;

use termimad::MadSkin;
use termimad as t;

use super::qa::*;
use super::querylist::*;
use super::util::Result;

use super::outcome::OutcomeId;

pub struct QueryResult {
    query: QueryId,
    answer: AnswerId,
    outcome_results: Vec<OutcomeResult>,
}

pub struct OutcomeResult {
    outcome: OutcomeId,
    output: String,
}

pub enum QuerierState {
    Inactive,
    OnQuery(QueryId),
    OnAnswer(AnswerId, QueryId),
    OnOutcome(OutcomeId, AnswerId, QueryId),
}

pub struct Querier<'a> {
    ql: QueryList<'a>,
    state: QuerierState,
}

impl<'a> Querier<'a> {
    pub fn new(qlist: QueryList) -> Querier {
        Querier {
            ql: qlist,
            state: QuerierState::Inactive,
        }
    }

    pub fn do_next_query(&mut self) -> Result<()> {
        todo!();
    }

    // if query, ask the query
    // if answer, do the outcome 
    // if no outcomes left, do next query
    pub fn progress(&mut self) -> Result<()> {
        todo!();
        //match self.state {
        //    QuerierState::Inactive => {
        //        todo!(); //get the default?
        //    },
        //    QuerierState::OnQuery(qid) => {
        //        let aid = self.execute_query(&qid)?;
        //        self.state = QuerierState::OnAnswer((&aid).clone(), qid);
        //        let display = self.ql.get_answer(&aid)?.display();
        //        Ok(())
        //    },
        //    QuerierState::OnAnswer(aid, qid) => {
        //        todo!();
        //    },
        //    QuerierState::OnOutcome(oid, aid, qid) => {
        //        todo!();
        //    },
        //}
    }

    fn execute_outcome(&mut self, oid: OutcomeId) -> Result<OutcomeResult> {
        let outcome = self.ql.get_outcome(&oid)?;
        let out = outcome.execute()?;
        Ok(OutcomeResult{
            outcome: oid,
            output: out,
        })

    }

    fn execute_query(&mut self, qid: &QueryId) -> Result<AnswerId> {

        let query = &self.ql.get_query(&qid)?;
        let text = &self.get_query_text(&query)?;

        // set up Termimad question engine
        let mut q = t::Question::new(text);
        let skin = MadSkin::default();

        // add answers to engine, making a new map to keep track of ids
        let mut ans_map = HashMap::new();
        for (i,a) in query.answers().iter().enumerate() {
            let ans = &self.ql.get_answer(a)?;
            q.add_answer(i+1, &ans.display());
            ans_map.insert(i.to_string(),a);
        };
       
        //actually prompt the user with the question, get resulting "key"
        let ans = q.ask(&skin)?;

        // get the AnswerId associated with the key
        let out = ans_map.get(&ans).unwrap().to_string();
        
        //return AnswerId
        Ok(out)
    }

    fn get_query_text(&self, query: &Query) -> Result<String> {
        match &query.get_seed() {
            QuerySeed::FromOutcome(o) => {
                let outcome = &self.ql.get_outcome(o)?;
                outcome.execute()
            },
            QuerySeed::Text(t) => {
                Ok(t.to_owned())
            },
        }
    }
}
