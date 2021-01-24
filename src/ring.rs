use egg::rewrite as rw;
type GasRules = Vec<egg::Rewrite<egg::SymbolLang, ()>>;

use crate::abelian::*;
use crate::group::*;

/**
 * Ring rules (Not commutative on *)
 * https://ncatlab.org/nlab/show/ring
 * Combination of Abelian Group(+, 0, neg) and Monoid (*, 1)
*/
#[rustfmt::skip]
pub fn ring_rules() -> GasRules {
    let mut rules = abelian_group_rules();
    rules.extend(group_mul_rules()); // or monoid_mul_rules skips mul inverse?
    let extra = vec![
    // Multiplication is distributive with regard to addition
    rw!("ring_distr-l"; "(* ?a (+ ?b ?c) )" => "(+ (* ?a ?b) (* ?a ?c))"),
    rw!("ring_distr-r"; "(* (+ ?b ?c) ?a )" => "(+ (* ?b ?a) (* ?c ?a))"),
    rw!("ring_negation"; "(neg ?a)" => "(* (neg 1) ?a)"),
    rw!("ring_neg_square"; "(* (neg 1) (neg 1))" => "1"),
    ];
    rules.extend(extra);
    rules
}
