extern crate move_fmt;

use move_fmt::{PestResult, Settings};


#[test]
fn test_cases() -> PestResult<()> {
    let cfg = Settings::default();
    cfg.format_file("tests/td.move", "tests/out/td.move")
}
