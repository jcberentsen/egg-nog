use egg::rewrite as rw;

fn main() {
    //! Based on egg tutorial code https://docs.rs/egg/0.6.0/egg/tutorials/_02_getting_started/index.html
    println!("Run cargo test instead");
}

pub fn gas_rules() -> [egg::Rewrite<egg::SymbolLang, ()>; 10] {
    [
        // G1
        rw!("distr-l"; "(* ?A (+ ?B ?C) )" => "(+ (* ?A ?B) (* ?A ?C))"),
        rw!("distr-r"; "(* (+ ?B ?C) ?A )" => "(+ (* ?B ?A) (* ?C ?A))"),
        // G2
        rw!("distr-g1-scalar1"; "(* (* a A) B )" => "(* A (* a B))"),
        rw!("distr-g1-scalar2"; "(* A (* a B) )" => "(* a (* A B))"),
        // G3
        rw!("assoc-1"; "(* A (* B C))" => "(* (* A B) C)"),
        rw!("assoc-2"; "(* (* A B) C)" => "(* A (* B C))"),
        // G4
        rw!("mul-1r"; "(* ?x 1)" => "?x"),
        rw!("mul-1l"; "(* 1 ?x)" => "?x"),
        // G5
        rw!("vector_metric-1"; "(* u u)" => "(dot u u)"),
        rw!("vector_metric-2"; "(dot u u)" => "(sqr (magnitude u))"),
    ]
}

#[cfg(test)]
mod gas_tests {
    use super::*;
    use egg::{AstSize, Extractor, Rewrite, Runner, SymbolLang};

    #[test]
    fn unit_g4() {
        simplifies_to(&gas_rules(), "(* a 1)", "a");
        simplifies_to(&gas_rules(), "(* 1 a)", "a");
    }

    #[test]
    fn distr_g1() {
        are_equivalent(&gas_rules(), "(* A (+ B C))", "(+ (* A B) (* A C))");
        are_equivalent(&gas_rules(), "(* (+ B C) A)", "(+ (* B A) (* C A))");
    }

    #[test]
    fn distr_g2_scalar() {
        are_equivalent(&gas_rules(), "(* (* a A) B)", "(* A (* a B))");

        are_equivalent(&gas_rules(), "(* A (* a B))", "(* a (* A B))");
    }

    #[test]
    fn assoc() {
        are_symmetric_equivalent(&gas_rules(), "(* A (* B C))", "(* (* A B) C)");
    }

    #[test]
    fn vector_metric() {
        are_equivalent(&gas_rules(), "(* u u)", "(dot u u)");
        are_equivalent(&gas_rules(), "(dot u u)", "(sqr (magnitude u))");
    }

    fn simplifies_to(rules: &[Rewrite<SymbolLang, ()>], start: &str, expect: &str) {
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

    fn are_equivalent(rules: &[Rewrite<SymbolLang, ()>], start: &str, expect: &str) {
        // While it may look like we are working with numbers,
        // SymbolLang stores everything as strings.
        // We can make our own Language later to work with other types.
        let start = start.parse().unwrap();
        let expect = expect.parse().unwrap();

        // That's it! We can run equality saturation now.
        let runner = Runner::default().with_expr(&start).run(rules);
        let eqs = runner.egraph.equivs(&start, &expect);
        assert_eq!(eqs.len(), 1);
    }

    fn are_symmetric_equivalent(rules: &[Rewrite<SymbolLang, ()>], start: &str, expect: &str) {
        // While it may look like we are working with numbers,
        // SymbolLang stores everything as strings.
        // We can make our own Language later to work with other types.
        let start = start.parse().unwrap();
        let expect = expect.parse().unwrap();

        // That's it! We can run equality saturation now.
        let runner1 = Runner::default().with_expr(&start).run(rules);
        let runner2 = Runner::default().with_expr(&expect).run(rules);
        let eqs = runner1.egraph.equivs(&start, &expect);
        assert_eq!(eqs.len(), 1, "Not able to infer equivalence from {} to {}", start, expect);
        let eqs = runner2.egraph.equivs(&start, &expect);
        assert_eq!(eqs.len(), 1, "Not able to infer equivalence from {} to {}", expect, start);
    }
}
