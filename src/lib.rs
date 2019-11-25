extern crate libc;
#[macro_use]
extern crate lazy_static;
pub mod eggmath;
pub mod rules;

use crate::eggmath::{Math, Meta, set_constant_folding};

use egg::{egraph::EGraph, expr::RecExpr, parse::ParsableLanguage, pattern::Rewrite};

pub type MathEGraph<M = Meta> = egg::egraph::EGraph<Math, M>;

use std::ffi::{CStr, CString};
use std::mem::transmute;
use std::os::raw::c_char;
use std::slice;

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

// a struct for loading rules from external source
#[repr(C)]
pub struct FFIRule {
    name: *const c_char,
    left: *const c_char,
    right: *const c_char,
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

// todo don't just unwrap, also make sure the rules are validly parsed
unsafe fn ffirule_to_tuple(rule_ptr: *mut FFIRule) -> (String, String, String) {
    let rule = &mut *rule_ptr;
    let bytes1 = CStr::from_ptr(rule.name).to_bytes();
    let string_result1 = String::from_utf8(bytes1.to_vec()).unwrap();
    let bytes2 = CStr::from_ptr(rule.left).to_bytes();
    let string_result2 = String::from_utf8(bytes2.to_vec()).unwrap();
    let bytes3 = CStr::from_ptr(rule.right).to_bytes();
    let string_result3 = String::from_utf8(bytes3.to_vec()).unwrap();
    (string_result1, string_result2, string_result3)
}

#[no_mangle]
pub unsafe extern "C" fn egraph_run_iter(
    egraph_ptr: *mut EGraph<Math, Meta>,
    limit: u32,
    rules_array_ptr: *const *mut FFIRule,
    is_constant_folding_enabled: bool,
    rules_array_length: u32,
) {
    set_constant_folding(is_constant_folding_enabled);

    let length: usize = rules_array_length as usize;
    let egraph = &mut *egraph_ptr;

    let ffi_rules: &[*mut FFIRule] = slice::from_raw_parts(rules_array_ptr, length);
    let mut ffi_tuples: Vec<(&str, &str, &str)> = vec![];
    let mut ffi_strings: Vec<(String, String, String)> = vec![];
    for ffi_rule in ffi_rules.iter() {
        let str_tuple = ffirule_to_tuple(*ffi_rule);
        ffi_strings.push(str_tuple);
    }

    for ffi_string in ffi_strings.iter() {
        ffi_tuples.push((&ffi_string.0, &ffi_string.1, &ffi_string.2));
    }

    let rules: Vec<Rewrite<Math, Meta>> = rules::mk_rules(&ffi_tuples);

    run_rules_once(egraph, limit, rules)
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

#[no_mangle]
pub unsafe extern "C" fn egraph_get_cost(egraph_ptr: *mut EGraph<Math, Meta>, node_id: u32) -> u32 {
    let egraph = &mut *egraph_ptr;
    let best = &egraph[node_id].metadata.cost;
    *best as u32
}

#[no_mangle]
pub unsafe extern "C" fn egraph_get_size(egraph_ptr: *mut EGraph<Math, Meta>) -> u32 {
    let egraph = &mut *egraph_ptr;
    egraph.total_size() as u32
}

fn run_rules_once(egraph: &mut EGraph<Math, Meta>, limit: u32, rules: Vec<Rewrite<Math, Meta>>) {
    let mut matches = Vec::new();
    for rule in rules.iter() {
        let ms = rule.search(&egraph);
        if !ms.is_empty() {
            matches.push(ms);
        }
    }

    for m in matches {
        m.apply_with_limit(egraph, limit as usize);
        if egraph.total_size() > limit as usize {
            break;
        }
    }
    egraph.rebuild();
}
