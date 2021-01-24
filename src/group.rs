use egg::rewrite as rw;
type GasRules = Vec<egg::Rewrite<egg::SymbolLang, ()>>;

use crate::monoid:: { monoid_mul_rules, monoid_add_rules };

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
pub fn group_add_rules() -> GasRules {
    let mut rules = monoid_add_rules();
    rules.extend(vec![
    rw!("group_add_inverse"; "(+ ?a (neg ?a))" => "0" ),
    rw!("group_minus_is_negation"; "(- ?a ?b)" => "(+ ?a (neg ?b))"),
    ]);
    rules
}

/** Monoid(*, 1) => Group(*, 1, inv)
 */
#[rustfmt::skip]
pub fn group_mul_rules() -> GasRules {
    let mut rules = monoid_mul_rules();
    let extra = vec![
    rw!("group_mul_inverse_l"; "(* ?a (inv ?a))" => "1" ),
    rw!("group_mul_inverse_r"; "(* (inv ?a) ?a)" => "1" ),
    rw!("group_div_is_inverse"; "(/ ?a ?b)" => "(* ?a (inv ?b))"),
    ];
    rules.extend(extra);
    rules
}
