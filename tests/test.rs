extern crate move_fmt;

use move_fmt::{PestResult, Settings, grammar::{Rule, MoveParser}};
use pest::{parses_to, consumes_to};

#[test]
fn test_address() {
    parses_to! {
        parser: MoveParser,
        input: "0x123411f134",
        rule: Rule::address,
        tokens: [
            address(0, 12)
        ]
    };
}


#[test]
fn test_move() -> PestResult<()> {
    let mut cfg = Settings::default();
    cfg.format_file("tests/td.move", "tests/out/td.move")
}

#[test]
fn test_mvir() -> PestResult<()> {
    let mut cfg = Settings::default();
    cfg.format_file("tests/td.mvir", "tests/out/td.mvir")
}
