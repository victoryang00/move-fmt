#![feature(fs_try_exists)]
extern crate move_fmt;

use move_fmt::{PestResult, Settings, grammar::{Rule, MoveParser}};
use pest::{parses_to, consumes_to};
use std::fs;

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
    match  fs::try_exists("tests/out/") {
       Ok(false)=> fs::create_dir("tests/out/"),
        _ => Ok(())
    };
    cfg.format_file("tests/td.move", "tests/out/td.move")
}

#[test]
fn test_mvir() -> PestResult<()> {
    let mut cfg = Settings::default();
    match  fs::try_exists("tests/out/") {
        Ok(false)=> fs::create_dir("tests/out/"),
        _ => Ok(())
    };
    cfg.format_file("tests/td.mvir", "tests/out/td.mvir")
}
