extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

mod pest_parser;

use crate::pest_parser::*;
use std::collections::HashMap;

fn main() {
    let mut priority = HashMap::<&str, (i32, bool)>::new();
    priority.insert("<", (5, true));
    priority.insert("<=", (5, true));
    priority.insert("==", (5, true));
    priority.insert(">=", (5, true));
    priority.insert(">", (5, true));
    priority.insert("!=", (5, true));
    priority.insert("+", (10, true));
    priority.insert("-", (10, true));
    priority.insert("*", (20, true));
    priority.insert("/", (20, true));
    let expr= walk_atom(XaderParser::parse(Rule::atom, "(3 + 4) * (5 / - 4) + 3 * 7").unwrap().collect::<RuleList>()[0].clone().into_inner().collect::<_>(), &priority);
    println!("{:?}", expr);
}
