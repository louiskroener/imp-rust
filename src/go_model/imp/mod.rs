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

pub struct Seq {
    pub stmts: [Box<dyn Stmt>; 2],
}
pub struct Decl {
    pub lhs: String,
    pub rhs: Box<dyn Exp>,
}
pub struct IfThenElse {
    pub cond: Box<dyn Exp>,
    pub then_stmt: Box<dyn Stmt>,
    pub else_stmt: Box<dyn Stmt>,
}
pub struct Assign {
    pub lhs: Var,
    pub rhs: Box<dyn Exp>,
}
pub struct While {
    pub cond: Box<dyn Exp>,
    pub stmt: Box<dyn Stmt>,
}
pub struct Print {
    pub print_exp: Box<dyn Exp>,
}
pub type Var = String;
pub type Bool = bool;
pub type Num = i32;
pub struct Plus {
    pub exp: [Box<dyn Exp>; 2],
}
pub struct Mult {
    pub exp: [Box<dyn Exp>; 2],
}
pub struct And {
    pub exp: [Box<dyn Exp>; 2],
}
pub struct Equ {
    pub exp: [Box<dyn Exp>; 2],
}
pub struct Or {
    pub exp: [Box<dyn Exp>; 2],
}
pub struct Less {
    pub exp: [Box<dyn Exp>; 2],
}
pub struct Neg {
    pub exp: Box<dyn Exp>,
}
pub struct Grp {
    pub exp: Box<dyn Exp>,
}
