use egg::*;

fn main() {
    //! From egg tutorial code https://docs.rs/egg/0.6.0/egg/tutorials/_02_getting_started/index.html
    println!("Hello, egg-world!");

    // Since parsing can return an error, `unwrap` just panics if the result doesn't return Ok
    let my_expression: RecExpr<SymbolLang> = "(foo a b c)".parse().unwrap();
    println!("This is my expression {}", my_expression);

    let mut expr = RecExpr::default();
    let a = expr.add(SymbolLang::leaf("a"));
    println!("Expression a {}", a);

    let b = expr.add(SymbolLang::leaf("b"));
    println!("Expression b {}", a);
    let foo = expr.add(SymbolLang::new("foo", vec![a, b]));
    println!("Expression foo {}", foo);

    // we can do the same thing with an EGraph
    let mut egraph: EGraph<SymbolLang, ()> = Default::default();
    let a = egraph.add(SymbolLang::leaf("a"));
    let b = egraph.add(SymbolLang::leaf("b"));
    let foo = egraph.add(SymbolLang::new("foo", vec![a, b]));
    println!("EGraph foo {}", foo);

    // we can also add RecExprs to an egraph
    let foo2 = egraph.add_expr(&expr);
    // note that if you add the same thing to an e-graph twice, you'll get back equivalent Ids
    assert_eq!(foo, foo2);

    // let's make an e-graph
    let mut egraph: EGraph<SymbolLang, ()> = Default::default();
    let a = egraph.add(SymbolLang::leaf("a"));
    let b = egraph.add(SymbolLang::leaf("b"));
    let foo = egraph.add(SymbolLang::new("foo", vec![a, b]));
    println!("EGraph foo {}", foo);

    // we can make Patterns by parsing, similar to RecExprs
    // names preceded by ? are parsed as Pattern variables and will match anything
    let pat: Pattern<SymbolLang> = "(foo ?x ?x)".parse().unwrap();

    // since we use ?x twice, it must match the same thing,
    // so this search will return nothing
    let matches = pat.search(&egraph);
    assert!(matches.is_empty());

    egraph.union(a, b);
    // recall that rebuild must be called to "see" the effects of unions
    egraph.rebuild();

    // now we can find a match since a = b
    let matches = pat.search(&egraph);
    assert!(!matches.is_empty())
}
