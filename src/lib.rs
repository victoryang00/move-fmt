#![feature(fs_try_exists)]
extern crate pest;
#[cfg(test)]
extern crate pest_generator;
#[cfg(test)]
extern crate proc_macro;
#[cfg(test)]
#[macro_use]
extern crate quote;
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod pre_build;
#[macro_use]
mod error;
pub mod formatter;
pub mod grammar;
pub mod utils;

pub use error::{PestError, PestResult};

pub struct Settings {
    pub indent: usize,
    pub choice_hanging: bool,
    pub choice_first: bool,
    pub set_alignment: bool,
    pub blank_lines: Option<usize>,
    /// spaces between `=`
    pub set_space: usize,
    /// spaces between `else`
    pub set_else: usize,
    current_indent: usize,
}
