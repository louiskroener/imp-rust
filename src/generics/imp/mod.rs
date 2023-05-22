use std::collections::HashMap;
use std::fmt::Display;

pub mod interpreter;

//TODO add error handling for Option types e.g if variable exisits in Valstate
//TODO Write better pretty methods
//TODO replace Box<dyn Exp> with a generic Type, and make let Generic Type Implement Traits
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
pub trait Exp {
    fn pretty(&self) -> String;
    fn eval(&self, s: &mut ValState) -> Val;
    fn infer(&self, t: &mut TyState) -> Type;
}

pub trait Stmt {
    fn pretty(&self) -> String;
    fn eval(&self, s: &mut ValState);
    fn check(&self, t: &mut TyState) -> bool;
}

pub struct Seq<T1: Stmt, T2: Stmt> {
    pub first: Box<T1>,
    pub second: Box<T2>,
}
pub struct Decl<T: Exp> {
    pub lhs: String,
    pub rhs: Box<T>,
}
pub struct IfThenElse<T1: Exp, T2: Stmt, T3: Stmt> {
    pub cond: Box<T1>,
    pub then_stmt: Box<T2>,
    pub else_stmt: Box<T3>,
}
pub struct Assign<T: Exp> {
    pub lhs: Var,
    pub rhs: Box<T>,
}
pub struct While<T1: Exp, T2: Stmt> {
    pub cond: Box<T1>,
    pub stmt: Box<T2>,
}
pub struct Print<T: Exp> {
    pub print_exp: Box<T>,
}
pub type Var = String;
pub type Bool = bool;
pub type Num = i32;
pub struct Plus<T1: Exp, T2: Exp> {
    pub left: Box<T1>,
    pub right: Box<T2>,
}
pub struct Mult<T1: Exp, T2: Exp> {
    pub left: Box<T1>,
    pub right: Box<T2>,
}
pub struct And<T1: Exp, T2: Exp> {
    pub left: Box<T1>,
    pub right: Box<T2>,
}
pub struct Equ<T1: Exp, T2: Exp> {
    pub left: Box<T1>,
    pub right: Box<T2>,
}
pub struct Or<T1: Exp, T2: Exp> {
    pub left: Box<T1>,
    pub right: Box<T2>,
}
pub struct Less<T1: Exp, T2: Exp> {
    pub left: Box<T1>,
    pub right: Box<T2>,
}
pub struct Neg<T: Exp> {
    pub exp: Box<T>,
}
pub struct Grp<T: Exp> {
    pub exp: Box<T>,
}
