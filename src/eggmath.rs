#![allow(clippy::cognitive_complexity)]
use egg::{
    define_term,
    egraph::EClass,
    expr::{Expr, Language, Name, RecExpr},
};

use std::sync::atomic::{AtomicBool, Ordering};

use num_bigint::BigInt;
use num_rational::{BigRational, Ratio};
use num_traits::{Pow, Zero};

pub type MathEGraph<M = Meta> = egg::egraph::EGraph<Math, M>;

lazy_static! {
    static ref IS_CONSTANT_FOLDING_ENABLED: AtomicBool = AtomicBool::new(true);
}

pub fn set_constant_folding(flag: bool) {
    IS_CONSTANT_FOLDING_ENABLED.store(flag, Ordering::Relaxed);
}

define_term! {
    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    pub enum FPConstant {
    True = "TRUE",
    False = "FALSE",
    E = "E",
    Log2E = "LOG2E",
    Log10E = "LOG10E",
    Ln2 = "LN2",
    Ln10 = "LN10",
    Pi = "PI",
    Pi2 = "PI_2",
    Pi4 = "PI_4",
    Pi1Alt = "1_PI",
    Pi2Alt = "2_PI",
    Sqrtpi2 = "2_SQRTPI",
    Sqrt2 = "SQRT2",
    Sqrt1_2 = "SQRT1_2",
    Infinity = "INFINITY",
    Nan = "NAN",
    }
}

type Constant = BigRational;
// operators from FPCore
define_term! {
    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    pub enum Math {
        Constant(Constant),

    // complex operators not from FPCore
    Re = "re",
    Im = "im",
    Complex = "complex",
    Conj = "conj",
    Addc = "+.c",
    Subc = "-.c",
    Negc = "neg.c",
    Divc = "/.c",
    Mulc = "*.c",


    // FPCore operations
    Erf = "erf",
    Erfc = "erfc",
    Tgamma = "tgamma",
    Lgamma = "lgamma",
    Ceil = "ceil",
    Floor = "floor",
    Fmod = "fmod",
    Remainder = "remainder",
    Fmax = "fmax",
    Fmin = "fmin",
    Fdim = "fdim",
    Copysign = "copysign",
    Trunc = "trunc",
    Round = "round",
    NearbyInt = "nearbyint",



        Add = "+",
        Neg = "neg",
        Sub = "-",
        Mul = "*",
        Div = "/",
        Pow = "pow",
        Exp = "exp",
    Exp2 = "exp2",
        Log = "log",
        Sqrt = "sqrt",
        Cbrt = "cbrt",
        Fabs = "fabs",
        Sin = "sin",
        Cos = "cos",
        Tan = "tan",
        Asin = "asin",
        Acos = "acos",
        Atan = "atan",
        Atan2 = "atan2",
        Sinh = "sinh",
        Cosh = "cosh",
        Tanh = "tanh",
        Asinh = "asinh",
        Acosh = "acosh",
        Atanh = "atanh",

        Fma = "fma",
        Log1p = "log1p",
    Log10 = "log10",
    Log2 = "log2",
        Expm1 = "expm1",
        Hypot = "hypot",

        PositAdd = "+.p16",
        PositSub = "-.p16",
        PositMul = "*.p16",
        PositDiv = "/.p16",
        RealToPosit = "real->posit",
    FPConstant(FPConstant),
        Variable(Name),
    }
}

impl Language for Math {
    fn cost(&self, children: &[u64]) -> u64 {
        let cost = match self {
            Math::Constant(_) | Math::Variable(_) | Math::FPConstant(_) => 0,
            _ => 1,
        };

        cost + children.iter().sum::<u64>()
    }
}

#[derive(Debug, Clone)]
pub struct Meta {
    pub cost: u64,
    pub best: RecExpr<Math>,
}

/*
fn eval_const(op: Math, args: &[Math]) -> Option<Math> {
    let a = |i| args.get(i).cloned();
    match op {
        Math::Cos => {
            if a(0)? == Math::FPConstant(FPConstant::Pi) {
                Some(Math::Constant(Ratio::from_integer(BigInt::from(-1))))
            } else {
                None
            }
        }
        _ => None,
    }
}*/

fn eval(op: Math, args: &[Math]) -> Option<Math> {
    let a = |i| match args.get(i)? {
        Math::Constant(ref c) => Some(c.clone()),
        _ => None,
    };

    let math = |i: usize| -> Option<Math> { args.get(i).cloned() };
    let result = |i: Constant| -> Option<Math> { Some(Math::Constant(i)) };

    match op {
        Math::Add => result(a(0)? + a(1)?),
        Math::Neg => result(-a(0)?),
        Math::Sub => result(a(0)? - a(1)?),
        Math::Mul => result(a(0)? * a(1)?),
        Math::Div => {
            if a(1)?.is_zero() {
                None
            } else {
                result(a(0)? / a(1)?)
            }
        }
        Math::Pow => {
            if a(1)?.is_integer() {
                let exponent = a(1)?.numer().to_biguint()?;
                let new_top = Pow::pow(a(0)?.numer(), &exponent);
                let new_bot = Pow::pow(a(0)?.denom(), &exponent);
                result(Ratio::new(new_top, new_bot))
            } else {
                None
            }
        }
        Math::Sqrt => {
            if *a(0)?.numer() > BigInt::from(0) && *a(0)?.denom() > BigInt::from(0) {
                let s1 = a(0)?.numer().sqrt();
                let s2 = a(0)?.denom().sqrt();
                let is_perfect = &(&s1 * &s1) == a(0)?.numer() && &(&s2 * &s2) == a(1)?.denom();
                if is_perfect {
                    result(Ratio::new(s1, s2))
                } else {
                    None
                }
            } else {
                None
            }
        }
        Math::Fabs => {
            if a(0)? < Ratio::from_integer(Zero::zero()) {
                result(-a(0)?)
            } else {
                result(a(0)?)
            }
        }
        Math::Sin => {
            if a(0)?.is_zero() {
                result(a(0)?)
            } else {
                None
            }
        }
        Math::Cos => {
            if math(0)? == Math::FPConstant(FPConstant::Pi) {
                result(Ratio::from_integer(BigInt::from(-1)))
            } else if a(1)?.is_integer() && a(0)?.numer() == &BigInt::from(1) {
                result(Ratio::from_integer(Zero::zero()))
            } else {
                None
            }
        }
        Math::Floor => result(a(0)?.floor()),
        Math::Ceil => result(a(0)?.ceil()),
        Math::Round => result(a(0)?.round()),
        Math::RealToPosit => result(a(0)?),
        _ => None,
    }
}

impl egg::egraph::Metadata<Math> for Meta {
    type Error = std::convert::Infallible;
    fn merge(&self, other: &Self) -> Self {
        if self.cost <= other.cost {
            self.clone()
        } else {
            other.clone()
        }
    }

    fn make(expr: Expr<Math, &Self>) -> Self {
        let expr = if !IS_CONSTANT_FOLDING_ENABLED.load(Ordering::Relaxed) {
            expr
        } else {
            let math_args: Vec<Math> = expr
                .children
                .iter()
                .map(|meta| meta.best.as_ref().op.clone())
                .collect();

            let eval_result = eval(expr.op.clone(), &math_args).map(Expr::unit);
            eval_result.unwrap_or(expr)
        };

        let best: RecExpr<_> = expr.map_children(|c| c.best.clone()).into();
        let children_costs: Vec<_> = expr.children.iter().map(|c| c.cost).collect();
        Self {
            best,
            cost: expr.op.cost(&children_costs),
        }
    }

    fn modify(eclass: &mut EClass<Math, Self>) {
        // NOTE pruning vs not pruning is decided right here
        let best = eclass.metadata.best.as_ref();
        if best.children.is_empty() {
            eclass.nodes.push(Expr::unit(best.op.clone()));
        }
    }
}
