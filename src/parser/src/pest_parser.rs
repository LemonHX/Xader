use pest::Parser;
use pest::iterators::Pair;
use crate::pest_parser::BaseExpr::{ExprNope, BinaryOp, Ident, UnaryOp};
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "../xader.pest"]
pub struct XaderParser;

pub(crate) type RuleList<'a> = Vec<Pair<'a, Rule>>;

// AST Definition
#[derive(Debug, Clone)]
pub enum BaseExpr {
    UnaryOp(String, Box<BaseExpr>),
    BinaryOp(String, Box<BaseExpr>, Box<BaseExpr>),
    TypeIdent(String),
    Ident(String),
    ConstantBool(bool),
    ConstantInt{raw: String, unsigned: bool},
    ConstantFloat(String),
    ExprNope
}

fn walk_literal(body: RuleList) {

}

fn walk_primary_value(body: RuleList) -> BaseExpr {
    match body[0].as_rule() {
        _ => BaseExpr::Ident(body[0].as_str().to_string()),
    }
}

fn walk_primary(body: RuleList, priority: &HashMap<&str, (i32, bool)>) -> BaseExpr {
    let primary = body[0].clone();
    println!("{:?}", body);
    match primary.as_rule() {
        Rule::primary_value => {
            walk_primary_value(primary.into_inner().collect())
        }
        Rule::prefix_expr => {
            let prefix_expr = primary.into_inner().collect::<RuleList>();
            UnaryOp(prefix_expr[0].as_str().to_string(), Box::new(walk_atom(prefix_expr[1].clone().into_inner().collect(), priority)))
        }
        _ => {
            ExprNope
        }
    }
}

fn walk_value_node(body: Pair<'_, Rule>, priority: &HashMap<&str, (i32, bool)>) -> BaseExpr {
    match body.as_rule() {
        Rule::atom => walk_atom(body.into_inner().collect(), priority),
        Rule::primary_raw => walk_primary(body.into_inner().collect(), priority),
        _ => ExprNope
    }
}

fn walk_prec_climber<'a>(composition: RuleList<'a>, prec_val : i32, priority: &HashMap<&str, (i32, bool)>) -> (BaseExpr, Option<RuleList<'a>>) {
    let lhs = walk_value_node(composition[0].clone(), priority);
    if composition.len() <= 1 {
        return (lhs, None);
    }
    let mut last_op = composition[1].clone();
    let mut op = last_op.as_str().to_string();
    let mut prior = priority.get(&op[..]).expect("Operator not found !");
    let mut result = lhs;
    let mut remnants: RuleList = composition[2..].to_vec();
    while prior.0 >= prec_val {
        //println!("{} {} {:?}", prec_val, op, prior);
        let next_prior = if prior.1 { prior.0 + 1 } else { prior.0 };
        let rhs = walk_prec_climber(remnants.clone(), next_prior, priority);
        let suc = rhs.1;
        result = BinaryOp(op, Box::new(result), Box::new(rhs.0));
        match suc {
            None => {
                return (result, None);
            }
            Some(expr) => {
                remnants = expr;
                last_op = remnants[0].clone();
                op = last_op.as_str().to_string();
                prior = priority.get(&op[..]).expect("Operator not found !");
                remnants = remnants[1..].to_vec();
            }
        }
    }
    if remnants.len() > 0 {
        let mut ret_vec = vec![last_op];
        ret_vec.extend(remnants.into_iter());
        (result, Some(ret_vec))
    } else {
        (result, None)
    }
}

// atom tokens passes through here
pub fn walk_atom(body: RuleList, priority: &HashMap<&str, (i32, bool)>) -> BaseExpr {
    if body.len() > 0 {
        return walk_prec_climber(body, 0, priority).0;
    }
    ExprNope
}
