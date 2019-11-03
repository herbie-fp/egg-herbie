extern crate libc;

use egg::{
    define_term,
    egraph::{EClass, EGraph},
    expr::{Expr, Language, Name, RecExpr},
    parse::ParsableLanguage,
};

use num_traits::{Zero};
use num_rational::{Ratio, BigRational};

pub type MathEGraph<M = Meta> = egg::egraph::EGraph<Math, M>;

mod rules;
pub use rules::rules;

use std::ffi::{CStr, CString};
use std::mem::transmute;
use std::os::raw::c_char;

unsafe fn cstring_to_recexpr(c_string: *const c_char) -> Option<RecExpr<Math>> {
    let bytes = CStr::from_ptr(c_string).to_bytes();
    let string_result = std::str::from_utf8(bytes);
    match string_result {
        Ok(expr_string) => {
            let parse_result = Math::parse_expr(expr_string);
            match parse_result {
                Ok(rec_expr) => Some(rec_expr),
                Err(_error) => None,
            }
        }
        Err(_error) => None,
    }
}

// I had to add $(rustc --print sysroot)/lib to LD_LIBRARY_PATH to get linking to work after installing rust with rustup
#[no_mangle]
pub unsafe extern "C" fn egraph_create() -> *mut EGraph<Math, Meta> {
    let egraph: EGraph<Math, Meta> = Default::default();

    Box::into_raw(Box::new(egraph))
}

#[no_mangle]
pub unsafe extern "C" fn egraph_destroy(egraph_ptr: *mut EGraph<Math, Meta>) {
    let _egraph_box: Box<EGraph<Math, Meta>> = transmute(egraph_ptr);
    // Drop
}

#[no_mangle]
pub unsafe extern "C" fn egraph_addresult_destroy(addresult_ptr: *mut EGraphAddResult) {
    let _addres_box: Box<EGraphAddResult> = transmute(addresult_ptr);
    // Drop
}

// a struct to report failure if the add fails
#[repr(C)]
pub struct EGraphAddResult {
    id: u32,
    successp: bool,
}

#[no_mangle]
pub unsafe extern "C" fn egraph_add_expr(
    egraph_ptr: *mut EGraph<Math, Meta>,
    expr: *const c_char,
) -> *mut EGraphAddResult {
    let egraph = &mut *egraph_ptr;
    let parsed_expr = cstring_to_recexpr(expr);

    let result = match parsed_expr {
        Some(rec_expr) => EGraphAddResult {
            id: egraph.add_expr(&rec_expr),
            successp: true,
        },
        None => EGraphAddResult {
            id: 0,
            successp: false,
        },
    };
    Box::into_raw(Box::new(result))
}

#[no_mangle]
pub unsafe extern "C" fn egraph_run_rules(
    egraph_ptr: *mut EGraph<Math, Meta>,
    iters: u32,
    limit: u32,
) {
    let egraph = &mut *egraph_ptr;
    run_rules(egraph, iters, limit);
}

#[no_mangle]
pub unsafe extern "C" fn egraph_get_simplest(
    egraph_ptr: *mut EGraph<Math, Meta>,
    node_id: u32,
) -> *const c_char {
    let egraph = &mut *egraph_ptr;

    let best = &egraph[node_id].metadata.best;

    let best_str = CString::new(best.to_sexp().to_string()).unwrap();
    let best_str_pointer = best_str.as_ptr();
    std::mem::forget(best_str);
    best_str_pointer
}

fn run_rules(egraph: &mut EGraph<Math, Meta>, iters: u32, limit: u32) {
    let rules = rules();

    for _i in 0..iters {
        let size_before = egraph.total_size();
        let mut matches = Vec::new();
        for (_name, list) in rules.iter() {
            for rule in list {
                let ms = rule.search(&egraph);
                if !ms.is_empty() {
                    matches.push(ms);
                }
                // rule.run(&mut egraph);
                // egraph.rebuild();
            }
        }

        for m in matches {
            m.apply_with_limit(egraph, limit as usize);
            if egraph.total_size() > limit as usize {
                egraph.rebuild();
                return;
            }
        }

        if size_before >= egraph.total_size() {
            egraph.rebuild();
            return;
        }

        egraph.rebuild();
    }
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
        Math::Pow => None, // a(0)?.powf(a(1)?),
        Math::Exp => None, // a(0)?.exp(),
        Math::Log => None, // a(0)?.ln(),
        Math::Sqrt => {
            None
            // unimplemented!()
            // if let Some(sqrt) = args[0].sqrt() {
            //     #[allow(clippy::float_cmp)]
            //     let is_int = sqrt == sqrt.trunc();
            //     if is_int {
            //         sqrt.into()
            //     } else {
            //         None
            //     }
            // } else {
            //     None
            // }
        }
        // Math::Cbrt => {
        //     if let Some(cbrt) = args[0].to_f64().map(f64::cbrt) {
        //         #[allow(clippy::float_cmp)]
        //         let is_int = cbrt == cbrt.trunc();
        //         if is_int {
        //             cbrt.into()
        //         } else {
        //             None
        //         }
        //     } else {
        //         None
        //     }
        // }
        Math::Fabs => {
            if a(0)? < Ratio::from_integer(Zero::zero()) {
                Some(-a(0)?)
            } else {
                Some(a(0)?)
            }
        }
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
        let expr = {
            let const_args: Option<Vec<Constant>> = expr
                .children
                .iter()
                .map(|meta| match meta.best.as_ref().op {
                    Math::Constant(ref c) => Some(c.clone()),
                    _ => None,
                })
                .collect();

            const_args
                .and_then(|a| eval(expr.op.clone(), &a))
                .map(|c| Expr::unit(Math::Constant(c)))
                .unwrap_or(expr)
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
