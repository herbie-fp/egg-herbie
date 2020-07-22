use crate::math::*;

use indexmap::IndexMap;
use std::str::FromStr;

pub fn mk_rules(tuples: &[(&str, &str, &str)]) -> Vec<Rewrite> {
    tuples
        .iter()
        .map(|(name, left, right)| {
            let left = Pattern::from_str(left).unwrap();
            let right = Pattern::from_str(right).unwrap();
            Rewrite::new(*name, *name, left, right).unwrap()
        })
        .collect()
}

pub fn math_rules() -> IndexMap<&'static str, Vec<Rewrite>> {
    let mut m = IndexMap::new();
    let mut add = |name, rules| {
        if m.contains_key(name) {
            panic!("{} was already there", name);
        }
        m.insert(name, mk_rules(rules));
    };

    add(
        "erf-rules",
        &[
            (
                "erf-odd",
                "(erf.f64 (neg.f64 ?x))",
                "(neg.f64 (erf.f64 ?x))",
            ),
            ("erf-erfc", "(erfc.f64 ?x)", "(-.f64 1 (erf.f64 ?x))"),
            ("erfc-erf", "(erf.f64 ?x)", "(-.f64 1 (erfc.f64 ?x))"),
        ],
    );
    add("complex-number-basics",
        &[
            ("real-part","(re (complex ?x ?y))","?x"),
            ("imag-part","(im (complex ?x ?y))","?y"),
            ("complex-add-def","(+.c (complex ?a ?b) (complex ?c ?d))","(complex (+.f64 ?a ?c) (+.f64 ?b ?d))"),
            ("complex-def-add","(complex (+.f64 ?a ?c) (+.f64 ?b ?d))","(+.c (complex ?a ?b) (complex ?c ?d))"),
            ("complex-sub-def","(-.c (complex ?a ?b) (complex ?c ?d))","(complex (-.f64 ?a ?c) (-.f64 ?b ?d))"),
            ("complex-def-sub","(complex (-.f64 ?a ?c) (-.f64 ?b ?d))","(-.c (complex ?a ?b) (complex ?c ?d))"),
            ("complex-neg-def","(neg.c (complex ?a ?b))","(complex (neg.f64 ?a) (neg.f64 ?b))"),
            ("complex-def-neg","(complex (neg.f64 ?a) (neg.f64 ?b))","(neg.c (complex ?a ?b))"),
            ("complex-mul-def","(*.c (complex ?a ?b) (complex ?c ?d))","(complex (-.f64 (*.f64 ?a ?c) (*.f64 ?b ?d)) (+.f64 (*.f64 ?a ?d) (*.f64 ?b ?c)))"),
            ("complex-div-def","(/.c (complex ?a ?b) (complex ?c ?d))","(complex (/.f64 (+.f64 (*.f64 ?a ?c) (*.f64 ?b ?d)) (+.f64 (*.f64 ?c ?c) (*.f64 ?d ?d))) (/.f64 (-.f64 (*.f64 ?b ?c) (*.f64 ?a ?d)) (+.f64 (*.f64 ?c ?c) (*.f64 ?d ?d))))"),
            ("complex-conj-def","(conj (complex ?a ?b))","(complex ?a (neg.f64 ?b))"),
        ],
    );
    add(
        "branch-reduce",
        &[
            ("if-true", "(if TRUE ?x ?y)", "?x"),
            ("if-false", "(if FALSE ?x ?y)", "?y"),
            ("if-same", "(if ?a ?x ?x)", "?x"),
            ("if-not", "(if (not ?a) ?x ?y)", "(if ?a ?y ?x)"),
            (
                "if-if-or",
                "(if ?a ?x (if ?b ?x ?y))",
                "(if (or ?a ?b) ?x ?y)",
            ),
            (
                "if-if-or-not",
                "(if ?a ?x (if ?b ?y ?x))",
                "(if (or ?a (not ?b)) ?x ?y)",
            ),
            (
                "if-if-and",
                "(if ?a (if ?b ?x ?y) ?y)",
                "(if (and ?a ?b) ?x ?y)",
            ),
            (
                "if-if-and-not",
                "(if ?a (if ?b ?y ?x) ?y)",
                "(if (and ?a (not ?b)) ?x ?y)",
            ),
        ],
    );
    add(
        "compare-reduce",
        &[
            ("lt-same", "(<.f64 ?x ?x)", "FALSE"),
            ("gt-same", "(>.f64 ?x ?x)", "FALSE"),
            ("lte-same", "(<=.f64 ?x ?x)", "TRUE"),
            ("gte-same", "(>=.f64 ?x ?x)", "TRUE"),
            ("not-lt", "(not (<.f64 ?x ?y))", "(>=.f64 ?x ?y)"),
            ("not-gt", "(not (>.f64 ?x ?y))", "(<=.f64 ?x ?y)"),
            ("not-lte", "(not (<=.f64 ?x ?y))", "(>.f64 ?x ?y)"),
            ("not-gte", "(not (>=.f64 ?x ?y))", "(<.f64 ?x ?y)"),
        ],
    );
    add(
        "bool-reduce",
        &[
            ("not-true", "(not TRUE)", "FALSE"),
            ("not-false", "(not FALSE)", "TRUE"),
            ("not-not", "(not (not ?a))", "?a"),
            ("not-and", "(not (and ?a ?b))", "(or (not ?a) (not ?b))"),
            ("not-or", "(not (or ?a ?b))", "(and (not ?a) (not ?b))"),
            ("and-true-l", "(and TRUE ?a)", "?a"),
            ("and-true-r", "(and ?a TRUE)", "?a"),
            ("and-false-l", "(and FALSE ?a)", "FALSE"),
            ("and-false-r", "(and ?a FALSE)", "FALSE"),
            ("and-same", "(and ?a ?a)", "?a"),
            ("or-true-l", "(or TRUE ?a)", "TRUE"),
            ("or-true-r", "(or ?a TRUE)", "TRUE"),
            ("or-false-l", "(or FALSE ?a)", "?a"),
            ("or-false-r", "(or ?a FALSE)", "?a"),
            ("or-same", "(or ?a ?a)", "?a"),
        ],
    );
    add(
        "htrig-reduce",
        &[
            ("sinh-def", "(sinh.f64 ?x)", "(/.f64 (-.f64 (exp.f64 ?x) (exp.f64 (neg.f64 ?x))) 2)"),
            ("cosh-def", "(cosh.f64 ?x)", "(/.f64 (+.f64 (exp.f64 ?x) (exp.f64 (neg.f64 ?x))) 2)"),
            (
                "tanh-def1",
                "(tanh.f64 ?x)",
                "(/.f64 (-.f64 (exp.f64 ?x) (exp.f64 (neg.f64 ?x))) (+.f64 (exp.f64 ?x) (exp.f64 (neg.f64 ?x))))",
            ),
            (
                "tanh-def2",
                "(tanh.f64 ?x)",
                "(/.f64 (-.f64 (exp.f64 (*.f64 2 ?x)) 1) (+.f64 (exp.f64 (*.f64 2 ?x)) 1))",
            ),
            (
                "tanh-def3",
                "(tanh.f64 ?x)",
                "(/.f64 (-.f64 1 (exp.f64 (*.f64 -2 ?x))) (+.f64 1 (exp.f64 (*.f64 -2 ?x))))",
            ),
            (
                "sinh-cosh",
                "(-.f64 (*.f64 (cosh.f64 ?x) (cosh.f64 ?x)) (*.f64 (sinh.f64 ?x) (sinh.f64 ?x)))",
                "1",
            ),
            ("sinh-+-cosh", "(+.f64 (cosh.f64 ?x) (sinh.f64 ?x))", "(exp.f64 ?x)"),
            ("sinh---cosh", "(-.f64 (cosh.f64 ?x) (sinh.f64 ?x))", "(exp.f64 (neg.f64 ?x))"),
        ],
    );
    add(
        "trig-reduce-fp-sound-nan",
        &[
            (
                "sin-neg",
                "(sin.f64 (neg.f64 ?x))",
                "(neg.f64 (sin.f64 ?x))",
            ),
            ("cos-neg", "(cos.f64 (neg.f64 ?x))", "(cos.f64 ?x)"),
            (
                "tan-neg",
                "(tan.f64 (neg.f64 ?x))",
                "(neg.f64 (tan.f64 ?x))",
            ),
        ],
    );
    add(
        "trig-reduce-fp-sound",
        &[
            ("sin-0", "(sin.f64 0)", "0"),
            ("cos-0", "(cos.f64 0)", "1"),
            ("tan-0", "(tan.f64 0)", "0"),
        ],
    );
    add(
        "trig-reduce",
        &[
            (
                "cos-sin-sum",
                "(+.f64 (*.f64 (cos.f64 ?a) (cos.f64 ?a)) (*.f64 (sin.f64 ?a) (sin.f64 ?a)))",
                "1",
            ),
            (
                "1-sub-cos",
                "(-.f64 1 (*.f64 (cos.f64 ?a) (cos.f64 ?a)))",
                "(*.f64 (sin.f64 ?a) (sin.f64 ?a))",
            ),
            (
                "1-sub-sin",
                "(-.f64 1 (*.f64 (sin.f64 ?a) (sin.f64 ?a)))",
                "(*.f64 (cos.f64 ?a) (cos.f64 ?a))",
            ),
            (
                "-1-add-cos",
                "(+.f64 (*.f64 (cos.f64 ?a) (cos.f64 ?a)) -1)",
                "(neg.f64 (*.f64 (sin.f64 ?a) (sin.f64 ?a)))",
            ),
            (
                "-1-add-sin",
                "(+.f64 (*.f64 (sin.f64 ?a) (sin.f64 ?a)) -1)",
                "(neg.f64 (*.f64 (cos.f64 ?a) (cos.f64 ?a)))",
            ),
            (
                "sub-1-cos",
                "(-.f64 (*.f64 (cos.f64 ?a) (cos.f64 ?a)) 1)",
                "(neg.f64 (*.f64 (sin.f64 ?a) (sin.f64 ?a)))",
            ),
            (
                "sub-1-sin",
                "(-.f64 (*.f64 (sin.f64 ?a) (sin.f64 ?a)) 1)",
                "(neg.f64 (*.f64 (cos.f64 ?a) (cos.f64 ?a)))",
            ),
            ("sin-PI/6", "(sin.f64 (/.f64 PI 6))", "1/2"),
            (
                "sin-PI/4",
                "(sin.f64 (/.f64 PI 4))",
                "(/.f64 (sqrt.f64 2) 2)",
            ),
            (
                "sin-PI/3",
                "(sin.f64 (/.f64 PI 3))",
                "(/.f64 (sqrt.f64 3) 2)",
            ),
            ("sin-PI/2", "(sin.f64 (/.f64 PI 2))", "1"),
            ("sin-PI", "(sin.f64 PI)", "0"),
            (
                "sin-+PI",
                "(sin.f64 (+.f64 ?x PI))",
                "(neg.f64 (sin.f64 ?x))",
            ),
            (
                "sin-+PI/2",
                "(sin.f64 (+.f64 ?x (/.f64 PI 2)))",
                "(cos.f64 ?x)",
            ),
            (
                "cos-PI/6",
                "(cos.f64 (/.f64 PI 6))",
                "(/.f64 (sqrt.f64 3) 2)",
            ),
            (
                "cos-PI/4",
                "(cos.f64 (/.f64 PI 4))",
                "(/.f64 (sqrt.f64 2) 2)",
            ),
            ("cos-PI/3", "(cos.f64 (/.f64 PI 3))", "1/2"),
            ("cos-PI/2", "(cos.f64 (/.f64 PI 2))", "0"),
            ("cos-PI", "(cos.f64 PI)", "-1"),
            (
                "cos-+PI",
                "(cos.f64 (+.f64 ?x PI))",
                "(neg.f64 (cos.f64 ?x))",
            ),
            (
                "cos-+PI/2",
                "(cos.f64 (+.f64 ?x (/.f64 PI 2)))",
                "(neg.f64 (sin.f64 ?x))",
            ),
            (
                "tan-PI/6",
                "(tan.f64 (/.f64 PI 6))",
                "(/.f64 1 (sqrt.f64 3))",
            ),
            ("tan-PI/4", "(tan.f64 (/.f64 PI 4))", "1"),
            ("tan-PI/3", "(tan.f64 (/.f64 PI 3))", "(sqrt.f64 3)"),
            ("tan-PI", "(tan.f64 PI)", "0"),
            ("tan-+PI", "(tan.f64 (+.f64 ?x PI))", "(tan.f64 ?x)"),
            (
                "tan-+PI/2",
                "(tan.f64 (+.f64 ?x (/.f64 PI 2)))",
                "(neg.f64 (/.f64 1 (tan.f64 ?x)))",
            ),
            (
                "hang-0p-tan",
                "(/.f64 (sin.f64 ?a) (+.f64 1 (cos.f64 ?a)))",
                "(tan.f64 (/.f64 ?a 2))",
            ),
            (
                "hang-0m-tan",
                "(/.f64 (neg.f64 (sin.f64 ?a)) (+.f64 1 (cos.f64 ?a)))",
                "(tan.f64 (/.f64 (neg.f64 ?a) 2))",
            ),
            (
                "hang-p0-tan",
                "(/.f64 (-.f64 1 (cos.f64 ?a)) (sin.f64 ?a))",
                "(tan.f64 (/.f64 ?a 2))",
            ),
            (
                "hang-m0-tan",
                "(/.f64 (-.f64 1 (cos.f64 ?a)) (neg.f64 (sin.f64 ?a)))",
                "(tan.f64 (/.f64 (neg.f64 ?a) 2))",
            ),
            (
                "hang-p-tan",
                "(/.f64 (+.f64 (sin.f64 ?a) (sin.f64 ?b)) (+.f64 (cos.f64 ?a) (cos.f64 ?b)))",
                "(tan.f64 (/.f64 (+.f64 ?a ?b) 2))",
            ),
            (
                "hang-m-tan",
                "(/.f64 (-.f64 (sin.f64 ?a) (sin.f64 ?b)) (+.f64 (cos.f64 ?a) (cos.f64 ?b)))",
                "(tan.f64 (/.f64 (-.f64 ?a ?b) 2))",
            ),
        ],
    );
    add("log-distribute-fp-safe", &[("log-E", "(log.f64 E)", "1")]);
    add(
        "log-distribute",
        &[
            (
                "log-prod",
                "(log.f64 (*.f64 ?a ?b))",
                "(+.f64 (log.f64 ?a) (log.f64 ?b))",
            ),
            (
                "log-div",
                "(log.f64 (/.f64 ?a ?b))",
                "(-.f64 (log.f64 ?a) (log.f64 ?b))",
            ),
            (
                "log-rec",
                "(log.f64 (/.f64 1 ?a))",
                "(neg.f64 (log.f64 ?a))",
            ),
            (
                "log-pow",
                "(log.f64 (pow.f64 ?a ?b))",
                "(*.f64 ?b (log.f64 ?a))",
            ),
        ],
    );
    add(
        "pow-canonicalize",
        &[
            (
                "exp-to-pow",
                "(exp.f64 (*.f64 (log.f64 ?a) ?b))",
                "(pow.f64 ?a ?b)",
            ),
            (
                "pow-plus",
                "(*.f64 (pow.f64 ?a ?b) ?a)",
                "(pow.f64 ?a (+.f64 ?b 1))",
            ),
            ("unpow1/2", "(pow.f64 ?a 1/2)", "(sqrt.f64 ?a)"),
            ("unpow2", "(pow.f64 ?a 2)", "(*.f64 ?a ?a)"),
            ("unpow3", "(pow.f64 ?a 3)", "(*.f64 (*.f64 ?a ?a) ?a)"),
            ("unpow1/3", "(pow.f64 ?a 1/3)", "(cbrt.f64 ?a)"),
        ],
    );
    add(
        "pow-reduce-fp-safe-nan",
        &[
            ("unpow0", "(pow.f64 ?a 0)", "1"),
            ("pow-base-1", "(pow.f64 1 ?a)", "1"),
        ],
    );
    add("pow-reduce-fp-safe", &[("unpow1", "(pow.f64 ?a 1)", "?a")]);
    add(
        "pow-reduce",
        &[("unpow-1", "(pow.f64 ?a -1)", "(/.f64 1 ?a)")],
    );
    add(
        "exp-factor",
        &[
            (
                "prod-exp",
                "(*.f64 (exp.f64 ?a) (exp.f64 ?b))",
                "(exp.f64 (+.f64 ?a ?b))",
            ),
            (
                "rec-exp",
                "(/.f64 1 (exp.f64 ?a))",
                "(exp.f64 (neg.f64 ?a))",
            ),
            (
                "div-exp",
                "(/.f64 (exp.f64 ?a) (exp.f64 ?b))",
                "(exp.f64 (-.f64 ?a ?b))",
            ),
            (
                "exp-prod",
                "(exp.f64 (*.f64 ?a ?b))",
                "(pow.f64 (exp.f64 ?a) ?b)",
            ),
            (
                "exp-sqrt",
                "(exp.f64 (/.f64 ?a 2))",
                "(sqrt.f64 (exp.f64 ?a))",
            ),
            (
                "exp-cbrt",
                "(exp.f64 (/.f64 ?a 3))",
                "(cbrt.f64 (exp.f64 ?a))",
            ),
            (
                "exp-lft-sqr",
                "(exp.f64 (*.f64 ?a 2))",
                "(*.f64 (exp.f64 ?a) (exp.f64 ?a))",
            ),
            (
                "exp-lft-cube",
                "(exp.f64 (*.f64 ?a 3))",
                "(pow.f64 (exp.f64 ?a) 3)",
            ),
        ],
    );
    add(
        "exp-distribute",
        &[
            (
                "exp-sum",
                "(exp.f64 (+.f64 ?a ?b))",
                "(*.f64 (exp.f64 ?a) (exp.f64 ?b))",
            ),
            (
                "exp-neg",
                "(exp.f64 (neg.f64 ?a))",
                "(/.f64 1 (exp.f64 ?a))",
            ),
            (
                "exp-diff",
                "(exp.f64 (-.f64 ?a ?b))",
                "(/.f64 (exp.f64 ?a) (exp.f64 ?b))",
            ),
        ],
    );
    add(
        "exp-constants",
        &[
            ("exp-0", "(exp.f64 0)", "1"),
            ("exp-1-e", "(exp.f64 1)", "E"),
            ("1-exp", "1", "(exp.f64 0)"),
            ("e-exp-1", "E", "(exp.f64 1)"),
        ],
    );
    add(
        "exp-reduce",
        &[
            ("rem-exp-log", "(exp.f64 (log.f64 ?x))", "?x"),
            ("rem-log-exp", "(log.f64 (exp.f64 ?x))", "?x"),
        ],
    );
    add(
        "cubes-canonicalize",
        &[("cube-unmult", "(*.f64 ?x (*.f64 ?x ?x))", "(pow.f64 ?x 3)")],
    );
    add(
        "cubes-distribute",
        &[
            (
                "cube-prod",
                "(pow.f64 (*.f64 ?x ?y) 3)",
                "(*.f64 (pow.f64 ?x 3) (pow.f64 ?y 3))",
            ),
            (
                "cube-div",
                "(pow.f64 (/.f64 ?x ?y) 3)",
                "(/.f64 (pow.f64 ?x 3) (pow.f64 ?y 3))",
            ),
            ("cube-mult", "(pow.f64 ?x 3)", "(*.f64 ?x (*.f64 ?x ?x))"),
        ],
    );
    add(
        "cubes-reduce",
        &[
            ("rem-cube-cbrt", "(pow.f64 (cbrt.f64 ?x) 3)", "?x"),
            ("rem-cbrt-cube", "(cbrt.f64 (pow.f64 ?x 3))", "?x"),
            (
                "cube-neg",
                "(pow.f64 (neg.f64 ?x) 3)",
                "(neg.f64 (pow.f64 ?x 3))",
            ),
        ],
    );
    add(
        "squares-reduce-fp-sound",
        &[(
            "sqr-neg",
            "(*.f64 (neg.f64 ?x) (neg.f64 ?x))",
            "(*.f64 ?x ?x)",
        )],
    );
    add(
        "squares-reduce",
        &[
            (
                "rem-square-sqrt",
                "(*.f64 (sqrt.f64 ?x) (sqrt.f64 ?x))",
                "?x",
            ),
            (
                "rem-sqrt-square",
                "(sqrt.f64 (*.f64 ?x ?x))",
                "(fabs.f64 ?x)",
            ),
        ],
    );
    add(
        "fractions-distribute.c",
        &[
            (
                "div-sub.c",
                "(/.c (-.c ?a ?b) ?c)",
                "(-.c (/.c ?a ?c) (/.c ?b ?c))",
            ),
            (
                "times-frac.c",
                "(/.c (*.c ?a ?b) (*.c ?c ?d))",
                "(*.c (/.c ?a ?c) (/.c ?b ?d))",
            ),
        ],
    );
    add(
        "fractions-distribute",
        &[
            (
                "div-sub",
                "(/.f64 (-.f64 ?a ?b) ?c)",
                "(-.f64 (/.f64 ?a ?c) (/.f64 ?b ?c))",
            ),
            (
                "times-frac",
                "(/.f64 (*.f64 ?a ?b) (*.f64 ?c ?d))",
                "(*.f64 (/.f64 ?a ?c) (/.f64 ?b ?d))",
            ),
        ],
    );
    add(
        "id-reduce-fp-safe",
        &[
            ("+-lft-identity", "(+.f64 0 ?a)", "?a"),
            ("+-rgt-identity", "(+.f64 ?a 0)", "?a"),
            ("--rgt-identity", "(-.f64 ?a 0)", "?a"),
            ("sub0-neg", "(-.f64 0 ?a)", "(neg.f64 ?a)"),
            ("remove-double-neg", "(neg.f64 (neg.f64 ?a))", "?a"),
            ("*-lft-identity", "(*.f64 1 ?a)", "?a"),
            ("*-rgt-identity", "(*.f64 ?a 1)", "?a"),
            ("/-rgt-identity", "(/.f64 ?a 1)", "?a"),
            ("mul-1-neg", "(*.f64 -1 ?a)", "(neg.f64 ?a)"),
        ],
    );
    add(
        "id-reduce-fp-safe-nan",
        &[
            ("+-inverses", "(-.f64 ?a ?a)", "0"),
            ("*-inverses", "(/.f64 ?a ?a)", "1"),
            ("div0", "(/.f64 0 ?a)", "0"),
            ("mul0l", "(*.f64 0 ?a)", "0"),
            ("mul0r", "(*.f64 ?a 0)", "0"),
        ],
    );
    add(
        "id-reduce",
        &[
            ("remove-double-div", "(/.f64 1 (/.f64 1 ?a))", "?a"),
            ("rgt-mult-inverse", "(*.f64 ?a (/.f64 1 ?a))", "1"),
            ("lft-mult-inverse", "(*.f64 (/.f64 1 ?a) ?a)", "1"),
        ],
    );
    add(
        "difference-of-squares-canonicalize",
        &[
            (
                "swap-sqr",
                "(*.f64 (*.f64 ?a ?b) (*.f64 ?a ?b))",
                "(*.f64 (*.f64 ?a ?a) (*.f64 ?b ?b))",
            ),
            (
                "unswap-sqr",
                "(*.f64 (*.f64 ?a ?a) (*.f64 ?b ?b))",
                "(*.f64 (*.f64 ?a ?b) (*.f64 ?a ?b))",
            ),
            (
                "difference-of-squares",
                "(-.f64 (*.f64 ?a ?a) (*.f64 ?b ?b))",
                "(*.f64 (+.f64 ?a ?b) (-.f64 ?a ?b))",
            ),
            (
                "difference-of-sqr-1",
                "(-.f64 (*.f64 ?a ?a) 1)",
                "(*.f64 (+.f64 ?a 1) (-.f64 ?a 1))",
            ),
            (
                "difference-of-sqr--1",
                "(+.f64 (*.f64 ?a ?a) -1)",
                "(*.f64 (+.f64 ?a 1) (-.f64 ?a 1))",
            ),
            (
                "sqr-pow",
                "(pow.f64 ?a ?b)",
                "(*.f64 (pow.f64 ?a (/.f64 ?b 2)) (pow.f64 ?a (/.f64 ?b 2)))",
            ),
            (
                "pow-sqr",
                "(*.f64 (pow.f64 ?a ?b) (pow.f64 ?a ?b))",
                "(pow.f64 ?a (*.f64 2 ?b))",
            ),
        ],
    );
    add(
        "distributivity-fp-safe",
        &[
            (
                "distribute-lft-neg-in",
                "(neg.f64 (*.f64 ?a ?b))",
                "(*.f64 (neg.f64 ?a) ?b)",
            ),
            (
                "distribute-rgt-neg-in",
                "(neg.f64 (*.f64 ?a ?b))",
                "(*.f64 ?a (neg.f64 ?b))",
            ),
            (
                "distribute-lft-neg-out",
                "(*.f64 (neg.f64 ?a) ?b)",
                "(neg.f64 (*.f64 ?a ?b))",
            ),
            (
                "distribute-rgt-neg-out",
                "(*.f64 ?a (neg.f64 ?b))",
                "(neg.f64 (*.f64 ?a ?b))",
            ),
            (
                "distribute-neg-in",
                "(neg.f64 (+.f64 ?a ?b))",
                "(+.f64 (neg.f64 ?a) (neg.f64 ?b))",
            ),
            (
                "distribute-neg-out",
                "(+.f64 (neg.f64 ?a) (neg.f64 ?b))",
                "(neg.f64 (+.f64 ?a ?b))",
            ),
            (
                "distribute-frac-neg",
                "(/.f64 (neg.f64 ?a) ?b)",
                "(neg.f64 (/.f64 ?a ?b))",
            ),
            (
                "distribute-neg-frac",
                "(neg.f64 (/.f64 ?a ?b))",
                "(/.f64 (neg.f64 ?a) ?b)",
            ),
        ],
    );
    add(
        "distributivity.c",
        &[
            (
                "distribute-lft-in.c",
                "(*.c ?a (+.c ?b ?c))",
                "(+.c (*.c ?a ?b) (*.c ?a ?c))",
            ),
            (
                "distribute-rgt-in.c",
                "(*.c ?a (+.c ?b ?c))",
                "(+.c (*.c ?b ?a) (*.c ?c ?a))",
            ),
            (
                "distribute-lft-out.c",
                "(+.c (*.c ?a ?b) (*.c ?a ?c))",
                "(*.c ?a (+.c ?b ?c))",
            ),
            (
                "distribute-lft-out--.c",
                "(-.c (*.c ?a ?b) (*.c ?a ?c))",
                "(*.c ?a (-.c ?b ?c))",
            ),
            (
                "distribute-rgt-out.c",
                "(+.c (*.c ?b ?a) (*.c ?c ?a))",
                "(*.c ?a (+.c ?b ?c))",
            ),
            (
                "distribute-rgt-out--.c",
                "(-.c (*.c ?b ?a) (*.c ?c ?a))",
                "(*.c ?a (-.c ?b ?c))",
            ),
            (
                "distribute-lft1-in.c",
                "(+.c (*.c ?b ?a) ?a)",
                "(*.c (+.c ?b (complex 1 0)) ?a)",
            ),
            (
                "distribute-rgt1-in.c",
                "(+.c ?a (*.c ?c ?a))",
                "(*.c (+.c ?c (complex 1 0)) ?a)",
            ),
        ],
    );
    add(
        "distributivity",
        &[
            (
                "distribute-lft-in",
                "(*.f64 ?a (+.f64 ?b ?c))",
                "(+.f64 (*.f64 ?a ?b) (*.f64 ?a ?c))",
            ),
            (
                "distribute-rgt-in",
                "(*.f64 ?a (+.f64 ?b ?c))",
                "(+.f64 (*.f64 ?b ?a) (*.f64 ?c ?a))",
            ),
            (
                "distribute-lft-out",
                "(+.f64 (*.f64 ?a ?b) (*.f64 ?a ?c))",
                "(*.f64 ?a (+.f64 ?b ?c))",
            ),
            (
                "distribute-lft-out--",
                "(-.f64 (*.f64 ?a ?b) (*.f64 ?a ?c))",
                "(*.f64 ?a (-.f64 ?b ?c))",
            ),
            (
                "distribute-rgt-out",
                "(+.f64 (*.f64 ?b ?a) (*.f64 ?c ?a))",
                "(*.f64 ?a (+.f64 ?b ?c))",
            ),
            (
                "distribute-rgt-out--",
                "(-.f64 (*.f64 ?b ?a) (*.f64 ?c ?a))",
                "(*.f64 ?a (-.f64 ?b ?c))",
            ),
            (
                "distribute-lft1-in",
                "(+.f64 (*.f64 ?b ?a) ?a)",
                "(*.f64 (+.f64 ?b 1) ?a)",
            ),
            (
                "distribute-rgt1-in",
                "(+.f64 ?a (*.f64 ?c ?a))",
                "(*.f64 (+.f64 ?c 1) ?a)",
            ),
        ],
    );
    add("counting", &[("count-2", "(+.f64 ?x ?x)", "(*.f64 2 ?x)")]);
    add(
        "associativity.c",
        &[
            (
                "associate-+r+.c",
                "(+.c ?a (+.c ?b ?c))",
                "(+.c (+.c ?a ?b) ?c)",
            ),
            (
                "associate-+l+.c",
                "(+.c (+.c ?a ?b) ?c)",
                "(+.c ?a (+.c ?b ?c))",
            ),
            (
                "associate-+r-.c",
                "(+.c ?a (-.c ?b ?c))",
                "(-.c (+.c ?a ?b) ?c)",
            ),
            (
                "associate-+l-.c",
                "(+.c (-.c ?a ?b) ?c)",
                "(-.c ?a (-.c ?b ?c))",
            ),
            (
                "associate--r+.c",
                "(-.c ?a (+.c ?b ?c))",
                "(-.c (-.c ?a ?b) ?c)",
            ),
            (
                "associate--l+.c",
                "(-.c (+.c ?a ?b) ?c)",
                "(+.c ?a (-.c ?b ?c))",
            ),
            (
                "associate--l-.c",
                "(-.c (-.c ?a ?b) ?c)",
                "(-.c ?a (+.c ?b ?c))",
            ),
            (
                "associate--r-.c",
                "(-.c ?a (-.c ?b ?c))",
                "(+.c (-.c ?a ?b) ?c)",
            ),
            (
                "associate-*r*.c",
                "(*.c ?a (*.c ?b ?c))",
                "(*.c (*.c ?a ?b) ?c)",
            ),
            (
                "associate-*l*.c",
                "(*.c (*.c ?a ?b) ?c)",
                "(*.c ?a (*.c ?b ?c))",
            ),
            (
                "associate-*r/.c",
                "(*.c ?a (/.c ?b ?c))",
                "(/.c (*.c ?a ?b) ?c)",
            ),
            (
                "associate-*l/.c",
                "(*.c (/.c ?a ?b) ?c)",
                "(/.c (*.c ?a ?c) ?b)",
            ),
            (
                "associate-/r*.c",
                "(/.c ?a (*.c ?b ?c))",
                "(/.c (/.c ?a ?b) ?c)",
            ),
            (
                "associate-/l*.c",
                "(/.c (*.c ?b ?c) ?a)",
                "(/.c ?b (/.c ?a ?c))",
            ),
            (
                "associate-/r/.c",
                "(/.c ?a (/.c ?b ?c))",
                "(*.c (/.c ?a ?b) ?c)",
            ),
            (
                "associate-/l/.c",
                "(/.c (/.c ?b ?c) ?a)",
                "(/.c ?b (*.c ?a ?c))",
            ),
            ("sub-neg.c", "(-.c ?a ?b)", "(+.c ?a (neg.c ?b))"),
            ("unsub-neg.c", "(+.c ?a (neg.c ?b))", "(-.c ?a ?b)"),
        ],
    );
    add(
        "associativity",
        &[
            (
                "associate-+r+",
                "(+.f64 ?a (+.f64 ?b ?c))",
                "(+.f64 (+.f64 ?a ?b) ?c)",
            ),
            (
                "associate-+l+",
                "(+.f64 (+.f64 ?a ?b) ?c)",
                "(+.f64 ?a (+.f64 ?b ?c))",
            ),
            (
                "associate-+r-",
                "(+.f64 ?a (-.f64 ?b ?c))",
                "(-.f64 (+.f64 ?a ?b) ?c)",
            ),
            (
                "associate-+l-",
                "(+.f64 (-.f64 ?a ?b) ?c)",
                "(-.f64 ?a (-.f64 ?b ?c))",
            ),
            (
                "associate--r+",
                "(-.f64 ?a (+.f64 ?b ?c))",
                "(-.f64 (-.f64 ?a ?b) ?c)",
            ),
            (
                "associate--l+",
                "(-.f64 (+.f64 ?a ?b) ?c)",
                "(+.f64 ?a (-.f64 ?b ?c))",
            ),
            (
                "associate--l-",
                "(-.f64 (-.f64 ?a ?b) ?c)",
                "(-.f64 ?a (+.f64 ?b ?c))",
            ),
            (
                "associate--r-",
                "(-.f64 ?a (-.f64 ?b ?c))",
                "(+.f64 (-.f64 ?a ?b) ?c)",
            ),
            (
                "associate-*r*",
                "(*.f64 ?a (*.f64 ?b ?c))",
                "(*.f64 (*.f64 ?a ?b) ?c)",
            ),
            (
                "associate-*l*",
                "(*.f64 (*.f64 ?a ?b) ?c)",
                "(*.f64 ?a (*.f64 ?b ?c))",
            ),
            (
                "associate-*r/",
                "(*.f64 ?a (/.f64 ?b ?c))",
                "(/.f64 (*.f64 ?a ?b) ?c)",
            ),
            (
                "associate-*l/",
                "(*.f64 (/.f64 ?a ?b) ?c)",
                "(/.f64 (*.f64 ?a ?c) ?b)",
            ),
            (
                "associate-/r*",
                "(/.f64 ?a (*.f64 ?b ?c))",
                "(/.f64 (/.f64 ?a ?b) ?c)",
            ),
            (
                "associate-/l*",
                "(/.f64 (*.f64 ?b ?c) ?a)",
                "(/.f64 ?b (/.f64 ?a ?c))",
            ),
            (
                "associate-/r/",
                "(/.f64 ?a (/.f64 ?b ?c))",
                "(*.f64 (/.f64 ?a ?b) ?c)",
            ),
            (
                "associate-/l/",
                "(/.f64 (/.f64 ?b ?c) ?a)",
                "(/.f64 ?b (*.f64 ?a ?c))",
            ),
            ("sub-neg", "(-.f64 ?a ?b)", "(+.f64 ?a (neg.f64 ?b))"),
            ("unsub-neg", "(+.f64 ?a (neg.f64 ?b))", "(-.f64 ?a ?b)"),
        ],
    );
    add(
        "commutativity.c",
        &[
            ("+.c-commutative", "(+.c ?a ?b)", "(+.c ?b ?a)"),
            ("*.c-commutative", "(*.c ?a ?b)", "(*.c ?b ?a)"),
        ],
    );
    add(
        "commutativity",
        &[
            ("+-commutative", "(+.f64 ?a ?b)", "(+.f64 ?b ?a)"),
            ("*-commutative", "(*.f64 ?a ?b)", "(*.f64 ?b ?a)"),
        ],
    );

    m
}
