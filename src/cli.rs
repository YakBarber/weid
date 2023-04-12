

#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use std::collections::hash_map::HashMap;

use clap_lex::{ArgCursor, RawArgs};

use crate::outcome::Outcome;

struct Query {
    display: String,
}
impl Query {
    pub fn from_text(display: &str) -> Query {
        Query {
            display: display.to_string(),
        }
    }
}

struct Answer {
    display: String,
}
impl Answer {
    pub fn from_text(display: &str) -> Answer {
        Answer {
            display: display.to_string(),
        }
    }
}

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

pub fn test() {
    //let args = vec![
    //    "-a", "yes", 
    //    "-a", "no", "-o", "exit", 
    //    "-q", "question 1?", 
    //    "-q", "question 2?", "-a", "yes", "-c", "ls"
    //];

    let mut active_q: Option<Query> = None;
    let mut active_a: Option<Answer> = None;

    let args = get_and_preprocess_args().unwrap();
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
}

// focus on cli? :
//
// $ weid -i questions.txt -a "yes" -a "no" -c "ls"
// $ weid -a "yes" -a "no" -o exit -q "question 1?" -q "question 2?" -a "yes" -c "ls"
// $ weid -f "weid.fifo"
// $ weid -f "weid.fifo" -q "question 1?" -a "yes" -a "no"
// 
// questions (and answers?) are indexed by the order they are instantiated
