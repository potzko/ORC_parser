mod tokens;
use pest::iterators::{Pair, Pairs};
use pest::pratt_parser::PrattParser;
use pest::Parser;

use tokens::token_types::*;

#[derive(pest_derive::Parser)]
#[grammar = "orc.pest"]
pub struct CalculatorParser;

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            // Addition and subtract have equal precedence
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left) | Op::infix(modulo, Left))
            .op(Op::prefix(unary_minus))
    };
}

pub fn parse_statement(pairs: Pairs<Rule>) -> Statement {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::statement => parse_statement(primary.into_inner()),
            Rule::let_statement => {
                let primary: Vec<_> = primary.into_inner().collect();
                assert!(primary.len() <= 3);
                assert!(primary.len() >= 1);
                let var_name;
                let var_type;
                let var_value;
                match primary.len() {
                    1 => {
                        var_name = primary[0].as_str();
                        var_value = Expr::Literal(Literal::None());
                        return Statement::LetStatement {
                            name: var_name.to_string(),
                            var_type: Type::Primitive(PrimitiveType::None),
                            value: var_value,
                        };
                    }
                    2 => {
                        var_name = primary[0].as_str();
                        match primary[1].as_rule() {
                            Rule::type_annotation => {
                                var_type = primary[1].clone();
                                var_value = Expr::Literal(Literal::None());
                            }
                            Rule::expr => {
                                var_value = parse_expr(Pairs::single(primary[1].clone()));
                                return Statement::LetStatement {
                                    name: var_name.to_string(),
                                    var_type: Type::Primitive(PrimitiveType::None),
                                    value: var_value,
                                };
                            }
                            _ => unreachable!(),
                        }
                    }
                    3 => {
                        var_name = primary[0].as_str();
                        var_type = primary[1].clone();
                        var_value = parse_expr(Pairs::single(primary[2].clone()));
                    }
                    _ => unreachable!(),
                }
                Statement::LetStatement {
                    name: var_name.to_string(),
                    var_type: parse_type(Pairs::single(var_type)),
                    value: var_value,
                }
            }
            rule => unreachable!("statement parse expected atom, found {:?}", rule),
        })
        .parse(pairs)
}

pub fn parse_expr(pairs: Pairs<Rule>) -> Expr {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::integer_literal => {
                Expr::Literal(Literal::Integer(primary.as_str().to_string()))
            }
            Rule::float_literal => Expr::Literal(Literal::Floating(primary.as_str().to_string())),
            Rule::variable => Expr::Variable(Variable {
                name: primary.as_str().to_string(),
                var_type: Type::Unknown,
            }),
            Rule::expr => parse_expr(primary.into_inner()),
            Rule::exp_block => {
                let mut internal: Vec<Pair<Rule>> = primary.into_inner().collect();
                let last = internal.pop().unwrap();
                let statements: Vec<Statement> = internal
                    .into_iter()
                    .map(|i| parse_statement(Pairs::single(i)))
                    .collect();
                Expr::ExprBlock(statements, Box::new(parse_expr(Pairs::single(last))))
            }
            Rule::bin_exp => parse_expr(primary.into_inner()),
            rule => unreachable!("Expr::parse expected atom, found {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| {
            let op = match op.as_rule() {
                Rule::add => BinaryOp::Add,
                Rule::subtract => BinaryOp::Subtract,
                Rule::multiply => BinaryOp::Multiply,
                Rule::divide => BinaryOp::Divide,
                Rule::modulo => BinaryOp::Modulo,
                rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
            };
            Expr::BinOp {
                left: Box::new(lhs),
                op,
                right: Box::new(rhs),
            }
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::unary_minus => Expr::UnaryMinus(Box::new(rhs)),
            _ => unreachable!(),
        })
        .parse(pairs)
}

pub fn parse_type(pairs: Pairs<Rule>) -> Type {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::r#type => {
                let type_name = primary.clone().into_inner().next().unwrap().as_str();
                match type_name {
                    "integer" => Type::Primitive(PrimitiveType::Integer),
                    "floating" => Type::Primitive(PrimitiveType::Floating),
                    "boolean" => Type::Primitive(PrimitiveType::Boolean),
                    "none" => Type::Primitive(PrimitiveType::None),
                    _ => {
                        let mut opts = primary.into_inner();
                        let base = opts.next().unwrap();
                        let base = Type::BaseType(base.as_str().to_string());
                        let type_block = opts.next();
                        match type_block {
                            Some(block) => {
                                let subs = block.into_inner();
                                let subs: Vec<Type> = subs.map(|i| parse_type(Pairs::single(i))).collect();
                                Type::CompoundType(Box::new(CompoundType{base, subtypes: subs}))
                            }
                            None => base
                        }
                    }
                }
            }
            Rule::variable => Type::BaseType(primary.as_str().to_string()),
            Rule::type_annotation => parse_type(primary.into_inner()),
            rule => unreachable!("Expr::parse expected atom, found {:?}", rule),
        })
        .parse(pairs)
}
fn main() {
    let line = "{let a: a<b, c<integer>> = 2; 1 + 1 + a / 2.2}";
    let ret;
    match CalculatorParser::parse(Rule::main, &line) {
        Ok(mut pairs) => {
            ret = parse_expr(pairs.next().unwrap().into_inner());
            println!(
                "Parsed: {:#?}",
                // inner of expr
                ret
            );
        }
        Err(e) => {
            println!("Parse failed: {:?}", e);
        }
    }
}
