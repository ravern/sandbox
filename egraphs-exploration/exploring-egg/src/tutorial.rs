use egg::{EGraph, Pattern, RecExpr, Searcher, SymbolLang};

pub fn run() {
    let mut add_expr = RecExpr::default();
    let x = add_expr.add(SymbolLang::leaf("x"));
    let y = add_expr.add(SymbolLang::leaf("y"));
    add_expr.add(SymbolLang::new("+", vec![x, y]));

    let mut e_graph = EGraph::<SymbolLang, ()>::default();
    e_graph.add_expr(&add_expr);
    e_graph.rebuild();

    let pattern: Pattern<SymbolLang> = "(+ ?a ?b)".parse().unwrap();

    let matches = pattern.search(&e_graph);
    println!("Matches found: {:?}", matches);
}
