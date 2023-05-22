mod ast;
mod imp;
use crate::generics::ast::*;
use crate::generics::imp::*;
use std::collections::HashMap;
fn run_exp(e: Box<dyn Exp>) {
    let mut s = HashMap::<String, Val>::new();
    let mut t = HashMap::<String, Type>::new();
    println!("*******");
    println!("{}", e.pretty());
    println!("{}", e.eval(&mut s));
    println!("{}", e.infer(&mut t));
}
fn run_stmt(stmt: Box<dyn Stmt>) {
    let mut s = HashMap::<String, Val>::new();
    let mut t = HashMap::<String, Type>::new();
    println!("*******");
    println!("{}", stmt.pretty());
    stmt.eval(&mut s);
    println!("state: {:?}", s);
    println!("type checker: {:?}", stmt.check(&mut t))
}
pub fn run() {
    let ast = number(5);
    run_exp(ast);
    let ast = plus(mult(number(1), number(2)), number(0));

    run_exp(ast);
    let ast = and(boolean(false), number(0));
    run_exp(ast);

    let ast = or(boolean(false), number(0));
    run_exp(ast);
    let ast = less(number(0), number(1));
    run_exp(ast);
    let ast_stmt = seq(
        decl("x".to_string(), number(1)),
        decl("y".to_string(), plus(number(6), variable("x".to_string()))),
    );
    run_stmt(ast_stmt);
}
