use std::collections::HashMap;

//TODO remove matches! with if let and match
//TODO add error handling for Option types e.g if variable exisits in Valstate
//TODO Implement fmt::Display for Val and Type and remove show_val and show_type
enum Kind {
    ValueInt,
    ValueBool,
    Undefined,
}
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
            x.pretty()
        }
        Kind::ValueBool => {
            let x: Bool = v.val_b.unwrap();
            x.pretty()
        }
        Kind::Undefined => return "undefined".to_string(),
    }
}
#[derive(Clone, Copy)]
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

type Seq = [Box<dyn Stmt>; 2];

impl Stmt for Seq {
    fn pretty(&self) -> String {
        let x = self[0].pretty() + "; " + &self[1].pretty();
        x
    }
    fn eval(&self, s: &mut ValState) {
        self[0].eval(s);
        self[1].eval(s);
    }
    fn check(&self, t: &mut TyState) -> bool {
        if !self[0].check(t) {
            return false;
        }
        self[1].check(t)
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
        *s.get_mut(&self.lhs).unwrap() = self.rhs.eval(s);
    }
    fn check(&self, t: &mut TyState) -> bool {
        let ty = self.rhs.infer(t);
        if let Type::TyIllTyped = ty {
            return false;
        }
        *t.get_mut(&self.lhs).unwrap() = ty;
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
        true
    }
}
struct Assign {
    lhs: Var,
    rhs: Box<dyn Exp>,
}
impl Stmt for Assign {
    fn pretty(&self) -> String {
        "undefined".to_string()
    }
    fn eval(&self, s: &mut ValState) {}
    fn check(&self, t: &mut TyState) -> bool {
        let x = &self.lhs.pretty();
        let y = self.rhs.infer(t);
        let w = t.get(x).unwrap();
        matches!(w, y)
    }
}
struct While {
    cond: Box<dyn Exp>,
    stmt: Box<dyn Stmt>,
}
struct Print {
    print_exp: Box<dyn Exp>,
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
        Val::mk_undefined()
    }
    fn infer(&self, t: &mut TyState) -> Type {
        Type::TyIllTyped
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
        Val::mk_undefined()
    }
    fn infer(&self, t: &mut TyState) -> Type {
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
        Val::mk_undefined()
    }
    fn infer(&self, t: &mut TyState) -> Type {
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
        Val::mk_undefined()
    }
    fn infer(&self, t: &mut TyState) -> Type {
        Type::TyIllTyped
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

fn run(e: Box<dyn Exp>) {
    let mut s = HashMap::<String, Val>::new();
    let mut t = HashMap::<String, Type>::new();
    println!("*******");
    println!("{}", e.pretty());
    println!("{}", show_val(e.eval(&mut s)));
    println!("{}", show_type(e.infer(&mut t)));
}
fn main() {
    let ast = number(5);
    run(ast);
}
