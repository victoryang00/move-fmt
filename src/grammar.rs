pub struct PestParser;

#[allow(dead_code, non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Rule {
    EOI,
    grammar_rules,
    grammar_rule,
    line_terminator,
    WHITESPACE,
    COMMENT,
    alpha,
    digit,
    ident,
    field_name,
    procedure_name,
    module_name,
    struct_name,
    var,
    kind,
    ground_type,
    module_alias,
    base_type,
    type_,
    tau_list,
    unsigned,
    address,
    bytes,
    value,
    SENDER_address,
    value_operator,
    reference_operator,
    module_operator,
    builtin,
    call,
    cmd,
    stmtx,
    stmt,
    expx,
    exp,
    import_decl,
    struct_decl,
    procedure_body,
    procedure_decl,
    module_decl,
    transaction_script,
    move_script,
    ScriptDef,
    ScriptBlock,
    ScriptBlockItems,
    ScriptItem,
    ScriptItemFirst,
    ScriptItem_recover,
    ScriptItem_item,
    AddressDef,
    AddressBlock,
    AddressBlockItems,
    AddressBlockItems_recover,
    ModuleDef,
    ModuleBlock,
    ModuleBlockItems,
    ModuleItem,
    Item_first,
    Item_recover,
    ModuleItem_item,
    ConstDef,
    NativeFunctionDef,
    NativeStructDef,
    FunctionSignatureWithOptionalVisibility,
    FunctionSignatureVisibility,
    FunctionSignature,
    FunctionSignature__recover,
    FunctionDef,
    FunctionVisibilityModifier,
    FunctionParameterList,
    FunctionParameter,
    ReturnType,
    ReturnTypeItem_with_recover,
    ReturnTypeItem_recover,
    AcquiresType,
    AcquiresType_items,
    AcquiresType_items_recover,
    StructSignature,
    StructDef,
    AbilitiesList,
    Ability,
    StructFieldsDefBlock,
    StructFieldDef_with_recover,
    StructFieldDef_recover,
    StructFieldDef,
    FriendStatement,
    ImportStatement,
    ModuleImport,
    ModuleItemsImport,
    MultiItemImport,
    MultiItemImport_member_with_recovery,
    ItemImport,
    ImportAlias,
    TypeAnnotation,
    Type,
    RefType,
    RefTypeStart,
    QualPathType,
    TupleType,
    TupleTypeItem_with_recover,
    TupleTypeItem_recover,
    LambdaType,
    TypeParameterList,
    TypeParameter_with_recover,
    TypeParameter_recover,
    TypeParameter,
    TypeParamBound,
    TypeParamBound_items,
    TypeParamBound_items_recover,
    TypeArgumentList,
    TypeArgument,
    Pat,
    WildPat,
    DerefPat,
    BorrowPat,
    DotPat,
    BindingPat,
    TuplePat,
    StructPat,
    StructPatFieldsBlock,
    StructPatField_with_recover,
    StructPatField_recover,
    StructPatField,
    StructPatFieldBinding,
    Statement,
    LetStatement,
    StatementExpr,
    AnyBlock,
    InlineBlock,
    CodeBlockExpr,
    CodeBlock,
    CodeBlockItems,
    CodeBlockItems_recover,
    Expr,
    MulExprItem,
    AddExprItem,
    LogicalEqExprItem,
    ControlFlowExpr,
    UnaryExpr,
    AtomExpr,
    EqualsExpr,
    NotEqualsExpr,
    OrExpr,
    AndExpr,
    LessExpr,
    GreaterExpr,
    LessEqualsExpr,
    GreaterEqualsExpr,
    BitOrExpr,
    BitAndExpr,
    BitXorExpr,
    CastExpr,
    AnnotatedExpPrefix,
    AnnotatedExpr,
    MulExpr,
    DivExpr,
    PlusExpr,
    MinusExpr,
    ModExpr,
    LeftShiftExpr,
    RightShiftExpr,
    ImplyOperatorsExprItem,
    ImplyOperatorExpr,
    PartialImplyOperatorExpr,
    BangExpr,
    DerefExpr,
    CopyExpr,
    MoveExpr,
    ReturnExpr,
    AbortExpr,
    BreakExpr,
    ContinueExpr,
    StructLiteralExpr,
    StructLiteralFieldsBlock,
    StructLiteralField_with_recover,
    StructLiteralField_recover,
    StructLiteralField,
    StructLiteralFieldAssignment,
    ParensExpr,
    LambdaExpr,
    RangeExpr,
    TupleLiteralExpr,
    EmptyTupleExpr,
    TupleExpr,
    LiteralExpr,
    CallExpr,
    CallArguments,
    IfExpr,
    Condition,
    ConditionBody,
    ConditionBody_recover,
    ElseBlock,
    LoopExpr,
    WhileExpr,
    AssignmentExpr,
    Initializer,
    BorrowExpr,
    DotExpr,
    StructFieldRef,
    IndexExpr,
    RefExpr,
    QualPath,
    ModuleRef,
    ImportedModuleRef,
    FullyQualifiedModuleRef,
    AddressRef,
    SpecDef,
    SchemaSpecDef,
    FunctionSpecDef,
    StructSpecDef,
    ModuleSpecDef,
    DefineFunctionSignature,
    DefineFunction,
    NativeDefineFunction,
    BlockSpecStatement,
    SpecBlock,
    SpecBlockStatement_with_recover,
    SpecBlockFinishingExpr_with_recover,
    SpecBlockStatement,
    SpecBlockFinishingExpr,
    DefineFunctionSpecDef,
    NativeDefineFunctionSpecDef,
    VariableStatement,
    LocalVariableStatement,
    GlobalVariableStatement,
    PragmaStatement,
    PragmaAttribute,
    SpecExpr,
    AssumeStatement,
    AssumeSpecExpr,
    AssertStatement,
    AssertSpecExpr,
    AbortsIfStatement,
    AbortsIfSpecExpr,
    WithExpr,
    SucceedsIfStatement,
    SucceedsIfSpecExpr,
    RequiresStatement,
    RequiresSpecExpr,
    EnsuresStatement,
    EnsuresSpecExpr,
    ModifiesStatement,
    ModifiesSpecExpr,
    IncludeStatement,
    FunctionPattern,
    InvariantStatement,
    InvariantSpecExpr,
    InvariantModifier,
    SpecVisibility,
    SpecVisibilityModifier,
    EmitsStatement,
    EmitsCondition,
    ApplyStatement,
    ApplySchemaName,
    ApplySchemaNameAttribute,
    Predicate,
    AggregatePredicateStatement,
    AggregateExpr,
    QuantifierWhere,
    QuantifierExpr,
    ForallQuantifier,
    ExistsQuantifier,
    QuantifierBindings,
    QuantifierBind,
    RangeQuantifierBind,
    TypeQuantifierBind,
}

#[allow(clippy::all)]
impl ::pest::Parser<Rule> for PestParser {
    fn parse<'i>(rule: Rule, input: &'i str) -> ::std::result::Result<::pest::iterators::Pairs<'i, Rule>, ::pest::error::Error<Rule>> {
        mod rules {
            pub mod hidden {
                use super::super::Rule;

                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn skip(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { if state.atomicity() == ::pest::Atomicity::NonAtomic { state.sequence(|state| { state.repeat(|state| super::visible::WHITESPACE(state)).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::visible::COMMENT(state).and_then(|state| { state.repeat(|state| super::visible::WHITESPACE(state)) }) }) }) }) }) } else { Ok(state) } }
            }

            pub mod visible {
                use super::super::Rule;

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn grammar_rules(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.sequence(|state| { self::SOI(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::grammar_rule(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::EOI(state) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn grammar_rule(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::grammar_rule, |state| { self::module_decl(state).or_else(|state| { self::transaction_script(state) }).or_else(|state| { self::move_script(state) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn line_terminator(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.match_string("\n").or_else(|state| { state.match_string("\r") }).or_else(|state| { state.match_string("\u{2028}") }).or_else(|state| { state.match_string("\u{2029}") }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn WHITESPACE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.atomic(::pest::Atomicity::Atomic, |state| { state.match_string("\t").or_else(|state| { state.match_string("\u{b}") }).or_else(|state| { state.match_string("\u{c}") }).or_else(|state| { state.match_string(" ") }).or_else(|state| { state.match_string("\u{a0}") }).or_else(|state| { state.match_string("\u{feff}") }).or_else(|state| { self::SPACE_SEPARATOR(state) }).or_else(|state| { self::line_terminator(state) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn COMMENT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.atomic(::pest::Atomicity::Atomic, |state| { state.sequence(|state| { state.match_string("/*").and_then(|state| { state.repeat(|state| { state.sequence(|state| { state.lookahead(false, |state| { state.match_string("*/") }).and_then(|state| { self::ANY(state) }) }) }) }).and_then(|state| { state.match_string("*/") }) }).or_else(|state| { state.sequence(|state| { state.match_string("//").and_then(|state| { state.repeat(|state| { self::ANY(state) }) }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn alpha(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::alpha, |state| { state.match_range('a'..'z').or_else(|state| { state.match_range('A'..'Z') }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn digit(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::digit, |state| { state.match_range('0'..'9') }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ident(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::ident, |state| { state.sequence(|state| { self::alpha(state).or_else(|state| { self::digit(state) }).or_else(|state| { state.match_string("$") }).or_else(|state| { state.match_string("_") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.sequence(|state| { state.optional(|state| { self::alpha(state).or_else(|state| { self::digit(state) }).or_else(|state| { state.match_string("$") }).or_else(|state| { state.match_string("_") }).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { self::alpha(state).or_else(|state| { self::digit(state) }).or_else(|state| { state.match_string("$") }).or_else(|state| { state.match_string("_") }) }) }) }) }) }) }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn field_name(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::field_name, |state| { self::ident(state) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn procedure_name(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::procedure_name, |state| { self::ident(state) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn module_name(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::module_name, |state| { self::ident(state) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn struct_name(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::struct_name, |state| { self::ident(state) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn var(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::var, |state| { self::ident(state) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn kind(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::kind, |state| { state.match_string("R").or_else(|state| { state.match_string("V") }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ground_type(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::ground_type, |state| { state.match_string("bool").or_else(|state| { state.match_string("u8") }).or_else(|state| { state.match_string("u32") }).or_else(|state| { state.match_string("u64") }).or_else(|state| { state.match_string("u128") }).or_else(|state| { state.match_string("address") }).or_else(|state| { state.match_string("bytearray") }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn module_alias(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::module_alias, |state| { self::module_name(state).or_else(|state| { state.match_string("Self") }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn base_type(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::base_type, |state| { self::ground_type(state).or_else(|state| { state.sequence(|state| { self::kind(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("#") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::module_alias(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(".") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::struct_name(state) }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn type_(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::type_, |state| { self::base_type(state).or_else(|state| { state.sequence(|state| { state.match_string("&").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::base_type(state) }) }) }).or_else(|state| { state.sequence(|state| { state.match_string("&mut").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::base_type(state) }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn tau_list(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::tau_list, |state| { state.match_string("unit").or_else(|state| { state.sequence(|state| { state.optional(|state| { self::type_(state).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { self::type_(state) }) }) }) }) }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn unsigned(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::unsigned, |state| { state.sequence(|state| { state.optional(|state| { self::digit(state).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { self::digit(state) }) }) }) }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn address(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::address, |state| { state.sequence(|state| { state.match_string("0x").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.sequence(|state| { state.optional(|state| { state.match_range('0'..'9').or_else(|state| { state.match_range('a'..'f') }).or_else(|state| { state.match_range('A'..'F') }).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { state.match_range('0'..'9').or_else(|state| { state.match_range('a'..'f') }).or_else(|state| { state.match_range('A'..'F') }) }) }) }) }) }) }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn bytes(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::bytes, |state| { state.sequence(|state| { state.match_string("b\"").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.sequence(|state| { state.optional(|state| { self::alpha(state).or_else(|state| { self::digit(state) }).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { self::alpha(state).or_else(|state| { self::digit(state) }) }) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("\"") }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn value(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::value, |state| { state.match_string("true").or_else(|state| { state.match_string("false") }).or_else(|state| { self::address(state) }).or_else(|state| { self::bytes(state) }).or_else(|state| { self::unsigned(state) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SENDER_address(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::SENDER_address, |state| { state.match_string("{{sender}}") }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn value_operator(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::value_operator, |state| { state.sequence(|state| { state.match_string("copy").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("(") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::var(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(")") }) }).or_else(|state| { state.sequence(|state| { state.match_string("move").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("(") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::var(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(")") }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn reference_operator(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::reference_operator, |state| { state.sequence(|state| { state.match_string("&").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::var(state) }) }).or_else(|state| { state.sequence(|state| { state.match_string("&").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::exp(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(".") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::field_name(state) }) }) }).or_else(|state| { state.sequence(|state| { state.match_string("*").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::exp(state) }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn module_operator(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::module_operator, |state| {
                        state.sequence(|state| { state.match_string("move_from").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("<") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::struct_name(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(">") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("(") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::exp(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(")") }) }).or_else(|state| { state.sequence(|state| { state.match_string("borrow_global").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("<") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::struct_name(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(">") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("(") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::exp(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(")") }) }) }).or_else(|state| { state.sequence(|state| { state.match_string("exist").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("<") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::struct_name(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(">") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("(") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::exp(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(")") }) }) })
                    })
                }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn builtin(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::builtin, |state| {
                        state.sequence(|state| { state.match_string("create_account").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("(") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::exp(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(")") }) }).or_else(|state| { state.sequence(|state| { state.match_string("release").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("(") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::exp(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(")") }) }) }).or_else(|state| { state.sequence(|state| { state.match_string("freeze").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("(") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::exp(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(")") }) }) })
                    })
                }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn call(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::call, |state| {
                        self::module_operator(state).or_else(|state| { self::builtin(state) }).or_else(|state| {
                            state.sequence(|state| {
                                self::module_alias(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(".") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::procedure_name(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("(") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { state.sequence(|state| { state.sequence(|state| { state.optional(|state| { state.sequence(|state| { self::exp(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { state.sequence(|state| { self::exp(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::exp(state) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(")") })
                            })
                        })
                    })
                }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn cmd(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::cmd, |state| {
                        state.sequence(|state| { self::var(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("=") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::call(state) }) }).or_else(|state| { state.sequence(|state| { state.sequence(|state| { state.optional(|state| { self::var(state).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { self::var(state) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("=") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::exp(state) }) }) }).or_else(|state| { self::call(state) }).or_else(|state| { state.sequence(|state| { state.match_string("*").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::var(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("=") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::exp(state) }) }) }).or_else(|state| { state.sequence(|state| { state.match_string("assert").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("(") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::exp(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::exp(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(")") }) }) }).or_else(|state| { state.match_string("break") }).or_else(|state| { state.match_string("continue") }).or_else(|state| { state.sequence(|state| { state.match_string("return").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { state.sequence(|state| { state.sequence(|state| { state.optional(|state| { state.sequence(|state| { self::exp(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { state.sequence(|state| { self::exp(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::exp(state) }) }) }) }) }) }).or_else(|state| {
                            state.sequence(|state| {
                                self::struct_name(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("{") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| {
                                    state.optional(|state| {
                                        state.sequence(|state| {
                                            state.sequence(|state| { state.optional(|state| { state.sequence(|state| { self::field_name(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(":") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::var(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { state.sequence(|state| { self::field_name(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(":") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::var(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::field_name(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(":") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::var(state) })
                                        })
                                    })
                                }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("}") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("=") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::exp(state) })
                            })
                        })
                    })
                }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn stmtx(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::stmtx, |state| { state.sequence(|state| { self::stmt(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::stmtx(state) }) }).or_else(|state| { self::stmt(state) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn stmt(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::stmt, |state| {
                        state.sequence(|state| {
                            state.match_string("if").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("(") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::exp(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(")") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("{") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::stmtx(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("}") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("else") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("{") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::stmtx(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("}") })
                        }).or_else(|state| { state.sequence(|state| { state.match_string("if").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("(") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::exp(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(")") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("{") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::stmtx(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("}") }) }) }).or_else(|state| { state.sequence(|state| { state.match_string("while").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("(") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::exp(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(")") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("{") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::stmtx(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("}") }) }) }).or_else(|state| { state.sequence(|state| { state.match_string("loop").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("{") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::stmtx(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("}") }) }) }).or_else(|state| { state.sequence(|state| { self::cmd(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(";") }) }) })
                    })
                }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn expx(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::expx, |state| {
                        self::value_operator(state).or_else(|state| { self::reference_operator(state) }).or_else(|state| {
                            state.sequence(|state| {
                                self::struct_name(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("{") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.sequence(|state| { state.optional(|state| { state.sequence(|state| { self::field_name(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(":") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::expx(state) }) }).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { state.sequence(|state| { self::field_name(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(":") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::expx(state) }) }) }) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("}") })
                            })
                        }).or_else(|state| { state.sequence(|state| { state.match_string("!").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::expx(state) }) }) }).or_else(|state| { self::value(state) })
                    })
                }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn exp(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::exp, |state| {
                        state.sequence(|state| { self::expx(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("||") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::exp(state) }) }).or_else(|state| { state.sequence(|state| { self::expx(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("&&") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::exp(state) }) }) }).or_else(|state| { state.sequence(|state| { self::expx(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(">=") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::exp(state) }) }) }).or_else(|state| { state.sequence(|state| { self::expx(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("<=") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::exp(state) }) }) }).or_else(|state| { state.sequence(|state| { self::expx(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(">") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::exp(state) }) }) }).or_else(|state| { state.sequence(|state| { self::expx(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("<") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::exp(state) }) }) }).or_else(|state| { state.sequence(|state| { self::expx(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("+") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::exp(state) }) }) }).or_else(|state| { state.sequence(|state| { self::expx(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("-") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::exp(state) }) }) }).or_else(|state| { state.sequence(|state| { self::expx(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("*") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::exp(state) }) }) }).or_else(|state| { state.sequence(|state| { self::expx(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("/") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::exp(state) }) }) }).or_else(|state| { state.sequence(|state| { self::expx(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("*") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::exp(state) }) }) }).or_else(|state| { state.sequence(|state| { self::expx(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("%") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::exp(state) }) }) }).or_else(|state| { state.sequence(|state| { self::expx(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("^") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::exp(state) }) }) }).or_else(|state| { state.sequence(|state| { self::expx(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("|") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::exp(state) }) }) }).or_else(|state| { state.sequence(|state| { self::expx(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("&") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::exp(state) }) }) }).or_else(|state| { state.sequence(|state| { self::expx(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("==") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::exp(state) }) }) }).or_else(|state| { state.sequence(|state| { self::expx(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("!=") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::exp(state) }) }) }).or_else(|state| { state.sequence(|state| { state.match_string("(").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::exp(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(")") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::exp(state) }) }) }).or_else(|state| { state.sequence(|state| { state.match_string("(").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::exp(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(")") }) }) }).or_else(|state| { self::expx(state) })
                    })
                }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn import_decl(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::import_decl, |state| { state.sequence(|state| { state.match_string("import").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::address(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(".") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::module_name(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.sequence(|state| { state.match_string("as").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::module_name(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(";") }) }).or_else(|state| { state.match_string(";") }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn struct_decl(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::struct_decl, |state| {
                        state.sequence(|state| {
                            state.match_string("resourse").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::struct_name(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("{") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| {
                                state.sequence(|state| { state.optional(|state| { state.sequence(|state| { self::field_name(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(":") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::base_type(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { state.sequence(|state| { self::field_name(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(":") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::base_type(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }) }) }) }) }) }) })
                            }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::field_name(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(":") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::base_type(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(" }") })
                        }).or_else(|state| {
                            state.sequence(|state| {
                                state.match_string("struct").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::struct_name(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("{") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| {
                                    state.optional(|state| {
                                        state.sequence(|state| {
                                            state.sequence(|state| { state.optional(|state| { state.sequence(|state| { self::field_name(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(":") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::base_type(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { state.sequence(|state| { self::field_name(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(":") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::base_type(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::field_name(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(":") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::base_type(state) })
                                        })
                                    })
                                }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("}") })
                            })
                        })
                    })
                }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn procedure_body(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::procedure_body, |state| {
                        state.sequence(|state| {
                            state.sequence(|state| {
                                state.optional(|state| {
                                    state.sequence(|state| { state.match_string("let").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::var(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { state.sequence(|state| { state.match_string(":").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ground_type(state) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(";") }) }).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { state.sequence(|state| { state.match_string("let").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::var(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { state.sequence(|state| { state.match_string(":").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ground_type(state) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(";") }) }) }) }) }) })
                                })
                            }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::stmtx(state) })
                        })
                    })
                }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn procedure_decl(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::procedure_decl, |state| {
                        state.sequence(|state| {
                            state.optional(|state| { state.match_string("public") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::procedure_name(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("(") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| {
                                state.optional(|state| {
                                    state.sequence(|state| {
                                        state.sequence(|state| { state.optional(|state| { state.sequence(|state| { self::var(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(":") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::type_(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { state.sequence(|state| { self::var(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(":") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::type_(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::var(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(":") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::type_(state) })
                                    })
                                })
                            }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(")") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(":") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::tau_list(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("{") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::procedure_body(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("}") })
                        }).or_else(|state| {
                            state.sequence(|state| {
                                state.match_string("native").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { state.match_string("public") }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::procedure_name(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("(") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| {
                                    state.optional(|state| {
                                        state.sequence(|state| {
                                            state.sequence(|state| { state.optional(|state| { state.sequence(|state| { self::var(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(":") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::type_(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { state.sequence(|state| { self::var(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(":") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::type_(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::var(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(":") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::type_(state) })
                                        })
                                    })
                                }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(")") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(":") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::tau_list(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(";") })
                            })
                        })
                    })
                }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn module_decl(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::module_decl, |state| {
                        state.sequence(|state| {
                            state.match_string("module").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::module_name(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("{") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.sequence(|state| { state.optional(|state| { self::import_decl(state).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { self::import_decl(state) }) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.sequence(|state| { state.optional(|state| { self::struct_decl(state).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { self::struct_decl(state) }) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.sequence(|state| { state.optional(|state| { self::procedure_decl(state).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { self::procedure_decl(state) }) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("}") })
                        })
                    })
                }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn transaction_script(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::transaction_script, |state| {
                        state.sequence(|state| {
                            state.sequence(|state| { state.optional(|state| { self::import_decl(state).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { self::import_decl(state) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("public") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("main") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("(") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| {
                                state.optional(|state| {
                                    state.sequence(|state| {
                                        state.sequence(|state| { state.optional(|state| { state.sequence(|state| { self::var(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(":") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ground_type(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { state.sequence(|state| { self::var(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(":") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ground_type(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::var(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(":") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ground_type(state) })
                                    })
                                })
                            }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(")") })
                        })
                    })
                }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn move_script(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::move_script, |state| { state.sequence(|state| { state.optional(|state| { self::ScriptDef(state).or_else(|state| { self::AddressDef(state) }).or_else(|state| { self::ModuleDef(state) }).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { self::ScriptDef(state).or_else(|state| { self::AddressDef(state) }).or_else(|state| { self::ModuleDef(state) }) }) }) }) }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ScriptDef(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::ScriptDef, |state| { state.sequence(|state| { state.match_string("script").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ScriptBlock(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ScriptBlock(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::ScriptBlock, |state| { state.sequence(|state| { state.match_string("{").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ScriptBlockItems(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("}") }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ScriptBlockItems(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::ScriptBlockItems, |state| { state.sequence(|state| { state.optional(|state| { self::ScriptItem(state).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { self::ScriptItem(state) }) }) }) }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ScriptItem(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::ScriptItem, |state| { state.sequence(|state| { state.lookahead(false, |state| { state.match_string("}").or_else(|state| { self::EOI(state) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ScriptItem_item(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ScriptItemFirst(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::ScriptItemFirst, |state| { state.match_string("use").or_else(|state| { state.match_string("const") }).or_else(|state| { state.match_string("fun") }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ScriptItem_recover(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::ScriptItem_recover, |state| { state.lookahead(false, |state| { state.match_string("}").or_else(|state| { self::EOI(state) }).or_else(|state| { self::ScriptItemFirst(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ScriptItem_item(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::ScriptItem_item, |state| { self::ImportStatement(state).or_else(|state| { self::ConstDef(state) }).or_else(|state| { self::FunctionDef(state) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn AddressDef(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::AddressDef, |state| { state.sequence(|state| { state.match_string("address").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::AddressRef(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::AddressBlock(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn AddressBlock(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::AddressBlock, |state| { state.sequence(|state| { state.match_string("{").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::AddressBlockItems(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("}") }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn AddressBlockItems(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::AddressBlockItems, |state| { state.sequence(|state| { state.optional(|state| { self::ModuleDef(state).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { self::ModuleDef(state) }) }) }) }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn AddressBlockItems_recover(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::AddressBlockItems_recover, |state| { state.lookahead(false, |state| { state.match_string("}") }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ModuleDef(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::ModuleDef, |state| { state.sequence(|state| { state.match_string("module").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ident(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ModuleBlock(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ModuleBlock(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::ModuleBlock, |state| { state.sequence(|state| { state.match_string("{").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ModuleBlockItems(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("}") }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ModuleBlockItems(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::ModuleBlockItems, |state| { state.sequence(|state| { state.optional(|state| { self::ModuleItem(state).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { self::ModuleItem(state) }) }) }) }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ModuleItem(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::ModuleItem, |state| { state.sequence(|state| { state.lookahead(false, |state| { state.match_string("}").or_else(|state| { self::EOI(state) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ModuleItem_item(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Item_first(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::Item_first, |state| { state.match_string("use").or_else(|state| { state.match_string("public") }).or_else(|state| { state.match_string("native") }).or_else(|state| { state.match_string("fun") }).or_else(|state| { state.match_string("const") }).or_else(|state| { state.match_string("struct") }).or_else(|state| { state.match_string("spec") }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Item_recover(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::Item_recover, |state| { state.lookahead(false, |state| { state.match_string("}").or_else(|state| { self::Item_first(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ModuleItem_item(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::ModuleItem_item, |state| { self::ImportStatement(state).or_else(|state| { self::FriendStatement(state) }).or_else(|state| { self::StructDef(state) }).or_else(|state| { self::FunctionDef(state) }).or_else(|state| { self::NativeStructDef(state) }).or_else(|state| { self::NativeFunctionDef(state) }).or_else(|state| { self::ConstDef(state) }).or_else(|state| { self::SpecDef(state) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ConstDef(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::ConstDef, |state| { state.sequence(|state| { state.match_string("const").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ident(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::TypeAnnotation(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Initializer(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(";") }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn NativeFunctionDef(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::NativeFunctionDef, |state| { state.sequence(|state| { state.match_string("native").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::FunctionSignatureWithOptionalVisibility(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(";") }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn NativeStructDef(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::NativeStructDef, |state| { state.sequence(|state| { state.match_string("native").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::StructSignature(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(";") }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn FunctionSignatureWithOptionalVisibility(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::FunctionSignatureWithOptionalVisibility, |state| { self::FunctionSignatureVisibility(state).or_else(|state| { self::FunctionSignature(state) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn FunctionSignatureVisibility(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::FunctionSignatureVisibility, |state| { state.sequence(|state| { self::FunctionVisibilityModifier(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::FunctionSignature(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn FunctionSignature(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::FunctionSignature, |state| { state.sequence(|state| { state.match_string("fun").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ident(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { self::TypeParameterList(state) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::FunctionParameterList(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { self::ReturnType(state) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { self::AcquiresType(state) }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn FunctionSignature__recover(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::FunctionSignature__recover, |state| { state.lookahead(false, |state| { state.match_string("{").or_else(|state| { state.match_string("}") }).or_else(|state| { state.match_string(";") }).or_else(|state| { self::EOI(state) }).or_else(|state| { self::Item_first(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn FunctionDef(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::FunctionDef, |state| { state.sequence(|state| { self::FunctionSignatureWithOptionalVisibility(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::CodeBlock(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn FunctionVisibilityModifier(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::FunctionVisibilityModifier, |state| { state.sequence(|state| { state.match_string("public").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { state.sequence(|state| { state.match_string("(").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("script").or_else(|state| { state.match_string("friend") }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(")") }) }) }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn FunctionParameterList(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::FunctionParameterList, |state| {
                        state.sequence(|state| { state.match_string("(").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { state.sequence(|state| { state.sequence(|state| { state.optional(|state| { state.sequence(|state| { self::FunctionParameter(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { state.sequence(|state| { self::FunctionParameter(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::FunctionParameter(state) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(")") }) })
                    })
                }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn FunctionParameter(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::FunctionParameter, |state| { state.sequence(|state| { self::ident(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::TypeAnnotation(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ReturnType(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::ReturnType, |state| { state.sequence(|state| { state.match_string(":").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ReturnTypeItem_with_recover(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ReturnTypeItem_with_recover(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::ReturnTypeItem_with_recover, |state| { self::Type(state) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ReturnTypeItem_recover(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::ReturnTypeItem_recover, |state| { state.lookahead(false, |state| { state.match_string("{").or_else(|state| { state.match_string(";") }).or_else(|state| { state.match_string("acquires") }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn AcquiresType(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::AcquiresType, |state| { state.sequence(|state| { state.match_string("acquires").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::AcquiresType_items(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn AcquiresType_items(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::AcquiresType_items, |state| { state.optional(|state| { state.sequence(|state| { state.sequence(|state| { state.optional(|state| { state.sequence(|state| { self::QualPathType(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { state.sequence(|state| { self::QualPathType(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::QualPathType(state) }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn AcquiresType_items_recover(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::AcquiresType_items_recover, |state| { state.lookahead(false, |state| { state.match_string(";").or_else(|state| { state.match_string("{") }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StructSignature(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::StructSignature, |state| { state.sequence(|state| { state.match_string("struct").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ident(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { self::TypeParameterList(state) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { self::AbilitiesList(state) }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StructDef(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::StructDef, |state| { state.sequence(|state| { self::StructSignature(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::StructFieldsDefBlock(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn AbilitiesList(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::AbilitiesList, |state| { state.sequence(|state| { state.match_string("has").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { state.sequence(|state| { state.sequence(|state| { state.optional(|state| { state.sequence(|state| { self::Ability(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { state.sequence(|state| { self::Ability(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Ability(state) }) }) }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Ability(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::Ability, |state| { state.match_string("copy").or_else(|state| { self::ident(state) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StructFieldsDefBlock(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::StructFieldsDefBlock, |state| { state.sequence(|state| { state.match_string("{").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.sequence(|state| { state.optional(|state| { self::StructFieldDef_with_recover(state).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { self::StructFieldDef_with_recover(state) }) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("}") }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StructFieldDef_with_recover(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::StructFieldDef_with_recover, |state| { state.sequence(|state| { state.lookahead(false, |state| { state.match_string("}") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::StructFieldDef(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",").or_else(|state| { state.lookahead(true, |state| { state.match_string("}") }) }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StructFieldDef_recover(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::StructFieldDef_recover, |state| { state.lookahead(false, |state| { state.match_string("}").or_else(|state| { self::ident(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StructFieldDef(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::StructFieldDef, |state| { state.sequence(|state| { self::ident(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::TypeAnnotation(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",").or_else(|state| { state.match_string("}") }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn FriendStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::FriendStatement, |state| { state.sequence(|state| { state.match_string("friend").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::FullyQualifiedModuleRef(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(";") }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ImportStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::ImportStatement, |state| { state.sequence(|state| { state.match_string("use").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ModuleItemsImport(state).or_else(|state| { self::ModuleImport(state) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(";") }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ModuleImport(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::ModuleImport, |state| { state.sequence(|state| { self::FullyQualifiedModuleRef(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { self::ImportAlias(state) }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ModuleItemsImport(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::ModuleItemsImport, |state| { state.sequence(|state| { self::FullyQualifiedModuleRef(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("::") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ItemImport(state).or_else(|state| { self::MultiItemImport(state) }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn MultiItemImport(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::MultiItemImport, |state| {
                        state.sequence(|state| {
                            state.match_string("{").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { state.sequence(|state| { state.sequence(|state| { state.optional(|state| { state.sequence(|state| { self::MultiItemImport_member_with_recovery(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { state.sequence(|state| { self::MultiItemImport_member_with_recovery(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::MultiItemImport_member_with_recovery(state) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("}") })
                        })
                    })
                }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn MultiItemImport_member_with_recovery(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::MultiItemImport_member_with_recovery, |state| { state.sequence(|state| { state.lookahead(false, |state| { state.match_string("}").or_else(|state| { state.match_string(";") }).or_else(|state| { self::EOI(state) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ItemImport(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ItemImport(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::ItemImport, |state| { state.sequence(|state| { self::ident(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { self::ImportAlias(state) }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ImportAlias(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::ImportAlias, |state| { state.sequence(|state| { state.match_string("as").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ident(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TypeAnnotation(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::TypeAnnotation, |state| { state.sequence(|state| { state.match_string(":").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Type(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Type(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::Type, |state| { self::RefType(state).or_else(|state| { self::QualPathType(state) }).or_else(|state| { self::TupleType(state) }).or_else(|state| { self::LambdaType(state) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn RefType(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::RefType, |state| { state.sequence(|state| { self::RefTypeStart(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Type(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn RefTypeStart(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::RefTypeStart, |state| { state.sequence(|state| { state.match_string("&").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { state.match_string("mut") }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn QualPathType(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::QualPathType, |state| { self::QualPath(state) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TupleType(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::TupleType, |state| { state.sequence(|state| { state.match_string("(").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.sequence(|state| { state.optional(|state| { self::TupleTypeItem_with_recover(state).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { self::TupleTypeItem_with_recover(state) }) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(")") }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TupleTypeItem_with_recover(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::TupleTypeItem_with_recover, |state| { state.sequence(|state| { state.lookahead(false, |state| { state.match_string(")") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Type(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",").or_else(|state| { state.lookahead(true, |state| { state.match_string(")") }) }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TupleTypeItem_recover(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::TupleTypeItem_recover, |state| { state.lookahead(false, |state| { state.match_string(")").or_else(|state| { state.match_string("{") }).or_else(|state| { self::ident(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LambdaType(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::LambdaType, |state| {
                        state.sequence(|state| {
                            state.match_string("|").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { state.sequence(|state| { state.sequence(|state| { state.optional(|state| { state.sequence(|state| { self::Type(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { state.sequence(|state| { self::Type(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Type(state) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("|") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Type(state) })
                        })
                    })
                }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TypeParameterList(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::TypeParameterList, |state| { state.sequence(|state| { state.match_string("<").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.sequence(|state| { state.optional(|state| { self::TypeParameter_with_recover(state).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { self::TypeParameter_with_recover(state) }) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(">") }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TypeParameter_with_recover(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::TypeParameter_with_recover, |state| { state.sequence(|state| { state.lookahead(false, |state| { state.match_string(">") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::TypeParameter(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",").or_else(|state| { state.lookahead(true, |state| { state.match_string(">") }) }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TypeParameter_recover(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::TypeParameter_recover, |state| { state.lookahead(false, |state| { state.match_string(">").or_else(|state| { state.match_string("(") }).or_else(|state| { state.match_string("{") }).or_else(|state| { self::ident(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TypeParameter(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::TypeParameter, |state| { state.sequence(|state| { self::ident(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { self::TypeParamBound(state) }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TypeParamBound(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::TypeParamBound, |state| { state.sequence(|state| { state.match_string(":").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::TypeParamBound_items(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TypeParamBound_items(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::TypeParamBound_items, |state| { state.sequence(|state| { self::Ability(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.sequence(|state| { state.optional(|state| { state.sequence(|state| { state.match_string("+").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Ability(state) }) }).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { state.sequence(|state| { state.match_string("+").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Ability(state) }) }) }) }) }) }) }) }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TypeParamBound_items_recover(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::TypeParamBound_items_recover, |state| { state.lookahead(false, |state| { state.match_string(">").or_else(|state| { state.match_string(",") }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TypeArgumentList(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::TypeArgumentList, |state| {
                        state.sequence(|state| {
                            state.match_string("<").and_then(|state| { super::hidden::skip(state) }).and_then(|state| {
                                state.optional(|state| {
                                    state.sequence(|state| {
                                        state.sequence(|state| { state.optional(|state| { state.sequence(|state| { self::TypeArgument(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",").or_else(|state| { state.match_string(">") }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { state.sequence(|state| { self::TypeArgument(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",").or_else(|state| { state.match_string(">") }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::TypeArgument(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",").or_else(|state| { state.match_string(">") }) })
                                    })
                                })
                            }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(">") })
                        })
                    })
                }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TypeArgument(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::TypeArgument, |state| { self::Type(state) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Pat(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::Pat, |state| { self::DerefPat(state).or_else(|state| { self::BorrowPat(state) }).or_else(|state| { self::TuplePat(state) }).or_else(|state| { self::StructPat(state) }).or_else(|state| { self::DotPat(state) }).or_else(|state| { self::WildPat(state) }).or_else(|state| { self::BindingPat(state) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn WildPat(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::WildPat, |state| { state.match_string("~_") }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn DerefPat(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::DerefPat, |state| { state.sequence(|state| { state.match_string("*").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Pat(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn BorrowPat(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::BorrowPat, |state| { state.sequence(|state| { state.match_string("&").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Pat(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn DotPat(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::DotPat, |state| { state.sequence(|state| { self::ident(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(".") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ident(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn BindingPat(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::BindingPat, |state| { self::ident(state) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TuplePat(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::TuplePat, |state| { state.sequence(|state| { state.match_string("(").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { state.sequence(|state| { state.sequence(|state| { state.optional(|state| { state.sequence(|state| { self::Pat(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { state.sequence(|state| { self::Pat(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Pat(state) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(")") }) }) })
                }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StructPat(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::StructPat, |state| { state.sequence(|state| { self::QualPath(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::StructPatFieldsBlock(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StructPatFieldsBlock(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::StructPatFieldsBlock, |state| { state.sequence(|state| { state.match_string("{").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.sequence(|state| { state.optional(|state| { self::StructPatField_with_recover(state).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { self::StructPatField_with_recover(state) }) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("}") }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StructPatField_with_recover(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::StructPatField_with_recover, |state| { state.sequence(|state| { state.lookahead(false, |state| { state.match_string("}") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::StructPatField(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",").or_else(|state| { state.match_string("}") }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StructPatField_recover(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::StructPatField_recover, |state| { state.lookahead(false, |state| { state.match_string("}").or_else(|state| { self::ident(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StructPatField(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::StructPatField, |state| { state.sequence(|state| { self::ident(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { self::StructPatFieldBinding(state) }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StructPatFieldBinding(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::StructPatFieldBinding, |state| { state.sequence(|state| { state.match_string(":").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Pat(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Statement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::Statement, |state| { self::LetStatement(state).or_else(|state| { state.sequence(|state| { self::StatementExpr(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(";") }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LetStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::LetStatement, |state| { state.sequence(|state| { state.match_string("let").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Pat(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { self::TypeAnnotation(state) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { self::Initializer(state) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(";") }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StatementExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::StatementExpr, |state| { self::IfExpr(state).or_else(|state| { self::LoopExpr(state) }).or_else(|state| { self::WhileExpr(state) }).or_else(|state| { self::Expr(state) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn AnyBlock(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::AnyBlock, |state| { self::CodeBlock(state).or_else(|state| { self::InlineBlock(state) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn InlineBlock(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::InlineBlock, |state| { self::Expr(state) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn CodeBlockExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::CodeBlockExpr, |state| { self::CodeBlock(state) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn CodeBlock(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::CodeBlock, |state| { state.sequence(|state| { state.match_string("{").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::CodeBlockItems(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("}") }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn CodeBlockItems(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::CodeBlockItems, |state| { state.sequence(|state| { state.sequence(|state| { state.optional(|state| { self::ImportStatement(state).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { self::ImportStatement(state) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.sequence(|state| { state.optional(|state| { self::Statement(state).or_else(|state| { self::BlockSpecStatement(state) }).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { self::Statement(state).or_else(|state| { self::BlockSpecStatement(state) }) }) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { self::Expr(state) }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn CodeBlockItems_recover(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::CodeBlockItems_recover, |state| { state.lookahead(false, |state| { state.match_string("}") }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Expr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::Expr, |state| { self::AssignmentExpr(state).or_else(|state| { self::ImplyOperatorsExprItem(state) }).or_else(|state| { self::OrExpr(state) }).or_else(|state| { self::AndExpr(state) }).or_else(|state| { self::LogicalEqExprItem(state) }).or_else(|state| { self::BitOrExpr(state) }).or_else(|state| { self::BitXorExpr(state) }).or_else(|state| { self::BitAndExpr(state) }).or_else(|state| { self::LeftShiftExpr(state) }).or_else(|state| { self::RightShiftExpr(state) }).or_else(|state| { self::AddExprItem(state) }).or_else(|state| { self::MulExprItem(state) }).or_else(|state| { self::ControlFlowExpr(state) }).or_else(|state| { self::CastExpr(state) }).or_else(|state| { self::UnaryExpr(state) }).or_else(|state| { self::BorrowExpr(state) }).or_else(|state| { self::AtomExpr(state) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn MulExprItem(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::MulExprItem, |state| { self::DivExpr(state).or_else(|state| { self::MulExpr(state) }).or_else(|state| { self::ModExpr(state) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn AddExprItem(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::AddExprItem, |state| { self::PlusExpr(state).or_else(|state| { self::MinusExpr(state) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LogicalEqExprItem(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::LogicalEqExprItem, |state| { self::EqualsExpr(state).or_else(|state| { self::NotEqualsExpr(state) }).or_else(|state| { self::LessEqualsExpr(state) }).or_else(|state| { self::LessExpr(state) }).or_else(|state| { self::GreaterEqualsExpr(state) }).or_else(|state| { self::GreaterExpr(state) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ControlFlowExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::ControlFlowExpr, |state| { self::IfExpr(state).or_else(|state| { self::LoopExpr(state) }).or_else(|state| { self::WhileExpr(state) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn UnaryExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::UnaryExpr, |state| { self::CopyExpr(state).or_else(|state| { self::MoveExpr(state) }).or_else(|state| { self::DerefExpr(state) }).or_else(|state| { self::BangExpr(state) }).or_else(|state| { self::ReturnExpr(state) }).or_else(|state| { self::ContinueExpr(state) }).or_else(|state| { self::BreakExpr(state) }).or_else(|state| { self::AbortExpr(state) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn AtomExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::AtomExpr, |state| { self::AnnotatedExpr(state).or_else(|state| { self::TupleLiteralExpr(state) }).or_else(|state| { self::ParensExpr(state) }).or_else(|state| { self::StructLiteralExpr(state) }).or_else(|state| { self::DotExpr(state) }).or_else(|state| { self::IndexExpr(state) }).or_else(|state| { self::CallExpr(state) }).or_else(|state| { self::RefExpr(state) }).or_else(|state| { self::LambdaExpr(state) }).or_else(|state| { self::RangeExpr(state) }).or_else(|state| { self::CodeBlockExpr(state) }).or_else(|state| { self::LiteralExpr(state) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn EqualsExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::EqualsExpr, |state| { state.sequence(|state| { self::Expr(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("==") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Expr(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn NotEqualsExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::NotEqualsExpr, |state| { state.sequence(|state| { self::Expr(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("!=") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Expr(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn OrExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::OrExpr, |state| { state.sequence(|state| { self::Expr(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("||") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Expr(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn AndExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::AndExpr, |state| { state.sequence(|state| { self::Expr(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("&&") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Expr(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LessExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::LessExpr, |state| { state.sequence(|state| { self::Expr(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.lookahead(false, |state| { state.match_string("<<").or_else(|state| { state.match_string("<==>") }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("<") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Expr(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn GreaterExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::GreaterExpr, |state| { state.sequence(|state| { self::Expr(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.lookahead(false, |state| { state.match_string(">>") }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(">") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Expr(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LessEqualsExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::LessEqualsExpr, |state| { state.sequence(|state| { self::Expr(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("<=") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Expr(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn GreaterEqualsExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::GreaterEqualsExpr, |state| { state.sequence(|state| { self::Expr(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(">=") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Expr(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn BitOrExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::BitOrExpr, |state| { state.sequence(|state| { self::Expr(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.lookahead(false, |state| { state.match_string("||") }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("|") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Expr(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn BitAndExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::BitAndExpr, |state| { state.sequence(|state| { self::Expr(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.lookahead(false, |state| { state.match_string("&&") }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("&") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Expr(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn BitXorExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::BitXorExpr, |state| { state.sequence(|state| { self::Expr(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("^") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Expr(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn CastExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::CastExpr, |state| { state.sequence(|state| { self::Expr(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("as") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Type(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn AnnotatedExpPrefix(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::AnnotatedExpPrefix, |state| { state.sequence(|state| { state.match_string("(").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Expr(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(":") }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn AnnotatedExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::AnnotatedExpr, |state| { state.sequence(|state| { self::AnnotatedExpPrefix(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Type(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(")") }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn MulExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::MulExpr, |state| { state.sequence(|state| { self::Expr(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("*") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Expr(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn DivExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::DivExpr, |state| { state.sequence(|state| { self::Expr(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("/") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Expr(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn PlusExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::PlusExpr, |state| { state.sequence(|state| { self::Expr(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("+") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Expr(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn MinusExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::MinusExpr, |state| { state.sequence(|state| { self::Expr(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("-") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Expr(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ModExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::ModExpr, |state| { state.sequence(|state| { self::Expr(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("%") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Expr(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LeftShiftExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::LeftShiftExpr, |state| { state.sequence(|state| { self::Expr(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("<<") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Expr(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn RightShiftExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::RightShiftExpr, |state| { state.sequence(|state| { self::Expr(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(">>") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Expr(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ImplyOperatorsExprItem(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::ImplyOperatorsExprItem, |state| { self::ImplyOperatorExpr(state).or_else(|state| { self::PartialImplyOperatorExpr(state) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ImplyOperatorExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::ImplyOperatorExpr, |state| { state.sequence(|state| { self::Expr(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("==>") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Expr(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn PartialImplyOperatorExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::PartialImplyOperatorExpr, |state| { state.sequence(|state| { self::Expr(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("<==>") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Expr(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn BangExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::BangExpr, |state| { state.sequence(|state| { state.match_string("!").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Expr(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn DerefExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::DerefExpr, |state| { state.sequence(|state| { state.match_string("*").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Expr(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn CopyExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::CopyExpr, |state| { state.sequence(|state| { state.match_string("copy").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Expr(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn MoveExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::MoveExpr, |state| { state.sequence(|state| { state.match_string("move").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Expr(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ReturnExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::ReturnExpr, |state| { state.sequence(|state| { state.match_string("return").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { self::Expr(state) }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn AbortExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::AbortExpr, |state| { state.sequence(|state| { state.match_string("abort").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Expr(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn BreakExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::BreakExpr, |state| { state.match_string("break") }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ContinueExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::ContinueExpr, |state| { state.match_string("continue") }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StructLiteralExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::StructLiteralExpr, |state| { state.sequence(|state| { self::QualPath(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::StructLiteralFieldsBlock(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StructLiteralFieldsBlock(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::StructLiteralFieldsBlock, |state| { state.sequence(|state| { state.match_string("{").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.sequence(|state| { state.optional(|state| { self::StructLiteralField_with_recover(state).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { self::StructLiteralField_with_recover(state) }) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("}") }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StructLiteralField_with_recover(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::StructLiteralField_with_recover, |state| { state.sequence(|state| { state.lookahead(false, |state| { state.match_string("}") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::StructLiteralField(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",").or_else(|state| { state.lookahead(true, |state| { state.match_string("}") }) }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StructLiteralField_recover(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::StructLiteralField_recover, |state| { state.lookahead(false, |state| { state.match_string("}").or_else(|state| { self::ident(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StructLiteralField(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::StructLiteralField, |state| { state.sequence(|state| { self::ident(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { self::StructLiteralFieldAssignment(state) }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StructLiteralFieldAssignment(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::StructLiteralFieldAssignment, |state| { state.sequence(|state| { state.match_string(":").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Expr(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ParensExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::ParensExpr, |state| { state.sequence(|state| { state.match_string("(").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Expr(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(")") }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LambdaExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::LambdaExpr, |state| {
                        state.sequence(|state| {
                            state.match_string("|").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { state.sequence(|state| { state.sequence(|state| { state.optional(|state| { state.sequence(|state| { self::ident(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { state.sequence(|state| { self::ident(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ident(state) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("|") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Expr(state) })
                        })
                    })
                }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn RangeExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::RangeExpr, |state| { state.sequence(|state| { self::Expr(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("..") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Expr(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TupleLiteralExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::TupleLiteralExpr, |state| { self::EmptyTupleExpr(state).or_else(|state| { self::TupleExpr(state) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn EmptyTupleExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::EmptyTupleExpr, |state| { state.sequence(|state| { state.match_string("(").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(")") }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TupleExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::TupleExpr, |state| { state.sequence(|state| { state.match_string("(").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { state.sequence(|state| { state.sequence(|state| { state.optional(|state| { state.sequence(|state| { self::Expr(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { state.sequence(|state| { self::Expr(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Expr(state) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(")") }) }) })
                }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LiteralExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::LiteralExpr, |state| { self::SENDER_address(state).or_else(|state| { self::address(state) }).or_else(|state| { self::value(state) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn CallExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::CallExpr, |state| { state.sequence(|state| { self::QualPath(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("(") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::CallArguments(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn CallArguments(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::CallArguments, |state| { state.sequence(|state| { state.match_string("(").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { state.sequence(|state| { self::Expr(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.lookahead(true, |state| { state.match_string(",").or_else(|state| { state.match_string(")") }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(")") }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn IfExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::IfExpr, |state| { state.sequence(|state| { state.match_string("if").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Condition(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::AnyBlock(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { self::ElseBlock(state) }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Condition(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::Condition, |state| { state.sequence(|state| { state.match_string("(").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ConditionBody(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(")") }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ConditionBody(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::ConditionBody, |state| { self::Expr(state) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ConditionBody_recover(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::ConditionBody_recover, |state| { state.lookahead(false, |state| { state.match_string(")") }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ElseBlock(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::ElseBlock, |state| { state.sequence(|state| { state.match_string("else").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::AnyBlock(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LoopExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::LoopExpr, |state| { state.sequence(|state| { state.match_string("loop").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::AnyBlock(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn WhileExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::WhileExpr, |state| { state.sequence(|state| { state.match_string("while").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Condition(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::AnyBlock(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn AssignmentExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::AssignmentExpr, |state| { state.sequence(|state| { self::Expr(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Initializer(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Initializer(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::Initializer, |state| { state.sequence(|state| { state.match_string("=").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Expr(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn BorrowExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::BorrowExpr, |state| { state.sequence(|state| { state.match_string("&").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { state.match_string("mut") }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Expr(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn DotExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::DotExpr, |state| { state.sequence(|state| { self::Expr(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(".") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::StructFieldRef(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StructFieldRef(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::StructFieldRef, |state| { self::ident(state) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn IndexExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::IndexExpr, |state| { state.sequence(|state| { self::Expr(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("[") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Expr(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("]") }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn RefExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::RefExpr, |state| { self::QualPath(state) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn QualPath(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::QualPath, |state| { state.sequence(|state| { state.optional(|state| { state.sequence(|state| { self::ModuleRef(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("::") }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ident(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { self::TypeArgumentList(state) }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ModuleRef(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::ModuleRef, |state| { self::ImportedModuleRef(state).or_else(|state| { self::FullyQualifiedModuleRef(state) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ImportedModuleRef(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::ImportedModuleRef, |state| { self::ident(state) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn FullyQualifiedModuleRef(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::FullyQualifiedModuleRef, |state| { state.sequence(|state| { self::AddressRef(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("::") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ident(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn AddressRef(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::AddressRef, |state| { self::SENDER_address(state).or_else(|state| { self::address(state) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SpecDef(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::SpecDef, |state| { self::FunctionSpecDef(state).or_else(|state| { self::ModuleSpecDef(state) }).or_else(|state| { self::StructSpecDef(state) }).or_else(|state| { self::SchemaSpecDef(state) }).or_else(|state| { self::DefineFunctionSpecDef(state) }).or_else(|state| { self::NativeDefineFunctionSpecDef(state) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SchemaSpecDef(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::SchemaSpecDef, |state| { state.sequence(|state| { state.match_string("spec").or_else(|state| { state.match_string("schema") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ident(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { self::TypeParameterList(state) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::SpecBlock(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn FunctionSpecDef(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::FunctionSpecDef, |state| { state.sequence(|state| { state.match_string("spec").or_else(|state| { state.match_string("fun") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ident(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::SpecBlock(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StructSpecDef(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::StructSpecDef, |state| { state.sequence(|state| { state.match_string("spec").or_else(|state| { state.match_string("struct") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ident(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::SpecBlock(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ModuleSpecDef(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::ModuleSpecDef, |state| { state.sequence(|state| { state.match_string("spec").or_else(|state| { state.match_string("module") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::SpecBlock(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn DefineFunctionSignature(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::DefineFunctionSignature, |state| { state.sequence(|state| { state.match_string("define").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ident(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { self::TypeParameterList(state) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::FunctionParameterList(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { self::ReturnType(state) }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn DefineFunction(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::DefineFunction, |state| { state.sequence(|state| { self::DefineFunctionSignature(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::SpecBlock(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn NativeDefineFunction(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::NativeDefineFunction, |state| { state.sequence(|state| { self::DefineFunctionSignature(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(";") }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn BlockSpecStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::BlockSpecStatement, |state| { state.sequence(|state| { state.match_string("spec").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::SpecBlock(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(";") }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SpecBlock(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::SpecBlock, |state| { state.sequence(|state| { state.match_string("{").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.sequence(|state| { state.optional(|state| { self::SpecBlockStatement_with_recover(state).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { self::SpecBlockStatement_with_recover(state) }) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { self::SpecBlockFinishingExpr_with_recover(state) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("}") }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SpecBlockStatement_with_recover(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::SpecBlockStatement_with_recover, |state| { state.sequence(|state| { state.lookahead(false, |state| { state.match_string("}").or_else(|state| { self::EOI(state) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::SpecBlockStatement(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SpecBlockFinishingExpr_with_recover(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::SpecBlockFinishingExpr_with_recover, |state| { state.sequence(|state| { state.lookahead(false, |state| { state.match_string("}").or_else(|state| { self::EOI(state) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::SpecBlockFinishingExpr(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SpecBlockStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::SpecBlockStatement, |state| {
                        self::VariableStatement(state).or_else(|state| { self::LocalVariableStatement(state) }).or_else(|state| { self::GlobalVariableStatement(state) }).or_else(|state| { self::ImportStatement(state) }).or_else(|state| { self::NativeFunctionDef(state) }).or_else(|state| { self::DefineFunction(state) }).or_else(|state| { self::DefineFunctionSpecDef(state) }).or_else(|state| { self::NativeDefineFunction(state) }).or_else(|state| { self::NativeDefineFunctionSpecDef(state) }).or_else(|state| { self::PragmaStatement(state) }).or_else(|state| { self::AssumeStatement(state) }).or_else(|state| { self::AssertStatement(state) }).or_else(|state| { self::AbortsIfStatement(state) }).or_else(|state| { self::SucceedsIfStatement(state) }).or_else(|state| { self::RequiresStatement(state) }).or_else(|state| { self::EnsuresStatement(state) }).or_else(|state| { self::ModifiesStatement(state) }).or_else(|state| { self::IncludeStatement(state) }).or_else(|state| { self::InvariantStatement(state) }).or_else(|state| { self::ApplyStatement(state) }).or_else(|state| { self::EmitsStatement(state) }).or_else(|state| { self::AggregatePredicateStatement(state) }).or_else(|state| { self::Statement(state) })
                    })
                }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SpecBlockFinishingExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::SpecBlockFinishingExpr, |state| { self::SpecExpr(state).or_else(|state| { self::AggregateExpr(state) }).or_else(|state| { self::Expr(state) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn DefineFunctionSpecDef(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::DefineFunctionSpecDef, |state| { state.sequence(|state| { state.match_string("spec").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::DefineFunctionSignature(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::SpecBlock(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn NativeDefineFunctionSpecDef(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::NativeDefineFunctionSpecDef, |state| { state.sequence(|state| { state.match_string("spec").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::DefineFunctionSignature(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(";") }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn VariableStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::VariableStatement, |state| { state.sequence(|state| { self::RefExpr(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::TypeAnnotation(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(";") }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LocalVariableStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::LocalVariableStatement, |state| { state.sequence(|state| { state.match_string("local").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::RefExpr(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::TypeAnnotation(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(";") }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn GlobalVariableStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::GlobalVariableStatement, |state| { state.sequence(|state| { state.match_string("global").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::RefExpr(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::TypeAnnotation(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(";") }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn PragmaStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::PragmaStatement, |state| {
                        state.sequence(|state| { state.match_string("pragma").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { state.sequence(|state| { state.sequence(|state| { state.optional(|state| { state.sequence(|state| { self::PragmaAttribute(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { state.sequence(|state| { self::PragmaAttribute(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::PragmaAttribute(state) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(";") }) })
                    })
                }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn PragmaAttribute(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::PragmaAttribute, |state| { state.sequence(|state| { self::ident(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { state.sequence(|state| { state.match_string("=").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::LiteralExpr(state) }) }) }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SpecExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::SpecExpr, |state| { self::AssumeSpecExpr(state).or_else(|state| { self::AssertSpecExpr(state) }).or_else(|state| { self::AbortsIfSpecExpr(state) }).or_else(|state| { self::SucceedsIfSpecExpr(state) }).or_else(|state| { self::RequiresSpecExpr(state) }).or_else(|state| { self::EnsuresSpecExpr(state) }).or_else(|state| { self::ModifiesSpecExpr(state) }).or_else(|state| { self::InvariantSpecExpr(state) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn AssumeStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::AssumeStatement, |state| { state.sequence(|state| { self::AssumeSpecExpr(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(";") }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn AssumeSpecExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::AssumeSpecExpr, |state| { state.sequence(|state| { state.match_string("assume").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Predicate(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn AssertStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::AssertStatement, |state| { state.sequence(|state| { self::AssertSpecExpr(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(";") }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn AssertSpecExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::AssertSpecExpr, |state| { state.sequence(|state| { state.match_string("assert").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Predicate(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn AbortsIfStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::AbortsIfStatement, |state| { state.sequence(|state| { self::AbortsIfSpecExpr(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(";") }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn AbortsIfSpecExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::AbortsIfSpecExpr, |state| { state.sequence(|state| { state.match_string("aborts_if").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Predicate(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { self::WithExpr(state) }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn WithExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::WithExpr, |state| { state.sequence(|state| { state.match_string("with").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Expr(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SucceedsIfStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::SucceedsIfStatement, |state| { state.sequence(|state| { self::SucceedsIfSpecExpr(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(";") }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SucceedsIfSpecExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::SucceedsIfSpecExpr, |state| { state.sequence(|state| { state.match_string("succeeds_if").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Predicate(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn RequiresStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::RequiresStatement, |state| { state.sequence(|state| { self::RequiresSpecExpr(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(";") }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn RequiresSpecExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::RequiresSpecExpr, |state| { state.sequence(|state| { state.match_string("requires").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { state.match_string("module") }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Predicate(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn EnsuresStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::EnsuresStatement, |state| { state.sequence(|state| { self::EnsuresSpecExpr(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(";") }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn EnsuresSpecExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::EnsuresSpecExpr, |state| { state.sequence(|state| { state.match_string("ensures").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Predicate(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ModifiesStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::ModifiesStatement, |state| { state.sequence(|state| { self::ModifiesSpecExpr(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(";") }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ModifiesSpecExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::ModifiesSpecExpr, |state| { state.sequence(|state| { state.match_string("modifies").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Expr(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn IncludeStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::IncludeStatement, |state| { state.sequence(|state| { state.match_string("include").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { self::SpecVisibility(state) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Expr(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(";") }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn FunctionPattern(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::FunctionPattern, |state| { state.sequence(|state| { state.optional(|state| { state.match_string("public").or_else(|state| { state.match_string("internal") }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ident(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { self::TypeArgumentList(state) }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn InvariantStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::InvariantStatement, |state| { state.sequence(|state| { self::InvariantSpecExpr(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(";") }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn InvariantSpecExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::InvariantSpecExpr, |state| { state.sequence(|state| { state.match_string("invariant").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { self::InvariantModifier(state) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Predicate(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn InvariantModifier(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::InvariantModifier, |state| { state.match_string("pack").or_else(|state| { state.match_string("unpack") }).or_else(|state| { state.match_string("module") }).or_else(|state| { state.match_string("update") }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SpecVisibility(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::SpecVisibility, |state| {
                        state.sequence(|state| {
                            state.match_string("[").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { state.sequence(|state| { state.sequence(|state| { state.optional(|state| { state.sequence(|state| { self::SpecVisibilityModifier(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { state.sequence(|state| { self::SpecVisibilityModifier(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::SpecVisibilityModifier(state) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("]") })
                        })
                    })
                }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SpecVisibilityModifier(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::SpecVisibilityModifier, |state| { state.match_string("global").or_else(|state| { state.match_string("isolated") }).or_else(|state| { state.match_string("deactivated") }).or_else(|state| { state.match_string("concrete") }).or_else(|state| { state.match_string("abstract") }).or_else(|state| { state.match_string("assert") }).or_else(|state| { state.match_string("assume") }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn EmitsStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::EmitsStatement, |state| { state.sequence(|state| { state.match_string("emits").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Expr(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("to") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Expr(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { self::EmitsCondition(state) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(";") }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn EmitsCondition(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::EmitsCondition, |state| { state.sequence(|state| { state.match_string("if").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Expr(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ApplyStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::ApplyStatement, |state| {
                        state.sequence(|state| {
                            state.match_string("apply").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ApplySchemaName(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("to") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { state.sequence(|state| { state.sequence(|state| { state.optional(|state| { state.sequence(|state| { self::FunctionPattern(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { state.sequence(|state| { self::FunctionPattern(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::FunctionPattern(state) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { state.sequence(|state| { state.match_string("except").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { state.sequence(|state| { state.sequence(|state| { state.optional(|state| { state.sequence(|state| { self::FunctionPattern(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { state.sequence(|state| { self::FunctionPattern(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::FunctionPattern(state) }) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(";") })
                        })
                    })
                }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ApplySchemaName(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::ApplySchemaName, |state| {
                        state.sequence(|state| {
                            self::QualPath(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| {
                                state.optional(|state| {
                                    state.sequence(|state| {
                                        state.match_string("{").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { state.sequence(|state| { state.sequence(|state| { state.optional(|state| { state.sequence(|state| { self::ApplySchemaNameAttribute(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { state.sequence(|state| { self::ApplySchemaNameAttribute(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ApplySchemaNameAttribute(state) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("}") })
                                    })
                                })
                            })
                        })
                    })
                }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ApplySchemaNameAttribute(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::ApplySchemaNameAttribute, |state| { state.sequence(|state| { self::ident(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(":") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Expr(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Predicate(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::Predicate, |state| { state.sequence(|state| { state.optional(|state| { self::SpecVisibility(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::AggregateExpr(state).or_else(|state| { self::Expr(state) }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn AggregatePredicateStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::AggregatePredicateStatement, |state| { state.sequence(|state| { self::AggregateExpr(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(";") }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn AggregateExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::AggregateExpr, |state| { state.sequence(|state| { self::QuantifierExpr(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { state.sequence(|state| { state.match_string(":").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Expr(state) }) }) }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn QuantifierWhere(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::QuantifierWhere, |state| { state.sequence(|state| { state.match_string("where").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Expr(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn QuantifierExpr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::QuantifierExpr, |state| { self::ForallQuantifier(state).or_else(|state| { self::ExistsQuantifier(state) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ForallQuantifier(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::ForallQuantifier, |state| { state.sequence(|state| { state.match_string("forall").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::QuantifierBindings(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { self::QuantifierWhere(state) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { state.sequence(|state| { state.match_string(":").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::QuantifierExpr(state) }) }) }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ExistsQuantifier(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::ExistsQuantifier, |state| { state.sequence(|state| { state.match_string("exists").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::QuantifierBindings(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { self::QuantifierWhere(state) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { state.sequence(|state| { state.match_string(":").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::QuantifierExpr(state) }) }) }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn QuantifierBindings(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::QuantifierBindings, |state| { state.optional(|state| { state.sequence(|state| { state.sequence(|state| { state.optional(|state| { state.sequence(|state| { self::QuantifierBind(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { state.sequence(|state| { self::QuantifierBind(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(",") }) }) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::QuantifierBind(state) }) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn QuantifierBind(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::QuantifierBind, |state| { self::RangeQuantifierBind(state).or_else(|state| { self::TypeQuantifierBind(state) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn RangeQuantifierBind(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::RangeQuantifierBind, |state| { state.sequence(|state| { self::ident(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("in") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::RangeExpr(state) }) }) }) }

                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TypeQuantifierBind(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::TypeQuantifierBind, |state| { state.sequence(|state| { self::ident(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(":") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::Type(state) }) }) }) }

                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ANY(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.skip(1) }

                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn EOI(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::EOI, |state| state.end_of_input()) }

                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn SOI(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.start_of_input() }

                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn SPACE_SEPARATOR(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.match_char_by(::pest::unicode::SPACE_SEPARATOR) }
            }

            pub use self::visible::*;
        }
        ::pest::state(input, |state| {
            match rule {
                Rule::grammar_rules => rules::grammar_rules(state),
                Rule::grammar_rule => rules::grammar_rule(state),
                Rule::line_terminator => rules::line_terminator(state),
                Rule::WHITESPACE => rules::WHITESPACE(state),
                Rule::COMMENT => rules::COMMENT(state),
                Rule::alpha => rules::alpha(state),
                Rule::digit => rules::digit(state),
                Rule::ident => rules::ident(state),
                Rule::field_name => rules::field_name(state),
                Rule::procedure_name => rules::procedure_name(state),
                Rule::module_name => rules::module_name(state),
                Rule::struct_name => rules::struct_name(state),
                Rule::var => rules::var(state),
                Rule::kind => rules::kind(state),
                Rule::ground_type => rules::ground_type(state),
                Rule::module_alias => rules::module_alias(state),
                Rule::base_type => rules::base_type(state),
                Rule::type_ => rules::type_(state),
                Rule::tau_list => rules::tau_list(state),
                Rule::unsigned => rules::unsigned(state),
                Rule::address => rules::address(state),
                Rule::bytes => rules::bytes(state),
                Rule::value => rules::value(state),
                Rule::SENDER_address => rules::SENDER_address(state),
                Rule::value_operator => rules::value_operator(state),
                Rule::reference_operator => rules::reference_operator(state),
                Rule::module_operator => rules::module_operator(state),
                Rule::builtin => rules::builtin(state),
                Rule::call => rules::call(state),
                Rule::cmd => rules::cmd(state),
                Rule::stmtx => rules::stmtx(state),
                Rule::stmt => rules::stmt(state),
                Rule::expx => rules::expx(state),
                Rule::exp => rules::exp(state),
                Rule::import_decl => rules::import_decl(state),
                Rule::struct_decl => rules::struct_decl(state),
                Rule::procedure_body => rules::procedure_body(state),
                Rule::procedure_decl => rules::procedure_decl(state),
                Rule::module_decl => rules::module_decl(state),
                Rule::transaction_script => rules::transaction_script(state),
                Rule::move_script => rules::move_script(state),
                Rule::ScriptDef => rules::ScriptDef(state),
                Rule::ScriptBlock => rules::ScriptBlock(state),
                Rule::ScriptBlockItems => rules::ScriptBlockItems(state),
                Rule::ScriptItem => rules::ScriptItem(state),
                Rule::ScriptItemFirst => rules::ScriptItemFirst(state),
                Rule::ScriptItem_recover => rules::ScriptItem_recover(state),
                Rule::ScriptItem_item => rules::ScriptItem_item(state),
                Rule::AddressDef => rules::AddressDef(state),
                Rule::AddressBlock => rules::AddressBlock(state),
                Rule::AddressBlockItems => rules::AddressBlockItems(state),
                Rule::AddressBlockItems_recover => rules::AddressBlockItems_recover(state),
                Rule::ModuleDef => rules::ModuleDef(state),
                Rule::ModuleBlock => rules::ModuleBlock(state),
                Rule::ModuleBlockItems => rules::ModuleBlockItems(state),
                Rule::ModuleItem => rules::ModuleItem(state),
                Rule::Item_first => rules::Item_first(state),
                Rule::Item_recover => rules::Item_recover(state),
                Rule::ModuleItem_item => rules::ModuleItem_item(state),
                Rule::ConstDef => rules::ConstDef(state),
                Rule::NativeFunctionDef => rules::NativeFunctionDef(state),
                Rule::NativeStructDef => rules::NativeStructDef(state),
                Rule::FunctionSignatureWithOptionalVisibility => rules::FunctionSignatureWithOptionalVisibility(state),
                Rule::FunctionSignatureVisibility => rules::FunctionSignatureVisibility(state),
                Rule::FunctionSignature => rules::FunctionSignature(state),
                Rule::FunctionSignature__recover => rules::FunctionSignature__recover(state),
                Rule::FunctionDef => rules::FunctionDef(state),
                Rule::FunctionVisibilityModifier => rules::FunctionVisibilityModifier(state),
                Rule::FunctionParameterList => rules::FunctionParameterList(state),
                Rule::FunctionParameter => rules::FunctionParameter(state),
                Rule::ReturnType => rules::ReturnType(state),
                Rule::ReturnTypeItem_with_recover => rules::ReturnTypeItem_with_recover(state),
                Rule::ReturnTypeItem_recover => rules::ReturnTypeItem_recover(state),
                Rule::AcquiresType => rules::AcquiresType(state),
                Rule::AcquiresType_items => rules::AcquiresType_items(state),
                Rule::AcquiresType_items_recover => rules::AcquiresType_items_recover(state),
                Rule::StructSignature => rules::StructSignature(state),
                Rule::StructDef => rules::StructDef(state),
                Rule::AbilitiesList => rules::AbilitiesList(state),
                Rule::Ability => rules::Ability(state),
                Rule::StructFieldsDefBlock => rules::StructFieldsDefBlock(state),
                Rule::StructFieldDef_with_recover => rules::StructFieldDef_with_recover(state),
                Rule::StructFieldDef_recover => rules::StructFieldDef_recover(state),
                Rule::StructFieldDef => rules::StructFieldDef(state),
                Rule::FriendStatement => rules::FriendStatement(state),
                Rule::ImportStatement => rules::ImportStatement(state),
                Rule::ModuleImport => rules::ModuleImport(state),
                Rule::ModuleItemsImport => rules::ModuleItemsImport(state),
                Rule::MultiItemImport => rules::MultiItemImport(state),
                Rule::MultiItemImport_member_with_recovery => rules::MultiItemImport_member_with_recovery(state),
                Rule::ItemImport => rules::ItemImport(state),
                Rule::ImportAlias => rules::ImportAlias(state),
                Rule::TypeAnnotation => rules::TypeAnnotation(state),
                Rule::Type => rules::Type(state),
                Rule::RefType => rules::RefType(state),
                Rule::RefTypeStart => rules::RefTypeStart(state),
                Rule::QualPathType => rules::QualPathType(state),
                Rule::TupleType => rules::TupleType(state),
                Rule::TupleTypeItem_with_recover => rules::TupleTypeItem_with_recover(state),
                Rule::TupleTypeItem_recover => rules::TupleTypeItem_recover(state),
                Rule::LambdaType => rules::LambdaType(state),
                Rule::TypeParameterList => rules::TypeParameterList(state),
                Rule::TypeParameter_with_recover => rules::TypeParameter_with_recover(state),
                Rule::TypeParameter_recover => rules::TypeParameter_recover(state),
                Rule::TypeParameter => rules::TypeParameter(state),
                Rule::TypeParamBound => rules::TypeParamBound(state),
                Rule::TypeParamBound_items => rules::TypeParamBound_items(state),
                Rule::TypeParamBound_items_recover => rules::TypeParamBound_items_recover(state),
                Rule::TypeArgumentList => rules::TypeArgumentList(state),
                Rule::TypeArgument => rules::TypeArgument(state),
                Rule::Pat => rules::Pat(state),
                Rule::WildPat => rules::WildPat(state),
                Rule::DerefPat => rules::DerefPat(state),
                Rule::BorrowPat => rules::BorrowPat(state),
                Rule::DotPat => rules::DotPat(state),
                Rule::BindingPat => rules::BindingPat(state),
                Rule::TuplePat => rules::TuplePat(state),
                Rule::StructPat => rules::StructPat(state),
                Rule::StructPatFieldsBlock => rules::StructPatFieldsBlock(state),
                Rule::StructPatField_with_recover => rules::StructPatField_with_recover(state),
                Rule::StructPatField_recover => rules::StructPatField_recover(state),
                Rule::StructPatField => rules::StructPatField(state),
                Rule::StructPatFieldBinding => rules::StructPatFieldBinding(state),
                Rule::Statement => rules::Statement(state),
                Rule::LetStatement => rules::LetStatement(state),
                Rule::StatementExpr => rules::StatementExpr(state),
                Rule::AnyBlock => rules::AnyBlock(state),
                Rule::InlineBlock => rules::InlineBlock(state),
                Rule::CodeBlockExpr => rules::CodeBlockExpr(state),
                Rule::CodeBlock => rules::CodeBlock(state),
                Rule::CodeBlockItems => rules::CodeBlockItems(state),
                Rule::CodeBlockItems_recover => rules::CodeBlockItems_recover(state),
                Rule::Expr => rules::Expr(state),
                Rule::MulExprItem => rules::MulExprItem(state),
                Rule::AddExprItem => rules::AddExprItem(state),
                Rule::LogicalEqExprItem => rules::LogicalEqExprItem(state),
                Rule::ControlFlowExpr => rules::ControlFlowExpr(state),
                Rule::UnaryExpr => rules::UnaryExpr(state),
                Rule::AtomExpr => rules::AtomExpr(state),
                Rule::EqualsExpr => rules::EqualsExpr(state),
                Rule::NotEqualsExpr => rules::NotEqualsExpr(state),
                Rule::OrExpr => rules::OrExpr(state),
                Rule::AndExpr => rules::AndExpr(state),
                Rule::LessExpr => rules::LessExpr(state),
                Rule::GreaterExpr => rules::GreaterExpr(state),
                Rule::LessEqualsExpr => rules::LessEqualsExpr(state),
                Rule::GreaterEqualsExpr => rules::GreaterEqualsExpr(state),
                Rule::BitOrExpr => rules::BitOrExpr(state),
                Rule::BitAndExpr => rules::BitAndExpr(state),
                Rule::BitXorExpr => rules::BitXorExpr(state),
                Rule::CastExpr => rules::CastExpr(state),
                Rule::AnnotatedExpPrefix => rules::AnnotatedExpPrefix(state),
                Rule::AnnotatedExpr => rules::AnnotatedExpr(state),
                Rule::MulExpr => rules::MulExpr(state),
                Rule::DivExpr => rules::DivExpr(state),
                Rule::PlusExpr => rules::PlusExpr(state),
                Rule::MinusExpr => rules::MinusExpr(state),
                Rule::ModExpr => rules::ModExpr(state),
                Rule::LeftShiftExpr => rules::LeftShiftExpr(state),
                Rule::RightShiftExpr => rules::RightShiftExpr(state),
                Rule::ImplyOperatorsExprItem => rules::ImplyOperatorsExprItem(state),
                Rule::ImplyOperatorExpr => rules::ImplyOperatorExpr(state),
                Rule::PartialImplyOperatorExpr => rules::PartialImplyOperatorExpr(state),
                Rule::BangExpr => rules::BangExpr(state),
                Rule::DerefExpr => rules::DerefExpr(state),
                Rule::CopyExpr => rules::CopyExpr(state),
                Rule::MoveExpr => rules::MoveExpr(state),
                Rule::ReturnExpr => rules::ReturnExpr(state),
                Rule::AbortExpr => rules::AbortExpr(state),
                Rule::BreakExpr => rules::BreakExpr(state),
                Rule::ContinueExpr => rules::ContinueExpr(state),
                Rule::StructLiteralExpr => rules::StructLiteralExpr(state),
                Rule::StructLiteralFieldsBlock => rules::StructLiteralFieldsBlock(state),
                Rule::StructLiteralField_with_recover => rules::StructLiteralField_with_recover(state),
                Rule::StructLiteralField_recover => rules::StructLiteralField_recover(state),
                Rule::StructLiteralField => rules::StructLiteralField(state),
                Rule::StructLiteralFieldAssignment => rules::StructLiteralFieldAssignment(state),
                Rule::ParensExpr => rules::ParensExpr(state),
                Rule::LambdaExpr => rules::LambdaExpr(state),
                Rule::RangeExpr => rules::RangeExpr(state),
                Rule::TupleLiteralExpr => rules::TupleLiteralExpr(state),
                Rule::EmptyTupleExpr => rules::EmptyTupleExpr(state),
                Rule::TupleExpr => rules::TupleExpr(state),
                Rule::LiteralExpr => rules::LiteralExpr(state),
                Rule::CallExpr => rules::CallExpr(state),
                Rule::CallArguments => rules::CallArguments(state),
                Rule::IfExpr => rules::IfExpr(state),
                Rule::Condition => rules::Condition(state),
                Rule::ConditionBody => rules::ConditionBody(state),
                Rule::ConditionBody_recover => rules::ConditionBody_recover(state),
                Rule::ElseBlock => rules::ElseBlock(state),
                Rule::LoopExpr => rules::LoopExpr(state),
                Rule::WhileExpr => rules::WhileExpr(state),
                Rule::AssignmentExpr => rules::AssignmentExpr(state),
                Rule::Initializer => rules::Initializer(state),
                Rule::BorrowExpr => rules::BorrowExpr(state),
                Rule::DotExpr => rules::DotExpr(state),
                Rule::StructFieldRef => rules::StructFieldRef(state),
                Rule::IndexExpr => rules::IndexExpr(state),
                Rule::RefExpr => rules::RefExpr(state),
                Rule::QualPath => rules::QualPath(state),
                Rule::ModuleRef => rules::ModuleRef(state),
                Rule::ImportedModuleRef => rules::ImportedModuleRef(state),
                Rule::FullyQualifiedModuleRef => rules::FullyQualifiedModuleRef(state),
                Rule::AddressRef => rules::AddressRef(state),
                Rule::SpecDef => rules::SpecDef(state),
                Rule::SchemaSpecDef => rules::SchemaSpecDef(state),
                Rule::FunctionSpecDef => rules::FunctionSpecDef(state),
                Rule::StructSpecDef => rules::StructSpecDef(state),
                Rule::ModuleSpecDef => rules::ModuleSpecDef(state),
                Rule::DefineFunctionSignature => rules::DefineFunctionSignature(state),
                Rule::DefineFunction => rules::DefineFunction(state),
                Rule::NativeDefineFunction => rules::NativeDefineFunction(state),
                Rule::BlockSpecStatement => rules::BlockSpecStatement(state),
                Rule::SpecBlock => rules::SpecBlock(state),
                Rule::SpecBlockStatement_with_recover => rules::SpecBlockStatement_with_recover(state),
                Rule::SpecBlockFinishingExpr_with_recover => rules::SpecBlockFinishingExpr_with_recover(state),
                Rule::SpecBlockStatement => rules::SpecBlockStatement(state),
                Rule::SpecBlockFinishingExpr => rules::SpecBlockFinishingExpr(state),
                Rule::DefineFunctionSpecDef => rules::DefineFunctionSpecDef(state),
                Rule::NativeDefineFunctionSpecDef => rules::NativeDefineFunctionSpecDef(state),
                Rule::VariableStatement => rules::VariableStatement(state),
                Rule::LocalVariableStatement => rules::LocalVariableStatement(state),
                Rule::GlobalVariableStatement => rules::GlobalVariableStatement(state),
                Rule::PragmaStatement => rules::PragmaStatement(state),
                Rule::PragmaAttribute => rules::PragmaAttribute(state),
                Rule::SpecExpr => rules::SpecExpr(state),
                Rule::AssumeStatement => rules::AssumeStatement(state),
                Rule::AssumeSpecExpr => rules::AssumeSpecExpr(state),
                Rule::AssertStatement => rules::AssertStatement(state),
                Rule::AssertSpecExpr => rules::AssertSpecExpr(state),
                Rule::AbortsIfStatement => rules::AbortsIfStatement(state),
                Rule::AbortsIfSpecExpr => rules::AbortsIfSpecExpr(state),
                Rule::WithExpr => rules::WithExpr(state),
                Rule::SucceedsIfStatement => rules::SucceedsIfStatement(state),
                Rule::SucceedsIfSpecExpr => rules::SucceedsIfSpecExpr(state),
                Rule::RequiresStatement => rules::RequiresStatement(state),
                Rule::RequiresSpecExpr => rules::RequiresSpecExpr(state),
                Rule::EnsuresStatement => rules::EnsuresStatement(state),
                Rule::EnsuresSpecExpr => rules::EnsuresSpecExpr(state),
                Rule::ModifiesStatement => rules::ModifiesStatement(state),
                Rule::ModifiesSpecExpr => rules::ModifiesSpecExpr(state),
                Rule::IncludeStatement => rules::IncludeStatement(state),
                Rule::FunctionPattern => rules::FunctionPattern(state),
                Rule::InvariantStatement => rules::InvariantStatement(state),
                Rule::InvariantSpecExpr => rules::InvariantSpecExpr(state),
                Rule::InvariantModifier => rules::InvariantModifier(state),
                Rule::SpecVisibility => rules::SpecVisibility(state),
                Rule::SpecVisibilityModifier => rules::SpecVisibilityModifier(state),
                Rule::EmitsStatement => rules::EmitsStatement(state),
                Rule::EmitsCondition => rules::EmitsCondition(state),
                Rule::ApplyStatement => rules::ApplyStatement(state),
                Rule::ApplySchemaName => rules::ApplySchemaName(state),
                Rule::ApplySchemaNameAttribute => rules::ApplySchemaNameAttribute(state),
                Rule::Predicate => rules::Predicate(state),
                Rule::AggregatePredicateStatement => rules::AggregatePredicateStatement(state),
                Rule::AggregateExpr => rules::AggregateExpr(state),
                Rule::QuantifierWhere => rules::QuantifierWhere(state),
                Rule::QuantifierExpr => rules::QuantifierExpr(state),
                Rule::ForallQuantifier => rules::ForallQuantifier(state),
                Rule::ExistsQuantifier => rules::ExistsQuantifier(state),
                Rule::QuantifierBindings => rules::QuantifierBindings(state),
                Rule::QuantifierBind => rules::QuantifierBind(state),
                Rule::RangeQuantifierBind => rules::RangeQuantifierBind(state),
                Rule::TypeQuantifierBind => rules::TypeQuantifierBind(state),
                Rule::EOI => rules::EOI(state)
            }
        })
    }
}
