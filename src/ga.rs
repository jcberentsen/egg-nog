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
