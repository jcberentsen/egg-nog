use egg::rewrite as rw;
type GasRules = Vec<egg::Rewrite<egg::SymbolLang, ()>>;

use crate::group:: { group_add_rules };

/** Abelian group
 * https://ncatlab.org/nlab/show/abelian+group
 * Group + commutativity
 * Group(+, 0, neg) => AbelianGroup(+, 0, neg)
*/
#[rustfmt::skip]
pub fn abelian_group_rules() -> GasRules {
    let mut rules = group_add_rules();
    let extra = vec![
    rw!("abelian_group_add_commutative"; "(+ ?a ?b))" => "(+ ?b ?a)" ),
    ];
    rules.extend(extra);
    rules
}
