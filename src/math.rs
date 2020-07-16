use egg::*;

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
        // special FP constants
        "TRUE" = True,
        "FALSE" = False,
        "E" = E,
        "LOG2E" = Log2E,
        "LOG10E" = Log10E,
        "LN2" = Ln2,
        "LN10" = Ln10,
        "PI" = Pi,
        "PI_2" = Pi2,
        "PI_4" = Pi4,
        "1_PI" = Pi1Alt,
        "2_PI" = Pi2Alt,
        "2_SQRTPI" = Sqrtpi2,
        "SQRT2" = Sqrt2,
        "SQRT1_2" = Sqrt1_2,
        "INFINITY" = Infinity,
        "NAN" = Nan,

        // logical operators
        "if" = If([Id; 3]),
        "not" = Not(Id),
        "and" = And([Id; 2]),
        "or" = Or([Id; 2]),

        // comparison
        "<" = Less([Id; 2]),
        ">" = Greater([Id; 2]),
        "<=" = LessEq([Id; 2]),
        ">=" = GreaterEq([Id; 2]),

        // FPCore operations
        "erf" = Erf(Id),
        "erfc" = Erfc(Id),
        "tgamma" = Tgamma(Id),
        "lgamma" = Lgamma(Id),
        "ceil" = Ceil(Id),
        "floor" = Floor(Id),
        "fmod" = Fmod([Id; 2]),
        "remainder" = Remainder([Id; 2]),
        "fmax" = Fmax([Id; 2]),
        "fmin" = Fmin([Id; 2]),
        "fdim" = Fdim([Id; 2]),
        "copysign" = Copysign(Id),
        "trunc" = Trunc(Id),
        "round" = Round(Id),
        "nearbyint" = NearbyInt(Id),

        "+" = Add([Id; 2]),
        "-" = Sub([Id; 2]),
        "*" = Mul([Id; 2]),
        "/" = Div([Id; 2]),
        "pow" = Pow([Id; 2]),
        "neg" = Neg(Id),
        "exp" = Exp(Id),
        "exp2" = Exp2(Id),
        "log" = Log(Id),
        "sqrt" = Sqrt(Id),
        "cbrt" = Cbrt(Id),
        "fabs" = Fabs(Id),
        "sin" = Sin(Id),
        "cos" = Cos(Id),
        "tan" = Tan(Id),
        "asin" = Asin(Id),
        "acos" = Acos(Id),
        "atan" = Atan(Id),
        "atan2" = Atan2([Id; 2]),
        "sinh" = Sinh(Id),
        "cosh" = Cosh(Id),
        "tanh" = Tanh(Id),
        "asinh" = Asinh(Id),
        "acosh" = Acosh(Id),
        "atanh" = Atanh(Id),

        "fma" = Fma([Id; 3]),
        "log1p" = Log1p(Id),
        "log10" = Log10(Id),
        "log2" = Log2(Id),
        "expm1" = Expm1(Id),
        "hypot" = Hypot([Id; 2]),

        // Complex numbers

        "+.c" = Addc([Id; 2]),
        "neg.c" = Negc(Id),
        "-.c" = Subc([Id; 2]),
        "*.c" = Mulc([Id; 2]),
        "/.c" = Divc([Id; 2]),
        "exp.c" = Expc(Id),
        "log.c" = Logc(Id),
        "pow.c" = Powc([Id; 2]),
        "sqrt.c" = Sqrtc(Id),
        "complex" = Complex([Id; 2]),
        "re" = Re(Id),
        "im" = Im(Id),
        "conj" = Conj(Id),

        // 8-bit posit numbers

        "+.p8" = Posit8Add([Id; 2]),
        "neg.p8" = Posit8Neg(Id),
        "-.p8" = Posit8Sub([Id; 2]),
        "*.p8" = Posit8Mul([Id; 2]),
        "/.p8" = Posit8Div([Id; 2]),
        "sqrt.p8" = Posit8Sqrt(Id),
        "<.p8" = Posit8Lt([Id; 2]),
        ">.p8" = Posit8Gt([Id; 2]),
        "<=.p8" = Posit8Lte([Id; 2]),
        ">=.p8" = Posit8Gte([Id; 2]),
        "real->posit8" = RealToPosit8(Id),
        "posit8->real" = Posit8ToReal(Id),
        "real->quire8" = RealToQuire8(Id),
        "quire8->real" = Quire8ToReal(Id),
        "quire8-mul-add" = Quire8ToReal([Id; 3]),
        "quire8-mul-sub" = Quire8ToReal([Id; 3]),
        "posit8->quire8" = Posit8ToQuire8(Id),
        "quire8->posit8" = Quire8ToPosit8(Id),

        // 16-bit posit numbers

        "+.p16" = Posit16Add([Id; 2]),
        "neg.p16" = Posit16Neg(Id),
        "-.p16" = Posit16Sub([Id; 2]),
        "*.p16" = Posit16Mul([Id; 2]),
        "/.p16" = Posit16Div([Id; 2]),
        "sqrt.p16" = Posit16Sqrt(Id),
        "<.p16" = Posit16Lt([Id; 2]),
        ">.p16" = Posit16Gt([Id; 2]),
        "<=.p16" = Posit16Lte([Id; 2]),
        ">=.p16" = Posit16Gte([Id; 2]),
        "real->posit16" = RealToPosit16(Id),
        "posit16->real" = Posit16ToReal(Id),
        "real->quire16" = RealToQuire16(Id),
        "quire16->real" = Quire16ToReal(Id),
        "quire16-mul-add" = Quire16ToReal([Id; 3]),
        "quire16-mul-sub" = Quire16ToReal([Id; 3]),
        "posit16->quire16" = Posit16ToQuire16(Id),
        "quire16->posit16" = Quire16ToPosit16(Id),

        // 32-bit posit numbers

        "+.p32" = Posit32Add([Id; 2]),
        "neg.p32" = Posit32Neg(Id),
        "-.p32" = Posit32Sub([Id; 2]),
        "*.p32" = Posit32Mul([Id; 2]),
        "/.p32" = Posit32Div([Id; 2]),
        "sqrt.p32" = Posit32Sqrt(Id),
        "<.p32" = Posit32Lt([Id; 2]),
        ">.p32" = Posit32Gt([Id; 2]),
        "<=.p32" = Posit32Lte([Id; 2]),
        ">=.p32" = Posit32Gte([Id; 2]),
        "real->posit32" = RealToPosit32(Id),
        "posit32->real" = Posit32ToReal(Id),
        "real->quire32" = RealToQuire32(Id),
        "quire32->real" = Quire32ToReal(Id),
        "quire32-mul-add" = Quire32ToReal([Id; 3]),
        "quire32-mul-sub" = Quire32ToReal([Id; 3]),
        "posit32->quire32" = Posit32ToQuire32(Id),
        "quire32->posit32" = Quire32ToPosit32(Id),

        Constant(Constant),
        Variable(egg::Symbol),
    }
}

pub struct ConstantFold {
    pub constant_fold: bool,
    pub prune: bool,
}

impl Default for ConstantFold {
    fn default() -> Self {
        Self {
            constant_fold: true,
            prune: true,
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
            Math::Add([a, b]) => Some(x(a)? + x(b)?),
            Math::Sub([a, b]) => Some(x(a)? - x(b)?),
            Math::Mul([a, b]) => Some(x(a)? * x(b)?),
            Math::Div([a, b]) => {
                if x(b)?.is_zero() {
                    None
                } else {
                    Some(x(a)? / x(b)?)
                }
            }
            Math::Neg(a) => Some(-x(a)?.clone()),
            Math::Pow([a, b]) => {
                if x(b)?.is_integer() {
                    Some(Pow::pow(x(a)?, x(b)?.to_integer()))
                } else {
                    None
                }
            }
            Math::Sqrt(a) => {
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
            Math::Fabs(a) => Some(x(a)?.clone().abs()),
            Math::Floor(a) => Some(x(a)?.floor()),
            Math::Ceil(a) => Some(x(a)?.ceil()),
            Math::Round(a) => Some(x(a)?.round()),
            // Math::RealToPosit(a) => result(x(0)?),
            _ => None,
        }
    }

    fn merge(&self, to: &mut Self::Data, from: Self::Data) -> bool {
        if to.is_none() && from.is_some() {
            *to = from;
            true
        } else {
            false
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
