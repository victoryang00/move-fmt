use crate::Settings;
use pest::Span;
use std::fmt::{Debug, Error, Formatter};

impl Default for Settings {
    fn default() -> Self {
        Settings {
            // tab = 4 space
            indent: 4,
            set_alignment: true,
            blank_lines: None,
            choice_first: true,
            choice_hanging: false,
            set_space: 0,
        }
    }
}

impl Settings {
    pub fn style(s: &str) -> Settings {
        match s {
            _ => Settings::default(),
        }
    }
}

pub fn is_one_line(span: Span) -> bool {
    let s = span.start_pos().line_col().0;
    let e = span.end_pos().line_col().0;
    return s == e;
}

pub fn get_lines(span: Span) -> (usize, usize) {
    let s = span.start_pos().line_col().0;
    let e = span.end_pos().line_col().0;
    return (s, e);
}

#[derive(Clone)]
pub struct GrammarRule {
    pub is_comment: bool,
    pub module_decl: String,
    pub transaction_script: String,
    pub code: String,
    pub lines: (usize, usize),
}

impl GrammarRule {
    pub fn comment(c: &str) -> Self {
        GrammarRule {
            /// is_comment_or_blank_line
            is_comment: true,
            module_decl: "".to_string(),
            transaction_script: "".to_string(),
            code: c.to_string(),
            lines: (0, 0),
        }
    }
    pub fn to_string(&self, indent: usize) -> String {
        if self.is_comment {
            return self.code.clone();
        }
        let mut code = self.transaction_script.clone();
        code.push_str(&self.module_decl);
        code.push_str(&self.code);
        return code;
    }
}

impl Debug for GrammarRule {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}: {:?}", self.transaction_script, self.lines)
    }
}
