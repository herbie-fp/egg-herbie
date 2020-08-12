use egg::*;

use std::sync::atomic::{AtomicBool, Ordering};

use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::{Pow, Signed, Zero};

pub type Constant = num_rational::BigRational;
pub type RecExpr = egg::RecExpr<Math>;
pub type Pattern = egg::Pattern<Math>;
pub type EGraph = egg::EGraph<Math, ConstantFold>;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub type Runner = egg::Runner<Math, ConstantFold, IterData>;
pub type Iteration = egg::Iteration<IterData>;

pub struct IterData {
    pub extracted: Vec<(Id, Extracted)>,
}

pub struct Extracted {
    pub best: RecExpr,
    pub cost: usize,
}

impl IterationData<Math, ConstantFold> for IterData {
    fn make(runner: &Runner) -> Self {
        let mut extractor = Extractor::new(&runner.egraph, AstSize);
        let extracted = runner
            .roots
            .iter()
            .map(|&root| {
                let (cost, best) = extractor.find_best(root);
                let ext = Extracted { cost, best };
                (root, ext)
            })
            .collect();
        Self { extracted }
    }
}

// operators from FPCore
define_language! {
    pub enum Math {

        // FPCore constants
        "TRUE" = True(Id),
        "FALSE" = False(Id),
        "E" = E(Id),
        "PI" = Pi(Id),
        "INFINITY" = Infinity(Id),
        "NAN" = Nan(Id),

        // operators

        "if" = If([Id; 4]),
        "not" = Not([Id; 2]),
        "and" = And([Id; 3]),
        "or" = Or([Id; 3]),

        "==" = Equal([Id; 3]),
        "!=" = NotEqual([Id; 3]),
        "<" = Less([Id; 3]),
        ">" = Greater([Id; 3]),
        "<=" = LessEq([Id; 3]),
        ">=" = GreaterEq([Id; 3]),

        "erf" = Erf([Id; 2]),
        "erfc" = Erfc([Id; 2]),
        "tgamma" = Tgamma([Id; 2]),
        "lgamma" = Lgamma([Id; 2]),
        "ceil" = Ceil([Id; 2]),
        "floor" = Floor([Id; 2]),
        "fmod" = Fmod([Id; 3]),
        "remainder" = Remainder([Id; 3]),
        "fmax" = Fmax([Id; 3]),
        "fmin" = Fmin([Id; 3]),
        "fdim" = Fdim([Id; 3]),
        "copysign" = Copysign([Id; 2]),
        "trunc" = Trunc([Id; 2]),
        "round" = Round([Id; 2]),
        "nearbyint" = NearbyInt([Id; 2]),

        "+" = Add([Id; 3]),
        "-" = Sub([Id; 3]),
        "*" = Mul([Id; 3]),
        "/" = Div([Id; 3]),
        "pow" = Pow([Id; 3]),
        "neg" = Neg([Id; 2]),
        "exp" = Exp([Id; 2]),
        "exp2" = Exp2([Id; 2]),
        "log" = Log([Id; 2]),
        "sqrt" = Sqrt([Id; 2]),
        "cbrt" = Cbrt([Id; 2]),
        "fabs" = Fabs([Id; 2]),
        "sin" = Sin([Id; 2]),
        "cos" = Cos([Id; 2]),
        "tan" = Tan([Id; 2]),
        "asin" = Asin([Id; 2]),
        "acos" = Acos([Id; 2]),
        "atan" = Atan([Id; 2]),
        "atan2" = Atan2([Id; 3]),
        "sinh" = Sinh([Id; 2]),
        "cosh" = Cosh([Id; 2]),
        "tanh" = Tanh([Id; 2]),
        "asinh" = Asinh([Id; 2]),
        "acosh" = Acosh([Id; 2]),
        "atanh" = Atanh([Id; 2]),

        "fma" = Fma([Id; 4]),
        "log1p" = Log1p([Id; 2]),
        "log10" = Log10([Id; 2]),
        "log2" = Log2([Id; 2]),
        "expm1" = Expm1([Id; 2]),
        "hypot" = Hypot([Id; 3]),

        // Complex numbers

        "complex" = Complex([Id; 3]),
        "re" = Re([Id; 2]),
        "im" = Im([Id; 2]),
        "conj" = Conj([Id; 2]),

        // 8-bit posit numbers

        // "binary64->posit8" = F64ToPosit8(Id),
        // "posit8->binary64" = Posit8ToF64(Id),
        // "binary64->quire8" = F64ToQuire8(Id),
        // "quire8->binary64" = Quire8ToF64(Id),
        // "quire8-mul-add" = Quire8MulAdd([Id; 3]),
        // "quire8-mul-sub" = Quire8MulSub([Id; 3]),
        // "posit8->quire8" = Posit8ToQuire8(Id),
        // "quire8->posit8" = Quire8ToPosit8(Id),

        // 16-bit posit numbers

        // "binary64->posit16" = F64ToPosit16(Id),
        // "posit16->binary64" = Posit16ToF64(Id),
        // "binary64->quire16" = F64ToQuire16(Id),
        // "quire16->binary64" = Quire16ToF64(Id),
        // "quire16-mul-add" = Quire16MulAdd([Id; 3]),
        // "quire16-mul-sub" = Quire16MulSub([Id; 3]),
        // "posit16->quire16" = Posit16ToQuire16(Id),
        // "quire16->posit16" = Quire16ToPosit16(Id),

        // 32-bit posit numbers

        // "binary64->posit32" = F64ToPosit32(Id),
        // "posit32->binary64" = Posit32ToF64(Id),
        // "binary64->quire32" = F64ToQuire32(Id),
        // "quire32->binary64" = Quire32ToF64(Id),
        // "quire32-mul-add" = Quire32MulAdd([Id; 3]),
        // "quire32-mul-sub" = Quire32MulSub([Id; 3]),
        // "posit32->quire32" = Posit32ToQuire32(Id),
        // "quire32->posit32" = Quire32ToPosit32(Id),

        // Integer

        // "remainder.i32" = RemainderInt64([Id; 2]),
        // "shl.i32" = ShlInt64([Id; 2]),
        // "shr.i32" = ShrInt64([Id; 2]),
        // "and.i32" = AndInt64([Id; 2]),
        // "or.i32" = OrInt64([Id; 2]),
        // "xor.i32" = XorInt64([Id; 2]),

        Constant(Constant),
        Symbol(egg::Symbol),
        Other(egg::Symbol, Vec<Id>),
    }
}

pub struct ConstantFold {
    pub unsound: AtomicBool,
    pub constant_fold: bool,
    pub prune: bool,
}

impl Default for ConstantFold {
    fn default() -> Self {
        Self {
            constant_fold: true,
            prune: true,
            unsound: AtomicBool::from(false),
        }
    }
}

impl Analysis<Math> for ConstantFold {
    type Data = Option<Constant>;
    fn make(egraph: &EGraph, enode: &Math) -> Self::Data {
        if !egraph.analysis.constant_fold {
            return None;
        }

        let x = |id: &Id| egraph[*id].data.as_ref();
        match enode {
            Math::Constant(c) => Some(c.clone()),

            // real
            Math::Add([_p, a, b]) => Some(x(a)? + x(b)?),
            Math::Sub([_p, a, b]) => Some(x(a)? - x(b)?),
            Math::Mul([_p, a, b]) => Some(x(a)? * x(b)?),
            Math::Div([_p, a, b]) => {
                if x(b)?.is_zero() {
                    None
                } else {
                    Some(x(a)? / x(b)?)
                }
            }
            Math::Neg([_p, a]) => Some(-x(a)?.clone()),
            Math::Pow([_p, a, b]) => {
                if x(b)?.is_integer() {
                    Some(Pow::pow(x(a)?, x(b)?.to_integer()))
                } else {
                    None
                }
            }
            Math::Sqrt([_p, a]) => {
                let a = x(a)?;
                if *a.numer() > BigInt::from(0) && *a.denom() > BigInt::from(0) {
                    let s1 = a.numer().sqrt();
                    let s2 = a.denom().sqrt();
                    let is_perfect = &(&s1 * &s1) == a.numer() && &(&s2 * &s2) == a.denom();
                    if is_perfect {
                        Some(Ratio::new(s1, s2))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Math::Fabs([_p, a]) => Some(x(a)?.clone().abs()),
            Math::Floor([_p, a]) => Some(x(a)?.floor()),
            Math::Ceil([_p, a]) => Some(x(a)?.ceil()),
            Math::Round([_p, a]) => Some(x(a)?.round()),

            // Math::RealToPosit(a) => result(x(0)?),
            _ => None,
        }
    }

    fn merge(&self, to: &mut Self::Data, from: Self::Data) -> bool {
        match (&to, from) {
            (None, None) => false,
            (Some(_), None) => false, // no update needed
            (None, Some(c)) => {
                *to = Some(c);
                true
            }
            (Some(a), Some(ref b)) => {
                if a != b {
                    if !self.unsound.swap(true, Ordering::SeqCst) {
                        log::warn!("Bad merge detected: {} != {}", a, b);
                    }
                }
                false
            }
        }
    }

    fn modify(egraph: &mut EGraph, id: Id) {
        if let Some(constant) = egraph[id].data.clone() {
            let added = egraph.add(Math::Constant(constant));
            let (id, _) = egraph.union(id, added);
            if egraph.analysis.prune {
                egraph[id].nodes.retain(|n| n.is_leaf())
            }
        }
    }
}
