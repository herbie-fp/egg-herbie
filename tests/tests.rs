use egg_math::{math::*, rules::math_rules};

fn rules() -> Vec<Rewrite> {
    math_rules()
        .into_iter()
        .flat_map(|(_group, rewrites)| rewrites)
        .collect()
}

egg::test_fn! {
    math_simplify_root, rules(),
    // runner = Runner::default().with_node_limit(75_000),
    r#"
    (/ 1
       (- (/ (+ 1 (sqrt five))
             2)
          (/ (- 1 (sqrt five))
             2)))"#
    =>
    "(/ 1 (sqrt five))"
}

egg::test_fn! {
    math_simplify_neg, rules(),
    "(neg 1)" => "-1"
}
