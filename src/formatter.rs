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
use crate::grammar::Rule::cmd;

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
        let mut code = String::new();
        let mut count = pairs.as_str().matches(",").count();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::COMMENT => match self.format_comment(pair) {
                    Ok(comment) => code.push_str(&*comment),
                    Err(e) => return Err(e),
                },
                Rule::import_decl => {
                    match self.format_import_decl(pair) {
                        Ok(tau_list) => {
                            code.push_str(&*tau_list);
                        }
                        Err(e) => return Err(e),
                    }
                }
                Rule::var => {
                    code.push_str(pair.as_str());
                    code.push_str(": ");
                }
                Rule::ground_type => {
                    code.push_str(pair.as_str());
                    if count > 0 {
                        code.push_str(", ");
                        count -= 1;
                    }
                }
                Rule::left_column => {
                    if pair.as_str().contains("public") {
                        code.push_str(" public ");
                    }
                    code.push_str("main");

                    code.push_str(pair.as_str());
                }
                Rule::right_column => {
                    code.push_str(pair.as_str());
                }
                _ => return Err(Unreachable(unreachable_rule!())),
            };
        }
        Ok(code)
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
                }
                Rule::var => {
                    code.push_str(pair.as_str());
                    code.push_str(": ");
                    counter += 1;
                }
                Rule::ir_type => {
                    code.push_str(pair.as_str());
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
                Rule::left_column => {
                    code.push_str("");
                    code.push_str(pair.as_str())
                }
                Rule::right_column => {
                    code.push_str(pair.as_str());
                    code.push_str("");
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
                        code.push_str("\n");
                        code.push_str(&*" ".repeat(self.current_indent));
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
    fn format_move_script(&mut self, pairs: Pair<Rule>) -> PestResult<String> {
        let mut code = String::new();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::COMMENT => match self.format_comment(pair) {
                    Ok(comment) => code.push_str(&*comment),
                    Err(e) => return Err(e),
                },
                Rule::script_def => match self.format_script_def(pair) {
                    Ok(stmt) => code.push_str(&*stmt),
                    Err(e) => return Err(e),
                },
                Rule::address_def => match self.format_address_def(pair) {
                    Ok(exp) => code.push_str(&*exp),
                    Err(e) => return Err(e),
                },
                Rule::module_def => match self.format_module_def(pair) {
                    Ok(cmd_) => {
                        code.push_str(&*cmd_);
                    }
                    Err(e) => return Err(e),
                },
                Rule::stmtx => match self.format_stmtx(pair) {
                    Ok(stmt) => code.push_str(&*stmt),
                    Err(e) => return Err(e),
                },
                _ => return Err(Unreachable(unreachable_rule!())),
            };
        }
        Ok(code)
    }
    fn format_script_def(&mut self, pairs: Pair<Rule>) -> PestResult<String> {
        let mut code = String::new();
        code.push_str("script {\n");
        code.push_str(" ".repeat(self.current_indent).as_str());
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::COMMENT => match self.format_comment(pair) {
                    Ok(comment) => code.push_str(&*comment),
                    Err(e) => return Err(e),
                },
                Rule::import_stmt => match self.format_import_stmt(pair) {
                    Ok(import_stm_) => code.push_str(&*import_stm_),
                    Err(e) => return Err(e),
                },
                Rule::const_def => match self.format_const_def(pair) {
                    Ok(const_def_) => code.push_str(&*const_def_),
                    Err(e) => return Err(e),
                },
                Rule::function_def => match self.format_function_def(pair) {
                    Ok(function_def_) =>
                        code.push_str(&*function_def_),
                    Err(e) => return Err(e),
                },
                _ => return Err(Unreachable(unreachable_rule!())),
            };
        }
        code.push_str("\n");
        code.push_str(" ".repeat(self.current_indent).as_str());
        code.push_str("}");
        Ok(code)
    }
    fn format_function_def(&mut self, pairs: Pair<Rule>) -> PestResult<String> {
        let mut code = String::new();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::COMMENT => match self.format_comment(pair) {
                    Ok(comment) => code.push_str(&*comment),
                    Err(e) => return Err(e),
                },
                Rule::function_sig_withoptionalvisibility => match self.format_function_sig_withoptionalvisibility(pair) {
                    Ok(function_sig_withoptionalvisibility_) => code.push_str(&*function_sig_withoptionalvisibility_),
                    Err(e) => return Err(e),
                },
                Rule::codeblock => match self.format_codeblock(pair) {
                    Ok(codeblock) => code.push_str(&*codeblock),
                    Err(e) => return Err(e),
                },
                _ => return Err(Unreachable(unreachable_rule!())),
            };
        }
        Ok(code)
    }
    fn format_codeblock(&mut self, pairs: Pair<Rule>) -> PestResult<String> {
        let mut code = String::new();
        code.push_str("{ ");
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::COMMENT => match self.format_comment(pair) {
                    Ok(comment) => code.push_str(&*comment),
                    Err(e) => return Err(e),
                },
                Rule::ident => code.push_str(pair.as_str()),
                Rule::codeblockitems => match self.format_codeblock(pair) {
                    Ok(typeannotation) => code.push_str(&*typeannotation),
                    Err(e) => return Err(e),
                },
                Rule::import_stmt => match self.format_import_stmt(pair) {
                    Ok(cmd_) => code.push_str(&*cmd_),
                    Err(e) => return Err(e),
                },
                Rule::_stmt => match self.format_stmt_(pair) {
                    Ok(stmt) => code.push_str(&*stmt),
                    Err(e) => return Err(e),
                },
                Rule::_expr => match self.format_expr_(pair) {
                    Ok(stmt) => code.push_str(&*stmt),
                    Err(e) => return Err(e),
                },
                Rule::blockspec_stmt => {
                    code.push_str("spec ");
                    match self.format_stmt_(pair) {
                        Ok(stmt) => code.push_str(&*stmt),
                        Err(e) => return Err(e),
                    }
                    code.push_str(";");
                }
                _ => return Err(Unreachable(unreachable_rule!())),
            };
        }
        code.push_str("}");

        Ok(code)
    }
    fn format_expr_(&mut self, pairs: Pair<Rule>) -> PestResult<String> {
        let mut code = String::new();
        let mut plus_count = 0;
        let mut count = 0;
        let mut is_index = false;
        let mut is_range = false;
        let mut is_lambda = false;

        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::COMMENT => match self.format_comment(pair) {
                    Ok(comment) => code.push_str(&*comment),
                    Err(e) => return Err(e),
                },
                Rule::ident => {
                    code.push_str(pair.as_str());
                    if count > 0 {
                        code.push_str(", ");
                        count -= 1;
                    }
                }
                Rule::controlflow_expr => match self.format_expr_(pair) {
                    Ok(comment) => code.push_str(&*comment),
                    Err(e) => return Err(e),
                },
                Rule::if_expr => match self.format_expr_(pair) {
                    Ok(comment) => code.push_str(&*comment),
                    Err(e) => return Err(e),
                },
                Rule::condition => match self.format_expr_(pair) {
                    Ok(comment) => code.push_str(&*comment),
                    Err(e) => return Err(e),
                },
                Rule::conditionbody => match self.format_expr_(pair) {
                    Ok(comment) => code.push_str(&*comment),
                    Err(e) => return Err(e),
                },
                Rule::_expr => {
                    if is_index {
                        code.push_str("[")
                    }
                    if is_range {
                        code.push_str("..");
                        is_range = false;
                    }
                    if is_lambda {
                        code.push_str("|");
                        is_lambda = false;
                    }
                    match self.format_expr_(pair) {
                        Ok(comment) => code.push_str(&*comment),
                        Err(e) => return Err(e),
                    }
                    if is_index {
                        code.push_str("]");
                        is_index = false;
                    }
                }
                Rule::anyblock => match self.format_expr_(pair) {
                    Ok(comment) => code.push_str(&*comment),
                    Err(e) => return Err(e),
                },
                Rule::codeblock => match self.format_codeblock(pair) {
                    Ok(comment) => code.push_str(&*comment),
                    Err(e) => return Err(e),
                }
                Rule::inlineblock => match self.format_expr_(pair) {
                    Ok(comment) => code.push_str(&*comment),
                    Err(e) => return Err(e),
                }
                Rule::elseblock => {
                    code.push_str("else ");
                    match self.format_expr_(pair) {
                        Ok(comment) => code.push_str(&*comment),
                        Err(e) => return Err(e),
                    }
                }
                Rule::loop_expr => {
                    code.push_str("loop ");
                    match self.format_expr_(pair) {
                        Ok(comment) => code.push_str(&*comment),
                        Err(e) => return Err(e),
                    }
                }
                Rule::while_expr => {
                    code.push_str("while ");
                    match self.format_expr_(pair) {
                        Ok(comment) => code.push_str(&*comment),
                        Err(e) => return Err(e),
                    }
                }
                Rule::unary_expr => {
                    match self.format_expr_(pair) {
                        Ok(comment) => code.push_str(&*comment),
                        Err(e) => return Err(e),
                    }
                }
                Rule::copy_expr => {
                    code.push_str("copy ");
                    match self.format_expr_(pair) {
                        Ok(comment) => code.push_str(&*comment),
                        Err(e) => return Err(e),
                    }
                }
                Rule::move_expr => {
                    if pair.as_str().starts_with("move_to") {
                        code.push_str("move_to ");
                    } else {
                        code.push_str("move");
                    }
                    match self.format_expr_(pair) {
                        Ok(comment) => code.push_str(&*comment),
                        Err(e) => return Err(e),
                    }
                }
                Rule::deref_expr => {
                    code.push_str("*");
                    match self.format_expr_(pair) {
                        Ok(comment) => code.push_str(&*comment),
                        Err(e) => return Err(e),
                    }
                }
                Rule::bang_expr => {
                    code.push_str("!");
                    match self.format_expr_(pair) {
                        Ok(comment) => code.push_str(&*comment),
                        Err(e) => return Err(e),
                    }
                }
                Rule::return_expr => {
                    code.push_str("return ");
                    match self.format_expr_(pair) {
                        Ok(comment) => code.push_str(&*comment),
                        Err(e) => return Err(e),
                    }
                }
                Rule::abort_expr => {
                    code.push_str("abort ");
                    match self.format_expr_(pair) {
                        Ok(comment) => code.push_str(&*comment),
                        Err(e) => return Err(e),
                    }
                }
                Rule::break_expr => {
                    code.push_str("break");
                }
                Rule::continue_expr => {
                    code.push_str("continue");
                }
                Rule::borrow_expr => {
                    code.push_str("&");
                    if pair.as_str().contains("mut") {
                        code.push_str(" mut ");
                    }
                    code.push_str(pair.as_str());
                }
                Rule::atom_expr => match self.format_expr_(pair) {
                    Ok(comment) => code.push_str(&*comment),
                    Err(e) => return Err(e),
                }
                Rule::annotated_expr => match self.format_expr_(pair) {
                    Ok(comment) => code.push_str(&*comment),
                    Err(e) => return Err(e),
                }
                Rule::tupleliteral_expr => match self.format_expr_(pair) {
                    Ok(comment) => code.push_str(&*comment),
                    Err(e) => return Err(e),
                }
                Rule::emptytuple_expr => match self.format_expr_(pair) {
                    Ok(comment) => code.push_str(&*comment),
                    Err(e) => return Err(e),
                }
                Rule::tuple_expr => {
                    count += pair.as_str().matches(",").count();
                    match self.format_expr_(pair) {
                        Ok(comment) => code.push_str(&*comment),
                        Err(e) => return Err(e),
                    }
                }
                Rule::annotatedexpprefix => {
                    match self.format_expr_(pair) {
                        Ok(comment) => code.push_str(&*comment),
                        Err(e) => return Err(e),
                    }
                    code.push_str(":");
                }
                Rule::parens_expr => {
                    match self.format_expr_(pair) {
                        Ok(comment) => code.push_str(&*comment),
                        Err(e) => return Err(e),
                    }
                }
                Rule::structliteral_expr => {
                    match self.format_expr_(pair) {
                        Ok(comment) => code.push_str(&*comment),
                        Err(e) => return Err(e),
                    }
                }
                Rule::qual_pathtype => match self.format_typeannotation(pair) {
                    Ok(cmd_) => {
                        code.push_str(&*cmd_);
                    }
                    Err(e) => return Err(e),
                }
                Rule::structliteral_fieldsblock => {
                    code.push_str("{");
                    match self.format_expr_(pair) {
                        Ok(cmd_) => {
                            code.push_str(&*cmd_);
                        }
                        Err(e) => return Err(e),
                    }
                    code.push_str("}");
                }
                Rule::structliteral_field => {
                    match self.format_expr_(pair) {
                        Ok(cmd_) => {
                            code.push_str(&*cmd_);
                        }
                        Err(e) => return Err(e),
                    }
                }
                Rule::structliteral_fieldassignment => {
                    code.push_str(":");
                    match self.format_expr_(pair) {
                        Ok(cmd_) => {
                            code.push_str(&*cmd_);
                        }
                        Err(e) => return Err(e),
                    }
                }
                Rule::dot_expr => match self.format_expr_(pair) {
                    Ok(cmd_) => {
                        code.push_str(&*cmd_);
                    }
                    Err(e) => return Err(e),
                }
                Rule::structfieldref => {
                    code.push_str(".");
                    match self.format_expr_(pair) {
                        Ok(cmd_) => {
                            code.push_str(&*cmd_);
                        }
                        Err(e) => return Err(e),
                    }
                }
                Rule::index_expr => {
                    match self.format_expr_(pair) {
                        Ok(cmd_) => {
                            code.push_str(&*cmd_);
                        }
                        Err(e) => return Err(e),
                    }
                }
                Rule::call_expr => {
                    match self.format_expr_(pair) {
                        Ok(cmd_) => {
                            code.push_str(&*cmd_);
                        }
                        Err(e) => return Err(e),
                    }
                }
                Rule::ref_expr => {
                    match self.format_expr_(pair) {
                        Ok(cmd_) => {
                            code.push_str(&*cmd_);
                        }
                        Err(e) => return Err(e),
                    }
                }
                Rule::lambda_expr => {
                    count += pair.as_str().matches(",").count();
                    is_lambda = true;
                    code.push_str("|");
                    match self.format_expr_(pair) {
                        Ok(cmd_) => {
                            code.push_str(&*cmd_);
                        }
                        Err(e) => return Err(e),
                    }
                }
                Rule::callarguments => {
                    count += pair.as_str().matches(",").count();
                    match self.format_expr_(pair) {
                        Ok(cmd_) => {
                            code.push_str(&*cmd_);
                        }
                        Err(e) => return Err(e),
                    }
                }
                Rule::_exprx => {
                    match self.format_expr_(pair) {
                        Ok(cmd_) => {
                            code.push_str(&*cmd_);
                        }
                        Err(e) => return Err(e),
                    }
                }
                Rule::type_ => {
                    match self.format_typeannotation(pair) {
                        Ok(cmd_) => {
                            code.push_str(&*cmd_);
                        }
                        Err(e) => return Err(e),
                    }
                }
                Rule::codeblock_expr => {
                    match self.format_expr_(pair) {
                        Ok(cmd_) => {
                            code.push_str(&*cmd_);
                        }
                        Err(e) => return Err(e),
                    }
                }
                Rule::_expr_op => code.push_str(pair.as_str()),
                Rule::ability => {
                    if pair.as_str().starts_with("copy") {
                        code.push_str("copy ");
                    }
                    match self.format_function_sig_withoptionalvisibility(pair) {
                        Ok(function_sig_visibility_) => code.push_str(&*function_sig_visibility_),
                        Err(e) => return Err(e),
                    }
                    if plus_count > 0 {
                        code.push_str(" + ");
                        plus_count -= 1;
                    }
                }
                Rule::typeparameter_with_recover => match self.format_function_sig_withoptionalvisibility(pair) {
                    Ok(function_sig_visibility_) => code.push_str(&*function_sig_visibility_),
                    Err(e) => return Err(e),
                },
                Rule::typeparameterlist => {
                    code.push_str("<");
                    match self.format_function_sig_withoptionalvisibility(pair) {
                        Ok(function_sig_visibility_) => code.push_str(&*function_sig_visibility_),
                        Err(e) => return Err(e),
                    }
                    code.push_str(">");
                }
                Rule::function_sig_visibility => match self.format_function_sig_withoptionalvisibility(pair) {
                    Ok(function_sig_visibility_) => code.push_str(&*function_sig_visibility_),
                    Err(e) => return Err(e),
                },
                Rule::function_sig_ => match self.format_function_sig_withoptionalvisibility(pair) {
                    Ok(function_sig_visibility_) => {
                        code.push_str("fun ");
                        code.push_str(&*function_sig_visibility_)
                    }
                    Err(e) => return Err(e),
                },
                Rule::function_parameterlist => {
                    count += pair.as_str().matches(",").count();
                    match self.format_function_sig_withoptionalvisibility(pair) {
                        Ok(stmt) => code.push_str(&*stmt),
                        Err(e) => return Err(e),
                    }
                }
                Rule::function_parameter => {
                    match self.format_function_sig_withoptionalvisibility(pair) {
                        Ok(stmt) => code.push_str(&*stmt),
                        Err(e) => return Err(e),
                    }
                    if count > 0 {
                        code.push_str(", ");
                        count -= 1;
                    }
                }
                Rule::typeannotation => match self.format_typeannotation(pair) {
                    Ok(typeannotation) => code.push_str(&*typeannotation),
                    Err(e) => return Err(e),
                },
                Rule::literal_if => code.push_str(pair.as_str()),
                Rule::left_column => code.push_str(pair.as_str()),
                Rule::right_column => code.push_str(pair.as_str()),
                _ => return Err(Unreachable(unreachable_rule!())),
            };
        }
        Ok(code)
    }
    fn format_stmt_(&mut self, pairs: Pair<Rule>) -> PestResult<String> {//TODO
        let mut code = String::new();
        let mut is_dot = false;
        let mut count = 0;
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::COMMENT => match self.format_comment(pair) {
                    Ok(comment) => code.push_str(&*comment),
                    Err(e) => return Err(e),
                },
                Rule::ident => {
                    code.push_str(pair.as_str());
                    if is_dot {
                        code.push_str(".");
                        is_dot = false;
                    }
                }
                Rule::left_column => code.push_str(pair.as_str()),
                Rule::equal => code.push_str(pair.as_str()),
                Rule::wild_pat => code.push_str(pair.as_str()),
                Rule::right_column => code.push_str(pair.as_str()),
                Rule::let_stmt => {
                    code.push_str("let ");
                    match self.format_stmt_(pair) {
                        Ok(stmt_) => code.push_str(&*stmt_),
                        Err(e) => return Err(e),
                    }
                    code.push_str(";");
                }
                Rule::specblock => {
                    code.push_str("{");
                    match self.format_stmt_(pair) {
                        Ok(stmt_) => code.push_str(&*stmt_),
                        Err(e) => return Err(e),
                    }
                    code.push_str("}");
                }
                Rule::specblock_stmt => {
                    match self.format_stmt_(pair) {
                        Ok(stmt_) => code.push_str(&*stmt_),
                        Err(e) => return Err(e),
                    }
                }
                Rule::variable_stmt => {
                    match self.format_stmt_(pair) {
                        Ok(stmt_) => code.push_str(&*stmt_),
                        Err(e) => return Err(e),
                    }
                    code.push_str(";");
                }
                Rule::localvariable_stmt => {
                    code.push_str("local ");
                    match self.format_stmt_(pair) {
                        Ok(stmt_) => code.push_str(&*stmt_),
                        Err(e) => return Err(e),
                    }
                    code.push_str(";");
                }
                Rule::globalvariable_stmt => {
                    code.push_str("global ");
                    match self.format_stmt_(pair) {
                        Ok(stmt_) => code.push_str(&*stmt_),
                        Err(e) => return Err(e),
                    }
                    code.push_str(";");
                }
                Rule::import_stmt => {
                    match self.format_import_stmt(pair) {
                        Ok(stmt_) => code.push_str(&*stmt_),
                        Err(e) => return Err(e),
                    }
                }
                Rule::nat_function_def => {
                    code.push_str("native ");
                    match self.format_function_sig_withoptionalvisibility(pair) {
                        Ok(stmt_) => code.push_str(&*stmt_),
                        Err(e) => return Err(e),
                    }
                    code.push_str(";");
                }
                Rule::_definefunction_ => {
                    match self.format_stmt_(pair) {
                        Ok(stmt_) => code.push_str(&*stmt_),
                        Err(e) => return Err(e),
                    }
                }
                Rule::_definefunction_sig_ => {
                    code.push_str("define ");
                    match self.format_function_sig_withoptionalvisibility(pair) {
                        Ok(stmt_) => code.push_str(&*stmt_),
                        Err(e) => return Err(e),
                    }
                }
                Rule::ref_expr => {
                    match self.format_stmt_(pair) {
                        Ok(stmt_) => code.push_str(&*stmt_),
                        Err(e) => return Err(e),
                    }
                    code.push_str(";");
                }
                Rule::specblockfinishing_expr => {
                    match self.format_stmt_(pair) {
                        Ok(stmt_) => code.push_str(&*stmt_),
                        Err(e) => return Err(e),
                    }
                }
                Rule::spec_expr => {
                    match self.format_expr_(pair) {
                        Ok(stmt_) => code.push_str(&*stmt_),
                        Err(e) => return Err(e),
                    }
                }
                Rule::aggregate_expr => {
                    match self.format_expr_(pair) {
                        Ok(stmt_) => code.push_str(&*stmt_),
                        Err(e) => return Err(e),
                    }
                }
                Rule::_exprx => {
                    match self.format_expr_(pair) {
                        Ok(stmt_) => code.push_str(&*stmt_),
                        Err(e) => return Err(e),
                    }
                }
                Rule::_stmt_expr => {
                    match self.format_expr_(pair) {
                        Ok(stmt_) => code.push_str(&*stmt_),
                        Err(e) => return Err(e),
                    }
                    code.push_str(";");
                }
                Rule::_pat => {
                    match self.format_stmt_(pair) {
                        Ok(stmt_) => code.push_str(&*stmt_),
                        Err(e) => return Err(e),
                    }
                    if count > 0 {
                        code.push_str(", ");
                        count -= 1;
                    }
                }
                Rule::_expr => {
                    match self.format_expr_(pair) {
                        Ok(stmt_) => code.push_str(&*stmt_),
                        Err(e) => return Err(e),
                    }
                }
                Rule::deref_pat => {
                    code.push_str("*");
                    match self.format_stmt_(pair) {
                        Ok(stmt_) => code.push_str(&*stmt_),
                        Err(e) => return Err(e),
                    }
                }
                Rule::borrow_pat => {
                    code.push_str("&");
                    match self.format_stmt_(pair) {
                        Ok(stmt_) => code.push_str(&*stmt_),
                        Err(e) => return Err(e),
                    }
                }
                Rule::binding_pat => {
                    match self.format_stmt_(pair) {
                        Ok(stmt_) => code.push_str(&*stmt_),
                        Err(e) => return Err(e),
                    }
                }
                Rule::dot_pat => {
                    is_dot = true;
                    match self.format_stmt_(pair) {
                        Ok(stmt_) => code.push_str(&*stmt_),
                        Err(e) => return Err(e),
                    }
                }
                Rule::struct_pat => {
                    match self.format_stmt_(pair) {
                        Ok(stmt_) => code.push_str(&*stmt_),
                        Err(e) => return Err(e),
                    }
                }
                Rule::struct_patfieldsblock => {
                    code.push_str("{");
                    match self.format_stmt_(pair) {
                        Ok(stmt_) => code.push_str(&*stmt_),
                        Err(e) => return Err(e),
                    }
                    code.push_str("}");
                }
                Rule::struct_patfieldbinding => {
                    code.push_str(":");
                    match self.format_stmt_(pair) {
                        Ok(stmt_) => code.push_str(&*stmt_),
                        Err(e) => return Err(e),
                    }
                }
                Rule::qual_path => match self.format_typeannotation(pair) {
                    Ok(cmd_) => {
                        code.push_str(&*cmd_);
                    }
                    Err(e) => return Err(e),
                },
                Rule::tuple_pat => {
                    count += pair.as_str().matches(",").count();
                    match self.format_stmt_(pair) {
                        Ok(cmd_) => {
                            code.push_str(&*cmd_);
                        }
                        Err(e) => return Err(e),
                    }
                }

                Rule::typeannotation => match self.format_typeannotation(pair) {
                    Ok(typeannotation) => code.push_str(&*typeannotation),
                    Err(e) => return Err(e),
                },
                _ => return Err(Unreachable(unreachable_rule!())),
            };
        }
        Ok(code)
    }
    fn format_function_sig_withoptionalvisibility(&mut self, pairs: Pair<Rule>) -> PestResult<String> {
        let mut code = String::new();
        let mut plus_count = 0;
        let mut count = 0;

        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::COMMENT => match self.format_comment(pair) {
                    Ok(comment) => code.push_str(&*comment),
                    Err(e) => return Err(e),
                },
                Rule::ident => code.push_str(pair.as_str()),
                Rule::left_column => code.push_str(pair.as_str()),
                Rule::right_column => code.push_str(pair.as_str()),
                Rule::typeparambound => {
                    code.push_str(":");
                    match self.format_function_sig_withoptionalvisibility(pair) {
                        Ok(function_sig_visibility_) => code.push_str(&*function_sig_visibility_),
                        Err(e) => return Err(e),
                    }
                }
                Rule::typeparambound_items => {
                    plus_count = pair.as_str().matches("+").count();
                    match self.format_function_sig_withoptionalvisibility(pair) {
                        Ok(function_sig_visibility_) => code.push_str(&*function_sig_visibility_),
                        Err(e) => return Err(e),
                    }
                }
                Rule::ability => {
                    code.push_str("copy ");
                    match self.format_function_sig_withoptionalvisibility(pair) {
                        Ok(function_sig_visibility_) => code.push_str(&*function_sig_visibility_),
                        Err(e) => return Err(e),
                    }
                    if plus_count > 0 {
                        code.push_str(" + ");
                        plus_count -= 1;
                    }
                }
                Rule::typeparameter_with_recover => match self.format_function_sig_withoptionalvisibility(pair) {
                    Ok(function_sig_visibility_) => code.push_str(&*function_sig_visibility_),
                    Err(e) => return Err(e),
                },
                Rule::typeparameterlist => {
                    code.push_str("<");
                    match self.format_function_sig_withoptionalvisibility(pair) {
                        Ok(function_sig_visibility_) => code.push_str(&*function_sig_visibility_),
                        Err(e) => return Err(e),
                    }
                    code.push_str(">");
                }
                Rule::function_sig_visibility => match self.format_function_sig_withoptionalvisibility(pair) {
                    Ok(function_sig_visibility_) => code.push_str(&*function_sig_visibility_),
                    Err(e) => return Err(e),
                },
                Rule::function_sig_ => match self.format_function_sig_withoptionalvisibility(pair) {
                    Ok(function_sig_visibility_) => {
                        code.push_str("fun ");
                        code.push_str(&*function_sig_visibility_)
                    }
                    Err(e) => return Err(e),
                },
                Rule::function_parameterlist => {
                    count = pair.as_str().matches(",").count();
                    match self.format_function_sig_withoptionalvisibility(pair) {
                        Ok(stmt) => code.push_str(&*stmt),
                        Err(e) => return Err(e),
                    }
                }
                Rule::function_parameter => {
                    match self.format_function_sig_withoptionalvisibility(pair) {
                        Ok(stmt) => code.push_str(&*stmt),
                        Err(e) => return Err(e),
                    }
                    if count > 0 {
                        code.push_str(", ");
                        count -= 1;
                    }
                }
                Rule::returntype => {
                    code.push_str(":");
                    match self.format_typeannotation(pair) {
                        Ok(stmt) => code.push_str(&*stmt),
                        Err(e) => return Err(e),
                    }
                }
                Rule::typeannotation => match self.format_typeannotation(pair) {
                    Ok(typeannotation) => code.push_str(&*typeannotation),
                    Err(e) => return Err(e),
                },
                _ => return Err(Unreachable(unreachable_rule!())),
            };
        }
        Ok(code)
    }
    fn format_const_def(&mut self, pairs: Pair<Rule>) -> PestResult<String> {
        let mut code = String::new();
        code.push_str("const ");
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::COMMENT => match self.format_comment(pair) {
                    Ok(comment) => code.push_str(&*comment),
                    Err(e) => return Err(e),
                },
                Rule::ident => code.push_str(pair.as_str()),
                Rule::equal => {
                    code.push_str(&*" ".repeat(self.set_space));
                    code.push_str(pair.as_str());
                    code.push_str(&*" ".repeat(self.set_space));
                }
                Rule::typeannotation => match self.format_typeannotation(pair) {
                    Ok(typeannotation) => code.push_str(&*typeannotation),
                    Err(e) => return Err(e),
                },
                Rule::cmd => match self.format_cmd(pair) {
                    Ok(cmd_) => {
                        code.push_str(&*cmd_);
                        code.push_str(";");
                    }
                    Err(e) => return Err(e),
                },
                Rule::_expr => match self.format_expr_(pair) {
                    Ok(stmt) => code.push_str(&*stmt),
                    Err(e) => return Err(e),
                },
                _ => return Err(Unreachable(unreachable_rule!())),
            };
        }
        code.push_str(";");
        Ok(code)
    }
    fn format_typeannotation(&mut self, pairs: Pair<Rule>) -> PestResult<String> {
        let mut code = String::new();
        let mut count = 0;

        code.push_str(": ");
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::COMMENT => match self.format_comment(pair) {
                    Ok(comment) => code.push_str(&*comment),
                    Err(e) => return Err(e),
                },
                Rule::ident => code.push_str(pair.as_str()),
                Rule::left_column => code.push_str(pair.as_str()),
                Rule::right_column => code.push_str(pair.as_str()),
                Rule::type_ => {
                    match self.format_typeannotation(pair) {
                        Ok(stmt) => code.push_str(&*stmt),
                        Err(e) => return Err(e),
                    }
                    if count > 0 {
                        code.push_str(", ");
                        count -= 1;
                    }
                }
                Rule::reftype => {
                    code.push_str("& ");
                    if pair.as_str().contains("mut") {
                        code.push_str("mut ");
                    }
                    match self.format_typeannotation(pair) {
                        Ok(exp) => code.push_str(&*exp),
                        Err(e) => return Err(e),
                    }
                }
                Rule::lambdatype => {
                    code.push_str("|");
                    count += pair.as_str().matches(",").count();
                    match self.format_typeannotation(pair) {
                        Ok(exp) => code.push_str(&*exp),
                        Err(e) => return Err(e),
                    }
                    code.push_str("|");
                }
                Rule::tupletype => {
                    count += pair.as_str().matches(",").count();
                    match self.format_typeannotation(pair) {
                        Ok(exp) => code.push_str(&*exp),
                        Err(e) => return Err(e),
                    }
                }
                Rule::qual_pathtype => match self.format_typeannotation(pair) {
                    Ok(cmd_) => {
                        code.push_str(&*cmd_);
                    }
                    Err(e) => return Err(e),
                }
                Rule::fullyqualifiedmoduleref => code.push_str(
                    pair.as_str().chars().into_iter().filter(|c| !c.is_whitespace()).collect::<String>().as_str()
                ),

                Rule::moduleref => {
                    match self.format_typeannotation(pair) {
                        Ok(cmd_) => {
                            code.push_str(&*cmd_);
                        }
                        Err(e) => return Err(e),
                    }
                    code.push_str("::");
                }
                Rule::qual_path => match self.format_typeannotation(pair) {
                    Ok(cmd_) => {
                        code.push_str(&*cmd_);
                    }
                    Err(e) => return Err(e),
                },
                Rule::typeargumentlist => {
                    code.push_str("<");
                    count += pair.as_str().matches(",").count();
                    match self.format_typeannotation(pair) {
                        Ok(stmt) => code.push_str(&*stmt),
                        Err(e) => return Err(e),
                    }
                    code.push_str(">");
                }
                _ => return Err(Unreachable(unreachable_rule!())),
            };
        }
        Ok(code)
    }
    fn format_import_stmt(&mut self, pairs: Pair<Rule>) -> PestResult<String> {
        let mut code = String::new();
        code.push_str("use ");
        let mut count = 0;
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::COMMENT => match self.format_comment(pair) {
                    Ok(comment) => code.push_str(&*comment),
                    Err(e) => return Err(e),
                },
                Rule::ident => code.push_str(pair.as_str()),
                Rule::module_items_import => match self.format_import_stmt(pair) {
                    Ok(stmt) => code.push_str(&*stmt),
                    Err(e) => return Err(e),
                },
                Rule::fullyqualifiedmoduleref => {
                    match self.format_import_stmt(pair) {
                        Ok(exp) => code.push_str(&*exp),
                        Err(e) => return Err(e),
                    }
                    code.push_str("::");
                }
                Rule::module_import =>
                    match self.format_import_stmt(pair) {
                        Ok(exp) => code.push_str(&*exp),
                        Err(e) => return Err(e),
                    }
                Rule::item_import => {
                    match self.format_import_stmt(pair) {
                        Ok(cmd_) => {
                            code.push_str(&*cmd_);
                            code.push_str(";");
                        }
                        Err(e) => return Err(e),
                    }
                    if count > 0 {
                        code.push_str(",");
                        count -= 1;
                    }
                }
                Rule::SENDER_address => {
                    code.push_str(pair.as_str());
                    code.push_str("::");
                }
                Rule::literal_expr => {
                    code.push_str(pair.as_str());
                }
                Rule::address => {
                    code.push_str(pair.as_str());
                    code.push_str("::");
                }
                Rule::multi_item_import => {
                    count = pair.as_str().matches(",").count();
                    code.push_str("{");
                    match self.format_import_stmt(pair) {
                        Ok(stmt) => code.push_str(&*stmt),
                        Err(e) => return Err(e),
                    }
                    code.push_str("}");
                }
                Rule::importalias => {
                    code.push_str("as ");
                    match self.format_import_stmt(pair) {
                        Ok(stmt) => code.push_str(&*stmt),
                        Err(e) => return Err(e),
                    }
                }
                _ => return Err(Unreachable(unreachable_rule!())),
            };
        }
        Ok(code)
    }
    fn format_address_def(&mut self, pairs: Pair<Rule>) -> PestResult<String> {
        let mut code = String::new();
        code.push_str("address ");
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::COMMENT => match self.format_comment(pair) {
                    Ok(comment) => code.push_str(&*comment),
                    Err(e) => return Err(e),
                },
                Rule::address => code.push_str(pair.as_str()),
                Rule::SENDER_address => code.push_str(pair.as_str()),
                Rule::module_def => {
                    code.push_str("{");
                    match self.format_module_def(pair) {
                        Ok(cmd_) => {
                            code.push_str(&*cmd_);
                        }
                        Err(e) => return Err(e),
                    }
                    code.push_str("}");
                }
                _ => return Err(Unreachable(unreachable_rule!())),
            };
        }
        Ok(code)
    }
    fn format_module_def(&mut self, pairs: Pair<Rule>) -> PestResult<String> {
        let mut code = String::new();
        code.push_str("module ");
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::COMMENT => match self.format_comment(pair) {
                    Ok(comment) => code.push_str(&*comment),
                    Err(e) => return Err(e),
                },
                Rule::ident => code.push_str(pair.as_str()),
                Rule::moduleitem_item => {
                    code.push_str("{");
                    match self.format_moduleitem_item(pair) {
                        Ok(stmt) => code.push_str(&*stmt),
                        Err(e) => return Err(e),
                    }
                    code.push_str("}");
                }
                _ => return Err(Unreachable(unreachable_rule!())),
            };
        }
        Ok(code)
    }
    fn format_moduleitem_item(&mut self, pairs: Pair<Rule>) -> PestResult<String> {
        let mut count = 0;
        let mut code = String::new();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::COMMENT => match self.format_comment(pair) {
                    Ok(comment) => code.push_str(&*comment),
                    Err(e) => return Err(e),
                },
                Rule::import_stmt => match self.format_import_stmt(pair) {
                    Ok(stmt) => code.push_str(&*stmt),
                    Err(e) => return Err(e),
                },
                Rule::friend_stmt => {
                    code.push_str("friend ");
                    match self.format_typeannotation(pair) {
                        Ok(cmd_) => {
                            code.push_str(&*cmd_);
                        }
                        Err(e) => return Err(e),
                    }
                    code.push_str(";");
                }
                Rule::struct_def => match self.format_moduleitem_item(pair) {
                    Ok(cmd_) => {
                        code.push_str(&*cmd_);
                    }
                    Err(e) => return Err(e),
                },
                Rule::nat_function_def => {
                    code.push_str("native ");
                    match self.format_function_sig_withoptionalvisibility(pair) {
                        Ok(stmt_) => code.push_str(&*stmt_),
                        Err(e) => return Err(e),
                    }
                    code.push_str(";");
                }
                Rule::const_def => match self.format_const_def(pair) {
                    Ok(const_def_) => code.push_str(&*const_def_),
                    Err(e) => return Err(e),
                },
                Rule::spec_def => match self.format_spec_def(pair) {
                    Ok(const_def_) => code.push_str(&*const_def_),
                    Err(e) => return Err(e),
                },
                Rule::nat_struct_def => {
                    code.push_str("native ");
                    match self.format_moduleitem_item(pair) {
                        Ok(cmd_) => {
                            code.push_str(&*cmd_);
                        }
                        Err(e) => return Err(e),
                    }
                    code.push_str(";");
                }
                Rule::function_def => match self.format_function_def(pair) {
                    Ok(function_def_) =>
                        code.push_str(&*function_def_),
                    Err(e) => return Err(e),
                },
                Rule::struct_sig_ => {
                    code.push_str("struct ");
                    match self.format_moduleitem_item(pair) {
                        Ok(cmd_) => {
                            code.push_str(&*cmd_);
                        }
                        Err(e) => return Err(e),
                    }
                }
                Rule::typeparameterlist => {
                    code.push_str("<");
                    match self.format_function_sig_withoptionalvisibility(pair) {
                        Ok(function_sig_visibility_) => code.push_str(&*function_sig_visibility_),
                        Err(e) => return Err(e),
                    }
                    code.push_str(">");
                }
                Rule::abilitieslist => {
                    count += pair.as_str().matches(",").count();
                    code.push_str("has ");
                    match self.format_moduleitem_item(pair) {
                        Ok(cmd_) => {
                            code.push_str(&*cmd_);
                        }
                        Err(e) => return Err(e),
                    }
                }
                Rule::ability => {
                    if pair.as_str().starts_with("copy") {
                        code.push_str("copy ");
                    }
                    match self.format_function_sig_withoptionalvisibility(pair) {
                        Ok(function_sig_visibility_) => code.push_str(&*function_sig_visibility_),
                        Err(e) => return Err(e),
                    }
                    if count > 0 {
                        code.push_str(" , ");
                        count -= 1;
                    }
                }
                Rule::structfields_defblock => {
                    code.push_str("{");
                    match self.format_moduleitem_item(pair) {
                        Ok(cmd_) => {
                            code.push_str(&*cmd_);
                        }
                        Err(e) => return Err(e),
                    }
                    code.push_str("}");
                }
                Rule::stmtx => match self.format_stmtx(pair) {
                    Ok(stmt) => code.push_str(&*stmt),
                    Err(e) => return Err(e),
                },
                _ => return Err(Unreachable(unreachable_rule!())),
            };
        }
        Ok(code)
    }
    fn format_spec_def(&mut self, pairs: Pair<Rule>) -> PestResult<String> {
        let mut count = 0;
        let mut code = String::new();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::COMMENT => match self.format_comment(pair) {
                    Ok(comment) => code.push_str(&*comment),
                    Err(e) => return Err(e),
                },
                Rule::function_spec_def => match self.format_import_stmt(pair) {
                    Ok(stmt) => code.push_str(&*stmt),
                    Err(e) => return Err(e),
                },
                Rule::modulespec_def => match self.format_import_stmt(pair) {
                    Ok(stmt) => code.push_str(&*stmt),
                    Err(e) => return Err(e),
                },
                Rule::schemaspec_def => match self.format_import_stmt(pair) {
                    Ok(stmt) => code.push_str(&*stmt),
                    Err(e) => return Err(e),
                },
                Rule::_definefunction_spec_def => match self.format_import_stmt(pair) {
                    Ok(stmt) => code.push_str(&*stmt),
                    Err(e) => return Err(e),
                },
                Rule::nat_definefunction_spec_def => match self.format_import_stmt(pair) {
                    Ok(stmt) => code.push_str(&*stmt),
                    Err(e) => return Err(e),
                },
                _ => return Err(Unreachable(unreachable_rule!())),
            };
        }
        Ok(code)
    }

    fn format_stmtx(&mut self, pairs: Pair<Rule>) -> PestResult<String> {
        let mut code = String::new();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::COMMENT => match self.format_comment(pair) {
                    Ok(comment) => code.push_str(&*comment),
                    Err(e) => return Err(e),
                },
                Rule::stmt => match self.format_stmt(pair) {
                    Ok(stmt) => code.push_str(&*stmt),
                    Err(e) => return Err(e),
                },
                Rule::exp => match self.format_exp(pair) {
                    Ok(exp) => code.push_str(&*exp),
                    Err(e) => return Err(e),
                },
                Rule::cmd => match self.format_cmd(pair) {
                    Ok(cmd_) => {
                        code.push_str(&*cmd_);
                        code.push_str(";");
                    }
                    Err(e) => return Err(e),
                },
                Rule::stmtx => match self.format_stmtx(pair) {
                    Ok(stmt) => code.push_str(&*stmt),
                    Err(e) => return Err(e),
                },
                _ => return Err(Unreachable(unreachable_rule!())),
            };
        }
        Ok(code)
    }
    fn format_stmt(&mut self, pairs: Pair<Rule>) -> PestResult<String> {
        let mut code = String::new();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::COMMENT => match self.format_comment(pair) {
                    Ok(comment) => code.push_str(&*comment),
                    Err(e) => return Err(e),
                },
                Rule::literal_if => {
                    code.push_str("\n");
                    code.push_str(" ".repeat(self.current_indent).as_str());
                    code.push_str(pair.as_str());
                }
                Rule::literal_else => {
                    code.push_str(" ".repeat(self.set_else).as_str());
                    code.push_str(pair.as_str());
                    code.push_str(" ".repeat(self.set_else).as_str());
                }
                Rule::literal_while => {
                    code.push_str(pair.as_str());
                }
                Rule::literal_loop => {
                    code.push_str(pair.as_str());
                }
                Rule::stmtx => {
                    self.current_indent += self.indent;
                    match self.format_stmtx(pair) {
                        Ok(stmt) => {
                            code.push_str("{");
                            code.push_str(&*stmt);
                            self.current_indent -= self.indent;
                            code.push_str("\n");
                            code.push_str(" ".repeat(self.current_indent).as_str());
                            code.push_str("}");
                        }
                        Err(e) => return Err(e),
                    }
                }
                Rule::left_column => code.push_str(pair.as_str()),
                Rule::right_column => code.push_str(pair.as_str()),
                Rule::cmd => match self.format_cmd(pair) {
                    Ok(cmd_) => {
                        self.current_indent += self.indent;
                        code.push_str(&*cmd_);
                        code.push_str(";");
                        self.current_indent -= self.indent;
                    }
                    Err(e) => return Err(e),
                },
                Rule::exp => match self.format_exp(pair) {
                    Ok(exp_) => {
                        code.push_str(&*exp_);
                    }
                    Err(e) => return Err(e),
                },
                _ => return Err(Unreachable(unreachable_rule!())),
            };
        }
        Ok(code)
    }
    fn format_exp(&self, pairs: Pair<Rule>) -> PestResult<String> {
        let mut code = String::new();
        // dbg!(pairs.borrow());
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::COMMENT => match self.format_comment(pair) {
                    Ok(comment) => code.push_str(&*comment),
                    Err(e) => return Err(e),
                },
                Rule::value => code.push_str(pair.as_str()),
                Rule::expx => match self.format_exp(pair) {
                    Ok(exp) => code.push_str(&*exp),
                    Err(e) => return Err(e),
                },
                Rule::exp => match self.format_exp(pair) {
                    Ok(exp) => code.push_str(&*exp),
                    Err(e) => return Err(e),
                },
                Rule::left_column => code.push_str(pair.as_str()),
                Rule::right_column => code.push_str(pair.as_str()),
                Rule::struct_name => { code.push_str(pair.as_str()) }
                Rule::field_name => {
                    code.push_str("{");
                    code.push_str(pair.as_str());
                    code.push_str(":");
                }
                Rule::value_operator => match self.format_value_operator(pair) {
                    Ok(vop) => code.push_str(&*vop),
                    Err(e) => return Err(e),
                },
                Rule::binary_exp => {
                    code.push_str(" ");
                    code.push_str(pair.as_str());
                    code.push_str(" ");
                }
                _ => return Err(Unreachable(unreachable_rule!())),
            };
        }
        Ok(code)
    }
    fn format_value_operator(&self, pairs: Pair<Rule>) -> PestResult<String> {
        let mut code = String::new();
        let raw = pairs.as_str();
        if raw.starts_with("copy") {
            code.push_str("copy");
        }
        if raw.starts_with("move") {
            code.push_str("move");
        }
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::COMMENT => match self.format_comment(pair) {
                    Ok(comment) => code.push_str(&*comment),
                    Err(e) => return Err(e),
                },
                Rule::left_column => {
                    code.push_str(pair.as_str());
                }
                Rule::right_column => {
                    code.push_str(pair.as_str());
                }
                Rule::var => {
                    code.push_str(pair.as_str());
                }
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
        code.push_str("\n");
        code.push_str(&*" ".repeat(self.current_indent));
        if raw.starts_with("return") {
            code.push_str("return ");
        }
        if raw.starts_with("assert") {
            code.push_str("assert ");
        }
        if raw.starts_with("break") {
            code.push_str("break ");
        }
        if raw.starts_with("*") {
            code.push_str("* ");
        }
        let mut has_comma = false;
        if raw.contains(",") {
            has_comma = true;
        }
        let mut has_m = false;
        if raw.contains(":") {
            has_m = true;
        }
        let mut is_close = false;
        if raw.contains("{") && raw.contains("}") {
            is_close = true;
        }
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::COMMENT => match self.format_comment(pair) {
                    Ok(comment) => code.push_str(&*comment),
                    Err(e) => return Err(e),
                },
                Rule::var => {
                    code.push_str(pair.as_str().trim());
                    if is_close {
                        code.push_str("}");
                    }
                }
                Rule::exp => {
                    match self.format_exp(pair) {
                        Ok(comment) => code.push_str(&*comment),
                        Err(e) => return Err(e),
                    }
                    if has_comma {
                        code.push_str(", ");
                    }
                }
                Rule::struct_name => {
                    code.push_str(pair.as_str());
                }
                Rule::field_name => {
                    code.push_str(pair.as_str());
                    if has_comma {
                        code.push_str(": ");
                    }
                }
                Rule::equal => {
                    code.push_str(&*" ".repeat(self.set_space));
                    code.push_str(pair.as_str());
                    code.push_str(&*" ".repeat(self.set_space));
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