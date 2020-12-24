use egg::rewrite as rw;

fn main() {
    //! Based on egg tutorial code https://docs.rs/egg/0.6.0/egg/tutorials/_02_getting_started/index.html
    println!("Run cargo test instead");
}

#[rustfmt::skip]
pub fn gas_rules() -> Vec<egg::Rewrite<egg::SymbolLang, ()>> {vec![
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
    rw!("vector_metric-3"; "(sqr (magnitude u))" => "(mag2 u)"),
    // Vector fundamental
    rw!("vector_product"; "(* u v)" => "(+ (dot u v) (hat u v))"),
    // Vector inverse
    rw!("vector_inverse"; "(inv v)" => "(* (invmag2 v) v)"),
]}

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
        simplifies_to(&gas_rules(), "(dot u u)", "(mag2 u)");
    }

    #[test]
    fn vector_product_fundamental() {
        are_equivalent(&gas_rules(), "(* u v)", "(+ (dot u v) (hat u v))");
    }

    #[test]
    fn vector_inverse() {
        are_equivalent(&gas_rules(), "(inv v)", "(* (invmag2 v) v)");
        // Challenge
        // are_equivalent(&gas_rules(), "(* v (inv (dot v v)))", "(inv v))");
    }

    #[test]
    fn list() {
        simplifies_to(&gas_rules(), "list a b c", "list a b");
        // Challenge
        // are_equivalent(&gas_rules(), "(* v (inv (dot v v)))", "(inv v))");
    }

    fn simplifies_to(rules: &[Rewrite<SymbolLang, ()>], start: &str, expect: &str) {
        let start = start.parse().unwrap();
        let runner = Runner::default().with_expr(&start).run(rules);
        let mut extractor = Extractor::new(&runner.egraph, AstSize);
        let (_best_cost, best_expr) = extractor.find_best(runner.roots[0]);

        assert_eq!(best_expr, expect.parse().unwrap());
    }

    fn are_equivalent(rules: &[Rewrite<SymbolLang, ()>], start: &str, expect: &str) {
        let start = start.parse().unwrap();
        let expect = expect.parse().unwrap();

        let runner = Runner::default()
            .with_expr(&start)
            .with_expr(&expect)
            .run(rules);
        assert_eq!(
            runner.egraph.find(runner.roots[0]),
            runner.egraph.find(runner.roots[1]),
            "Not able to infer equivalence between {} to {}",
            start,
            expect
        );
    }

    fn are_symmetric_equivalent(rules: &[Rewrite<SymbolLang, ()>], start: &str, expect: &str) {
        let start = start.parse().unwrap();
        let expect = expect.parse().unwrap();

        let runner1 = Runner::default().with_expr(&start).run(rules);
        let runner2 = Runner::default().with_expr(&expect).run(rules);
        let eqs = runner1.egraph.equivs(&start, &expect);
        assert_eq!(
            eqs.len(),
            1,
            "Not able to infer equivalence from {} to {}",
            start,
            expect
        );
        let eqs = runner2.egraph.equivs(&start, &expect);
        assert_eq!(
            eqs.len(),
            1,
            "Not able to infer equivalence from {} to {}",
            expect,
            start
        );
    }
}
