use crate::imp::*;
pub fn number(x: i32) -> Box<dyn Exp> {
    let y: Num = x;
    return Box::new(y);
}
pub fn boolean(x: bool) -> Box<dyn Exp> {
    let y: Bool = x;
    return Box::new(y);
}
pub fn mult(x: Box<dyn Exp>, y: Box<dyn Exp>) -> Box<dyn Exp> {
    Box::new(Mult { exp: [x, y] })
}
pub fn plus(x: Box<dyn Exp>, y: Box<dyn Exp>) -> Box<dyn Exp> {
    Box::new(Plus { exp: [x, y] })
}

pub fn and(x: Box<dyn Exp>, y: Box<dyn Exp>) -> Box<dyn Exp> {
    Box::new(And { exp: [x, y] })
}
pub fn or(x: Box<dyn Exp>, y: Box<dyn Exp>) -> Box<dyn Exp> {
    Box::new(Or { exp: [x, y] })
}
pub fn less(x: Box<dyn Exp>, y: Box<dyn Exp>) -> Box<dyn Exp> {
    Box::new(Less { exp: [x, y] })
}
pub fn equal(x: Box<dyn Exp>, y: Box<dyn Exp>) -> Box<dyn Exp> {
    Box::new(Equ { exp: [x, y] })
}
pub fn not(exp: Box<dyn Exp>) -> Box<dyn Exp> {
    Box::new(Neg { exp })
}
pub fn group(exp: Box<dyn Exp>) -> Box<dyn Exp> {
    Box::new(Grp { exp })
}
pub fn seq(x: Box<dyn Stmt>, y: Box<dyn Stmt>) -> Box<dyn Stmt> {
    Box::new(Seq { stmts: [x, y] })
}
pub fn decl(lhs: String, rhs: Box<dyn Exp>) -> Box<dyn Stmt> {
    Box::new(Decl { lhs, rhs })
}
pub fn variable(name: String) -> Box<dyn Exp> {
    let x: Var = name;
    Box::new(x)
}
pub fn assign(lhs: Var, rhs: Box<dyn Exp>) -> Box<dyn Stmt> {
    Box::new(Assign { lhs, rhs })
}
pub fn ifthenelse(
    cond: Box<dyn Exp>,
    then_stmt: Box<dyn Stmt>,
    else_stmt: Box<dyn Stmt>,
) -> Box<dyn Stmt> {
    Box::new(IfThenElse {
        cond,
        then_stmt,
        else_stmt,
    })
}
pub fn _while(cond: Box<dyn Exp>, stmt: Box<dyn Stmt>) -> Box<dyn Stmt> {
    Box::new(While { cond, stmt })
}
pub fn print(print_exp: Box<dyn Exp>) -> Box<dyn Stmt> {
    Box::new(Print { print_exp })
}
