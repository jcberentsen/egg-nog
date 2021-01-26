use egg::{rewrite as rw};

pub type GaLanguage = egg::SymbolLang;
pub type GaRules = Vec<egg::Rewrite<GaLanguage, ()>>;
