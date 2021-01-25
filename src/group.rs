use crate::language::GaRules;
use egg::rewrite as rw;

use crate::monoid::{monoid_add_rules, monoid_mul_rules};

/**
 * Group rules
 * Monoid + inverses
 * https://ncatlab.org/nlab/show/group#internalization
 * A group object internal to a category CC with finite products is an object GG together with maps
 * mult:G×G→G
 * id:1→G
 * inv:G→G
 * such that various diagrams expressing associativity, unitality, and inverses commute.
*/

/** Monoid(+, 0) => Group(+, 0, neg)
 */
#[rustfmt::skip]
pub fn group_add_rules() -> GaRules {
    let mut rules = monoid_add_rules();
    rules.extend(vec![
    rw!("group_add_inverse"; "(+ ?a (neg ?a))" => "0" ),
    rw!("group_minus_is_negation"; "(- ?a ?b)" => "(+ ?a (neg ?b))"),
    rw!("group_unary_minus_is_negation"; "(- ?a)" => "(neg ?a)"),

    rw!("group_inverse_l"; "(+ ?a (neg ?a))" => "0"),
    rw!("group_inverse_r"; "(+ (neg ?a) ?a)" => "0"),
    rw!("group_double_neg"; "(neg (neg ?a))" => "?a"), // inverse is unique - also see ring_double_neg
    ]);
    rules
}

egg::test_fn! {double_neg, group_add_rules(), "(- (- a))" =>  "a"} // see ring
egg::test_fn! {cancel_l, group_add_rules(), "(+ a (- a))" =>  "0"}
egg::test_fn! {cancel_r, group_add_rules(), "(+ (- a) a)" =>  "0"}

egg::test_fn! {
    #[should_panic(expected = "Could not prove goal 0")]
    group_not_commutative, group_add_rules(), "(+ a b)" =>  "(+ b a)"
}

/** Monoid(*, 1) => Group(*, 1, inv)
 */
#[rustfmt::skip]
pub fn group_mul_rules() -> GaRules {
    let mut rules = monoid_mul_rules();
    let extra = vec![
    rw!("group_mul_inverse_l"; "(* ?a (inv ?a))" => "1" ),
    rw!("group_mul_inverse_r"; "(* (inv ?a) ?a)" => "1" ),
    rw!("group_div_is_inverse"; "(/ ?a ?b)" => "(* ?a (inv ?b))"),
    rw!("group_double_inverse"; "(inv (inv ?a))" => "?a"), // inv is unique
    ];
    rules.extend(extra);
    rules
}

egg::test_fn! {double_inv, group_mul_rules(), "(inv (inv a))" =>  "a"}
