extern crate pest;
#[macro_use]
extern crate pest_derive;

//use self::AstNode::*;
use pest::error::Error;
use pest::Parser;

#[derive(Parser)]
#[grammar = "../ik.pest"]
pub struct IKParser;


#[derive(PartialEq, Debug, Clone)]
pub enum AstNode {
    Boolean(bool),
    Int(i32),
    Float(f32),
    Double(f64),
    Ident(String),
    String(String),

    FunctionCall {
        caller: String,
        ident: String,
        expr_list: String,
        ret: String,
    },
    GlobalAssign {
        ident: String,
        expr: Box<AstNode>,
    },
    TopLevelDecl {
        modifier: String,
        typ: String,
        ident: String,
        expr: Box<AstNode>,
    },
    Infix {
        a: Box<AstNode>,
        op: String,
        b: Box<AstNode>,
    },
    IsExpr {
        a: Box<AstNode>,
        b: Box<AstNode>,
    },
    None,
}

pub fn parse(source: &str) -> Result<Vec<AstNode>, Error<Rule>> {
    let mut ast = vec![];

    let pairs = IKParser::parse(Rule::program, source)?;
    for pair in pairs {
        match pair.as_rule() {
            Rule::assgmtExpr => {
                ast.push(build_ast_from_expr(pair));
            }
            Rule::declStmt => {
                ast.push(build_ast_from_expr(pair));
            }
            Rule::functionCall => {
                ast.push(build_ast_from_expr(pair));
            }
            _ => {}
        }
    }
    Ok(ast)
}

// by copied
fn build_ast_from_literal(pair: pest::iterators::Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::integer => {
            let istr = pair.as_str();
            let (sign, istr) = match &istr[..1] {
                "_" => (-1, &istr[1..]),
                _ => (1, &istr[..]),
            };
            let integer: i32 = istr.parse().unwrap();
            AstNode::Int(sign * integer)
        }
        Rule::decimal => {
            let dstr = pair.as_str();
            let last = &dstr[dstr.len() - 1..];
            let (typ, dstr) = match last {
                "f" => (32, &dstr[..dstr.len() - 1]),
                "F" => (32, &dstr[..dstr.len() - 1]),
                _ => (64, &dstr[..])
            };
            match typ {
                32 => AstNode::Float(dstr.parse().unwrap()),
                _ => AstNode::Double(dstr.parse().unwrap())
            }
        }
        Rule::boolean => {
            let dstr = pair.as_str();
            let dstr = &dstr[..];
            let value: bool = dstr.parse().unwrap();
            AstNode::Boolean(value)
        }
        Rule::string => {
            let str = &pair.as_str();
            let str = &str[1..str.len() - 1];
            // Escaped string quotes become single quotes here.
            let str = str.replace("''", "'");
            AstNode::String(String::from(&str[..]))
        }
        unknown_term => panic!("Unexpected term: {:?}", unknown_term),
    }
}


fn build_ast_from_expr(pair: pest::iterators::Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::decimal
        | Rule::integer
        | Rule::infixChar
        | Rule::boolean
        | Rule::string => build_ast_from_literal(pair),
        Rule::ident => AstNode::Ident(String::from(pair.as_str())),

        Rule::declStmt => {
            let mut pair = pair.into_inner();
            let modifier = pair.next().unwrap().as_str();// var val

            let mut ident = pair.next().unwrap().as_str();

            ident = ident.trim_matches('`');
//            let string = ident.chars().collect::<Vec<_>>();
//            let first = string.first().unwrap();
//            let last = string.last().unwrap();
//            if first == &'`' && last == &'`' {
//                ident = ident.trim_matches('`');
//            }

            let expr;
            let mut type_str;
            let third_expr = pair.next().unwrap();// :Type or expr after `=`
            if third_expr.as_rule() == Rule::typeNotation {
                let str = third_expr.as_str();
                let str = &str[1..str.len()];
                type_str = String::from(str);
                expr = build_ast_from_expr(pair.next().unwrap());
            } else {
                expr = build_ast_from_expr(third_expr);
                type_str = get_type_from(&expr);
            }
            AstNode::TopLevelDecl {
                modifier: String::from(modifier),
                typ: type_str,
                ident: String::from(ident),
                expr: Box::new(expr),
            }
        }

        Rule::assgmtExpr => {
            let mut pair = pair.into_inner();
            let ident = pair.next().unwrap();
            let expr = pair.next().unwrap();
            let expr = build_ast_from_expr(expr);
            AstNode::GlobalAssign {
                ident: String::from(ident.as_str()),
                expr: Box::new(expr),
            }
        }
        Rule::functionCall => {
            let mut pair = pair.into_inner();
            let ident = pair.next().unwrap();
            let mut caller = "";
            let id = match ident.as_rule() {
                Rule::functionCaller => {
                    caller = ident.as_str().trim_end_matches(".");
                    pair.next().unwrap().as_str()
                }
                _ => ident.as_str()
            };
            let expr_list = pair.next();
            let list = match expr_list {
                None => "none",
                _ => expr_list.unwrap().as_str()
            };
            AstNode::FunctionCall {
                caller: String::from(caller),
                ident: String::from(id),
                expr_list: String::from(list),
                ret: String::from("Unit"),
            }
        }

        Rule::isExpr => build_ast_is_expr(pair),
        Rule::infixExpr => build_ast_expr(pair),
        Rule::expr => build_ast_expr(pair),
        unknown_expr => panic!("Unexpected expression: {:?}", unknown_expr),
    }
}

fn get_type_from(ast: &AstNode) -> String {
    match ast {
        AstNode::Ident(_)=> String::from("<unknown>"),
        AstNode::IsExpr { a: _, b: _ } => String::from("Boolean"),
        AstNode::Infix { a: _, op: _, b: _ } => String::from("<unknown>"),
        _ => {
            let str = format!("{:?}", ast.to_owned());
            let len = str.len();
            let size = str.find("(");
            let ret = match size {
                None => len,
                _ => size.unwrap()
            };
            String::from(&str[..ret])
        }
    }
}

fn build_ast_is_expr(pair: pest::iterators::Pair<Rule>) -> AstNode {
    let mut inner_par = pair.into_inner();
    let a = inner_par.next().unwrap();
    let a = build_ast_from_expr(a);

    let b = inner_par.next().unwrap();
    let b = build_ast_from_expr(b);

    AstNode::IsExpr { a: Box::new(a), b: Box::new(b) }
}

fn build_ast_expr(pair: pest::iterators::Pair<Rule>) -> AstNode {
    let mut inner_par = pair.into_inner();
    let a = inner_par.next();
    if a == None {
        AstNode::None
    } else {
        let expr = build_ast_from_expr(a.unwrap());
        let op = inner_par.next();
        if op == None {
            expr
        } else {
            let opstr = op.unwrap().as_str();
            let b = inner_par.next().unwrap();
            let expr_b = build_ast_from_expr(b);
            AstNode::Infix {
                a: Box::new(expr),
                op: String::from(opstr),
                b: Box::new(expr_b),
            }
        }
    }
}

fn main() {
    let unparsed_file = std::fs::read_to_string("sample.kt").expect("cannot read ikt file");
    let astnode = parse(&unparsed_file).expect("unsuccessful parse");
    println!("{:#?}", &astnode);
}