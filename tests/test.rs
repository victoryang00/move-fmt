extern crate move_fmt;

use move_fmt::{PestResult, Settings,grammar::{Rule,MoveParser}};
use pest::{parses_to,consumes_to};

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
fn test_naive_move_script() {
    parses_to! {
        parser: MoveParser,
        input: "module M {
  public fun theRun() :u64{
  }
}",
        rule: Rule::move_script,
        tokens: [
            move_script(0, 12)
        ]
    };
}

#[test]
fn test_naive_spec_def() {
    parses_to! {
        parser: MoveParser,
        input:"spec fun is_valid_transaction_timestamp {
aborts_if timestamp <= 9223372036854;
aborts_if timestamp <= 9223372036854 && !exists<TTL>(0xA550C18);
aborts_if timestamp <= 9223372036854 && global<DiemTimestamp::CurrentTimeMicroseconds>(0xA550C18).microseconds + global<TTL>(0xA550C18).duration_microseconds > max_u64();
aborts_if timestamp <= 9223372036854 && timestamp * 1000000 > max_u64();
ensures timestamp > 9223372036854 ==> result == false;
ensures timestamp <= 9223372036854 ==> result == (global<DiemTimestamp::CurrentTimeMicroseconds>(0xA550C18).microseconds < timestamp * 1000000);
}",
        rule: Rule::spec_def,
        tokens: [
            spec_def(0, 12)
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
