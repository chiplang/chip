use std::fmt::{Debug, Error, Formatter};

pub struct ExprList<'a>(pub Vec<Box<Expr<'a>>>);

pub enum Expr<'a> {
    Number(Number),
    Op(Box<Expr<'a>>, Opcode, Box<Expr<'a>>),
    FunctionCall(FunctionCall<'a>),
    If(Box<Expr<'a>>, ExprList<'a>),
    IfElse(Box<Expr<'a>>, ExprList<'a>, ExprList<'a>),
    Ident(Ident<'a>),
    Error,
}

pub struct FunctionCall<'a> {
    pub ident: Ident<'a>,
    pub args: ArgList<'a>,
}

pub struct ArgList<'a>(pub Vec<Box<Expr<'a>>>);

pub struct Ident<'a>(pub &'a str);

pub enum Number {
    Int(i64),
    Float(f64),
}

pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}

impl Debug for ExprList<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        if self.0.is_empty() {
            return write!(fmt, "");
        }
        let mut comma_separated = String::from('\n');

        for item in &self.0[0..self.0.len() - 1] {
            comma_separated.push_str(&format!("{:?}\n", item));
        }

        comma_separated.push_str(&format!("{:?}", self.0[self.0.len() - 1]));
        write!(fmt, "{{ {}\n }}", comma_separated)
    }
}

impl Debug for Expr<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match self {
            Number(n) => write!(fmt, "{:?}", n),
            Op(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            FunctionCall(f) => write!(fmt, "{:?}", f),
            If(cond, exprs) => write!(fmt, "if {:?} {:?}", cond, exprs),
            IfElse(cond, if_exprs, else_exprs) => {
                write!(fmt, "if {:?} {:?} else {:?}", cond, if_exprs, else_exprs)
            }
            Ident(i) => write!(fmt, "{:?}", i),
            Error => write!(fmt, "error"),
        }
    }
}

impl Debug for Number {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Number::*;
        match *self {
            Int(i) => write!(fmt, "{}", i),
            Float(i) => write!(fmt, "{}", i),
        }
    }
}

impl Debug for FunctionCall<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "{:?}{:?}", self.ident, self.args)
    }
}

impl Debug for ArgList<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        if self.0.is_empty() {
            return write!(fmt, "()");
        }
        let mut comma_separated = String::new();

        for item in &self.0[0..self.0.len() - 1] {
            comma_separated.push_str(&format!("{:?}", item));
            comma_separated.push_str(", ");
        }

        comma_separated.push_str(&format!("{:?}", self.0[self.0.len() - 1]));
        write!(fmt, "({})", comma_separated)
    }
}

impl Debug for Ident<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "{}", self.0)
    }
}

impl Debug for Opcode {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Opcode::*;
        match *self {
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
        }
    }
}
