#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::fmt::Debug;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::marker::PhantomData;

use super::util::Result;
use super::outcome::{Outcome, OutcomeResult};

type AnswerId = u16;
type QueryId = u16;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Answer {
    pub id: AnswerId,
    pub outcomes: Vec<Box<dyn Outcome>>,
    pub display: String,
    pub output: Option<String>,
}

impl Answer {

    pub fn execute_outcomes(&self) -> Vec<OutcomeResult> {
        let mut output = Vec::new();

        for outcome in (&self.outcomes).iter() {
            let out = outcome.handler(&self.display[..]);
            output.push(out);
        };
        output
    }
}

#[derive(Debug)]
pub struct Query {
    pub id: QueryId,
    pub text: String,
    pub answers: Vec<Answer>,
}

impl PartialEq for Query {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub fn answers_to_asks(answers: &Vec<Answer>) -> HashMap<String, &Answer> {
    let mut out = HashMap::new();
    for a in answers.iter() {
        out.insert(a.id.to_string(), a);
    };
    out
}

pub fn make_answer(id: AnswerId, display: String) -> Answer{
    let ans = Answer { 
        id: id,
        display: display,
        output: None,
        outcomes: Vec::new(),
    };
    ans
}

#[derive(Debug)]
pub struct QueryList {
    position: usize,
    queries: HashMap<QueryId, Query>,
    key_order: VecDeque<QueryId>,
}

impl QueryList {
    pub fn new() -> QueryList {
        QueryList {
            position: 0,
            queries: HashMap::new(),
            key_order: VecDeque::new(),
        }
    }

    pub fn append(&mut self, query: Query) {
        let id = query.id;
        self.queries.insert(id, query); //don't allow overwrite?
        self.key_order.push_back(id);
    }

    pub fn jump(&mut self, query_id: QueryId) -> Result<usize> {

        match self.index(&query_id) {
            Ok(index) => {
                self.position = index;
                Ok(index)
            },
            Err(_) => Err("Requested Query is not available.".into()),
        }
    }

    // no I will NOT `impl std::ops::Index`.
    fn index(&self, query_id: &QueryId) -> Result<usize> {
        match self.key_order.binary_search(&query_id) {
            Ok(index) => {
                Ok(index)
            },
            Err(e) => Err("Out of bounds.".into()),
        }
    }

    pub fn get(&self, id: &QueryId) -> Option<&Query> {
        self.queries.get(id)
    }
}

impl Iterator for QueryList {
    type Item = Query;

    fn next(&mut self) -> Option<Self::Item> {
        let out_id = self.key_order.get(self.position)?;
        let out = self.queries.remove(&(*out_id as QueryId))?;
        self.position = self.position + 1;
        Some(out)
    }
}

impl FromIterator<Query> for QueryList {
    fn from_iter<I: IntoIterator<Item=Query>>(iter: I) -> Self {
        let mut out = QueryList::new();
        for item in iter {
            out.append(item);
        };
        out
    }
}

#[cfg(test)]
mod test {

    use super::*;

    fn make_query(id: QueryId, num_answers: usize) -> Query {
        Query {
            id: id,
            text: format!("q{} text", id),
            answers: {
                let mut v = Vec::new();
                for i in 0..num_answers {
                    v.push(make_answer(i as u16,format!("q{}a{}",id,i)));
                };
                v
            },
        }
    }

    #[test]
    fn querylist_append(){
        let mut qlist = QueryList::new();
        let q1 = make_query(1, 2);
        qlist.append(q1);

        assert_eq!(qlist.position, 0);
        assert_eq!(qlist.queries.len(), 1);
        assert_eq!(qlist.key_order, vec![1]);
    }


    #[test]
    fn querylist_jump(){
        let mut qlist = QueryList::new();
        let q1 = make_query(1, 2);
        let q2 = make_query(3, 2); //non-sequential id
        qlist.append(q1);

        let j0 = &qlist.jump(1);
        assert_eq!(qlist.position, 0);
        assert_eq!(qlist.key_order.len(), 1);
        assert!(j0.is_ok());

        let j2_fail = &qlist.jump(2);
        assert_eq!(qlist.position, 0);
        assert!(j2_fail.is_err());
        
        let j3_pass = &qlist.jump(1);
        assert_eq!(qlist.position, 0);
        assert_eq!(qlist.key_order.len(), 1);
        assert_eq!(qlist.queries.len(), 1);
        assert!(j3_pass.is_ok());

        qlist.append(q2);
        let j4_pass = &qlist.jump(3);
        assert_eq!(qlist.position, 1);
        assert_eq!(qlist.key_order.len(), 2);
        assert_eq!(qlist.queries.len(), 2);
        assert!(j4_pass.is_ok());

    }
    
    #[test]
    fn querylist_get(){
        let mut qlist = QueryList::new();
        let q1 = make_query(1, 2);
        let q2 = make_query(2, 2);
        let q3 = make_query(4, 2);
        qlist.append(q1);
        qlist.append(q2);
        qlist.append(q3);

        let g4 = qlist.get(&4);
        assert_eq!(g4, Some(&make_query(4,2)));

        let g1 = qlist.get(&1);
        assert_eq!(g1, Some(&make_query(1,3))); // diff num of answers

        let g0_fail = qlist.get(&0);
        assert_eq!(g0_fail, None);
    }
    
    #[test]
    fn querylist_iterate() {
        let mut qlist = QueryList::new();
        let q1 = make_query(1, 2);
        let q2 = make_query(2, 2);
        let q3 = make_query(3, 2);
        qlist.append(q1);
        qlist.append(q2);
        qlist.append(q3);

        let mut outs = Vec::new();

        for o in qlist {
            outs.push(o.id);
        };

        assert_eq!(outs, vec![1,2,3]);

    }

    #[test]
    fn querylist_from_iter() {
        let mut qvec = Vec::new();
        let q1 = make_query(1, 2);
        let q2 = make_query(2, 2);
        qvec.push(q1);
        qvec.push(q2);

        let qlist = QueryList::from_iter(qvec);

        assert_eq!(qlist.queries.len(), 2);
        assert_eq!(qlist.key_order.len(), 2);
        assert_eq!(qlist.key_order, VecDeque::from([1,2]));
    }
    

    //#[test]
    //fn querylist_insert(){
    //    let mut qlist = QueryList::new();
    //    let q1 = make_query(1, 2);
    //    let q2 = make_query(3, 2);
    //    let q3 = make_query(3, 2);
    //    let q4 = make_query(3, 2);

    //    qlist.insert(0, q1);
    //    assert_eq!(qlist.position, 0);
    //    assert_eq!(qlist.key_order.len(), 1);
    //    assert_eq!(qlist.queries.len(), 1);

    //    qlist.insert(0, q2);
    //    assert_eq!(qlist.position, 0);
    //    assert_eq!(qlist.queries.len(), 2);

    //    qlist.insert(2, q3);
    //    assert_eq!(qlist.position, 0);
    //    assert_eq!(qlist.queries.len(), 3);

    //    qlist.insert(4, q4);
    //    assert_eq!(qlist.position, 0);
    //}


    //#[test]
    //fn querylist_iterate_update(){
    //}

}
