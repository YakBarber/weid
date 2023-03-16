
use super::qa::*;
use super::querylist::*;

use super::outcome::OutcomeId;

pub struct Querier<'a> {
    ql: QueryList<'a>,
    active_query: Option<QueryId>,
    active_answer: Option<AnswerId>,
    active_outcome: Option<OutcomeId>,
}

impl<'a> Querier<'a> {
    pub fn new(qlist: QueryList) -> Querier {
        Querier {
            ql: qlist,
            active_query: None,
            active_answer: None,
            active_outcome: None,
        }
    }

    // if query, ask the query
    // if answer, do the outcome 
    // if no outcomes left, do next query
    pub fn progress(&mut self) {
        todo!()
    }
}
