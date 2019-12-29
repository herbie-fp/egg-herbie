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
    fn cost(&self, children: &[f64]) -> f64 {
        let cost = match self {
            Math::Constant(_) | Math::Variable(_) | Math::FPConstant(_) => 0.0,
            _ => 1.0,
        };

        cost + children.iter().sum::<f64>()
    }
}

#[derive(Debug, Clone)]
pub struct Meta {
    pub cost: f64,
    pub best: RecExpr<Math>,
}

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
}

fn eval(op: Math, args: &[Constant]) -> Option<Constant> {
    let a = |i| args.get(i).cloned();
    match op {
        Math::Add => Some(a(0)? + a(1)?),
        Math::Sub => Some(a(0)? - a(1)?),
        Math::Mul => Some(a(0)? * a(1)?),
        Math::Div => {
            if a(1)?.is_zero() {
                None
            } else {
                Some(a(0)? / a(1)?)
            }
        }
        Math::Pow => {
            if a(1)?.is_integer() {
                let exponent = a(1)?.numer().to_biguint()?;
                let new_top = Pow::pow(a(0)?.numer(), &exponent);
                let new_bot = Pow::pow(a(0)?.denom(), &exponent);
                Some(Ratio::new(new_top, new_bot))
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
                    Some(Ratio::new(s1, s2))
                } else {
                    None
                }
            } else {
                None
            }
        }
        Math::Fabs => {
            if a(0)? < Ratio::from_integer(Zero::zero()) {
                Some(-a(0)?)
            } else {
                Some(a(0)?)
            }
        }
        Math::Sin => {
            if a(0)?.is_zero() {
                Some(a(0)?)
            } else {
                None
            }
        }
        Math::Cos => {
            if a(1)?.is_integer() && a(0)?.numer() == &BigInt::from(1) {
                Some(Ratio::from_integer(Zero::zero()))
            } else {
                None
            }
        }
        Math::Floor => Some(a(0)?.floor()),
        Math::Ceil => Some(a(0)?.ceil()),
        Math::Round => Some(a(0)?.round()),
        Math::RealToPosit => Some(a(0)?),
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
            let const_args: Option<Vec<Constant>> = expr
                .children
                .iter()
                .map(|meta| match meta.best.as_ref().op {
                    Math::Constant(ref c) => Some(c.clone()),
                    _ => None,
                })
                .collect();

            let math_args: Vec<Math> = expr
                .children
                .iter()
                .map(|meta| meta.best.as_ref().op.clone())
                .collect();

            let eval_const_result = eval_const(expr.op.clone(), &math_args).map(Expr::unit);

            let eval_result = const_args
                .and_then(|a| eval(expr.op.clone(), &a))
                .map(|c| Expr::unit(Math::Constant(c)))
                .unwrap_or(expr);

            eval_const_result.unwrap_or(eval_result)
        };

        let best: RecExpr<_> = expr.map_children(|c| c.best.clone()).into();
        Self {
            best,
            cost: expr.map_children(|c| c.cost).cost(),
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
