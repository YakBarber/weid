
#![allow(dead_code)]

use std::collections::HashMap;

use nanoid::nanoid;

use super::util::Result;
use super::outcome::{Outcome, OutcomeId};
use super::qa::*;

enum QLChange<'a> {
    Query(Query),
    Answer(Answer),
    Outcome(Outcome<'a>),
}

#[derive(Clone)]
pub struct QueryList<'a> {
    queries: HashMap<QueryId, Query>,
    answers: HashMap<AnswerId, Answer>,
    outcomes: HashMap<OutcomeId, Outcome<'a>>,
}

impl<'a> QueryList<'a> {
    pub fn new() -> QueryList<'a> {
        QueryList {
            queries: HashMap::new(),
            answers: HashMap::new(),
            outcomes: HashMap::new(),
        }
    }

    //TODO: Add reference checks
    pub fn new_from_vecs(queries: Vec<Query>, answers: Vec<Answer>, outcomes: Vec<Outcome<'a>>) -> QueryList<'a> {
        QueryList {
            queries: {
                let mut new = HashMap::new();
                for q in queries {
                    new.insert(q.id().clone(), q);
                };
                new
            },
            answers: {
                let mut new = HashMap::new();
                for a in answers {
                    new.insert(a.id().clone(), a);
                };
                new
            },
            outcomes: {
                let mut new = HashMap::new();
                for o in outcomes {
                    new.insert(o.id().clone(), o);
                };
                new
            },
        }
    }

    pub fn insert_query(&mut self, query: Query) -> Option<QueryId> {
        let change = QLChange::Query(query);
        self.make_change(change)
    }

    pub fn insert_answer(&mut self, answer: Answer) -> Option<AnswerId> {
        let change = QLChange::Answer(answer);
        self.make_change(change)
    }

    pub fn insert_outcome(&mut self, outcome: Outcome) -> Option<OutcomeId> {
        let change = QLChange::Outcome(outcome);
        self.make_change(change)
    }

    pub fn link_answer_to_query(&mut self, aid: &AnswerId, qid: &QueryId) -> Option<QueryId> {
        let mut new_query = self.get_query(qid)?.clone();
        new_query.add_answer(aid);
        self.insert_query(new_query)
    }

    pub fn link_outcome_to_answer(&mut self, oid: &OutcomeId, aid: &AnswerId) -> Option<AnswerId> {
        let mut new_answer = self.get_answer(aid)?.clone();
        new_answer.add_outcome(aid);
        self.insert_answer(new_answer)
    }
 
    pub fn get_query(&self, qid: &QueryId) -> Option<&Query> {
        self.queries.get(qid)
    }

    pub fn get_answer(&self, aid: &AnswerId) -> Option<&Answer> {
        self.answers.get(aid)
    }

    pub fn get_outcome(&self, oid: &OutcomeId) -> Option<&Outcome> {
        self.outcomes.get(oid)
    }

    fn make_change(&mut self, change: QLChange) -> Option<String> {
        if self.validate_change(&change) {
            match change {
                QLChange::Query(q) => {
                    self.queries.insert((&q.id()).to_string(), q);
                    Some(&q.id().to_string())
                },
                QLChange::Answer(a) => {
                    self.answers.insert(a.id().clone(), a);
                    Some(a.id().clone())
                },
                QLChange::Outcome(o) => {
                    self.outcomes.insert(o.id().clone(), o);
                    Some(o.id().clone())
                },
            }
        }
        else {
            None
        }
    }

    pub fn extend(&mut self, new_ql: QueryList<'a>) -> Option<()> {
        let mut new = self.clone();

        for (qid, q) in new_ql.queries {
            new.queries.insert(qid, q);
        };
        for (aid, a) in new_ql.answers {
            new.answers.insert(aid, a);
        };
        for (oid, o) in new_ql.outcomes {
            new.outcomes.insert(oid, o);
        };

        if QueryList::validate(&new) {
            self.queries = new.queries;
            self.answers = new.answers;
            self.outcomes = new.outcomes;
            Some(())
        }
        else {
            None
        }
    }

    fn validate_change(&self, change: &QLChange) -> bool {
        match change {
            
            //nothing to check when adding an outcome
            QLChange::Outcome(_) => {
                return true
            },
            
            //make sure all outcomes exist
            QLChange::Answer(ans) => {
                for o in ans.outcomes() {
                    if !&self.outcomes.contains_key(o) {
                        return false
                    };
                };
            },

            //make sure all answers and outcomes exist, if referenced
            QLChange::Query(query) => {
                if let Some(ans) = &query.answers() {
                    for a in ans.iter() {
                        if !&self.answers.contains_key(a) {
                            return false
                        };
                    };
                };
                if let Some(QuerySeed::FromOutcome(o)) = query.get_seed() {
                    if !&self.outcomes.contains_key(&o[..]) {
                        return false
                    };
                };
            },
        };
        false
    }

    fn validate(new_ql: &'a QueryList) -> bool {
        for a in new_ql.answers.values() {
            for o in a.outcomes() {
                if !new_ql.outcomes.contains_key(o) {
                    return false
                };
            };
        };

        for q in new_ql.queries.values(){
            if let Some(QuerySeed::FromOutcome(o)) = &q.get_seed() {
                if !new_ql.outcomes.contains_key(o) {
                    return false
                };
            };
            if let Some(anss) = &q.answers() {
                for a in anss.iter() {
                    if !new_ql.answers.contains_key(a) {
                        return false
                    };
                };
            }
        };
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn make_simple_querylist<'a>() -> QueryList<'a> {
        let mut q1 = Query::from_text("a question");
        let mut a1 = Answer::from_text("an answer");
        let o1 = Outcome::new_cmd(&["echo", "lol"]);

        q1.add_answer(a1.id());
        a1.add_outcome(&o1.id());

        QueryList::new_from_vecs(vec![q1], vec![a1], vec![o1])

    }


    #[test]
    fn check_querylist_init_update() {
        let mut new = QueryList::new();
    }
}
