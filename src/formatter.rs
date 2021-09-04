use crate::{
    error::PestError::Unreachable,
    grammar::{MoveParser, Rule},
    utils::GrammarRule,
    PestError, PestResult, Settings,
};
use pest::{iterators::Pair, Parser};
use std::{
    fs::{read_to_string, File},
    io::Write,
};
use text_utils::indent;
use std::borrow::Borrow;

#[macro_use]
macro_rules! move_unimplemented {
    () => (Ok("".to_string()));
    ($($arg:tt)+) => (Ok("{}".to_string(), $crate::format_args!($($arg)+)));
}

impl Settings {
    pub fn format_file(&mut self, path_from: &str, path_to: &str) -> PestResult<()> {
        let r = read_to_string(path_from)?;
        let s = self.format(&r)?;
        let mut file = File::create(path_to)?;
        file.write_all(s.as_bytes())?;
        Ok(())
    }
    pub fn format(&mut self, text: &str) -> PestResult<String> {
        let pairs = match MoveParser::parse(Rule::grammar_rules, text) {
            Ok(pairs) => pairs,
            Err(e) => return Err(PestError::ParseFail(e.to_string())),
        };
        let mut code = String::new();
        let mut codes = vec![];
        for pair in pairs {
            match pair.as_rule() {
                Rule::EOI => continue,
                Rule::COMMENT => {
                    let start = pair.as_span().start_pos().line_col().0;
                    let end = pair.as_span().end_pos().line_col().0;
                    codes.push(GrammarRule { is_comment: true, module_decl: String::new(), transaction_script: String::new(), code: pair.as_str().to_string(), lines: (start, end) })
                }
                Rule::grammar_rule => match self.format_grammar_rule(pair) {
                    Ok(rule) => codes.push(rule),
                    Err(e) => return Err(e),
                },
                Rule::WHITESPACE => continue,
                _ => return Err(Unreachable(unreachable_rule!())),
            };
        }
        let mut last = 0 as usize;
        let mut group = vec![];
        let mut groups = vec![];
        for i in codes {
            let (s, e) = i.lines;
            if last + 1 == s {
                group.push(i)
            } else {
                if group.len() != 0 {
                    groups.push(group);
                }
                group = vec![i]
            }
            last = e
        }
        groups.push(group);
        for g in groups {
            let mut length = vec![];
            for r in &g {
                length.push(r.transaction_script.chars().count())
            }
            let max = length.iter().max().unwrap();

            for r in &g {
                code.push_str(&r.to_string(*max));
                code.push_str("\n");
            }
            code.push_str("\n");
        }
        return Ok(code);
    }
    fn format_grammar_rule(&mut self, pairs: Pair<Rule>) -> PestResult<GrammarRule> {
        let mut code = String::new();
        let mut module_decl = " ".to_string();
        let mut transaction_script = String::new();
        let start = pairs.as_span().start_pos().line_col().0;
        let end = pairs.as_span().end_pos().line_col().0;
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::module_decl => match self.format_module_decl(pair) {
                    Ok(string) => module_decl = string,
                    Err(e) => return Err(e),
                },
                Rule::transaction_script => match self.format_transaction_script(pair) {
                    Ok(string) => transaction_script = string,
                    Err(e) => return Err(e),
                },
                Rule::move_script => match self.format_move_script(pair) {
                    Ok(string) => code.push_str(&string),
                    Err(e) => return Err(e),
                },
                _ => {}
            };
        }
        return Ok(GrammarRule { is_comment: false, module_decl, transaction_script, code, lines: (start, end) });
    }


    fn format_module_decl(&mut self, pairs: Pair<Rule>) -> PestResult<String> {
        let mut code = String::new();
        self.current_indent += self.indent;
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::COMMENT => match self.format_comment(pair) {
                    Ok(comment) => code.push_str(&*comment),
                    Err(e) => return Err(e),
                },
                Rule::module_name => {
                    code.push_str("module ");
                    code.push_str(pair.as_str());
                    code.push_str(" {\n");
                }
                Rule::import_decl => {
                    match self.format_import_decl(pair) {
                        Ok(import_decl) => {
                            code.push_str(" ".repeat(self.current_indent).as_str());
                            code.push_str(&*import_decl);
                            code.push_str("\n");
                        }
                        Err(e) => return Err(e),
                    }
                }
                Rule::struct_decl => code.push_str(pair.as_str()),
                Rule::procedure_decl => {
                    match self.format_procedure_decl(pair) {
                        Ok(format_procedure_decl) => {
                            code.push_str(" ".repeat(self.indent).as_str());
                            code.push_str(&*format_procedure_decl);
                            code.push_str("\n");
                        }
                        Err(e) => return Err(e),
                    }
                }
                _ => return Err(Unreachable(unreachable_rule!())),
            };
        }
        code.push_str("\n}\n");
        Ok(code)
    }
    fn format_transaction_script(&self, pairs: Pair<Rule>) -> PestResult<String> {
        move_unimplemented!()
    }
    fn format_procedure_decl(&mut self, pairs: Pair<Rule>) -> PestResult<String> {
        let mut code = String::new();
        let raw = pairs.as_str();
        let all = raw.matches(":").count();
        dbg!(all);
        let mut counter = 0;
        if raw.contains("public") {
            code.push_str("public ");
        }
        if raw.contains("native") {
            code.push_str("native ");
        }
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::COMMENT => match self.format_comment(pair) {
                    Ok(comment) => code.push_str(&*comment),
                    Err(e) => return Err(e),
                },
                Rule::procedure_name => {
                    code.push_str(pair.as_str());
                    code.push_str("(");
                }
                Rule::var => {
                    code.push_str(pair.as_str());
                    code.push_str(": ");
                    counter += 1;
                }
                Rule::ir_type => {
                    code.push_str(pair.as_str());
                    if counter < all {
                        code.push_str(")");
                    }
                }
                Rule::tau_list => match self.format_tau_list(pair) {
                    Ok(tau_list) => {
                        code.push_str(": ");
                        code.push_str(&*tau_list);
                    }
                    Err(e) => return Err(e),
                },
                Rule::procedure_body => match self.format_procedure_body(pair) {
                    Ok(procedure_body) => {
                        code.push_str("{ ");
                        code.push_str(&*procedure_body);

                        code.push_str("\n");
                        code.push_str(" ".repeat(self.current_indent).as_str());
                        code.push_str("}");
                    }
                    Err(e) => return Err(e),
                }
                _ => return Err(Unreachable(unreachable_rule!())),
            };
        }
        Ok(code)
    }
    fn format_procedure_body(&mut self, pairs: Pair<Rule>) -> PestResult<String> {
        let mut code = String::new();
        let raw = pairs.as_str();
        let all = raw.matches("let").count();
        dbg!(all);
        let mut counter = 0;
        if raw.contains("public") {
            code.push_str("public ");
        }
        if raw.contains("native") {
            code.push_str("native ");
        }
        self.current_indent += self.indent;
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::COMMENT => match self.format_comment(pair) {
                    Ok(comment) => code.push_str(&*comment),
                    Err(e) => return Err(e),
                },
                Rule::var => {
                    code.push_str("\n");
                    code.push_str(&*" ".repeat(self.current_indent));
                    code.push_str("let ");
                    code.push_str(pair.as_str());
                }
                Rule::ground_type => {
                    code.push_str(": ");
                    code.push_str(pair.as_str());
                    code.push_str(";");
                }
                Rule::stmtx => match self.format_stmtx(pair) {
                    Ok(stmtx) => {
                        code.push_str(&*stmtx);
                    }
                    Err(e) => return Err(e),
                },
                Rule::procedure_body => match self.format_procedure_body(pair) {
                    Ok(procedure_body) => {
                        code.push_str("{ ");
                        code.push_str(&*procedure_body);
                        code.push_str("\n{ ");
                    }
                    Err(e) => return Err(e),
                }
                _ => return Err(Unreachable(unreachable_rule!())),
            };
        }
        self.current_indent -= self.indent;

        Ok(code)
    }

    fn format_move_script(&self, pairs: Pair<Rule>) -> PestResult<String> {
        move_unimplemented!()
    }
    fn format_stmtx(&self, pairs: Pair<Rule>) -> PestResult<String> {
        let mut code = String::new();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::COMMENT => match self.format_comment(pair) {
                    Ok(comment) => code.push_str(&*comment),
                    Err(e) => return Err(e),
                },
                Rule::stmt => match self.format_stmtx(pair) {
                    Ok(stmt) => code.push_str(&*stmt),
                    Err(_) => {}
                },
                Rule::exp => match self.format_exp(pair) {
                    Ok(exp) => code.push_str(&*exp),
                    Err(_) => {}
                },
                Rule::cmd => match self.format_cmd(pair) {
                    Ok(cmd) => {
                        code.push_str(&*cmd);
                        code.push_str(";");
                    }
                    Err(_) => {}
                },
                Rule::stmtx => match self.format_stmtx(pair) {
                    Ok(stmt) => code.push_str(&*stmt),
                    Err(_) => {}
                },
                _ => return Err(Unreachable(unreachable_rule!())),
            };
        }
        Ok(code)
    }
    fn format_exp(&self, pairs: Pair<Rule>) -> PestResult<String> {
        let mut code = String::new();
        dbg!(pairs.borrow());
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::COMMENT => match self.format_comment(pair) {
                    Ok(comment) => code.push_str(&*comment),
                    Err(e) => return Err(e),
                },
                Rule::expx => match self.format_exp(pair) {
                    Ok(exp) => code.push_str(&*exp),
                    Err(_) => {}
                },
                Rule::value_operator => code.push_str(pair.as_str()),
                _ => return Err(Unreachable(unreachable_rule!())),
            };
        }
        Ok(code)
    }
    fn format_tau_list(&self, pairs: Pair<Rule>) -> PestResult<String> {
        let mut code = String::new();
        let raw = pairs.as_str();
        if raw.starts_with("unit") {
            code.push_str(" unit ");
        }
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::COMMENT => match self.format_comment(pair) {
                    Ok(comment) => code.push_str(&*comment),
                    Err(e) => return Err(e),
                },
                Rule::ir_type => {
                    code.push_str(pair.as_str());
                }
                _ => return Err(Unreachable(unreachable_rule!())),
            };
        }
        Ok(code)
    }

    fn format_cmd(&self, pairs: Pair<Rule>) -> PestResult<String> {
        let mut code = String::new();
        let raw = pairs.as_str();
        if raw.starts_with("unit") {
            code.push_str(" unit ");
        }
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::COMMENT => match self.format_comment(pair) {
                    Ok(comment) => code.push_str(&*comment),
                    Err(e) => return Err(e),
                },
                Rule::var => {
                    code.push_str(pair.as_str());
                }
                Rule::exp => {
                    code.push_str(pair.as_str());
                }
                Rule::struct_name => {
                    code.push_str(pair.as_str());
                }
                Rule::field_name => {
                    code.push_str(pair.as_str());
                }
                _ => return Err(Unreachable(unreachable_rule!())),
            };
        }
        Ok(code)
    }
    fn format_import_decl(&self, pairs: Pair<Rule>) -> PestResult<String> {
        let mut code = String::new();
        let mut counter = 0;
        code.push_str("import ");
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::COMMENT => match self.format_comment(pair) {
                    Ok(comment) => code.push_str(&*comment),
                    Err(e) => return Err(e),
                },
                Rule::address => {
                    code.push_str(pair.as_str());
                    code.push_str(".");
                }
                Rule::module_name => {
                    if counter == 1 {
                        code.push_str(" as ");
                    }
                    code.push_str(pair.as_str());
                    counter += 1;
                }
                _ => return Err(Unreachable(unreachable_rule!())),
            };
        }
        code.push_str(";");
        Ok(code)
    }
    fn format_comment(&self, pairs: Pair<Rule>) -> PestResult<String> {
        let mut code = String::new();
        let raw = pairs.as_str();
        if raw.starts_with("//") {
            code.push_str("//");
            code.push_str(raw[2..raw.len()].trim());
            code.push('\n')
        } else {
            // block comment
            code.push_str("/* ");
            code.push_str(raw[2..raw.len() - 2].trim());
            code.push_str(" */\n");
        }
        Ok(code)
    }
}