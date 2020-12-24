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

egg::test_fn! {right_unit_g4, gas_rules(), "(* a 1)" =>  "a"}
egg::test_fn! {left_unit_g4, gas_rules(), "(* 1 a)" =>  "a"}

egg::test_fn! { dist1_g1, gas_rules(), "(* A (+ B C))" => "(+ (* A B) (* A C))" }
egg::test_fn! { dist2_g1, gas_rules(), "(* (+ B C) A)" => "(+ (* B A) (* C A))" }
egg::test_fn! { dist_g1_scalar1, gas_rules(), "(* (* a A) B)" => "(* A (* a B))" }
egg::test_fn! { dist_g1_scalar2, gas_rules(),  "(* A (* a B))" => "(* a (* A B))" }
egg::test_fn! { assoc, gas_rules(), "(* A (* B C))" => "(* (* A B) C)"}

egg::test_fn! { vector_metric1, gas_rules(), "(* u u)" => "(dot u u)" }
egg::test_fn! { vector_metric2, gas_rules(), "(dot u u)" => "(sqr (magnitude u))"}
egg::test_fn! { vector_metric3, gas_rules(), "(dot u u)" => "(mag2 u)" }

egg::test_fn! { vector_product_fundamental, gas_rules(), "(* u v)"=> "(+ (dot u v) (hat u v))"}
egg::test_fn! { vector_inverse, gas_rules(), "(inv v)" => "(* (invmag2 v) v)"}

// TODO egg::test_fn! { challenging, gas_rules(), "(* v (inv (dot v v)))" => "(inv v))" }
