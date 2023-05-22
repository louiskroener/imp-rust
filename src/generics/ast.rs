use crate::generics::imp::*;
pub fn number(x: i32) -> Box<Num> {
    let y: Num = x;
    return Box::new(y);
}
pub fn boolean(x: bool) -> Box<Bool> {
    let y: Bool = x;
    return Box::new(y);
}
pub fn mult<T1: Exp, T2: Exp>(left: Box<T1>, right: Box<T2>) -> Box<Mult<T1, T2>> {
    Box::new(Mult { left, right })
}
pub fn plus<T1: Exp, T2: Exp>(left: Box<T1>, right: Box<T2>) -> Box<Plus<T1, T2>> {
    Box::new(Plus { left, right })
}
pub fn and<T1: Exp, T2: Exp>(left: Box<T1>, right: Box<T2>) -> Box<And<T1, T2>> {
    Box::new(And { left, right })
}
pub fn or<T1: Exp, T2: Exp>(left: Box<T1>, right: Box<T2>) -> Box<Or<T1, T2>> {
    Box::new(Or { left, right })
}
pub fn less<T1: Exp, T2: Exp>(left: Box<T1>, right: Box<T2>) -> Box<Less<T1, T2>> {
    Box::new(Less { left, right })
}
pub fn equal<T1: Exp, T2: Exp>(left: Box<T1>, right: Box<T2>) -> Box<Equ<T1, T2>> {
    Box::new(Equ { left, right })
}
pub fn not<T: Exp>(exp: Box<T>) -> Box<Neg<T>> {
    Box::new(Neg { exp })
}
pub fn group<T: Exp>(exp: Box<T>) -> Box<Grp<T>> {
    Box::new(Grp { exp })
}
pub fn seq<T1: Stmt, T2: Stmt>(first: Box<T1>, second: Box<T2>) -> Box<Seq<T1, T2>> {
    Box::new(Seq { first, second })
}
pub fn decl<T: Exp>(lhs: String, rhs: Box<T>) -> Box<Decl<T>> {
    Box::new(Decl { lhs, rhs })
}
pub fn variable(name: String) -> Box<Var> {
    let x: Var = name;
    Box::new(x)
}
pub fn assign<T: Exp>(lhs: Var, rhs: Box<T>) -> Box<Assign<T>> {
    Box::new(Assign { lhs, rhs })
}
pub fn ifthenelse<T1: Exp, T2: Stmt, T3: Stmt>(
    cond: Box<T1>,
    then_stmt: Box<T2>,
    else_stmt: Box<T3>,
) -> Box<IfThenElse<T1, T2, T3>> {
    Box::new(IfThenElse {
        cond,
        then_stmt,
        else_stmt,
    })
}
pub fn _while<T1: Exp, T2: Stmt>(cond: Box<T1>, stmt: Box<T2>) -> Box<While<T1, T2>> {
    Box::new(While { cond, stmt })
}
pub fn print<T: Exp>(print_exp: Box<T>) -> Box<Print<T>> {
    Box::new(Print { print_exp })
}
