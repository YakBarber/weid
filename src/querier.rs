#![allow(dead_code, unused_variables)]

use std::collections::hash_map::HashMap;

use termimad::MadSkin;
use termimad as t;

use super::qa::*;
use super::querylist::*;
use super::util::Result;

use super::outcome::OutcomeId;

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

    // TODO: this is gross
    fn pick_new_query(&self) -> Option<QueryId> {
        Some(self.ql.get_query_ids().get(0)?.to_owned())
    }

    fn get_next_query(&self) -> Option<Query> {
        let qid = self.next.clone().or(self.pick_new_query())?;
        self.ql.get_query(&qid).cloned()
    }

    // TODO make a better return type with an enum/struct later
    pub fn do_next_query(&mut self) -> Result<Option<Vec<OutcomeResult>>> {
        let next = &self.get_next_query();
        match next {
            Some(n) => {
                let ans_id = self.execute_query(n)?;

                let ans = self.ql.get_answer(&ans_id).ok_or("can't find answer")?;
                let mut ors = Vec::new();

                for o_id in ans.outcomes() {
                    let r = self.execute_outcome(o_id)?;
                    ors.push(r);
                };
                Ok(Some(ors))
            },
            None => Ok(None)
        }
    }

    fn execute_outcome(&self, oid: &OutcomeId) -> Result<OutcomeResult> {
        let outcome = self.ql.get_outcome(oid).ok_or("can't find outcome")?;
        let out = outcome.execute()?;
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
            let ans = self.ql.get_answer(a).ok_or("can't find answer")?;
            q.add_answer(i+1, ans.display());
            ans_map.insert(i.to_string(),a);
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
                let outcome = &self.ql.get_outcome(o).ok_or("can't find outcome")?;
                outcome.execute()
            },
            QuerySeed::Text(t) => {
                Ok(t.to_owned())
            },
        }
    }
}
