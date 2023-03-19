
#![allow(dead_code)]

use std::collections::HashMap;
use std::error::Error;
use std::fmt::Debug;

use super::util::Result;
use super::outcome::{Outcome, OutcomeId};
use super::qa::*;

#[derive(Debug)]
enum QLChange<'a> {
    Query(Query),
    Answer(Answer),
    Outcome(Outcome<'a>),
}

#[derive(Clone, Debug)]
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

    pub fn insert_outcome(&mut self, outcome: Outcome<'a>) -> Option<OutcomeId> {
        let change = QLChange::Outcome(outcome);
        self.make_change(change)
    }

    pub fn link_answer_to_query(&mut self, aid: &AnswerId, qid: &QueryId) -> Option<QueryId> {
        let mut new_query = self.get_query(qid).ok()?.clone();
        new_query.add_answer(aid);
        self.insert_query(new_query)
    }

    pub fn link_outcome_to_answer(&mut self, oid: &OutcomeId, aid: &AnswerId) -> Option<AnswerId> {
        let mut new_answer = self.get_answer(aid).ok()?.clone();
        new_answer.add_outcome(oid);
        self.insert_answer(new_answer)
    }
 
    pub fn get_query(&self, qid: &QueryId) -> Result<&Query> {
        self.queries.get(qid).ok_or(Box::<dyn Error>::from("No Query by that QueryId"))
    }

    pub fn get_answer(&self, aid: &AnswerId) -> Result<&Answer> {
        self.answers.get(aid).ok_or(Box::<dyn Error>::from("No Answer by that AnswerId"))
    }

    pub fn get_outcome(&self, oid: &OutcomeId) -> Result<&Outcome> {
        self.outcomes.get(oid).ok_or(Box::<dyn Error>::from("No Outcome by that OutcomeId"))
    }

    fn make_change(&mut self, change: QLChange<'a>) -> Option<String> {
        if self.validate_change(&change) {
            match change {
                QLChange::Query(q) => {
                    let qid = &q.id().clone();
                    self.queries.insert(qid.to_string(), q);
                    Some(qid.to_string())
                },
                QLChange::Answer(a) => {
                    let aid = &a.id().clone();
                    self.answers.insert(aid.to_string(), a);
                    Some(aid.to_string())
                },
                QLChange::Outcome(o) => {
                    let oid = &o.id().clone();
                    self.outcomes.insert(oid.to_string(), o);
                    Some(oid.to_string())
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
                    if self.outcomes.contains_key(o) {
                        return true
                    };
                };
            },

            //make sure all answers and outcomes exist, if referenced
            QLChange::Query(query) => {
                let ans = &query.answers();
                for a in ans.iter() {
                    if self.answers.contains_key(a) {
                        return true
                    };
                };
                if let QuerySeed::FromOutcome(o) = query.get_seed() {
                    if self.outcomes.contains_key(&o[..]) {
                        return true
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
            if let QuerySeed::FromOutcome(o) = &q.get_seed() {
                if !new_ql.outcomes.contains_key(o) {
                    return false
                };
            };
            let anss = &q.answers();
            for a in anss.iter() {
                if !new_ql.answers.contains_key(a) {
                    return false
                };
            };
        };
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // nothing is linked or referenced here so it's useless, but it is valid
    fn make_simple_querylist<'a>() -> QueryList<'a> {
        let mut q1 = Query::from_text("a question");
        let mut a1 = Answer::from_text("an answer");
        let o1 = Outcome::new_cmd(&["echo", "lol"]);

        q1.add_answer(a1.id());
        a1.add_outcome(&o1.id());

        QueryList::new_from_vecs(vec![q1], vec![a1], vec![o1])

    }

    #[test]
    fn validate_validation() {
        //create known-good querylist
        let mut ql = make_simple_querylist();

        assert!(QueryList::validate(&ql));

        // prep some new changes
        let bad_o_1 = Outcome::new(|| todo!()); //the closures shouldn't run
        let bad_o_2 = Outcome::new(|| todo!());

        let bad_q_1 = Query::from_outcome(bad_o_1.id().to_owned());
        let mut bad_q_2 = Query::from_text("asdf");

        let mut bad_a_1 = Answer::from_text("some answer");

        bad_q_2.add_answer(&bad_a_1.id());
        bad_a_1.add_outcome(&bad_o_2.id());

        // adding the queries should both fail as-is
        assert_eq!(None, ql.insert_query(bad_q_1.clone()));
        assert_eq!(None, ql.insert_query(bad_q_2.clone()));

        // adding the answer should also fail as-is
        assert_eq!(None, ql.insert_answer(bad_a_1.clone()));

        // add in the first outcome, and the query referencing it should be allowed
        ql.insert_outcome(bad_o_1);
        assert_eq!(Some(bad_q_1.id().clone()), ql.insert_query(bad_q_1));

        // and it the whole thing should still be valid as well
        assert!(QueryList::validate(&ql));

        // if we backdoor the other query in, the whole thing should now be invalid
        ql.queries.insert(bad_q_2.id().to_owned(), bad_q_2);
        assert!(!QueryList::validate(&ql));

        // inserting the referenced answer should still fail because its outcome isn't present
        assert_eq!(None, ql.insert_answer(bad_a_1.clone()));

        // if we backdoor the answer in, the whole thing is still invalid
        ql.answers.insert(bad_a_1.id().to_owned(), bad_a_1);
        assert!(!QueryList::validate(&ql));

        //only by inserting the missing outcome does the whole thing validate
        ql.insert_outcome(bad_o_2);
        assert!(QueryList::validate(&ql));

    }

    #[test]
    fn validate_linking() {
        // create minimal querylist
        let mut ql = make_simple_querylist();
        let aid = ql.answers.keys().next().unwrap().clone();
        let qid = ql.queries.keys().next().unwrap().clone();
        let oid = ql.outcomes.keys().next().unwrap().clone();

        // link the answer to the query, make sure nothing else changes
        ql.link_answer_to_query(&aid, &qid);
        assert!(QueryList::validate(&ql));
        assert_eq!(aid.to_owned(), ql.queries.get(&qid).unwrap().answers()[0]);
        assert_eq!(1, ql.queries.len());
        assert_eq!(1, ql.answers.len());
        assert_eq!(1, ql.outcomes.len());
        
        // link the outcome to the answer, make sure nothing else changes
        ql.link_outcome_to_answer(&aid, &qid);
        assert!(QueryList::validate(&ql));
        assert_eq!(oid.to_owned(), ql.answers.get(&aid).unwrap().outcomes()[0]);
        assert_eq!(1, ql.queries.len());
        assert_eq!(1, ql.answers.len());
        assert_eq!(1, ql.outcomes.len());
    }
}
