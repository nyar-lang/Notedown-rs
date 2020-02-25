pub struct NoteDownParser;
#[allow(dead_code, non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Rule {
    EOI,
    program,
    TextMode,
    ListMode,
    statement,
    EmptyLine,
    Header,
    TextBlock,
    TextHeaderCharacter,
    TextBlockLine,
    TextElement,
    TextRest,
    Code,
    CodeAction,
    CodeLevel,
    CodeText,
    CodeMark,
    Table,
    TableEnd,
    TableMode,
    TableLine,
    TableEscape,
    TableText,
    TableRest,
    TableMark,
    List,
    ListText,
    ListEnd,
    ListMark,
    HorizontalRule,
    MathStatement,
    Math,
    MathLevel,
    MathText,
    MathRest,
    RawStatement,
    Raw,
    RawLevel,
    RawText,
    RawRest,
    StyleStatement,
    Style,
    StyleLevel,
    StyleText,
    StyleRest,
    LineStatement,
    Line,
    LineLevel,
    LineText,
    LineRest,
    Command,
    CommandBlock,
    arguments,
    argument_literal,
    argument,
    key_value,
    key,
    value,
    expression,
    function,
    CommandLine,
    command,
    RestOfLine,
    Number,
    Decimal,
    DecimalBad,
    Integer,
    Zero,
    String,
    S1,
    S2,
    SYMBOL,
    Escaped,
    Keywords,
    COMMENT,
    LineComment,
    BlockComment,
    MultiLineComment,
    Comment,
    NEWLINE,
    SPACE_SEPARATOR,
    TAB,
    CR,
    LF,
    Escape,
    At,
    Sharp,
    Underline,
    Asterisk,
    Comma,
    Dot,
    Set,
    Colon,
    Vertical,
    Plus,
    Minus,
    QuoteMark,
    Accent,
    Dollar,
    Tilde,
}
#[allow(clippy::all)]
impl ::pest::Parser<Rule> for NoteDownParser {
    fn parse<'i>(rule: Rule, input: &'i str) -> ::std::result::Result<::pest::iterators::Pairs<'i, Rule>, ::pest::error::Error<Rule>> {
        mod rules {
            pub mod hidden {
                use super::super::Rule;
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn skip(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    if state.atomicity() == ::pest::Atomicity::NonAtomic { state.repeat(|state| super::visible::COMMENT(state)) } else { Ok(state) }
                }
            }
            pub mod visible {
                use super::super::Rule;
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn program(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| self::SOI(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::statement(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| self::statement(state))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::EOI(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TextMode(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| self::SOI(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::TextElement(state)).or_else(|state| self::TextRest(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| self::TextElement(state)).or_else(|state| self::TextRest(state))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::EOI(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ListMode(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::ListMode, |state| state.sequence(|state| state.sequence(|state| state.optional(|state| self::SPACE_SEPARATOR(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::SPACE_SEPARATOR(state))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::ListMark(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ListText(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ListEnd(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn statement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::Header(state).or_else(|state| self::HorizontalRule(state)).or_else(|state| state.restore_on_err(|state| self::Code(state))).or_else(|state| self::Table(state)).or_else(|state| self::List(state)).or_else(|state| self::Command(state)).or_else(|state| state.sequence(|state| self::EmptyLine(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::EmptyLine(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::EmptyLine(state)))))))))).or_else(|state| self::TextBlock(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn EmptyLine(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| state.sequence(|state| state.optional(|state| self::SPACE_SEPARATOR(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::SPACE_SEPARATOR(state))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::NEWLINE(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Header(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::Header, |state| state.sequence(|state| state.repeat(|state| self::SPACE_SEPARATOR(state)).and_then(|state| state.sequence(|state| self::Sharp(state).and_then(|state| state.optional(|state| self::Sharp(state))).and_then(|state| state.optional(|state| self::Sharp(state))).and_then(|state| state.optional(|state| self::Sharp(state))).and_then(|state| state.optional(|state| self::Sharp(state))).and_then(|state| state.optional(|state| self::Sharp(state))))).and_then(|state| self::RestOfLine(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TextBlock(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::TextBlock, |state| state.sequence(|state| self::TextHeaderCharacter(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::TextBlockLine(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::TextBlockLine(state))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TextHeaderCharacter(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TextBlockLine(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::EmptyLine(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::EmptyLine(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::EmptyLine(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::EmptyLine(state))))))))).or_else(|state| self::Sharp(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TextElement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.restore_on_err(|state| self::StyleStatement(state)).or_else(|state| state.restore_on_err(|state| self::LineStatement(state))).or_else(|state| state.restore_on_err(|state| self::MathStatement(state))).or_else(|state| state.restore_on_err(|state| self::RawStatement(state))).or_else(|state| self::NEWLINE(state)).or_else(|state| self::Escaped(state)).or_else(|state| self::Command(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TextRest(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::TextRest, |state| state.sequence(|state| state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state).or_else(|state| self::Escape(state)).or_else(|state| self::Tilde(state)).or_else(|state| self::Asterisk(state)).or_else(|state| self::Dollar(state)).or_else(|state| self::Accent(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state).or_else(|state| self::Escape(state)).or_else(|state| self::Tilde(state)).or_else(|state| self::Asterisk(state)).or_else(|state| self::Dollar(state)).or_else(|state| self::Accent(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state).or_else(|state| self::Escape(state)).or_else(|state| self::Tilde(state)).or_else(|state| self::Asterisk(state)).or_else(|state| self::Dollar(state)).or_else(|state| self::Accent(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state))))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Code(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Code, |state| state.sequence(|state| state.sequence(|state| state.optional(|state| self::SPACE_SEPARATOR(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::SPACE_SEPARATOR(state))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::CodeLevel(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::CodeAction(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::SYMBOL(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::arguments(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::SPACE_SEPARATOR(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::SPACE_SEPARATOR(state)))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::CodeText(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::POP(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::CodeMark(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::CodeMark(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::CodeMark(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn CodeAction(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::CodeAction, |state| self::At(state).or_else(|state| self::Asterisk(state)).or_else(|state| self::Sharp(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn CodeLevel(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::CodeLevel, |state| state.sequence(|state| state.sequence(|state| self::CodeMark(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::CodeMark(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::CodeMark(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.stack_push(|state| state.sequence(|state| state.optional(|state| self::CodeMark(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::CodeMark(state)))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn CodeText(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::CodeText, |state| state.sequence(|state| state.optional(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::CodeMark(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::PEEK(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::CodeMark(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::PEEK(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn CodeMark(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::CodeMark, |state| state.atomic(::pest::Atomicity::Atomic, |state| self::Accent(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Table(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Table, |state| state.sequence(|state| state.sequence(|state| state.optional(|state| self::SPACE_SEPARATOR(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::SPACE_SEPARATOR(state))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::TableMark(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.sequence(|state| state.lookahead(false, |state| self::TableEnd(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| state.lookahead(false, |state| self::TableEnd(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state)))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::TableEnd(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TableEnd(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::TableEnd, |state| state.sequence(|state| self::EmptyLine(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::EmptyLine(state))).or_else(|state| state.sequence(|state| self::EmptyLine(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::EOI(state)))).or_else(|state| self::EOI(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TableMode(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| self::SOI(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::TableLine(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| self::TableLine(state))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::EOI(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TableLine(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::TableEscape(state).or_else(|state| self::TableMark(state)).or_else(|state| state.restore_on_err(|state| self::TextElement(state))).or_else(|state| self::TableRest(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TableEscape(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::TableEscape, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::Escape(state).and_then(|state| self::TableMark(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TableText(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::TableText, |state| state.sequence(|state| self::TextElement(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::TextElement(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| self::TextElement(state)))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TableRest(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::TableRest, |state| state.sequence(|state| state.sequence(|state| state.lookahead(false, |state| self::TableMark(state).or_else(|state| self::Escape(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.sequence(|state| state.lookahead(false, |state| self::TableMark(state).or_else(|state| self::Escape(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| state.lookahead(false, |state| self::TableMark(state).or_else(|state| self::Escape(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state))))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TableMark(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::TableMark, |state| state.atomic(::pest::Atomicity::Atomic, |state| self::Vertical(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn List(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::List, |state| state.sequence(|state| state.sequence(|state| state.optional(|state| self::SPACE_SEPARATOR(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::SPACE_SEPARATOR(state))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ListMark(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ListText(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ListEnd(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ListText(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::ListText, |state| state.sequence(|state| state.optional(|state| state.sequence(|state| state.lookahead(false, |state| self::ListEnd(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| state.lookahead(false, |state| self::ListEnd(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ListEnd(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::ListEnd, |state| state.sequence(|state| self::EmptyLine(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::EmptyLine(state))).or_else(|state| state.sequence(|state| self::EmptyLine(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::EOI(state)))).or_else(|state| self::EOI(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ListMark(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::ListMark, |state| state.atomic(::pest::Atomicity::Atomic, |state| self::Minus(state).or_else(|state| self::QuoteMark(state)).or_else(|state| state.sequence(|state| self::Integer(state).and_then(|state| self::Dot(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn HorizontalRule(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::HorizontalRule, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| state.repeat(|state| self::SPACE_SEPARATOR(state)).and_then(|state| self::Minus(state)).and_then(|state| self::Minus(state)).and_then(|state| self::Minus(state)).and_then(|state| state.repeat(|state| self::Minus(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn MathStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.restore_on_err(|state| self::Math(state)).or_else(|state| self::MathRest(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Math(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Math, |state| state.sequence(|state| self::MathLevel(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::MathText(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::POP(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Dollar(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn MathLevel(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::MathLevel, |state| state.sequence(|state| self::Dollar(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.stack_push(|state| state.sequence(|state| state.optional(|state| self::Dollar(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::Dollar(state)))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn MathText(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::MathText, |state| state.sequence(|state| state.optional(|state| state.sequence(|state| self::Escape(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Dollar(state))).or_else(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::Dollar(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::PEEK(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| self::Escape(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Dollar(state))).or_else(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::Dollar(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::PEEK(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state)))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn MathRest(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::MathRest, |state| state.sequence(|state| self::Dollar(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::Dollar(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::Dollar(state))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn RawStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.restore_on_err(|state| self::Raw(state)).or_else(|state| self::RawRest(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Raw(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Raw, |state| state.sequence(|state| self::RawLevel(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::RawText(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::POP(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Accent(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn RawLevel(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::RawLevel, |state| state.sequence(|state| self::Accent(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.stack_push(|state| state.sequence(|state| state.optional(|state| self::Accent(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::Accent(state)))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn RawText(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::RawText, |state| state.sequence(|state| state.optional(|state| state.sequence(|state| self::Escape(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Accent(state))).or_else(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::Accent(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::PEEK(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| self::Escape(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Accent(state))).or_else(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::Accent(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::PEEK(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state)))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn RawRest(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::RawRest, |state| state.sequence(|state| self::Accent(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::Accent(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::Accent(state))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StyleStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.restore_on_err(|state| self::Style(state)).or_else(|state| self::StyleRest(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Style(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Style, |state| state.sequence(|state| self::StyleLevel(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::StyleText(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::POP(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Asterisk(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StyleLevel(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::StyleLevel, |state| state.sequence(|state| self::Asterisk(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.stack_push(|state| state.sequence(|state| state.optional(|state| self::Asterisk(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::Asterisk(state)))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StyleText(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::StyleText, |state| state.sequence(|state| state.optional(|state| state.sequence(|state| self::Escape(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Asterisk(state))).or_else(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::Asterisk(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::PEEK(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| self::Escape(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Asterisk(state))).or_else(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::Asterisk(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::PEEK(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state)))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StyleRest(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::StyleRest, |state| state.sequence(|state| self::Asterisk(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::Asterisk(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::Asterisk(state))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LineStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.restore_on_err(|state| self::Line(state)).or_else(|state| self::LineRest(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Line(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Line, |state| state.sequence(|state| self::LineLevel(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::LineText(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::POP(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Tilde(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LineLevel(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::LineLevel, |state| state.sequence(|state| self::Tilde(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.stack_push(|state| state.sequence(|state| state.optional(|state| self::Tilde(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::Tilde(state)))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LineText(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::LineText, |state| state.sequence(|state| state.optional(|state| state.sequence(|state| self::Escape(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Tilde(state))).or_else(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::Tilde(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::PEEK(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| self::Escape(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Tilde(state))).or_else(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::Tilde(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::PEEK(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state)))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LineRest(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::LineRest, |state| state.sequence(|state| self::Tilde(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::Tilde(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::Tilde(state))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Command(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| state.sequence(|state| state.optional(|state| self::SPACE_SEPARATOR(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::SPACE_SEPARATOR(state))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::CommandLine(state).or_else(|state| self::CommandBlock(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn CommandBlock(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::CommandBlock, |state| state.sequence(|state| self::command(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::arguments(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::arguments(state)))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn arguments(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| state.match_string("{").and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::SPACE_SEPARATOR(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::SPACE_SEPARATOR(state)))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::key_value(state).or_else(|state| self::argument(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::key_value(state).or_else(|state| self::argument(state))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::SPACE_SEPARATOR(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::SPACE_SEPARATOR(state)))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("}"))).or_else(|state| state.sequence(|state| state.match_string("[").and_then(|state| super::hidden::skip(state)).and_then(|state| self::argument_literal(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("]"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn argument_literal(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::argument_literal, |state| state.sequence(|state| state.optional(|state| state.sequence(|state| self::Escape(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Escape(state).or_else(|state| state.match_string("]")))).or_else(|state| state.sequence(|state| state.lookahead(false, |state| state.match_string("]")).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| self::Escape(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Escape(state).or_else(|state| state.match_string("]")))).or_else(|state| state.sequence(|state| state.lookahead(false, |state| state.match_string("]")).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state)))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn argument(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::argument, |state| state.sequence(|state| self::value(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::SPACE_SEPARATOR(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::SPACE_SEPARATOR(state)))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::Comma(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::SPACE_SEPARATOR(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::SPACE_SEPARATOR(state))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn key_value(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::key_value, |state| state.sequence(|state| self::key(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::SPACE_SEPARATOR(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::SPACE_SEPARATOR(state)))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Set(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::SPACE_SEPARATOR(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::SPACE_SEPARATOR(state)))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::value(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::SPACE_SEPARATOR(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::SPACE_SEPARATOR(state)))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::Comma(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::SPACE_SEPARATOR(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::SPACE_SEPARATOR(state))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn key(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::key, |state| self::Integer(state).or_else(|state| self::String(state)).or_else(|state| self::SYMBOL(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn value(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::value, |state| self::Integer(state).or_else(|state| self::String(state)).or_else(|state| self::expression(state)).or_else(|state| self::Keywords(state)).or_else(|state| self::SYMBOL(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn expression(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::expression, |state| state.sequence(|state| self::SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| self::Dot(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::function(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.sequence(|state| self::Dot(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::function(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| self::Dot(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::function(state))))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn function(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::function, |state| state.sequence(|state| self::SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.sequence(|state| state.match_string("(").and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::SPACE_SEPARATOR(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::SPACE_SEPARATOR(state)))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::key_value(state).or_else(|state| self::argument(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::key_value(state).or_else(|state| self::argument(state))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::SPACE_SEPARATOR(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::SPACE_SEPARATOR(state)))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string(")")))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn CommandLine(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::CommandLine, |state| state.sequence(|state| self::command(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Colon(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::RestOfLine(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn command(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::command, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::Escape(state).and_then(|state| self::SYMBOL(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn RestOfLine(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::RestOfLine, |state| state.sequence(|state| state.optional(|state| state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state)))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Number(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::Number, |state| self::Decimal(state).or_else(|state| self::DecimalBad(state)).or_else(|state| self::Integer(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Decimal(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::Decimal, |state| state.sequence(|state| self::Integer(state).and_then(|state| self::Dot(state)).and_then(|state| self::ASCII_DIGIT(state)).and_then(|state| state.repeat(|state| self::ASCII_DIGIT(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn DecimalBad(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::DecimalBad, |state| state.sequence(|state| self::Integer(state).and_then(|state| self::Dot(state))).or_else(|state| state.sequence(|state| self::Dot(state).and_then(|state| self::ASCII_DIGIT(state)).and_then(|state| state.repeat(|state| self::ASCII_DIGIT(state)))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Integer(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Integer, |state| state.atomic(::pest::Atomicity::Atomic, |state| self::Zero(state).or_else(|state| state.sequence(|state| self::ASCII_DIGIT(state).and_then(|state| state.repeat(|state| state.sequence(|state| state.optional(|state| self::Underline(state)).and_then(|state| self::ASCII_NONZERO_DIGIT(state)))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Zero(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Zero, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("0")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn String(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::String, |state| state.sequence(|state| self::S1(state).and_then(|state| state.repeat(|state| state.sequence(|state| self::Escape(state).and_then(|state| self::Escape(state).or_else(|state| self::S1(state)))).or_else(|state| state.sequence(|state| state.lookahead(false, |state| self::S1(state)).and_then(|state| self::ANY(state)))))).and_then(|state| self::S1(state))).or_else(|state| state.sequence(|state| self::S2(state).and_then(|state| state.repeat(|state| state.sequence(|state| self::Escape(state).and_then(|state| self::Escape(state).or_else(|state| self::S2(state)))).or_else(|state| state.sequence(|state| state.lookahead(false, |state| self::S2(state)).and_then(|state| self::ANY(state)))))).and_then(|state| self::S2(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn S1(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("'")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn S2(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("\"")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SYMBOL(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::SYMBOL, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::XID_START(state).and_then(|state| state.repeat(|state| self::XID_CONTINUE(state)))).or_else(|state| state.sequence(|state| self::Underline(state).and_then(|state| self::XID_CONTINUE(state)).and_then(|state| state.repeat(|state| self::XID_CONTINUE(state)))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Escaped(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Escaped, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::Escape(state).and_then(|state| self::Escape(state).or_else(|state| self::Tilde(state)).or_else(|state| self::Asterisk(state)).or_else(|state| self::Dollar(state)).or_else(|state| self::Accent(state)).or_else(|state| self::NEWLINE(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Keywords(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Keywords, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("true").or_else(|state| state.match_string("false")).or_else(|state| state.match_string("null"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn COMMENT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::COMMENT, |state| self::BlockComment(state).or_else(|state| self::LineComment(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LineComment(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::LineComment, |state| state.sequence(|state| self::Comment(state).and_then(|state| state.repeat(|state| self::SPACE_SEPARATOR(state))).and_then(|state| self::Colon(state)).and_then(|state| self::RestOfLine(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn BlockComment(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::BlockComment, |state| state.sequence(|state| self::Comment(state).and_then(|state| state.repeat(|state| self::SPACE_SEPARATOR(state))).and_then(|state| state.match_string("{")).and_then(|state| self::MultiLineComment(state)).and_then(|state| state.match_string("}")))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn MultiLineComment(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::MultiLineComment, |state| state.repeat(|state| self::BlockComment(state).or_else(|state| state.sequence(|state| state.lookahead(false, |state| state.match_string("}")).and_then(|state| self::ANY(state)))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Comment(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Comment, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::Escape(state).and_then(|state| state.match_string("comment")))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn NEWLINE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::NEWLINE, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::CR(state).and_then(|state| self::LF(state))).or_else(|state| self::CR(state)).or_else(|state| self::LF(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SPACE_SEPARATOR(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::SPACE_SEPARATOR, |state| state.match_string(" ").or_else(|state| state.match_string("\t")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TAB(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::TAB, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("\t")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn CR(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::CR, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("\r")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LF(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::LF, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("\n")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Escape(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Escape, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("\\")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn At(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::At, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("@")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Sharp(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Sharp, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("#")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Underline(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Underline, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("_")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Asterisk(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Asterisk, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("*")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Comma(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Comma, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(",")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Dot(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Dot, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(".")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Set(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Set, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("=")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Colon(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Colon, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(":")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Vertical(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Vertical, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("|")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Plus(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Plus, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("+")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Minus(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Minus, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("-")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn QuoteMark(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::QuoteMark, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(">")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Accent(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Accent, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("`")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Dollar(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Dollar, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("$")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Tilde(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Tilde, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("~")))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn PEEK(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.stack_peek()
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ANY(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.skip(1)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn POP(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.stack_pop()
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_NONZERO_DIGIT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('1'..'9')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn SOI(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.start_of_input()
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn XID_CONTINUE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_char_by(::pest::unicode::XID_CONTINUE)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn EOI(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::EOI, |state| state.end_of_input())
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn XID_START(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_char_by(::pest::unicode::XID_START)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_DIGIT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('0'..'9')
                }
            }
            pub use self::visible::*;
        }
        ::pest::state(input, |state| match rule {
            Rule::program => rules::program(state),
            Rule::TextMode => rules::TextMode(state),
            Rule::ListMode => rules::ListMode(state),
            Rule::statement => rules::statement(state),
            Rule::EmptyLine => rules::EmptyLine(state),
            Rule::Header => rules::Header(state),
            Rule::TextBlock => rules::TextBlock(state),
            Rule::TextHeaderCharacter => rules::TextHeaderCharacter(state),
            Rule::TextBlockLine => rules::TextBlockLine(state),
            Rule::TextElement => rules::TextElement(state),
            Rule::TextRest => rules::TextRest(state),
            Rule::Code => rules::Code(state),
            Rule::CodeAction => rules::CodeAction(state),
            Rule::CodeLevel => rules::CodeLevel(state),
            Rule::CodeText => rules::CodeText(state),
            Rule::CodeMark => rules::CodeMark(state),
            Rule::Table => rules::Table(state),
            Rule::TableEnd => rules::TableEnd(state),
            Rule::TableMode => rules::TableMode(state),
            Rule::TableLine => rules::TableLine(state),
            Rule::TableEscape => rules::TableEscape(state),
            Rule::TableText => rules::TableText(state),
            Rule::TableRest => rules::TableRest(state),
            Rule::TableMark => rules::TableMark(state),
            Rule::List => rules::List(state),
            Rule::ListText => rules::ListText(state),
            Rule::ListEnd => rules::ListEnd(state),
            Rule::ListMark => rules::ListMark(state),
            Rule::HorizontalRule => rules::HorizontalRule(state),
            Rule::MathStatement => rules::MathStatement(state),
            Rule::Math => rules::Math(state),
            Rule::MathLevel => rules::MathLevel(state),
            Rule::MathText => rules::MathText(state),
            Rule::MathRest => rules::MathRest(state),
            Rule::RawStatement => rules::RawStatement(state),
            Rule::Raw => rules::Raw(state),
            Rule::RawLevel => rules::RawLevel(state),
            Rule::RawText => rules::RawText(state),
            Rule::RawRest => rules::RawRest(state),
            Rule::StyleStatement => rules::StyleStatement(state),
            Rule::Style => rules::Style(state),
            Rule::StyleLevel => rules::StyleLevel(state),
            Rule::StyleText => rules::StyleText(state),
            Rule::StyleRest => rules::StyleRest(state),
            Rule::LineStatement => rules::LineStatement(state),
            Rule::Line => rules::Line(state),
            Rule::LineLevel => rules::LineLevel(state),
            Rule::LineText => rules::LineText(state),
            Rule::LineRest => rules::LineRest(state),
            Rule::Command => rules::Command(state),
            Rule::CommandBlock => rules::CommandBlock(state),
            Rule::arguments => rules::arguments(state),
            Rule::argument_literal => rules::argument_literal(state),
            Rule::argument => rules::argument(state),
            Rule::key_value => rules::key_value(state),
            Rule::key => rules::key(state),
            Rule::value => rules::value(state),
            Rule::expression => rules::expression(state),
            Rule::function => rules::function(state),
            Rule::CommandLine => rules::CommandLine(state),
            Rule::command => rules::command(state),
            Rule::RestOfLine => rules::RestOfLine(state),
            Rule::Number => rules::Number(state),
            Rule::Decimal => rules::Decimal(state),
            Rule::DecimalBad => rules::DecimalBad(state),
            Rule::Integer => rules::Integer(state),
            Rule::Zero => rules::Zero(state),
            Rule::String => rules::String(state),
            Rule::S1 => rules::S1(state),
            Rule::S2 => rules::S2(state),
            Rule::SYMBOL => rules::SYMBOL(state),
            Rule::Escaped => rules::Escaped(state),
            Rule::Keywords => rules::Keywords(state),
            Rule::COMMENT => rules::COMMENT(state),
            Rule::LineComment => rules::LineComment(state),
            Rule::BlockComment => rules::BlockComment(state),
            Rule::MultiLineComment => rules::MultiLineComment(state),
            Rule::Comment => rules::Comment(state),
            Rule::NEWLINE => rules::NEWLINE(state),
            Rule::SPACE_SEPARATOR => rules::SPACE_SEPARATOR(state),
            Rule::TAB => rules::TAB(state),
            Rule::CR => rules::CR(state),
            Rule::LF => rules::LF(state),
            Rule::Escape => rules::Escape(state),
            Rule::At => rules::At(state),
            Rule::Sharp => rules::Sharp(state),
            Rule::Underline => rules::Underline(state),
            Rule::Asterisk => rules::Asterisk(state),
            Rule::Comma => rules::Comma(state),
            Rule::Dot => rules::Dot(state),
            Rule::Set => rules::Set(state),
            Rule::Colon => rules::Colon(state),
            Rule::Vertical => rules::Vertical(state),
            Rule::Plus => rules::Plus(state),
            Rule::Minus => rules::Minus(state),
            Rule::QuoteMark => rules::QuoteMark(state),
            Rule::Accent => rules::Accent(state),
            Rule::Dollar => rules::Dollar(state),
            Rule::Tilde => rules::Tilde(state),
            Rule::EOI => rules::EOI(state),
        })
    }
}
