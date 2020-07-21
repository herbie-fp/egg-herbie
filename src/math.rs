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
        "TRUE" = True,
        "FALSE" = False,
        "E" = E,
        "PI" = Pi,
        "INFINITY" = Infinity,
        "NAN" = Nan,

        // parameterized constants: binary64
        "E.f64" = Ef64,
        "PI.f64" = Pif64,
        "INFINITY.f64" = Infinityf64,
        "NAN.f64" = Nanf64,

        // parameterized constants: binary32
        "E.f32" = Ef32,
        "PI.f32" = Pif32,
        "INFINITY.f32" = Infinityf32,
        "NAN.f32" = Nanf32,

        // logical operators
        "if" = If([Id; 3]),
        "not" = Not(Id),
        "and" = And([Id; 2]),
        "or" = Or([Id; 2]),

        // unparameterized, (Herbie <=1.4)

        "<" = Less([Id; 2]),
        ">" = Greater([Id; 2]),
        "<=" = LessEq([Id; 2]),
        ">=" = GreaterEq([Id; 2]),

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

        // binary64

        "<.f64" = Lessf64([Id; 2]),
        ">.f64" = Greaterf64([Id; 2]),
        "<=.f64" = LessEqf64([Id; 2]),
        ">=.f64" = GreaterEqf64([Id; 2]),

        "erf.f64" = Erff64(Id),
        "erfc.f64" = Erfcf64(Id),
        "tgamma.f64" = Tgammaf64(Id),
        "lgamma.f64" = Lgammaf64(Id),
        "ceil.f64" = Ceilf64(Id),
        "floor.f64" = Floorf64(Id),
        "fmod.f64" = Fmodf64([Id; 2]),
        "remainder.f64" = Remainderf64([Id; 2]),
        "fmax.f64" = Fmaxf64([Id; 2]),
        "fmin.f64" = Fminf64([Id; 2]),
        "fdim.f64" = Fdimf64([Id; 2]),
        "copysign.f64" = Copysignf64(Id),
        "trunc.f64" = Truncf64(Id),
        "round.f64" = Roundf64(Id),
        "nearbyint.f64" = NearbyIntf64(Id),

        "+.f64" = Addf64([Id; 2]),
        "-.f64" = Subf64([Id; 2]),
        "*.f64" = Mulf64([Id; 2]),
        "/.f64" = Divf64([Id; 2]),
        "pow.f64" = Powf64([Id; 2]),
        "neg.f64" = Negf64(Id),
        "exp.f64" = Expf64(Id),
        "exp2.f64" = Exp2f64(Id),
        "log.f64" = Logf64(Id),
        "sqrt.f64" = Sqrtf64(Id),
        "cbrt.f64" = Cbrtf64(Id),
        "fabs.f64" = Fabsf64(Id),
        "sin.f64" = Sinf64(Id),
        "cos.f64" = Cosf64(Id),
        "tan.f64" = Tanf64(Id),
        "asin.f64" = Asinf64(Id),
        "acos.f64" = Acosf64(Id),
        "atan.f64" = Atanf64(Id),
        "atan2.f64" = Atan2f64([Id; 2]),
        "sinh.f64" = Sinhf64(Id),
        "cosh.f64" = Coshf64(Id),
        "tanh.f64" = Tanhf64(Id),
        "asinh.f64" = Asinhf64(Id),
        "acosh.f64" = Acoshf64(Id),
        "atanh.f64" = Atanhf64(Id),

        "fma.f64" = Fmaf64([Id; 3]),
        "log1p.f64" = Log1pf64(Id),
        "log10.f64" = Log10f64(Id),
        "log2.f64" = Log2f64(Id),
        "expm1.f64" = Expm1f64(Id),
        "hypot.f64" = Hypotf64([Id; 2]),

        // binary32

        "<.f32" = Lessf32([Id; 2]),
        ">.f32" = Greaterf32([Id; 2]),
        "<=.f32" = LessEqf32([Id; 2]),
        ">=.f32" = GreaterEqf32([Id; 2]),

        "erf.f32" = Erff32(Id),
        "erfc.f32" = Erfcf32(Id),
        "tgamma.f32" = Tgammaf32(Id),
        "lgamma.f32" = Lgammaf32(Id),
        "ceil.f32" = Ceilf32(Id),
        "floor.f32" = Floorf32(Id),
        "fmod.f32" = Fmodf32([Id; 2]),
        "remainder.f32" = Remainderf32([Id; 2]),
        "fmax.f32" = Fmaxf32([Id; 2]),
        "fmin.f32" = Fminf32([Id; 2]),
        "fdim.f32" = Fdimf32([Id; 2]),
        "copysign.f32" = Copysignf32(Id),
        "trunc.f32" = Truncf32(Id),
        "round.f32" = Roundf32(Id),
        "nearbyint.f32" = NearbyIntf32(Id),

        "+.f32" = Addf32([Id; 2]),
        "-.f32" = Subf32([Id; 2]),
        "*.f32" = Mulf32([Id; 2]),
        "/.f32" = Divf32([Id; 2]),
        "pow.f32" = Powf32([Id; 2]),
        "neg.f32" = Negf32(Id),
        "exp.f32" = Expf32(Id),
        "exp2.f32" = Exp2f32(Id),
        "log.f32" = Logf32(Id),
        "sqrt.f32" = Sqrtf32(Id),
        "cbrt.f32" = Cbrtf32(Id),
        "fabs.f32" = Fabsf32(Id),
        "sin.f32" = Sinf32(Id),
        "cos.f32" = Cosf32(Id),
        "tan.f32" = Tanf32(Id),
        "asin.f32" = Asinf32(Id),
        "acos.f32" = Acosf32(Id),
        "atan.f32" = Atanf32(Id),
        "atan2.f32" = Atan2f32([Id; 2]),
        "sinh.f32" = Sinhf32(Id),
        "cosh.f32" = Coshf32(Id),
        "tanh.f32" = Tanhf32(Id),
        "asinh.f32" = Asinhf32(Id),
        "acosh.f32" = Acoshf32(Id),
        "atanh.f32" = Atanhf32(Id),

        "fma.f32" = Fmaf32([Id; 3]),
        "log1p.f32" = Log1pf32(Id),
        "log10.f32" = Log10f32(Id),
        "log2.f32" = Log2f32(Id),
        "expm1.f32" = Expm1f32(Id),
        "hypot.f32" = Hypotf32([Id; 2]),

        // Complex numbers

        "re" = Re(Id),
        "im" = Im(Id),
        "complex" = Complex([Id; 2]),
        "conj" = Conj(Id),
        "+.c" = Addc([Id; 2]),
        "-.c" = Subc([Id; 2]),
        "neg.c" = Negc(Id),
        "/.c" = Divc([Id; 2]),
        "*.c" = Mulc([Id; 2]),

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
        "binary64->posit8" = F64ToPosit8(Id),
        "posit8->binary64" = Posit8ToF64(Id),
        "binary64->quire8" = F64ToQuire8(Id),
        "quire8->binary64" = Quire8ToF64(Id),
        "quire8-mul-add" = Quire8MulAdd([Id; 3]),
        "quire8-mul-sub" = Quire8MulSub([Id; 3]),
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
        "binary64->posit16" = F64ToPosit16(Id),
        "posit16->binary64" = Posit16ToF64(Id),
        "binary64->quire16" = F64ToQuire16(Id),
        "quire16->binary64" = Quire16ToF64(Id),
        "quire16-mul-add" = Quire16MulAdd([Id; 3]),
        "quire16-mul-sub" = Quire16MulSub([Id; 3]),
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
        "binary64->posit32" = F64ToPosit32(Id),
        "posit32->binary64" = Posit32ToF64(Id),
        "binary64->quire32" = F64ToQuire32(Id),
        "quire32->binary64" = Quire32ToF64(Id),
        "quire32-mul-add" = Quire32MulAdd([Id; 3]),
        "quire32-mul-sub" = Quire32MulSub([Id; 3]),
        "posit32->quire32" = Posit32ToQuire32(Id),
        "quire32->posit32" = Quire32ToPosit32(Id),

        Constant(Constant),
        Variable(egg::Symbol),
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

            // binary64
            Math::Addf64([a, b]) => Some(x(a)? + x(b)?),
            Math::Subf64([a, b]) => Some(x(a)? - x(b)?),
            Math::Mulf64([a, b]) => Some(x(a)? * x(b)?),
            Math::Divf64([a, b]) => {
                if x(b)?.is_zero() {
                    None
                } else {
                    Some(x(a)? / x(b)?)
                }
            }
            Math::Negf64(a) => Some(-x(a)?.clone()),
            Math::Powf64([a, b]) => {
                if x(b)?.is_integer() {
                    Some(Pow::pow(x(a)?, x(b)?.to_integer()))
                } else {
                    None
                }
            }
            Math::Sqrtf64(a) => {
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
            Math::Fabsf64(a) => Some(x(a)?.clone().abs()),
            Math::Floorf64(a) => Some(x(a)?.floor()),
            Math::Ceilf64(a) => Some(x(a)?.ceil()),
            Math::Roundf64(a) => Some(x(a)?.round()),

            // binary32
            Math::Addf32([a, b]) => Some(x(a)? + x(b)?),
            Math::Subf32([a, b]) => Some(x(a)? - x(b)?),
            Math::Mulf32([a, b]) => Some(x(a)? * x(b)?),
            Math::Divf32([a, b]) => {
                if x(b)?.is_zero() {
                    None
                } else {
                    Some(x(a)? / x(b)?)
                }
            }
            Math::Negf32(a) => Some(-x(a)?.clone()),
            Math::Powf32([a, b]) => {
                if x(b)?.is_integer() {
                    Some(Pow::pow(x(a)?, x(b)?.to_integer()))
                } else {
                    None
                }
            }
            Math::Sqrtf32(a) => {
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
            Math::Fabsf32(a) => Some(x(a)?.clone().abs()),
            Math::Floorf32(a) => Some(x(a)?.floor()),
            Math::Ceilf32(a) => Some(x(a)?.ceil()),
            Math::Roundf32(a) => Some(x(a)?.round()),

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
