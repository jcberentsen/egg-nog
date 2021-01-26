use egg::{rewrite as rw, *};

define_language! {
    /// Syntax
    /// Scalars 0 1 2 a b c
    /// Vectors x y z u v w
    /// (u + v)
    /// (a + b)
    /// See: https://docs.rs/egg/0.6.0/egg/macro.define_language.html
    /// Ref https://github.com/uwplse/szalinski/blob/master/src/cad.rs
    pub enum VectorLang {
        Scalar(Scalar), // TODO we probably want a more advanced scalar type
        "+" = Add([Id; 2]), // Can we constrain the children to Scalars here
        Symbol(Symbol),
    }
}

pub type EGraph = egg::EGraph<VectorLang, ConstantFold>;
pub type Rewrite = egg::Rewrite<VectorLang, ConstantFold>;
pub type Lang = VectorLang;

use ordered_float::NotNan;
pub type Scalar = NotNan<f64>;

/// Meta data for constant tracking
/// See https://github.com/egraphs-good/egg/blob/master/tests/math.rs
#[derive(Default)]
pub struct ConstantFold;
impl Analysis<Lang> for ConstantFold {
    type Data = Option<Scalar>;

    fn merge(&self, to: &mut Self::Data, from: Self::Data) -> bool {
        if let (Some(c1), Some(c2)) = (to.as_ref(), from.as_ref()) {
            assert_eq!(c1, c2);
        }
        merge_if_different(to, to.or(from))
    }

    fn make(egraph: &EGraph, enode: &Lang) -> Self::Data {
        let x = |i: &Id| egraph[*i].data;
        Some(match enode {
            Lang::Scalar(c) => *c,
            Lang::Add([a, b]) => x(a)? + x(b)?,
            //Math::Sub([a, b]) => x(a)? - x(b)?,
            //Math::Mul([a, b]) => x(a)? * x(b)?,
            //Math::Div([a, b]) if x(b) != Some(0.0.into()) => x(a)? / x(b)?,
            _ => return None,
        })
    }

    fn modify(egraph: &mut EGraph, id: Id) {
        let class = &mut egraph[id];
        if let Some(c) = class.data {
            let added = egraph.add(Lang::Scalar(c));
            let (id, _did_something) = egraph.union(id, added);
            // to not prune, comment this out
            // egraph[id].nodes.retain(|n| n.is_leaf());

            assert!(
                !egraph[id].nodes.is_empty(),
                "empty eclass! {:#?}",
                egraph[id]
            );
            #[cfg(debug_assertions)]
            egraph[id].assert_unique_leaves();
        }
    }
}


/// Rules for VectorLang
#[rustfmt::skip]
pub fn rules() -> Vec<egg::Rewrite<VectorLang, ConstantFold>> {vec![
    rw!("monoid_add_unit_l"; "(+ ?a 0)" => "?a" ),
    rw!("monoid_add_unit_r"; "(+ 0 ?a)" => "?a" ),
    rw!("monoid_add_associative_l"; "(+ ?a (+ ?b ?c)))" => "(+ (+ ?a ?b) ?c))"),
    rw!("monoid_add_associative_r"; "(+ (+ ?a ?b) ?c))" => "(+ ?a (+ ?b ?c)))" ),
]}

egg::test_fn! { scalar_parse, rules(), "0" =>  "0"}
egg::test_fn! { scalar_parse_pi, rules(), "3.14" =>  "3.14"}
egg::test_fn! { vector_parse, rules(), "u" =>  "u"}
egg::test_fn! { sum_constant_fold, rules(), "(+ 1 1)" =>  "2"}
// Types. Some arrows don't exist
// scalar + scalar is ok but
// scalar + vector is only available in GA
//
