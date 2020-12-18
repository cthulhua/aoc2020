#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate pest_derive;

use pest::iterators::{Pair, Pairs};
use pest::prec_climber::{Assoc, Operator, PrecClimber};
use pest::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Parser)]
#[grammar = "math.pest"]
struct MathParser;

lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use Assoc::*;
        use Rule::*;

        PrecClimber::new(vec![
            Operator::new(add, Left)
                | Operator::new(subtract, Left)
                | Operator::new(multiply, Left)
                | Operator::new(divide, Left),
        ])
    };
}
fn main() -> Result<(), Box<dyn Error>> {
    let filename = std::env::args().nth(1).unwrap();
    let input = File::open(filename)?;
    let buffered = BufReader::new(input);
    let v: f64 = buffered
        .lines()
        .map(|r| eval(MathParser::parse(Rule::calculation, &r.unwrap()).unwrap()))
        .sum();
    dbg!(v);
    Ok(())
}

fn eval(expression: Pairs<Rule>) -> f64 {
    PREC_CLIMBER.climb(
        expression,
        |pair: Pair<Rule>| match pair.as_rule() {
            Rule::num => pair.as_str().parse::<f64>().unwrap(),
            Rule::expr => eval(pair.into_inner()),
            _ => unreachable!(),
        },
        |lhs: f64, op: Pair<Rule>, rhs: f64| match op.as_rule() {
            Rule::add => lhs + rhs,
            Rule::subtract => lhs - rhs,
            Rule::multiply => lhs * rhs,
            Rule::divide => lhs / rhs,
            _ => unreachable!(),
        },
    )
}
