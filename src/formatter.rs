use crate::{
    error::PestError::Unreachable,
    grammar::{PestParser, Rule},
    utils::GrammarRule,
    PestError, PestResult, Settings,
};
use pest::{iterators::Pair, Parser};
use std::{
    fs::{read_to_string, File},
    io::Write,
};
use text_utils::indent;

impl Settings {
    pub fn format_file(&self, path_from: &str, path_to: &str) -> PestResult<()> {
        let r = read_to_string(path_from)?;
        let s = self.format(&r)?;
        let mut file = File::create(path_to)?;
        file.write_all(s.as_bytes())?;
        Ok(())
    }
    pub fn format(&self, text: &str) -> PestResult<String> {
        let pairs = match PestParser::parse(Rule::grammar_rules, text) {
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
    fn format_grammar_rule(&self, pairs: Pair<Rule>) -> PestResult<GrammarRule> {
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


    fn format_module_decl(&self, pairs: Pair<Rule>) -> PestResult<String> {
        let mut code = String::new();
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
                            code.push_str(" ".repeat(self.indent).as_str());
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
        unimplemented!()
    }
    fn format_procedure_decl(&self, pairs: Pair<Rule>) -> PestResult<String> {
        let mut code = String::new();
        let raw = pairs.as_str();
        let mut counter = raw.count(":");

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
                }
                Rule::type_ => code.push_str(pair.as_str()),
                Rule::tau_list => {
                }
                Rule::procedure_body=>{

                }
                _ => return Err(Unreachable(unreachable_rule!())),
            };
        }
        Ok(code)
    }
    fn format_move_script(&self, pairs: Pair<Rule>) -> PestResult<String> {
        unimplemented!()
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