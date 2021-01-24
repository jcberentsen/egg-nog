
fn main() {
    //! Based on egg tutorial code https://docs.rs/egg/0.6.0/egg/tutorials/_02_getting_started/index.html
    println!("Run cargo test instead");
}

mod monoid;
mod group;
mod abelian;
mod ring;
mod field;
mod ga;
use crate::group:: { group_mul_rules };
use crate::ring::*;
use crate::ga::*;

#[rustfmt::skip]
egg::test_fn! { double_neg, ring_rules(), "(neg (neg a))" =>  "a"}
// (neg (neg a))
// (neg (* (neg 1) a))
// (* (neg 1) (* (neg 1) a))
// (* (* (neg 1) (neg 1)) a) -- assoc
// (* 1 a) -- neg 1 * neg 1 = 1
// a

egg::test_fn! { double_inv, group_mul_rules(), "(inv (inv a))" =>  "a"}
// inv (inv a))
// inv ()

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

