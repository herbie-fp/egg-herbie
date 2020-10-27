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


pub struct AstSizeDifferentiated;
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
            Math::Lim([_p, _original, _num, denom, _var, _value, _count]) => {
                if costs(*denom).is_zero() {
                    high_price / 2  // Has the potential to resolve in simplification phase of differentiation
                } else {
                    high_price / 4
                }
            },

            Math::Div(_) => 30, // division expensive because it's bad for taylor
            Math::Tan(_) => 30, // tan has asymptotes 

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
        "tan" = Tan([Id; 2]),

        "subst" = Subst([Id; 4]),
        "d" = D([Id; 3]),
        "lim" = Lim([Id; 7]),

        Constant(Constant),
        Symbol(egg::Symbol),
        Other(egg::Symbol, Vec<Id>),
    }
}

impl Math {
    pub fn map_children<F: FnMut(&Id) -> Id>(&self, mut f: F) -> Self {
        match self {
            Math::Add([a, b, c]) => Math::Add([f(a), f(b), f(c)]),
            Math::Sub([a, b, c]) => Math::Sub([f(a), f(b), f(c)]),
            Math::Mul([a, b, c]) => Math::Mul([f(a), f(b), f(c)]),
            Math::Div([a, b, c]) => Math::Div([f(a), f(b), f(c)]),
            Math::Tan([a, b])   => Math::Tan([f(a), f(b)]),
            Math::TryDiv([a, b, c, d, e]) => Math::TryDiv([f(a), f(b), f(c), f(d), f(e)]),
            Math::Pow([a, b, c]) => Math::Pow([f(a), f(b), f(c)]),
            Math::Neg([a, b]) => Math::Neg([f(a), f(b)]),
            Math::Sqrt([a, b]) => Math::Sqrt([f(a), f(b)]),
            Math::Fabs([a, b]) => Math::Fabs([f(a), f(b)]),
            Math::Ceil([a, b]) => Math::Ceil([f(a), f(b)]),
            Math::Floor([a, b]) => Math::Floor([f(a), f(b)]),
            Math::Round([a, b]) => Math::Round([f(a), f(b)]),
            Math::Subst([a, b, c, d]) => Math::Subst([f(a), f(b), f(c), f(d)]),
            Math::D([a, b, c]) => Math::D([f(a), f(b), f(c)]),
            Math::Lim([a, b, c, d, e, g, h]) => Math::Lim([f(a), f(b), f(c), f(d), f(e), f(g), f(h)]),
            Math::Constant(c) => Math::Constant(c.clone()),
            Math::Symbol(s) => Math::Symbol(*s),
            Math::Other(s, v) => Math::Other(*s, v.into_iter().map(f).collect()),
        }
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

#[derive(Debug, Clone, PartialEq)]
pub struct Metadata {
    pub folddata: Option<FoldData>,
    pub fullyderived: bool,
}

impl Display for FoldData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        match *self {
            FoldData::Const(ref c) => ::std::fmt::Display::fmt(c, f),
            FoldData::Var(ref s) => ::std::fmt::Display::fmt(s, f),
        }
    }
}


impl Display for Metadata {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        match &self.folddata {
            Some(data) => write!(f, "folddata: {} \n fullyderived: {}", data, self.fullyderived),
            _ => write!(f, "folddata: None \n fullyderived: {}", self.fullyderived)
        }
        
    }
}

fn is_constant_or_different_variable(egraph: &EGraph, cid: &Id, vid: &Id) -> bool {
    let get = |id: &Id| egraph[*id].data.folddata.as_ref();
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

fn is_not_subst(egraph: &EGraph, var: &Id) -> bool {
    !egraph[*var]
            .nodes
            .iter()
            .any(|n| match n {
                    Math::Subst(_) => true,
                    _ => false
                })
}

fn constant_fold(egraph: &EGraph, enode: &Math) -> Option<FoldData> {
    if !egraph.analysis.constant_fold {
        return None;
    }

    let x = |id: &Id| {
        let data = egraph[*id].data.folddata.as_ref();
        match data {
            Some(fdata) => match fdata {
                FoldData::Const(c) => Some(c),
                _ => None,
            },
            None => None,
        }
    };

    let is_zero = |id: &Id| {
        let data = egraph[*id].data.folddata.as_ref();
        match data {
            Some(fdata) => match fdata {
                FoldData::Const(c) => c.is_zero(),
                _ => false,
            },
            None => false,
        }
    };

    let has_data = |id: &Id| {
        match egraph[*id].data.folddata.as_ref() {
            Some(_) => true,
            _ => false,
        }
    };

    let ret_c = |c: Constant| Some(FoldData::Const(c));

    let ret_var = |c: Symbol| Some(FoldData::Var(c));

    let ret = |c: Option<FoldData>| c;

    match enode {
        Math::Constant(c) => ret_c(c.clone()),
        Math::Symbol(s) => ret_var(s.clone()),

        // real
        Math::Add([_p, a, b]) => ret_c(x(a)? + x(b)?),
        Math::Sub([_p, a, b]) => ret_c(x(a)? - x(b)?),
        Math::Mul([_p, a, b]) => {
            // check has_data when subst so we don't multiply (* 0 (trydiv 1 0))
            if (is_zero(a) && (has_data(b) || is_not_subst(egraph, b)))
                || (is_zero(b) && (has_data(a) || is_not_subst(egraph, a))) {
                ret_c(Ratio::new(BigInt::from(0), BigInt::from(1)))
            } else {
                ret_c(x(a)? * x(b)?)
            }
        }
        Math::Div([_p, a, b]) => {
            if is_zero(b) {
                ret(None)
            } else {
                ret_c(x(a)? / x(b)?)
            }
        }
        Math::TryDiv([_p, _originalexpr, a, b, _hist]) => {
            if is_zero(b) {
                ret(None)
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
                ret(None)
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
                    ret(None)
                }
            } else {
                ret(None)
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
                ret(None)
            }
        }

        // constant fold limits
        Math::Lim([_p, _original, num, denom, _var, _value, count]) => {
            if x(denom)?.is_zero() {
                ret(None)
            } else {
                if x(count)?.is_zero() {
                    ret_c(x(num)? / x(denom)?)
                } else {
                    ret(None)
                }
            }
        }

        _ => ret(None),
    }
}

impl Analysis<Math> for ConstantFold {
    type Data = Metadata;
    fn make(egraph: &EGraph, enode: &Math) -> Self::Data {
        let op_derived =
            match enode {
                Math::D(_) => false,
                Math::Subst(_) => false,
                // check division by zero
                Math::TryDiv([_p, _oldexpr, _num, denom, _hist]) => {
                    if egraph[*denom].data.folddata.as_ref() == Some(&FoldData::Const(Ratio::new(BigInt::from(0), BigInt::from(1)))) {
                        false
                    } else {
                        true
                    }
                },
                Math::Lim(_) => false,
                _ => true,
            };
        let mut children_derived: bool = true;

        let finder = |child: &Id| {
            if !egraph[*child].data.fullyderived {
                children_derived = false;
            }
            *child
        };
        enode.map_children(finder);
        

        Metadata {
            folddata: constant_fold(egraph, enode),
            fullyderived: children_derived && op_derived
        }
    }

    fn merge(&self, to: &mut Self::Data, from: Self::Data) -> bool {
        let oldderived = to.fullyderived;
        let derived = to.fullyderived || from.fullyderived;
        let changed = match (&to.folddata, from.folddata) {
            (None, None) => false,
            (Some(_), None) => false, // no update needed
            (None, Some(ref c)) => {
                to.folddata = Some(c.clone());
                true
            }
            (Some(a), Some(ref b)) => {    
                if a != b && !self.unsound.swap(true, Ordering::SeqCst) {
                    println!("Bad merge detected: {} != {}", a, b);
                    log::warn!("Bad merge detected: {} != {}", a, b);
                }
                false
            }
        };
        to.fullyderived = derived;
        changed || (to.fullyderived != oldderived)
    }

    fn modify(egraph: &mut EGraph, id: Id) {
        if let Some(constant) = egraph[id].data.folddata.clone() {
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

