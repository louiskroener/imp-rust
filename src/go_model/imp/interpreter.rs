use super::*;
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

// Exp
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
            return Val::mk_bool(&(n1.val_i.unwrap() < n2.val_i.unwrap()));
        }
        Val::mk_undefined()
    }
    fn infer(&self, t: &mut TyState) -> Type {
        let t1 = self.exp[0].infer(t);
        let t2 = self.exp[1].infer(t);
        if let (Type::TyInt, Type::TyInt) = (t1, t2) {
            return Type::TyBool;
        }
        Type::TyIllTyped
    }
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
