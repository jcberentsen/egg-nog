use egg::rewrite as rw;

use crate::language::GaRules;

use crate::abelian::*;
use crate::group::*;

/**
 * Ring rules (Not commutative on *)
 * https://ncatlab.org/nlab/show/ring
 * Combination of Abelian Group(+, 0, neg) and Monoid (*, 1)
*/
#[rustfmt::skip]
pub fn ring_rules() -> GaRules {
    let mut rules = abelian_group_rules();
    rules.extend(group_mul_rules()); // or monoid_mul_rules skips mul inverse?
    let extra = vec![
    // Multiplication is distributive with regard to addition
    rw!("ring_distr-l"; "(* ?a (+ ?b ?c) )" => "(+ (* ?a ?b) (* ?a ?c))"),
    rw!("ring_distr-r"; "(* (+ ?b ?c) ?a )" => "(+ (* ?b ?a) (* ?c ?a))"),
    rw!("ring_negation"; "(neg ?a)" => "(* (neg 1) ?a)"),
    rw!("ring_neg_square"; "(* (neg 1) (neg 1))" => "1"), // NOTE squint and this looks like squared magnitude
    ];
    rules.extend(extra);
    rules
}

#[rustfmt::skip]
egg::test_fn! { double_neg, ring_rules(), "(neg (neg a))" =>  "a"}
// (neg (neg a))
// (neg (* (neg 1) a))
// (* (neg 1) (* (neg 1) a))
// (* (* (neg 1) (neg 1)) a) -- assoc
// (* 1 a) -- neg 1 * neg 1 = 1
// a
