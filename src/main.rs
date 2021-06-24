extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fs;

use pest::{error::Error, iterators::Pair, Parser};

#[derive(Parser)]
#[grammar = "ts.pest"]
pub struct TSParser;

#[derive(Debug)]
pub enum Operator {
    NotEqual,
    DoubleEqual,
    TripleEqual,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
    Minus,
    Plus,
    Multiply,
    Divide,
    Invert,
}

impl From<Pair<'_, Rule>> for Operator {
    fn from(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::not_equal => Operator::NotEqual,
            Rule::double_equal => Operator::DoubleEqual,
            Rule::triple_equal => Operator::TripleEqual,
            Rule::greater_than => Operator::GreaterThan,
            Rule::greater_than_equal => Operator::GreaterThanEqual,
            Rule::less_than => Operator::LessThan,
            Rule::less_than_equal => Operator::LessThanEqual,
            Rule::plus => Operator::Plus,
            Rule::minus => Operator::Minus,
            Rule::divide => Operator::Divide,
            Rule::multiply => Operator::Multiply,
            Rule::inverse => Operator::Invert,
            _ => panic!(
                "Attempted to convert a non-operator to an operator. This is a bug, report it."
            ),
        }
    }
}

#[derive(Debug)]
pub enum AstNode {
    String(String),
    Number(f64),
    Bool(bool),
    Binary {
        left: Box<AstNode>,
        right: Box<AstNode>,
        operator: Operator,
    },
    Unary {
        operator: Operator,
        right: Box<AstNode>,
    },
    Grouping(Box<AstNode>),
}

fn main() {
    let unparsed_file = fs::read_to_string("test.ts").expect("Failed to read test.ts");

    println!("{:#?}", parse(&unparsed_file).unwrap());
}

pub fn parse(source: &str) -> Result<Vec<AstNode>, Error<Rule>> {
    let mut ast = Vec::new();

    let program = TSParser::parse(Rule::program, source)?.next().unwrap();
    for pair in program.into_inner() {
        match pair.as_rule() {
            Rule::expression => ast.push(parse_expression(pair)),
            _ => {}
        }
    }

    Ok(ast)
}

pub fn parse_expression(pair: Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::STRING => todo!(),
        Rule::NUMBER => AstNode::Number(pair.as_str().parse().unwrap()),
        Rule::BOOL_TRUE => AstNode::Bool(true),
        Rule::BOOL_FALSE => AstNode::Bool(false),
        Rule::NULL => todo!(),
        Rule::expression => parse_expression(pair.into_inner().next().unwrap()),
        Rule::equality => {
            let mut pair = pair.into_inner();
            let left = parse_expression(pair.next().unwrap());

            if let Some(operator) = pair.next() {
                let right = parse_expression(pair.next().unwrap());

                AstNode::Binary {
                    left: Box::from(left),
                    right: Box::from(right),
                    operator: Operator::from(operator),
                }
            } else {
                // This will run when this is a pass through rule. For example, when parsing
                // 5 > 4
                // this equality will be trigured as a pass through rule
                left
            }
        }
        Rule::comparision => {
            let mut pair = pair.into_inner();
            let left = parse_expression(pair.next().unwrap());

            if let Some(operator) = pair.next() {
                let right = parse_expression(pair.next().unwrap());

                AstNode::Binary {
                    left: Box::from(left),
                    right: Box::from(right),
                    operator: Operator::from(operator),
                }
            } else {
                // This will run when this is a pass through rule. For example, when parsing
                // 5 - 4
                // this comparison will be trigured as a pass through rule
                left
            }
        }
        Rule::term => {
            let mut pair = pair.into_inner();
            let left = parse_expression(pair.next().unwrap());

            if let Some(operator) = pair.next() {
                let right = parse_expression(pair.next().unwrap());

                AstNode::Binary {
                    left: Box::from(left),
                    right: Box::from(right),
                    operator: Operator::from(operator),
                }
            } else {
                // This will run when this is a pass through rule. For example, when parsing
                // 5 * 4
                // this term will be trigured as a pass through rule
                left
            }
        }
        Rule::factor => {
            let mut pair = pair.into_inner();
            let left = parse_expression(pair.next().unwrap());

            if let Some(operator) = pair.next() {
                let right = parse_expression(pair.next().unwrap());

                AstNode::Binary {
                    left: Box::from(left),
                    right: Box::from(right),
                    operator: Operator::from(operator),
                }
            } else {
                // This will run when this is a pass through rule. For example, when parsing
                // -4
                // this factor will be trigured as a pass through rule
                left
            }
        }
        Rule::unary => {
            let mut pair = pair.into_inner();
            let left = pair.next().unwrap();

            if left.as_rule() == Rule::minus || left.as_rule() == Rule::inverse {
                let right = parse_expression(pair.next().unwrap());
                AstNode::Unary {
                    operator: Operator::from(left),
                    right: Box::from(right),
                }
            } else {
                // This will run when this is a pass through rule. For example, when parsing
                // 4
                // this unary will be trigured as a pass through rule
                parse_expression(left)
            }
        }
        Rule::grouping => parse_expression(pair.into_inner().next().unwrap()),
        _ => unimplemented!(),
    }
}
