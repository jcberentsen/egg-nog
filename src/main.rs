use egg::rewrite as rw;

fn main() {
    //! From egg tutorial code https://docs.rs/egg/0.6.0/egg/tutorials/_02_getting_started/index.html
    println!("Run cargo test instead");
}

const cas_rules: [egg::Rewrite<egg::SymbolLang, ()>; 5] = [
    rw!("commute-add"; "(+ ?x ?y)" => "(+ ?y ?x)"),
    rw!("commute-mul"; "(* ?x ?y)" => "(* ?y ?x)"),
    rw!("add-0"; "(+ ?x 0)" => "?x"),
    rw!("mul-0"; "(* ?x 0)" => "0"),
    rw!("mul-1"; "(* ?x 1)" => "?x"),
];

const gas_rules: [egg::Rewrite<egg::SymbolLang, ()>; 1] = [rw!("mul-1"; "(* ?x 1)" => "?x")];

#[cfg(test)]
mod gas_tests {
    use super::*;
    use egg::{AstSize, Extractor, Rewrite, Runner, SymbolLang};
    #[test]
    fn unit() {
        simplifies_to(&cas_rules, "(* 1 a)", "a");
    }

    #[test]
    fn test_egg_tutorial() {
        simplifies_to(&cas_rules, "(+ 0 (* 1 a))", "a");
    }

    fn simplifies_to(rules: &[Rewrite<SymbolLang, ()>], start: &str, expect: &str) {
        // let rules: &[Rewrite<SymbolLang, ()>] = &cas_rules();

        // While it may look like we are working with numbers,
        // SymbolLang stores everything as strings.
        // We can make our own Language later to work with other types.
        let start = start.parse().unwrap();

        // That's it! We can run equality saturation now.
        let runner = Runner::default().with_expr(&start).run(rules);

        // Extractors can take a user-defined cost function,
        // we'll use the egg-provided AstSize for now
        let mut extractor = Extractor::new(&runner.egraph, AstSize);

        // We want to extract the best expression represented in the
        // same e-class as our initial expression, not from the whole e-graph.
        // Luckily the runner stores the eclass Id where we put the initial expression.
        let (best_cost, best_expr) = extractor.find_best(runner.roots[0]);

        // we found the best thing, which is just "a" in this case
        assert_eq!(best_expr, expect.parse().unwrap());
        assert_eq!(best_cost, 1);
    }
}
