use egg::rewrite as rw;
type GasRules = Vec<egg::Rewrite<egg::SymbolLang, ()>>;

use crate::ring:: { ring_rules };

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
pub fn gas_rules() -> GasRules {
    let mut rules = ring_rules();

    let extra = vec![
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
    ];
    rules.extend(extra);
    rules
}

egg::test_fn! { add_commute, gas_rules(), "(+ a b)" =>  "(+ b a)"}
egg::test_fn! { right_unit, gas_rules(), "(* b 1)" =>  "b"}
egg::test_fn! { left_unit, gas_rules(), "(* 1 b)" =>  "b"}
egg::test_fn! { cancel, gas_rules(), "(- a a)" =>  "0"}
egg::test_fn! { cancel_inv, gas_rules(), "(* a (inv a))" =>  "1"}

egg::test_fn! { dist1_g1, gas_rules(), "(* A (+ B C))" => "(+ (* A B) (* A C))" }
egg::test_fn! { dist2_g1, gas_rules(), "(* (+ B C) A)" => "(+ (* B A) (* C A))" }
egg::test_fn! { dist_g1_scalar1, gas_rules(), "(* (* s A) B)" => "(* A (* s B))" }
egg::test_fn! { dist_g1_scalar2, gas_rules(),  "(* A (* s B))" => "(* s (* A B))" }
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
