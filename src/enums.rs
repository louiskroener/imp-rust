use std::collections::HashMap;
use std::fmt::Display;
//TODO Find way to use enum variants as types or add errors for mismatched variant
#[derive(PartialEq, Debug)]
pub enum Kind {
    ValueInt,
    ValueBool,
    Undefined,
}
#[derive(Debug)]
pub struct Val {
    pub flag: Kind,
    pub val_i: Option<i32>,
    pub val_b: Option<bool>,
}
impl Val {
    fn mk_int(x: &i32) -> Val {
        Val {
            flag: Kind::ValueInt,
            val_i: Some(*x),
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
            let x = Exp::Num {
                val: v.val_i.unwrap(),
            };
            return x.pretty();
        }
        Kind::ValueBool => {
            let x = Exp::Bool {
                val: v.val_b.unwrap(),
            };
            return x.pretty();
        }
        Kind::Undefined => return "undefined".to_string(),
    }
}
impl Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.flag {
            Kind::ValueInt => write!(f, "{}", self.val_i.unwrap()),
            Kind::ValueBool => write!(f, "{}", self.val_b.unwrap()),
            Kind::Undefined => write!(f, "{}", "undefined"),
        }
    }
}
#[derive(Clone, Copy, PartialEq)]
pub enum Type {
    TyIllTyped,
    TyInt,
    TyBool,
}

fn show_type(t: Type) -> String {
    match t {
        Type::TyInt => return "int".to_string(),
        Type::TyBool => return "bool".to_string(),
        Type::TyIllTyped => return "Illtyped".to_string(),
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::TyInt => write!(f, "{}", "int"),
            Type::TyBool => write!(f, "{}", "bool"),
            Type::TyIllTyped => write!(f, "{}", "illtyped"),
        }
    }
}
type ValState = HashMap<String, Val>;
type TyState = HashMap<String, Type>;
pub enum Exp {
    Var { name: String },
    Num { val: i32 },
    Bool { val: bool },
    Plus { left: Box<Exp>, right: Box<Exp> },
    Mult { left: Box<Exp>, right: Box<Exp> },
    And { left: Box<Exp>, right: Box<Exp> },
    Or { left: Box<Exp>, right: Box<Exp> },
    Neg { exp: Box<Exp> },
    Equ { left: Box<Exp>, right: Box<Exp> },
    Less { left: Box<Exp>, right: Box<Exp> },
    Grp { exp: Box<Exp> },
}

pub enum Stmt {
    Seq {
        first: Box<Stmt>,
        second: Box<Stmt>,
    },
    Decl {
        lhs: String,
        rhs: Box<Exp>,
    },
    While {
        cond: Box<Exp>,
        stmt: Box<Stmt>,
    },
    IfThenElse {
        cond: Box<Exp>,
        then_stmt: Box<Stmt>,
        else_stmt: Box<Stmt>,
    },
    Assign {
        lhs: String,
        rhs: Box<Exp>,
    },
    Print {
        print_exp: Box<Exp>,
    },
}
impl Exp {
    fn eval(&self, s: &mut ValState) -> Val {
        match &self {
            Exp::Var { name } => {
                let x = s.get(name).unwrap();
                match x.flag {
                    Kind::ValueInt => return Val::mk_int(&x.val_i.unwrap()),
                    Kind::ValueBool => return Val::mk_bool(&x.val_b.unwrap()),
                    Kind::Undefined => return Val::mk_undefined(),
                }
            }
            Exp::Num { val } => return Val::mk_int(&val),
            Exp::Bool { val } => return Val::mk_bool(&val),
            Exp::Mult { left, right } => {
                let n1 = left.eval(s);
                let n2 = right.eval(s);
                if let (Kind::ValueInt, Kind::ValueInt) = (n1.flag, n2.flag) {
                    return Val::mk_int(&(n1.val_i.unwrap() * n2.val_i.unwrap()));
                }
                Val::mk_undefined()
            }
            Exp::Plus { left, right } => {
                let n1 = left.eval(s);
                let n2 = right.eval(s);
                if let (Kind::ValueInt, Kind::ValueInt) = (n1.flag, n2.flag) {
                    return Val::mk_int(&(n1.val_i.unwrap() + n2.val_i.unwrap()));
                }
                Val::mk_undefined()
            }
            Exp::And { left, right } => {
                let b1 = left.eval(s);
                let b2 = right.eval(s);
                if let (Kind::ValueBool, Kind::ValueBool) = (b1.flag, b2.flag) {
                    return Val::mk_bool(&(b1.val_b.unwrap() && b2.val_b.unwrap()));
                }
                return Val::mk_undefined();
            }
            Exp::Or { left, right } => {
                let b1 = left.eval(s);
                let b2 = right.eval(s);
                if let (Kind::ValueBool, Kind::ValueBool) = (b1.flag, b2.flag) {
                    return Val::mk_bool(&(b1.val_b.unwrap() || b2.val_b.unwrap()));
                }
                return Val::mk_undefined();
            }
            Exp::Equ { left, right } => {
                let v1 = left.eval(s);
                let v2 = right.eval(s);
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
            Exp::Less { left, right } => {
                let n1 = left.eval(s);
                let n2 = right.eval(s);
                if let (Kind::ValueInt, Kind::ValueInt) = (n1.flag, n2.flag) {
                    return Val::mk_bool(&(n1.val_i.unwrap() < n2.val_i.unwrap()));
                }
                Val::mk_undefined()
            }
            Exp::Neg { exp } => {
                let b = exp.eval(s);
                if let Kind::ValueBool = b.flag {
                    return Val::mk_bool(&(!b.val_b.unwrap()));
                }
                Val::mk_undefined()
            }
            Exp::Grp { exp } => {
                let v = exp.eval(s);
                match v.flag {
                    Kind::ValueBool => Val::mk_bool(&(v.val_b.unwrap())),
                    Kind::ValueInt => Val::mk_int(&(v.val_i.unwrap())),
                    _ => Val::mk_undefined(),
                }
            }
        }
    }
    fn pretty(&self) -> String {
        match self {
            Exp::Var { name } => name.to_string(),
            Exp::Num { val } => val.to_string(),
            Exp::Bool { val } => val.to_string(),
            Exp::Mult { left, right } => {
                let mut x = String::new();
                x.push_str("(");
                x.push_str(&left.pretty());
                x.push_str("*");
                x.push_str(&right.pretty());
                x.push_str(")");
                x
            }

            Exp::Plus { left, right } => {
                let mut x = String::new();
                x.push_str("(");
                x.push_str(&left.pretty());
                x.push_str("+");
                x.push_str(&right.pretty());
                x.push_str(")");
                x
            }

            Exp::And { left, right } => {
                let mut x = String::new();
                x.push_str("(");
                x.push_str(&left.pretty());
                x.push_str("&&");
                x.push_str(&right.pretty());
                x.push_str(")");
                x
            }

            Exp::Or { left, right } => {
                let mut x = String::new();
                x.push_str("(");
                x.push_str(&left.pretty());
                x.push_str("||");
                x.push_str(&right.pretty());
                x.push_str(")");
                x
            }
            Exp::Equ { left, right } => {
                let mut x = String::new();
                x.push_str("(");
                x.push_str(&left.pretty());
                x.push_str("==");
                x.push_str(&right.pretty());
                x.push_str(")");
                x
            }

            Exp::Less { left, right } => {
                let mut x = String::new();
                x.push_str("(");
                x.push_str(&left.pretty());
                x.push_str("<");
                x.push_str(&right.pretty());
                x.push_str(")");
                x
            }
            Exp::Neg { exp } => {
                let mut x = String::new();
                x.push_str("(");
                x.push_str("!");
                x.push_str(&exp.pretty());
                x.push_str(")");
                x
            }
            Exp::Grp { exp } => {
                let mut x = String::new();
                x.push_str("(");
                x.push_str(&exp.pretty());
                x.push_str(")");
                x
            }
        }
    }
    fn infer(&self, t: &mut TyState) -> Type {
        match self {
            Exp::Var { name } => {
                if let Some(x) = t.get(name) {
                    return *x; //Todo check if clone is needed
                } else {
                    return Type::TyIllTyped;
                }
            }
            Exp::Bool { .. } => Type::TyBool,
            Exp::Num { .. } => Type::TyInt,
            Exp::Mult { left, right } => {
                let t1 = left.infer(t);
                let t2 = right.infer(t);
                if let (Type::TyInt, Type::TyInt) = (t1, t2) {
                    return Type::TyInt;
                }
                Type::TyIllTyped
            }
            Exp::Plus { left, right } => {
                let t1 = left.infer(t);
                let t2 = right.infer(t);
                if let (Type::TyInt, Type::TyInt) = (t1, t2) {
                    return Type::TyInt;
                }
                Type::TyIllTyped
            }
            Exp::And { left, right } => {
                let t1 = left.infer(t);
                let t2 = right.infer(t);
                if let (Type::TyBool, Type::TyBool) = (t1, t2) {
                    return Type::TyBool;
                }
                Type::TyIllTyped
            }
            Exp::Or { left, right } => {
                let t1 = left.infer(t);
                let t2 = right.infer(t);
                if let (Type::TyBool, Type::TyBool) = (t1, t2) {
                    return Type::TyBool;
                }
                Type::TyIllTyped
            }
            Exp::Equ { left, right } => {
                let t1 = left.infer(t);
                let t2 = right.infer(t);
                match (t1, t2) {
                    (Type::TyBool, Type::TyBool) => return Type::TyBool,
                    (Type::TyInt, Type::TyInt) => return Type::TyInt,
                    _ => Type::TyIllTyped,
                }
            }
            Exp::Less { left, right } => {
                let t1 = left.infer(t);
                let t2 = right.infer(t);
                if let (Type::TyInt, Type::TyInt) = (t1, t2) {
                    return Type::TyBool;
                }
                Type::TyIllTyped
            }
            Exp::Neg { exp } => {
                let t = exp.infer(t);
                if let Type::TyBool = t {
                    return Type::TyBool;
                }
                Type::TyIllTyped
            }
            Exp::Grp { exp } => {
                let t = exp.infer(t);
                match t {
                    Type::TyBool => Type::TyBool,
                    Type::TyInt => Type::TyInt,
                    _ => Type::TyIllTyped,
                }
            }
        }
    }
}
impl Stmt {
    fn pretty(&self) -> String {
        match self {
            Stmt::Seq { first, second } => {
                let mut x = String::new();
                x.push_str(&first.pretty());
                x.push_str(";");
                x.push_str(&second.pretty());
                x
            }
            Stmt::Decl { lhs, rhs } => {
                let mut x = String::new();
                x.push_str(&lhs);
                x.push_str(":= ");
                x.push_str(&rhs.pretty());
                x
            }
            Stmt::IfThenElse {
                cond,
                then_stmt,
                else_stmt,
            } => {
                let mut x = String::new();
                x.push_str("if ");
                x.push_str(&cond.pretty());
                x.push_str(" then ");
                x.push_str(&mut then_stmt.pretty());
                x.push_str(" else ");
                x.push_str(&mut else_stmt.pretty());
                x
            }
            Stmt::Assign { lhs, rhs } => {
                let mut x = String::new();
                x.push_str(&lhs);
                x.push_str(" = ");
                x.push_str(&rhs.pretty());
                x
            }
            Stmt::While { cond, stmt } => {
                let mut x = String::new();
                x.push_str("while ");
                x.push_str(&cond.pretty());
                x.push_str(" ");
                x.push_str(&stmt.pretty());
                x
            }
            Stmt::Print { print_exp } => {
                let mut x = String::new();
                x.push_str("print ");
                x.push_str(&print_exp.pretty());
                x
            }
        }
    }
    fn eval(&self, s: &mut ValState) {
        match self {
            Stmt::Seq { first, second } => {
                first.eval(s);
                second.eval(s);
            }
            Stmt::Decl { lhs, rhs } => {
                let x = rhs.eval(s);
                s.insert(lhs.clone(), x);
            }
            Stmt::IfThenElse {
                cond,
                then_stmt,
                else_stmt,
            } => {
                let v = cond.eval(s);
                if let Kind::ValueBool = v.flag {
                    if v.val_b.unwrap() {
                        then_stmt.eval(s);
                    } else {
                        else_stmt.eval(s);
                    }
                } else {
                    println!("Error Parsing IfThenElse");
                }
            }
            Stmt::Assign { lhs, rhs } => {
                //TODO there should be a cleaner solution
                let v = Exp::Var {
                    name: lhs.to_string(),
                }
                .eval(s);
                let val = rhs.eval(s);
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
                *s.get_mut(lhs).unwrap() = val;
            }
            Stmt::While { cond, stmt } => {
                let mut cond_i = cond.eval(s);
                if let Kind::ValueBool = cond_i.flag {
                    while cond_i.val_b.unwrap() {
                        stmt.eval(s);
                        cond_i = cond.eval(s);
                    }
                } else {
                    println!("cond is no bool");
                }
            }
            Stmt::Print { print_exp } => {
                let v = print_exp.eval(s);
                match v.flag {
                    Kind::ValueBool => println!("Output {}", v.val_b.unwrap()),
                    Kind::ValueInt => println!("Ouput {}", v.val_i.unwrap()),
                    _ => println!("Output Undefined"),
                }
            }
        }
    }
    fn check(&self, t: &mut TyState) -> bool {
        match self {
            Stmt::Seq { first, second } => {
                if !first.check(t) {
                    return false;
                }
                second.check(t)
            }
            Stmt::Decl { lhs, rhs } => {
                let ty = rhs.infer(t);
                if let Type::TyIllTyped = ty {
                    return false;
                }
                t.insert(lhs.clone(), ty);
                return true;
            }
            Stmt::While { cond, stmt } => {
                let ty = cond.infer(t);
                if let Type::TyIllTyped = ty {
                    return false;
                }
                stmt.check(t)
            }
            Stmt::Assign { lhs, rhs } => {
                let x = lhs;
                let y = rhs.infer(t);
                let w = t.get(x).unwrap();
                *w == y
            }
            Stmt::IfThenElse {
                cond,
                then_stmt,
                else_stmt,
            } => {
                let ty = cond.infer(t);
                if let Type::TyIllTyped = ty {
                    return false;
                }
                then_stmt.check(t) && else_stmt.check(t)
            }
            Stmt::Print { print_exp } => {
                let ty = print_exp.infer(t);
                if let Type::TyIllTyped = ty {
                    return false;
                }
                true
            }
        }
    }
}

pub fn number(val: i32) -> Box<Exp> {
    return Box::new(Exp::Num { val });
}
pub fn boolean(val: bool) -> Box<Exp> {
    return Box::new(Exp::Bool { val });
}
pub fn mult(left: Box<Exp>, right: Box<Exp>) -> Box<Exp> {
    Box::new(Exp::Mult { left, right })
}
pub fn plus(left: Box<Exp>, right: Box<Exp>) -> Box<Exp> {
    Box::new(Exp::Plus { left, right })
}
pub fn and(left: Box<Exp>, right: Box<Exp>) -> Box<Exp> {
    Box::new(Exp::And { left, right })
}
pub fn or(left: Box<Exp>, right: Box<Exp>) -> Box<Exp> {
    Box::new(Exp::Or { left, right })
}
pub fn less(left: Box<Exp>, right: Box<Exp>) -> Box<Exp> {
    Box::new(Exp::Less { left, right })
}
pub fn equal(left: Box<Exp>, right: Box<Exp>) -> Box<Exp> {
    Box::new(Exp::Equ { left, right })
}
pub fn not(exp: Box<Exp>) -> Box<Exp> {
    Box::new(Exp::Neg { exp })
}
pub fn group(exp: Box<Exp>) -> Box<Exp> {
    Box::new(Exp::Grp { exp })
}
pub fn seq(first: Box<Stmt>, second: Box<Stmt>) -> Box<Stmt> {
    Box::new(Stmt::Seq { first, second })
}
pub fn decl(lhs: String, rhs: Box<Exp>) -> Box<Stmt> {
    Box::new(Stmt::Decl { lhs, rhs })
}
pub fn variable(name: String) -> Box<Exp> {
    let x = Exp::Var { name };
    Box::new(x)
}
pub fn assign(lhs: String, rhs: Box<Exp>) -> Box<Stmt> {
    Box::new(Stmt::Assign { lhs, rhs })
}
pub fn ifthenelse(cond: Box<Exp>, then_stmt: Box<Stmt>, else_stmt: Box<Stmt>) -> Box<Stmt> {
    Box::new(Stmt::IfThenElse {
        cond,
        then_stmt,
        else_stmt,
    })
}
pub fn _while(cond: Box<Exp>, stmt: Box<Stmt>) -> Box<Stmt> {
    Box::new(Stmt::While { cond, stmt })
}
pub fn print(print_exp: Box<Exp>) -> Box<Stmt> {
    Box::new(Stmt::Print { print_exp })
}

fn run_exp(e: Box<Exp>) {
    let mut s = HashMap::<String, Val>::new();
    let mut t = HashMap::<String, Type>::new();
    println!("*******");
    println!("{}", e.pretty());
    println!("{}", e.eval(&mut s));
    println!("{}", e.infer(&mut t));
}
fn run_stmt(stmt: Box<Stmt>) {
    let mut s = HashMap::<String, Val>::new();
    let mut t = HashMap::<String, Type>::new();
    println!("*******");
    println!("{}", stmt.pretty());
    stmt.eval(&mut s);
    println!("state: {:?}", s);
    println!("type checker: {:?}", stmt.check(&mut t))
}
pub fn run() {
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
