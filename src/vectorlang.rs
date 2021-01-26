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
        Scalar(i32), // TODO we probably want a more advanced scalar type
        "+" = Add([Id; 2]),
        Symbol(Symbol),
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TensorShape(u32);

use std::str::FromStr;

// impl<S: AsRef<str>> From<S> for TensorShape {
//     fn from(s: S) -> Self {
//         TensorShape(from_str(s))
//     }
// }

use std::num::ParseIntError;

impl FromStr for TensorShape {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.trim_matches(|p| p == '[' || p == ']' )
                                 .split(',')
                                 .collect();

        let n = coords[0].parse::<u32>()?;
        // let y_fromstr = coords[1].parse::<i32>()?;
        Ok(TensorShape(n))
    }
}

use std::fmt;
impl fmt::Display for TensorShape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Debug for TensorShape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Rules for TensorLang
/// Monoid +
#[rustfmt::skip]
pub fn rules() -> Vec<egg::Rewrite<TensorLang, ()>> {vec![
    rw!("monoid_add_unit_l"; "(+ ?a 0)" => "?a" ),
    rw!("monoid_add_unit_r"; "(+ 0 ?a)" => "?a" ),
    rw!("monoid_add_associative_l"; "(+ ?a (+ ?b ?c)))" => "(+ (+ ?a ?b) ?c))"),
    rw!("monoid_add_associative_r"; "(+ (+ ?a ?b) ?c))" => "(+ ?a (+ ?b ?c)))" ),
]}

egg::test_fn! { scalar_parse, rules(), "0" =>  "0"}
egg::test_fn! { sum_constant_fold, rules(), "(+ 0 1)" =>  "1"}
egg::test_fn! { vector_parse, rules(), "(Tensor [3] a b c)" =>  "(Tensor [3] a b c)"}
