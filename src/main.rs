

//#![allow(unused_imports)]

//use atty;
//use clap::Parser;
use std::env;
use std::fmt::{Debug};

mod pbin;

#[allow(dead_code)]
#[derive(Debug)]
struct Answer {
    id: u32,
    //outcomes: Vec<Outcome>,
    display: Option<String>,
}

fn main() {
    let auth: String = env::var("PINBOARD_API_TOKEN").unwrap();
    let p = pbin::PinboardClient::new(auth);
    println!("{:?}", p.get_all_posts(vec!(("tag".to_string(), "collapse".to_string()))));
}




// struct Question {
//     id: u32,
//     text: String,
//     answers: Vec<Answer>,
// 
// }
// 
// trait Query {
//     fn ask(question: Question) -> Vec<Answer>;
// }


// struct Outcome {
//     handler: Handler, //?
// 
// }
// 
// trait Handler {
//     fn deal_with() -> ();
// }

//in the future, use subprocesses to make it so piping and keyboard can both work
//
//fn get_piped() -> Option<Vec<String>> {
//    if atty::isnt(atty::Stream::Stdin) {
//        let lines = io::stdin().lines();
//        lines.collect::<Result<Vec<String>, io::Error>>().ok()
//    }
//    else {
//        None
//    }
//}
//
//fn questions_from_file<T: io::BufRead>(fhandle: T) -> Option<Vec<String>> {
//    let lines = fhandle.lines();
//    lines.collect::<Result<Vec<String>, io::Error>>().ok()
//}

