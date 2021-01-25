use crate::language::GaRules;
use egg::rewrite as rw;

use crate::ring:: { ring_rules };

/**
 * Field is Ring + multiplicative commutativity
 * NOTE: The geometric algebra is _not_ a field.
 * Geometric Algebra is OVER a Vectorspace of a Field
*/
#[rustfmt::skip]
pub fn field_rules() -> GaRules {
    let mut rules = ring_rules();
    let extra = vec![
    rw!("field_mul_commutative"; "(* ?a ?b)" => "(* ?b ?a)" ),

    // rw!("field_multiplicative_inverse"; "(* ?a (inv ?a))" => "1"), // from group_mul_rules

    rw!("monoid_unit"; "(* ?a 1)" => "?a"), // sufficient due to commutativity
    rw!("mul_identity"; "(* ?a 0)" => "0"), // sufficient due to commutativity
    ];
    rules.extend(extra);
    rules
}

egg::test_fn! { field_comm, field_rules(), "(* a (+ 1 b))" =>  "(* (+ 1 b) a)"}
