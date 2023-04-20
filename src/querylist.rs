
#![allow(dead_code)]
#![allow(unused_imports)]

use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt;
use std::cmp::PartialEq;

use anyhow::Result;
use rand::seq::IteratorRandom;

use super::outcome::Outcome;
use super::qa::*;

pub type QueryId = usize;

#[derive(PartialEq, Clone, Eq, Hash)]
pub struct AnswerId {
    qid: usize,
    sub: usize,
}

impl fmt::Debug for AnswerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "q{}a{}", self.qid, self.sub)
    }
}

#[derive(Debug, Clone)]
pub struct QueryList<'a> {
    queries: HashMap<QueryId, Query<'a>>,
    paths: HashMap<AnswerId,QueryId>,
    next_id: usize,
}


impl<'a> QueryList<'a> {
    pub fn new() -> Self {
        QueryList {
            queries: HashMap::new(),
            paths: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn add_path(&mut self, aid: AnswerId, target: QueryId) {
        self.paths.insert(aid, target);
    }

    pub fn get_path(&self, aid: AnswerId) -> Option<QueryId> {
        self.paths.get(&aid).map(|qid| qid.clone())
    }

    pub fn get_random_query(&self) -> Option<Query<'a>> {
        let mut r = rand::thread_rng();
        match self.queries.keys().choose(&mut r) {
            None => None,
            Some(qid) => self.get_query(*qid),
        }
    }

    pub fn get_query(&self, qid: QueryId) -> Option<Query<'a>> {
        self.queries.get(&qid).map(|q| q.clone())
    }

    pub fn get_next_query(&self, aid: AnswerId) -> Option<Query<'a>> {
        match self.get_path(aid) {
            Some(qid) => self.get_query(qid),
            None => self.get_random_query(),
        }
    }

    pub fn insert_query(&mut self, query: Query<'a>) -> QueryId {
        let out_qid = self.next_id;
        self.queries.insert(out_qid, query);
        self.next_id = self.next_id + 1;
        out_qid
    }

    pub fn peek_queries(&self) -> &HashMap<QueryId, Query<'a>> {
        &self.queries
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn gen_query<'a>(num_answers: i8) -> Query<'a> {
        let mut q = Query::from_text("q0".to_string());
        for i in 0..num_answers {
            let text = format!("a{}", i);
            let ans = Answer::from_text(text);
            q.add_answer(ans.clone());
        }

        q
    }

    #[test]
    fn querylist() {
        let mut ql = QueryList::new();
        let start_id = ql.next_id;

        let q0 = gen_query(2);

        let q0_id = ql.insert_query(q0);

        assert_eq!(q0_id, start_id);
        assert_eq!(ql.next_id, start_id + 1);

        let q1 = gen_query(1);
        let q1_id = ql.insert_query(q1.clone());

        let aid_0 = AnswerId {
            qid: q0_id,
            sub: 0,
        };

        //let aid_1 = AnswerId {
        //    qid: q0_id,
        //    sub: 1,
        //};

        ql.add_path(aid_0.clone(), q1_id);

        let qid = ql.get_path(aid_0).unwrap();
        assert_eq!(qid, q1_id);

        let q_out = ql.get_query(qid).unwrap();
        assert_eq!(q_out, q1);
    }
}


