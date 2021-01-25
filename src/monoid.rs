use egg::rewrite as rw;
use crate::language::GaRules;

/**
* Monoid rules
* https://ncatlab.org/nlab/show/monoid
* Monoid(op, neutral)
* Associativity and neutrality
*/

/** Monoid(+, 0)
*/
#[rustfmt::skip]
pub fn monoid_add_rules() -> GaRules {vec![
    rw!("monoid_add_unit_l"; "(+ ?a 0)" => "?a" ),
    rw!("monoid_add_unit_r"; "(+ 0 ?a)" => "?a" ),
    rw!("monoid_add_associative_l"; "(+ ?a (+ ?b ?c)))" => "(+ (+ ?a ?b) ?c))"),
    rw!("monoid_add_associative_r"; "(+ (+ ?a ?b) ?c))" => "(+ ?a (+ ?b ?c)))" ),
]}

/** Monoid(*, 1)
*/
#[rustfmt::skip]
pub fn monoid_mul_rules() -> GaRules {vec![
    rw!("monoid_mul_associative_l"; "(* ?a (* ?b ?c)))" => "(* (* ?a ?b) ?c))"),
    rw!("monoid_mul_associative_r"; "(* (* ?a ?b) ?c))" => "(* ?a (* ?b ?c)))" ),
    rw!("monoid_mul_unit_l"; "(* ?a 1)" => "?a" ),
    rw!("monoid_mul_unit_r"; "(* 1 ?a)" => "?a" ),
]}
