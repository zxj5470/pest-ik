extern crate pest;
#[macro_use]
extern crate pest_derive;

//use self::AstNode::*;
use pest::error::Error;
use pest::Parser;
use std::ffi::CString;

#[derive(Parser)]
#[grammar = "ik.pest"]
pub struct IKParser;


#[derive(PartialEq, Debug, Clone)]
pub enum AstNode {
    Print(Box<AstNode>),
    Int(i32),

    Float(f32),
    Double(f64),
    GlobalAssign {
        ident: String,
        expr: Box<AstNode>,
    },
    GlobalDecl {
        modifier: String,
        type_str: String,
        ident: String,
        expr: Box<AstNode>,
    },
    Ident(String),
    Str(CString),
}

pub fn parse(source: &str) -> Result<Vec<AstNode>, Error<Rule>> {
    let mut ast = vec![];

    let pairs = IKParser::parse(Rule::program, source)?;
    for pair in pairs {
        match pair.as_rule() {
            Rule::expr => {
                ast.push(build_ast_from_expr(pair));
            }
            Rule::declStmt => {
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
        unknown_term => panic!("Unexpected term: {:?}", unknown_term),
    }
}


fn build_ast_from_expr(pair: pest::iterators::Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::expr => build_ast_from_expr(pair.into_inner().next().unwrap()),
        Rule::ident => AstNode::Ident(String::from(pair.as_str())),
        Rule::decimal => build_ast_from_literal(pair),
        Rule::declStmt => {
            let mut pair = pair.into_inner();
            let modifier = pair.next().unwrap().as_str();// var val
            let mut ident = pair.next().unwrap().as_str();
            let third_expr = pair.next().unwrap();// :Type or expr after `=`
            let mut type_str = "";
            let mut expr;

            let str = ident;
            if &str[0..1] == "`" && &str[str.len() - 1..str.len()] == "`" {
                ident = &str[1..str.len() - 1]
            }

            if third_expr.as_rule() == Rule::typeNotation {
                type_str = third_expr.as_str();
                type_str = &type_str[1..type_str.len()];
                expr = build_ast_from_expr(pair.next().unwrap());
            } else {
                expr = build_ast_from_expr(third_expr);
            }

            AstNode::GlobalDecl {
                modifier: String::from(modifier),
                type_str: String::from(type_str),
                ident: String::from(ident),
                expr: Box::new(expr),
            }
        }
        Rule::integer => build_ast_from_literal(pair),
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
        Rule::string => {
            let str = &pair.as_str();
            // Strip leading and ending quotes.
            let str = &str[1..str.len() - 1];
            // Escaped string quotes become single quotes here.
            let str = str.replace("''", "'");
            AstNode::Str(CString::new(&str[..]).unwrap())
        }
        unknown_expr => panic!("Unexpected expression: {:?}", unknown_expr),
    }
}

fn main() {
    let unparsed_file = std::fs::read_to_string("sample.kt").expect("cannot read ikt file");
    let astnode = parse(&unparsed_file).expect("unsuccessful parse");
    println!("{:#?}", &astnode);
}