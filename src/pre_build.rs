use pest_generator::derive_parser;
use std::{fs::File, io::prelude::*, path::Path};

#[test]
pub fn gen_parser() {
    let pest = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/grammar.pest"));
    let rs = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/grammar.rs"));

    let derived = {
        let path = pest.to_string_lossy();
        let pest = quote! {
            #[grammar = #path]
            pub struct MoveParser;
        };
        derive_parser(pest, false)
    };

    let mut file = File::create(rs).unwrap();
// TODO left recursive
    let test = "use pest::prec_climber::{Assoc, PrecClimber, Operator};


lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = build_precedence_climber();
}
fn build_precedence_climber() -> PrecClimber<Rule> {
    PrecClimber::new(vec![
        Operator::new(Rule::_exprx,  Assoc::Left),
        Operator::new(Rule::mul_expr_item, Assoc::Left),
        Operator::new(Rule::add_expr_item, Assoc::Left)
    ])
}";
    let out = format!("pub struct MoveParser;{}\n{}",test, derived);

    writeln!(file, "{}", out).unwrap();
}
