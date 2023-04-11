

#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use clap_lex;

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




pub fn test() {
    //let args = vec![
    //    "-a", "yes", 
    //    "-a", "no", "-o", "exit", 
    //    "-q", "question 1?", 
    //    "-q", "question 2?", "-a", "yes", "-c", "ls"
    //];

    let raw = clap_lex::RawArgs::from_args();
    let mut cur = raw.cursor();
    let _bin = raw.next_os(&mut cur);

    let mut active_q: Option<Query> = None;
    let mut active_a: Option<Answer> = None;

    while let Some(a) = raw.next(&mut cur) {

        if a.is_short() {

        }
        else if a.is_long() {
        }
        else {
        };

        println!("---\n{:?}", a);  
        println!("is_short {:?}", a.is_short());  
        println!("is_long {:?}", a.is_long());  
        println!("to_long {:?}", a.to_long());  
        println!("to_short {:?}", a.to_short());  
        println!("to_value {:?}", a.to_value());  
        println!("to_value_os {:?}", a.to_value_os());  

    };


}


// focus on cli? :
//
// $ weid -i questions.txt -a "yes" -a "no" -c "ls"
// $ weid -a "yes" -a "no" -o exit -q "question 1?" -q "question 2?" -a "yes" -c "ls"
// $ weid -f "weid.fifo"
// $ weid -f "weid.fifo" -q "question 1?" -a "yes" -a "no"
// 
// questions (and answers?) are indexed by the order they are instantiated
