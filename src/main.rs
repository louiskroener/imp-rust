use std::collections::HashMap;

//TODO remove matches! with if let and match
//TODO add error handling for Option types e.g if variable exisits in Valstate
//TODO Implement fmt::Display for Val and Type and remove show_val and show_type
//TODO Write better pretty methods
#[derive(PartialEq, Debug)]
enum Kind {
    ValueInt,
    ValueBool,
    Undefined,
}
#[derive(Debug)]
struct Val {
    flag: Kind,
    val_i: Option<i32>,
    val_b: Option<bool>,
}
impl Val {
    fn mk_int(x: &i32) -> Val {
        Val {
            flag: Kind::ValueInt,
            val_i: Some(*x), // does it need to be cloned?
            val_b: None,
        }
    }
    fn mk_bool(x: &bool) -> Val {
        Val {
            flag: Kind::ValueBool,
            val_i: None,
            val_b: Some(*x),
        }
    }
    fn mk_undefined() -> Val {
        Val {
            flag: Kind::Undefined,
            val_i: None,
            val_b: None,
        }
    }
}
fn show_val(v: Val) -> String {
    match v.flag {
        Kind::ValueInt => {
            let x: Num = v.val_i.unwrap();
            return x.pretty();
        }
        Kind::ValueBool => {
            let x: Bool = v.val_b.unwrap();
            return x.pretty();
        }
        Kind::Undefined => return "undefined".to_string(),
    }
}
#[derive(Clone, Copy, PartialEq)]
enum Type {
    TyIllTyped,
    TyInt,
    TyBool,
}

// TODO implement as fmt:Display
fn show_type(t: Type) -> String {
    match t {
        Type::TyInt => return "int".to_string(),
        Type::TyBool => return "bool".to_string(),
        Type::TyIllTyped => return "Illtyped".to_string(),
    }
}
type ValState = HashMap<String, Val>;
type TyState = HashMap<String, Type>;

trait Exp {
    fn pretty(&self) -> String;
    fn eval(&self, s: &mut ValState) -> Val;
    fn infer(&self, t: &mut TyState) -> Type;
}

trait Stmt {
    fn pretty(&self) -> String;
    fn eval(&self, s: &mut ValState);
    fn check(&self, t: &mut TyState) -> bool;
}

struct Seq {
    stmts: [Box<dyn Stmt>; 2],
}

impl Stmt for Seq {
    fn pretty(&self) -> String {
        let x = self.stmts[0].pretty() + "; " + &self.stmts[1].pretty();
        x
    }
    fn eval(&self, s: &mut ValState) {
        self.stmts[0].eval(s);
        self.stmts[1].eval(s);
    }
    fn check(&self, t: &mut TyState) -> bool {
        if !self.stmts[0].check(t) {
            return false;
        }
        self.stmts[1].check(t)
    }
}

struct Decl {
    lhs: String,
    rhs: Box<dyn Exp>,
}

impl Stmt for Decl {
    fn pretty(&self) -> String {
        let mut x = String::new();
        x.push_str(&self.lhs);
        x.push_str(":= ");
        x.push_str(&self.rhs.pretty());
        x
    }
    fn eval(&self, s: &mut ValState) {
        let x = self.rhs.eval(s);
        s.insert(self.lhs.clone(), x);
    }
    fn check(&self, t: &mut TyState) -> bool {
        let ty = self.rhs.infer(t);
        if let Type::TyIllTyped = ty {
            return false;
        }
        t.insert(self.lhs.clone(), ty);
        return true;
    }
}
struct IfThenElse {
    cond: Box<dyn Exp>,
    then_stmt: Box<dyn Stmt>,
    else_stmt: Box<dyn Stmt>,
}
impl Stmt for IfThenElse {
    fn pretty(&self) -> String {
        let mut x = String::new();
        x.push_str("if ");
        x.push_str(&self.cond.pretty());
        x.push_str(" then ");
        x.push_str(&mut self.then_stmt.pretty());
        x.push_str(" else ");
        x.push_str(&mut self.else_stmt.pretty());
        x
    }
    fn eval(&self, s: &mut ValState) {
        let v = self.cond.eval(s);
        if let Kind::ValueBool = v.flag {
            if v.val_b.unwrap() {
                self.then_stmt.eval(s);
            } else {
                self.else_stmt.eval(s);
            }
        } else {
            println!("Error Parsing IfThenElse");
        }
    }
    fn check(&self, t: &mut TyState) -> bool {
        let ty = self.cond.infer(t);
        if let Type::TyIllTyped = ty {
            return false;
        }
        self.then_stmt.check(t) && self.else_stmt.check(t)
    }
}
struct Assign {
    lhs: Var,
    rhs: Box<dyn Exp>,
}
impl Stmt for Assign {
    fn pretty(&self) -> String {
        let mut x = String::new();
        x.push_str(&self.lhs.pretty());
        x.push_str(" = ");
        x.push_str(&self.rhs.pretty());
        x
    }
    fn eval(&self, s: &mut ValState) {
        let v = self.lhs.eval(s);
        let val = self.rhs.eval(s);
        if let Kind::Undefined = val.flag {
            println!("value is undefined");
            return;
        }
        if let Kind::Undefined = v.flag {
            println!("var is undefined");
            return;
        }
        if v.flag == val.flag {
            println!("var and value not the same Kind");
            return;
        }
        *s.get_mut(&self.lhs).unwrap() = val;
    }
    fn check(&self, t: &mut TyState) -> bool {
        let x = self.lhs.pretty();
        let y = self.rhs.infer(t);
        let w = t.get(&x).unwrap();
        *w == y
    }
}
struct While {
    cond: Box<dyn Exp>,
    stmt: Box<dyn Stmt>,
}
impl Stmt for While {
    fn pretty(&self) -> String {
        let mut x = String::new();
        x.push_str("while ");
        x.push_str(&self.cond.pretty());
        x.push_str(" ");
        x.push_str(&self.stmt.pretty());
        x
    }
    fn eval(&self, s: &mut ValState) {
        let mut cond = self.cond.eval(s);
        if let Kind::ValueBool = cond.flag {
            while cond.val_b.unwrap() {
                self.stmt.eval(s);
                cond = self.cond.eval(s);
            }
        } else {
            println!("cond is no bool");
        }
    }
    fn check(&self, t: &mut TyState) -> bool {
        let ty = self.cond.infer(t);
        if let Type::TyIllTyped = ty {
            return false;
        }
        self.stmt.check(t)
    }
}
struct Print {
    print_exp: Box<dyn Exp>,
}
impl Stmt for Print {
    fn pretty(&self) -> String {
        let mut x = String::new();
        x.push_str("print ");
        x.push_str(&self.print_exp.pretty());
        x
    }
    fn eval(&self, s: &mut ValState) {
        let v = self.print_exp.eval(s);
        match v.flag {
            Kind::ValueBool => println!("Output {}", v.val_b.unwrap()),
            Kind::ValueInt => println!("Ouput {}", v.val_i.unwrap()),
            _ => println!("Output Undefined"),
        }
    }
    fn check(&self, t: &mut TyState) -> bool {
        let ty = self.print_exp.infer(t);
        if let Type::TyIllTyped = ty {
            return false;
        }
        true
    }
}
struct Block {
    stmt: Box<dyn Stmt>,
}

// Exp
type Var = String;
impl Exp for Var {
    fn pretty(&self) -> String {
        self.to_string()
    }
    fn eval(&self, s: &mut ValState) -> Val {
        let x = s.get(self).unwrap();
        match x.flag {
            Kind::ValueInt => return Val::mk_int(&x.val_i.unwrap()),
            Kind::ValueBool => return Val::mk_bool(&x.val_b.unwrap()),
            Kind::Undefined => return Val::mk_undefined(),
        }
    }
    fn infer(&self, t: &mut TyState) -> Type {
        if let Some(x) = t.get(self) {
            return *x; //Todo check if clone is needed
        } else {
            return Type::TyIllTyped;
        }
    }
}

type Bool = bool;
impl Exp for Bool {
    fn pretty(&self) -> String {
        self.to_string()
    }
    fn eval(&self, s: &mut ValState) -> Val {
        Val::mk_bool(self)
    }
    fn infer(&self, t: &mut TyState) -> Type {
        Type::TyBool
    }
}
type Num = i32;
impl Exp for Num {
    fn pretty(&self) -> String {
        self.to_string()
    }
    fn eval(&self, s: &mut ValState) -> Val {
        Val::mk_int(self)
    }
    fn infer(&self, t: &mut TyState) -> Type {
        Type::TyInt
    }
}
struct Mult {
    exp: [Box<dyn Exp>; 2],
}
impl Exp for Mult {
    fn pretty(&self) -> String {
        let mut x = String::new();
        x.push_str("(");
        x.push_str(&self.exp[0].pretty());
        x.push_str("*");
        x.push_str(&self.exp[1].pretty());
        x.push_str(")");
        x
    }
    fn eval(&self, s: &mut ValState) -> Val {
        let n1 = self.exp[0].eval(s);
        let n2 = self.exp[1].eval(s);
        if let (Kind::ValueInt, Kind::ValueInt) = (n1.flag, n2.flag) {
            return Val::mk_int(&(n1.val_i.unwrap() * n2.val_i.unwrap()));
        }
        Val::mk_undefined()
    }
    fn infer(&self, t: &mut TyState) -> Type {
        let t1 = self.exp[0].infer(t);
        let t2 = self.exp[1].infer(t);
        if let (Type::TyInt, Type::TyInt) = (t1, t2) {
            return Type::TyInt;
        }
        Type::TyIllTyped
    }
}
struct Plus {
    exp: [Box<dyn Exp>; 2],
}
impl Exp for Plus {
    fn pretty(&self) -> String {
        let mut x = String::new();
        x.push_str("(");
        x.push_str(&self.exp[0].pretty());
        x.push_str("+");
        x.push_str(&self.exp[1].pretty());
        x.push_str(")");
        x
    }
    fn eval(&self, s: &mut ValState) -> Val {
        let n1 = self.exp[0].eval(s);
        let n2 = self.exp[1].eval(s);
        if let (Kind::ValueInt, Kind::ValueInt) = (n1.flag, n2.flag) {
            return Val::mk_int(&(n1.val_i.unwrap() + n2.val_i.unwrap()));
        }
        Val::mk_undefined()
    }
    fn infer(&self, t: &mut TyState) -> Type {
        let t1 = self.exp[0].infer(t);
        let t2 = self.exp[1].infer(t);
        if let (Type::TyInt, Type::TyInt) = (t1, t2) {
            return Type::TyInt;
        }
        Type::TyIllTyped
    }
}
struct And {
    exp: [Box<dyn Exp>; 2],
}
impl Exp for And {
    fn pretty(&self) -> String {
        let mut x = String::new();
        x.push_str("(");
        x.push_str(&self.exp[0].pretty());
        x.push_str("&&");
        x.push_str(&self.exp[1].pretty());
        x.push_str(")");
        x
    }
    fn eval(&self, s: &mut ValState) -> Val {
        let b1 = self.exp[0].eval(s);
        let b2 = self.exp[1].eval(s);
        if let (Kind::ValueBool, Kind::ValueBool) = (b1.flag, b2.flag) {
            return Val::mk_bool(&(b1.val_b.unwrap() && b2.val_b.unwrap()));
        }
        return Val::mk_undefined();
    }
    fn infer(&self, t: &mut TyState) -> Type {
        let t1 = self.exp[0].infer(t);
        let t2 = self.exp[1].infer(t);
        if let (Type::TyBool, Type::TyBool) = (t1, t2) {
            return Type::TyBool;
        }
        Type::TyIllTyped
    }
}
struct Or {
    exp: [Box<dyn Exp>; 2],
}
impl Exp for Or {
    fn pretty(&self) -> String {
        let mut x = String::new();
        x.push_str("(");
        x.push_str(&self.exp[0].pretty());
        x.push_str("||");
        x.push_str(&self.exp[1].pretty());
        x.push_str(")");
        x
    }
    fn eval(&self, s: &mut ValState) -> Val {
        let b1 = self.exp[0].eval(s);
        let b2 = self.exp[1].eval(s);
        if let (Kind::ValueBool, Kind::ValueBool) = (b1.flag, b2.flag) {
            return Val::mk_bool(&(b1.val_b.unwrap() || b2.val_b.unwrap()));
        }
        return Val::mk_undefined();
    }
    fn infer(&self, t: &mut TyState) -> Type {
        let t1 = self.exp[0].infer(t);
        let t2 = self.exp[1].infer(t);
        if let (Type::TyBool, Type::TyBool) = (t1, t2) {
            return Type::TyBool;
        }
        Type::TyIllTyped
    }
}
struct Equ {
    exp: [Box<dyn Exp>; 2],
}
impl Exp for Equ {
    fn pretty(&self) -> String {
        let mut x = String::new();
        x.push_str("(");
        x.push_str(&self.exp[0].pretty());
        x.push_str("==");
        x.push_str(&self.exp[1].pretty());
        x.push_str(")");
        x
    }
    fn eval(&self, s: &mut ValState) -> Val {
        let v1 = self.exp[0].eval(s);
        let v2 = self.exp[1].eval(s);
        match (v1.flag, v2.flag) {
            (Kind::ValueInt, Kind::ValueInt) => {
                return Val::mk_bool(&(v1.val_i.unwrap() == v2.val_i.unwrap()))
            }
            (Kind::ValueBool, Kind::ValueBool) => {
                return Val::mk_bool(&(v1.val_b.unwrap() == v2.val_b.unwrap()))
            }
            _ => return Val::mk_undefined(),
        }
    }
    fn infer(&self, t: &mut TyState) -> Type {
        let t1 = self.exp[0].infer(t);
        let t2 = self.exp[1].infer(t);
        match (t1, t2) {
            (Type::TyBool, Type::TyBool) => return Type::TyBool,
            (Type::TyInt, Type::TyInt) => return Type::TyInt,
            _ => Type::TyIllTyped,
        }
    }
}

struct Less {
    exp: [Box<dyn Exp>; 2],
}
impl Exp for Less {
    fn pretty(&self) -> String {
        let mut x = String::new();
        x.push_str("(");
        x.push_str(&self.exp[0].pretty());
        x.push_str("<");
        x.push_str(&self.exp[1].pretty());
        x.push_str(")");
        x
    }
    fn eval(&self, s: &mut ValState) -> Val {
        let n1 = self.exp[0].eval(s);
        let n2 = self.exp[1].eval(s);
        if let (Kind::ValueInt, Kind::ValueInt) = (n1.flag, n2.flag) {
            return Val::mk_int(&(n1.val_i.unwrap() + n2.val_i.unwrap()));
        }
        Val::mk_undefined()
    }
    fn infer(&self, t: &mut TyState) -> Type {
        let t1 = self.exp[0].infer(t);
        let t2 = self.exp[1].infer(t);
        if let (Type::TyInt, Type::TyInt) = (t1, t2) {
            return Type::TyInt;
        }
        Type::TyIllTyped
    }
}

struct Neg {
    exp: Box<dyn Exp>,
}
impl Exp for Neg {
    fn pretty(&self) -> String {
        let mut x = String::new();
        x.push_str("(");
        x.push_str("!");
        x.push_str(&self.exp.pretty());
        x.push_str(")");
        x
    }
    fn eval(&self, s: &mut ValState) -> Val {
        let b = self.exp.eval(s);
        if let Kind::ValueBool = b.flag {
            return Val::mk_bool(&(!b.val_b.unwrap()));
        }
        Val::mk_undefined()
    }
    fn infer(&self, t: &mut TyState) -> Type {
        let t = self.exp.infer(t);
        if let Type::TyBool = t {
            return Type::TyBool;
        }
        Type::TyIllTyped
    }
}

struct Grp {
    exp: Box<dyn Exp>,
}
impl Exp for Grp {
    fn pretty(&self) -> String {
        let mut x = String::new();
        x.push_str("(");
        x.push_str(&self.exp.pretty());
        x.push_str(")");
        x
    }
    fn eval(&self, s: &mut ValState) -> Val {
        let v = self.exp.eval(s);
        match v.flag {
            Kind::ValueBool => Val::mk_bool(&(v.val_b.unwrap())),
            Kind::ValueInt => Val::mk_int(&(v.val_i.unwrap())),
            _ => Val::mk_undefined(),
        }
    }
    fn infer(&self, t: &mut TyState) -> Type {
        let t = self.exp.infer(t);
        match t {
            Type::TyBool => Type::TyBool,
            Type::TyInt => Type::TyInt,
            _ => Type::TyIllTyped,
        }
    }
}

//Enum Approach
/*
enum Exp {
    Bool { val: bool },
    Num { val: i32 },
    Plus { left: Box<Exp>, right: Box<Exp> },
    Mult { left: Box<Exp>, right: Box<Exp> },
    And { left: Box<Exp>, right: Box<Exp> },
    Or { left: Box<Exp>, right: Box<Exp> },
    Var { val: String },
    Neg { exp: Box<Exp> },
    Equ { left: Box<Exp>, right: Box<Exp> },
    Less { left: Box<Exp>, right: Box<Exp> },
    Grp { exp: Box<Exp> },
} */

//helper
fn number(x: i32) -> Box<dyn Exp> {
    let y: Num = x;
    return Box::new(y);
}
fn boolean(x: bool) -> Box<dyn Exp> {
    let y: Bool = x;
    return Box::new(y);
}
fn mult(x: Box<dyn Exp>, y: Box<dyn Exp>) -> Box<dyn Exp> {
    Box::new(Mult { exp: [x, y] })
}
fn plus(x: Box<dyn Exp>, y: Box<dyn Exp>) -> Box<dyn Exp> {
    Box::new(Plus { exp: [x, y] })
}

fn and(x: Box<dyn Exp>, y: Box<dyn Exp>) -> Box<dyn Exp> {
    Box::new(And { exp: [x, y] })
}
fn or(x: Box<dyn Exp>, y: Box<dyn Exp>) -> Box<dyn Exp> {
    Box::new(Or { exp: [x, y] })
}
fn less(x: Box<dyn Exp>, y: Box<dyn Exp>) -> Box<dyn Exp> {
    Box::new(Less { exp: [x, y] })
}
fn equal(x: Box<dyn Exp>, y: Box<dyn Exp>) -> Box<dyn Exp> {
    Box::new(Equ { exp: [x, y] })
}
fn not(exp: Box<dyn Exp>) -> Box<dyn Exp> {
    Box::new(Neg { exp })
}
fn group(exp: Box<dyn Exp>) -> Box<dyn Exp> {
    Box::new(Grp { exp })
}
fn seq(x: Box<dyn Stmt>, y: Box<dyn Stmt>) -> Box<dyn Stmt> {
    Box::new(Seq { stmts: [x, y] })
}
fn decl(lhs: String, rhs: Box<dyn Exp>) -> Box<dyn Stmt> {
    Box::new(Decl { lhs, rhs })
}
fn variable(name: String) -> Box<dyn Exp> {
    let x: Var = name;
    Box::new(x)
}
fn assign(lhs: Var, rhs: Box<dyn Exp>) -> Box<dyn Stmt> {
    Box::new(Assign { lhs, rhs })
}
fn ifthenelse(
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
fn _while(cond: Box<dyn Exp>, stmt: Box<dyn Stmt>) -> Box<dyn Stmt> {
    Box::new(While { cond, stmt })
}
fn print(print_exp: Box<dyn Exp>) -> Box<dyn Stmt> {
    Box::new(Print { print_exp })
}
fn run_exp(e: Box<dyn Exp>) {
    let mut s = HashMap::<String, Val>::new();
    let mut t = HashMap::<String, Type>::new();
    println!("*******");
    println!("{}", e.pretty());
    println!("{}", show_val(e.eval(&mut s)));
    println!("{}", show_type(e.infer(&mut t)));
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
fn main() {
    let mut ast = number(5);
    run_exp(ast);
    ast = plus(mult(number(1), number(2)), number(0));

    run_exp(ast);
    ast = and(boolean(false), number(0));
    run_exp(ast);

    ast = or(boolean(false), number(0));
    run_exp(ast);
    ast = less(number(0), number(1));
    run_exp(ast);
    let ast_stmt = seq(
        decl("x".to_string(), number(1)),
        decl("y".to_string(), plus(number(6), variable("x".to_string()))),
    );
    run_stmt(ast_stmt);
}
