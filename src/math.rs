use egg::*;

use std::fmt::Display;
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

struct AstSizeDifferentiated;
impl CostFunction<Math> for AstSizeDifferentiated {
    type Cost = usize;
    fn cost<C>(&mut self, enode: &Math, mut costs: C) -> Self::Cost
    where
        C: FnMut(Id) -> Self::Cost,
    {
        let high_price = 10000000;
        let op_cost = match enode {
            Math::D(_) => high_price,
            Math::Subst(_) => high_price,
            Math::Constant(c) => {
                if c.is_zero() {
                    0
                } else {
                    1
                }
            },
            // check division by zero
            Math::TryDiv([_p, _oldexpr, _num, denom, _hist]) => {
                if costs(*denom).is_zero() {
                    high_price*high_price // not useful at all
                } else {
                    1
                }
            },
            Math::Lim([_p, _originalnum, _originaldenom, _num, denom, _var, _value]) => {
                if costs(*denom).is_zero() {
                    high_price / 2  // Has the potential to resolve in simplification phase of differentiation
                } else {
                    high_price / 4
                }
            },

            _ => 1,
        };
        let rest_cost = match enode {
            Math::TryDiv([_p, _oldexpr, num, denom, _hist]) => {
                costs(*num) + costs(*denom)
            },
            _ => enode.fold(0, |sum, id| sum + costs(id))
        };

        op_cost + rest_cost
    }
}

impl IterationData<Math, ConstantFold> for IterData {
    fn make(runner: &Runner) -> Self {
        let mut extractor = Extractor::new(&runner.egraph, AstSizeDifferentiated);
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

        // constant-folding operators

        "+" = Add([Id; 3]),
        "-" = Sub([Id; 3]),
        "*" = Mul([Id; 3]),
        "/" = Div([Id; 3]),
        "try-/" = TryDiv([Id; 5]),
        "pow" = Pow([Id; 3]),
        "neg" = Neg([Id; 2]),
        "sqrt" = Sqrt([Id; 2]),
        "fabs" = Fabs([Id; 2]),
        "ceil" = Ceil([Id; 2]),
        "floor" = Floor([Id; 2]),
        "round" = Round([Id; 2]),

        "subst" = Subst([Id; 4]),
        "d" = D([Id; 3]),
        "lim" = Lim([Id; 7]),

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

#[derive(Debug, Clone, PartialEq)]
pub enum FoldData {
    Const(Constant),
    Var(Symbol),
}

impl Display for FoldData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        match *self {
            FoldData::Const(ref c) => ::std::fmt::Display::fmt(c, f),
            FoldData::Var(ref s) => ::std::fmt::Display::fmt(s, f),
        }
    }
}

fn is_constant_or_different_variable(egraph: &EGraph, cid: &Id, vid: &Id) -> bool {
    let get = |id: &Id| egraph[*id].data.as_ref();
    let d_in = match get(vid) {
        Some(FoldData::Var(v)) => v,
        _ => return false,
    };
    match get(cid) {
        Some(FoldData::Var(v)) => {
            v != d_in // test if different variables
        }
        Some(FoldData::Const(_)) => true,
        _ => false,
    }
}

impl Analysis<Math> for ConstantFold {
    type Data = Option<FoldData>;
    fn make(egraph: &EGraph, enode: &Math) -> Self::Data {
        if !egraph.analysis.constant_fold {
            return None;
        }

        let x = |id: &Id| {
            let data = egraph[*id].data.as_ref();
            match data {
                Some(fdata) => match fdata {
                    FoldData::Const(c) => Some(c),
                    FoldData::Var(_) => None,
                },
                None => None,
            }
        };

        let is_zero = |id: &Id| {
            match x(id) {
                Some(c) => c.is_zero(),
                _ => false
            }
        };

        let ret_c = |c: Constant| Some(FoldData::Const(c));
        let ret_var = |c: Symbol| Some(FoldData::Var(c));
        match enode {
            Math::Constant(c) => ret_c(c.clone()),
            Math::Symbol(s) => ret_var(s.clone()),

            // real
            Math::Add([_p, a, b]) => ret_c(x(a)? + x(b)?),
            Math::Sub([_p, a, b]) => ret_c(x(a)? - x(b)?),
            Math::Mul([_p, a, b]) => {
                if is_zero(a) || is_zero(b) {
                    ret_c(Ratio::new(BigInt::from(0), BigInt::from(1)))
                } else {
                    ret_c(x(a)? * x(b)?)
                }
            }
            Math::Div([_p, a, b]) => {
                if is_zero(b) {
                    None
                } else {
                    ret_c(x(a)? / x(b)?)
                }
            }
            Math::TryDiv([_p, _originalexpr, a, b, _hist]) => {
                if is_zero(b) {
                    None
                } else {
                    ret_c(x(a)? / x(b)?)
                }
            }
            Math::Neg([_p, a]) => ret_c(-x(a)?.clone()),
            Math::Pow([_p, a, b]) => {
                if  is_zero(b) && !is_zero(a) {
                    ret_c(Ratio::new(BigInt::from(1), BigInt::from(1)))
                } else if  is_zero(a) && !is_zero(b) {
                    ret_c(Ratio::new(BigInt::from(0), BigInt::from(1)))
                } else if x(b)?.is_integer()
                    && !(x(a)?.is_zero() && (x(b)?.is_zero() || x(b)?.is_negative()))
                {
                    ret_c(Pow::pow(x(a)?, x(b)?.to_integer()))
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
                        ret_c(Ratio::new(s1, s2))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Math::Fabs([_p, a]) => ret_c(x(a)?.clone().abs()),
            Math::Floor([_p, a]) => ret_c(x(a)?.floor()),
            Math::Ceil([_p, a]) => ret_c(x(a)?.ceil()),
            Math::Round([_p, a]) => ret_c(x(a)?.round()),

            // derivative rule for deriving constant or different variable
            Math::D([_p, a, v]) => {
                if is_constant_or_different_variable(egraph, a, v) {
                    ret_c(Ratio::new(BigInt::from(0), BigInt::from(1)))
                } else {
                    None
                }
            }

            // substitution rule for substituting constant or different variable
            Math::Subst([_p, expr, var, _value]) => {
                if is_constant_or_different_variable(egraph, expr, var) {
                    Some(egraph[*expr].data.as_ref()?.clone())
                } else {
                    None
                }
            }

            // constant fold limits
            Math::Lim([_p, _originalnum, _originaldenom, num, denom, _var, _value]) => {
                if x(denom)?.is_zero() {
                    None
                } else {
                    ret_c(x(num)? / x(denom)?)
                }
            }

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
                        println!("Bad merge detected: {} != {}", a, b);
                        log::warn!("Bad merge detected: {} != {}", a, b);
                    }
                }
                false
            }
        }
    }

    fn modify(egraph: &mut EGraph, id: Id) {
        if let Some(constant) = egraph[id].data.clone() {
            let added = match constant {
                FoldData::Const(c) => egraph.add(Math::Constant(c)),
                FoldData::Var(v) => egraph.add(Math::Symbol(v)),
            };
            let (id, _) = egraph.union(id, added);
            if egraph.analysis.prune {
                egraph[id].nodes.retain(|n| n.is_leaf())
            }
        }
    }
}
