extern crate move_fmt;

use move_fmt::{PestResult, Settings};


// #[test]
// fn test_move() -> PestResult<()> {
//     let cfg = Settings::default();
//     cfg.format_file("tests/td.move", "tests/out/td.move")
// }

#[test]
fn test_mvir() -> PestResult<()> {
    let cfg = Settings::default();
    cfg.format_file("tests/td.mvir", "tests/out/td.mvir")
}
