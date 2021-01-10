use egg::rewrite as rw;

fn main() {
    //! Based on egg tutorial code https://docs.rs/egg/0.6.0/egg/tutorials/_02_getting_started/index.html
    println!("Run cargo test instead");
}

/**
 * Ring rules
*/
#[rustfmt::skip]
pub fn ring_rules() -> Vec<egg::Rewrite<egg::SymbolLang, ()>> {vec![
    // Abelian (Additive)
    rw!("ring_add_associative_l"; "(+ ?a (+ ?b ?c)))" => "(+ (+ ?a ?b) ?c))"),
    rw!("ring_add_associative_r"; "(+ (+ ?a ?b) ?c))" => "(+ ?a (+ ?b ?c)))" ),
    rw!("ring_add_commutative"; "(+ ?a ?b)" => "(+ ?b ?a)" ),
    rw!("ring_additive_identity"; "(+ ?a 0)" => "?a" ),
    rw!("ring_additive_inverse"; "(+ ?a (neg ?a))" => "0" ),
    rw!("ring_minus_is_negation"; "(- ?a ?b)" => "(+ ?a (neg ?b))"),

    // Monoid (Multiplicative)
    rw!("ring_monoid_associative_l"; "(* ?a (* ?b ?c)))" => "(* (* ?a ?b) ?c))"),
    rw!("ring_monoid_associative_r"; "(* (* ?a ?b) ?c))" => "(* ?a (* ?b ?c)))" ),

    rw!("ring_monoid_unit_l"; "(* ?a 1)" => "?a"),
    rw!("ring_monoid_unit_r"; "(* 1 ?a)" => "?a"),

    // Multiplication is distributive with regard to addition
    // for vectors a, b, c
    rw!("ring_distr-l"; "(* ?a (+ ?b ?c) )" => "(+ (* ?a ?b) (* ?a ?c))"),
    rw!("ring_distr-r"; "(* (+ ?b ?c) ?a )" => "(+ (* ?b ?a) (* ?c ?a))"),
]}

/**
 * Field is Ring + multiplicative commutativity
*/
#[rustfmt::skip]
pub fn field_over_ring_rules() -> Vec<egg::Rewrite<egg::SymbolLang, ()>> {vec![
    // Monoid (Multiplicative)
    rw!("field_mul_commutative"; "(* ?a ?b)" => "(* ?b ?a)" ),

    rw!("field_div_is_inverse"; "(/ ?a ?b)" => "(* ?a (inv ?b))"),
    rw!("field_multiplicative_inverse"; "(* ?a (inv ?a))" => "1"), // TODO condition a /= 0?

    rw!("monoid_unit"; "(* ?a 1)" => "?a"),

    rw!("mul_identity"; "(* ?a 0)" => "0"), // only need one side as mul is commutative
]}

/**
 * Geometric Algebra (GAS) language
 * a,b,c.. p are vectors
 * s,t,u,v, .. x,y,z are scalars
 * All uppercase are Multivectors
 * e0,e1,e3.... are basis vectors |e_n| = 1
 * e01, e02, e12, e_m_n are bivectors
 * I is the 'end?' e012
*/
#[rustfmt::skip]
pub fn gas_rules() -> Vec<egg::Rewrite<egg::SymbolLang, ()>> {vec![
    // Field F
    // Abelian
    rw!("add_associative_l"; "(+ ?a (+ ?b ?c)))" => "(+ (+ ?a ?b) ?c))"),
    rw!("add_associative_r"; "(+ (+ ?a ?b) ?c))" => "(+ ?a (+ ?b ?c)))" ),
    rw!("add_commutative"; "(+ ?a ?b)" => "(+ ?b ?a)" ),
    rw!("additive_identity"; "(+ ?a 0)" => "?a" ),
    rw!("additive_inverse"; "(+ ?a (neg ?a))" => "0" ),
    rw!("minus_is_negation"; "(- ?a ?b)" => "(+ ?a (neg ?b))"),

    // Monoid (Multiplicative)
    rw!("monoid_associative_l"; "(* ?a (* ?b ?c)))" => "(* (* ?a ?b) ?c))"),
    rw!("monoid_associative_r"; "(* (* ?a ?b) ?c))" => "(* ?a (* ?b ?c)))" ),
    rw!("multiplicative_inverse"; "(* ?a (inv ?a))" => "1"), // TODO need ? condition a /= 0
    rw!("div_is_inverse"; "(/ ?a ?b)" => "(* ?a (inv ?b))"),

    rw!("monoid_unit_l"; "(* ?a 1)" => "?a"),
    rw!("monoid_unit_r"; "(* 1 ?a)" => "?a"),

    rw!("mul_identity_r"; "(* ?a 0)" => "0"),
    rw!("mul_identity_l"; "(* 0 ?a)" => "0"),


    // Multiplication is distributive with regard to addition
    // for vectors a, b, c
    rw!("distr-l"; "(* ?a (+ ?b ?c) )" => "(+ (* ?a ?b) (* ?a ?c))"),
    rw!("distr-r"; "(* (+ ?b ?c) ?a )" => "(+ (* ?b ?a) (* ?c ?a))"),



    rw!("double"; "(+ ?a ?a)" => "(* 2 ?a)"),
    // contraction
    rw!("contraction"; "(sqr ?a)" => "(sqr (magnitude ?a))"), // magnitude is scalar!

    // Vector fundamental
    rw!("vector_product"; "(* ?a ?b)" => "(+ (dot ?a ?b) (hat ?a ?b))"),
    rw!("vector_commutative_dot"; "(dot ?a ?b)" => "(dot ?b ?a)"),

    // hat
    rw!("hat"; "(hat ?a ?b)" => "(* (inv 2) (- (* ?a ?b) (* ?b ?a)))"),
    rw!("hat_reverse"; "(hat ?a ?b)" => "(hat (neg b) a))"),
    rw!("hat_cancel"; "(hat ?a ?a)" => "0"),

    // Vector inverse
    rw!("vector_inverse"; "(inv v)" => "(* (inv (dot v v)) v)"),
    rw!("vector_invmag2"; "(inv (dot v v))" => "(invmag2 v)"),
    rw!("vector_toinv_l"; "(* (invmag2 v) v))" => "(inv v)"),
    rw!("vector_toinv_r"; "(* v (invmag2 v)))" => "(inv v)"),
]}

#[rustfmt::skip]
egg::test_fn! {right_unit_g4, gas_rules(), "(* a 1)" =>  "a"}
egg::test_fn! {left_unit_g4, gas_rules(), "(* 1 a)" =>  "a"}

egg::test_fn! { dist1_g1, gas_rules(), "(* A (+ B C))" => "(+ (* A B) (* A C))" }
egg::test_fn! { dist2_g1, gas_rules(), "(* (+ B C) A)" => "(+ (* B A) (* C A))" }
//egg::test_fn! { dist_g1_scalar1, gas_rules(), "(* (* s A) B)" => "(* A (* s B))" }
//egg::test_fn! { dist_g1_scalar2, gas_rules(),  "(* A (* s B))" => "(* s (* A B))" }
egg::test_fn! { assoc, gas_rules(), "(* A (* B C))" => "(* (* A B) C)"}

egg::test_fn! { vector_metric1, gas_rules(), "(* a a)" => "(dot a a)" }
//egg::test_fn! { vector_metric2, gas_rules(), "(dot a a)" => "(sqr (magnitude a))"}
//egg::test_fn! { vector_metric3, gas_rules(), "(dot a a)" => "(mag2 a)" }

egg::test_fn! { vector_product_fundamental, gas_rules(), "(* u v)"=> "(+ (dot u v) (hat u v))"}
egg::test_fn! { product_cancel, gas_rules(), "(- (* a a) (* a a))"=> "0"}
egg::test_fn! { inv_cancel, gas_rules(), "(* (inv 2) (- a a))"=> "0"}
egg::test_fn! { hat_cancel, gas_rules(), "(hat a a)"=> "0"}
egg::test_fn! { vector_inverse, gas_rules(), "(inv v)" => "(* (invmag2 v) v)"}

egg::test_fn! { challenging, gas_rules(), "(* (inv (dot v v)) v)" => "(inv v))" }
egg::test_fn! { challenging_scalar_commute, gas_rules(), "(* v (inv (dot v v)))" => "(inv v))" }
// We probably need to add Analysis about scalars, vectors, multivectors, even, odd

egg::test_fn! {right_unit, gas_rules(), "(* b 1)" =>  "b"}
egg::test_fn! {left_unit, gas_rules(), "(* 1 b)" =>  "b"}
egg::test_fn! {cancel, gas_rules(), "(- a a)" =>  "0"}
egg::test_fn! {cancel_inv, gas_rules(), "(* a (inv a))" =>  "1"}
