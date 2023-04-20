

#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::hash_map::HashMap;
use std::cmp::PartialEq;

use clap_lex::{ArgCursor, RawArgs};
use anyhow::{Result, bail};
use log::debug;

use crate::qa::*;
use crate::querylist::*;
use crate::outcome::Outcome;

//struct Query {
#[derive(Clone,Debug)]
struct Cli {
    answers: Vec<String>,

    queries: Vec<String>,

    outcomes: Vec<String>,
}


pub fn get_and_preprocess_args() -> Option<Vec<(String, String)>> {
    let raw = RawArgs::from_args();
    
    _get_and_preprocess_args(raw)
}

// TODO: this needs a refactor
// TODO: should I be bothering with converting to String? (no)
// TODO: this should return a meaningful Result<...>
fn _get_and_preprocess_args(raw: RawArgs) -> Option<Vec<(String, String)>> {
    let mut cur = raw.cursor();

    let mut args = Vec::new();

    while let Some(a) = raw.next(&mut cur) {
        
        //not supporting multi-flag shorts
        if a.is_short() {
            let flag = a.to_short()?.next()?.unwrap().to_string(); 
            let val = raw.next(&mut cur)?.to_value().unwrap().to_string();
            args.push((flag,val));
        }

        else if a.is_long() {
            let (flag_raw, val_raw) = a.to_long()?;
            let flag = flag_raw.unwrap().to_string();
            match val_raw {
                Some(val) => {
                    args.push((flag, val.to_str()?.to_string()));
                },
                None => {
                    let val = raw.next(&mut cur)?.to_value().unwrap().to_string();
                    args.push((flag, val));
                },
            }
        }
    }

    Some(args)
}

fn _to_querylist<'a>(args: Vec<(String, String)>) -> Result<QueryList<'a>> {

    let mut ql = QueryList::new();

    let mut active_q: Option<Query> = None;
    let mut active_a: Option<Answer> = None;
    let mut defaults: Vec<Answer> = Vec::new();

    let mut args_iter = args.iter();

    while let Some((flag, val)) = args_iter.next() {
        if ["q".to_string(), "query".to_string()].contains(flag) {
            if let Some(a) = active_a.take() {
                if let Some(q) = &mut active_q {
                    q.add_answer(a);
                }
                else {
                    defaults.push(a);
                };
            };
            if let Some(q) = active_q.take() {
                ql.insert_query(q);
            };
            let mut query = Query::from_text(val.to_string());
            query.add_answers(defaults.clone());
            debug!("{:?}",&query);

            active_a = None;
            active_q = Some(query);
        }
        else if ["a".to_string(), "answer".to_string()].contains(flag) {
            let mut answer = Answer::from_text(val.to_string());

            if let Some(a) = active_a.take() {
                if let Some(q) = &mut active_q {
                    q.add_answer(a);
                }
                else {
                    defaults.push(a);
                };
            };
            active_a = Some(answer);

        }
        else if ["o".to_string(), "outcome".to_string()].contains(flag) {
            let outcome = Outcome::Command(val.to_owned());
            if let Some(ans) = &mut active_a {
                ans.add_outcome(outcome.clone());
            }
            else {
                //bail!("Malformed arguments: Outcome has no Answer");
            };
        };
    };

    if let Some(a) = active_a.take() {
        if let Some(q) = &mut active_q {
            q.add_answer(a);
        }
    };

    if let Some(q) = active_q.take() {
        ql.insert_query(q);
    };

    Ok(ql)
}

pub fn get_arg_queries<'a>() -> Result<QueryList<'a>> {
    //let args = vec![
    //    "-a", "yes", 
    //    "-a", "no", "-o", "exit", 
    //    "-q", "question 1?", 
    //    "-q", "question 2?", "-a", "yes", "-c", "ls"
    //];

    let mut active_q: Option<Query> = None;
    let mut active_a: Option<Answer> = None;

    let args = get_and_preprocess_args().unwrap();

    _to_querylist(args)
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn preprocessing_args() {
        let args = "-a a1 --query q1 --answer=a2 -o ls -q q2".split(" ");
        let correct: Vec<(String, String)> = Vec::from([
            ("a", "a1"),
            ("query", "q1"),
            ("answer", "a2"),
            ("o", "ls"),
            ("q", "q2"),
        ]).iter().map(|(s,t)| (s.to_string(), t.to_string())).collect();


        let raw = RawArgs::new(args);
        let processed = _get_and_preprocess_args(raw).unwrap();

        assert_eq!(processed, correct);
    }

    #[test]
    fn args_to_querylist() {
        let args_raw = "-a a1 --query q1 --answer=a2 -o ls -q q2".split(" ");

        let correct = {
            let a1 = Answer::from_text("a1".to_string());
            let mut a2 = Answer::from_text("a2".to_string());
            a2.add_outcome(Outcome::Command("ls".to_string()));

            let mut q1 = Query::from_text("q1".to_string());
            q1.add_answers(Vec::from([a1.clone(), a2.clone()]));
            let mut q2 = Query::from_text("q2".to_string());
            q2.add_answer(a1.clone());

            HashMap::from([(0, q1), (1, q2)])
        };

        let args = _get_and_preprocess_args(RawArgs::new(args_raw)).unwrap();

        let out = _to_querylist(args).unwrap();
        assert_eq!(&correct, out.peek_queries());
    }
}

// focus on cli? :
//
// $ weid -i questions.txt -a "yes" -a "no" -c "ls"
// $ weid -a "yes" -a "no" -o exit -q "question 1?" -q "question 2?" -a "yes" -c "ls"
// $ weid -f "weid.fifo"
// $ weid -f "weid.fifo" -q "question 1?" -a "yes" -a "no"
// 
// questions (and answers?) are indexed by the order they are instantiated
