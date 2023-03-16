#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::fmt::Debug;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::marker::PhantomData;

use nanoid::nanoid;

use super::util::Result;
use super::outcome::*;

pub type AnswerId = String;
pub type QueryId = String;

#[derive(Clone, Hash, PartialEq)]
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

    pub fn next_query(&mut self, qid: &QueryId) {
        self.next_query = Some(qid.clone());
    }

    pub fn add_outcome(&mut self, oid: &OutcomeId) {
        self.outcomes.push(oid.clone());
    }
}

#[derive(Clone)]
pub enum QuerySeed {
    Text(String),
    FromOutcome(OutcomeId),
}

#[derive(Clone)]
pub struct Query { 
    _id: QueryId,
    seed: QuerySeed,
    answers: Option<Vec<AnswerId>>,
}

impl Query {
    pub fn from_text(display: &str) -> Query {
        Query {
            seed: QuerySeed::Text(display.to_owned()),
            _id: nanoid!(),
            answers: None,
        }
    }

    pub fn from_outcome(outcome: OutcomeId) -> Query {
        Query {
            seed: QuerySeed::FromOutcome(outcome),
            _id: nanoid!(),
            answers: None,
        }
    }

    pub fn id(&self) -> &QueryId {
        &self._id
    }

    pub fn add_answer(&mut self, ans: &AnswerId) {
        match &mut self.answers {
            None => self.answers = Some(vec![ans.clone()]),
            Some(vec) => vec.push(ans.clone()),
        };
    }
}

pub fn answers_to_asks(answers: &Vec<Answer>) -> HashMap<String, &Answer> {
    let mut out = HashMap::new();
    for a in answers.iter() {
        out.insert(a.id.to_string(), a);
    };
    out
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
