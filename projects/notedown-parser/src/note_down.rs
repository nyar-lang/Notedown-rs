pub struct NoteDownParser;
#[allow(dead_code, non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Rule {
    EOI,
    program,
    statement,
    EmptyLine,
    BlockEnd,
    RestOfLine,
    Header,
    TextBlock,
    TextElement,
    TextRest,
    URL,
    Code,
    CodeAction,
    CodeLevel,
    CodeText,
    CodeMark,
    Table,
    TableFirstLine,
    TableRestLine,
    List,
    ListFirstLine,
    ListRestLine,
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
    TildeStatement,
    TildeLine,
    TildeLevel,
    TildeText,
    TildeRest,
    Template,
    HTMLOpenClose,
    HTMLOpen,
    HTMLClose,
    HTMLSelfClose,
    Command,
    CommandLine,
    CommandBlock,
    call,
    arguments,
    argument_literal,
    argument,
    key_value,
    key,
    value,
    Number,
    Decimal,
    DecimalBad,
    Integer,
    Sign,
    String,
    S1,
    S2,
    SYMBOL,
    Escaped,
    Keywords,
    LINE_SEPARATOR,
    WHITE_SPACE,
    PATTERN_WHITE_SPACE,
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
                    Ok(state)
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
                pub fn statement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.restore_on_err(|state| self::Header(state)).or_else(|state| self::HorizontalRule(state)).or_else(|state| state.restore_on_err(|state| self::Code(state))).or_else(|state| state.restore_on_err(|state| self::Table(state))).or_else(|state| state.restore_on_err(|state| self::List(state))).or_else(|state| state.restore_on_err(|state| self::Template(state))).or_else(|state| self::Command(state)).or_else(|state| state.sequence(|state| self::EmptyLine(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::EmptyLine(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::EmptyLine(state)))))))))).or_else(|state| state.restore_on_err(|state| self::TextBlock(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn EmptyLine(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| state.sequence(|state| state.optional(|state| self::WHITE_SPACE(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::WHITE_SPACE(state))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::LINE_SEPARATOR(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn BlockEnd(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::BlockEnd, |state| state.sequence(|state| self::EmptyLine(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::EmptyLine(state))).or_else(|state| state.sequence(|state| self::EmptyLine(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::EOI(state)))).or_else(|state| self::EOI(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn RestOfLine(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::RestOfLine, |state| state.sequence(|state| state.optional(|state| state.sequence(|state| state.lookahead(false, |state| self::LINE_SEPARATOR(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| state.lookahead(false, |state| self::LINE_SEPARATOR(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state)))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Header(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::Header, |state| state.sequence(|state| state.repeat(|state| self::WHITE_SPACE(state)).and_then(|state| state.sequence(|state| self::Sharp(state).and_then(|state| state.optional(|state| self::Sharp(state))).and_then(|state| state.optional(|state| self::Sharp(state))).and_then(|state| state.optional(|state| self::Sharp(state))).and_then(|state| state.optional(|state| self::Sharp(state))).and_then(|state| state.optional(|state| self::Sharp(state))))).and_then(|state| self::TextElement(state)).and_then(|state| state.repeat(|state| state.restore_on_err(|state| self::TextElement(state)))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TextBlock(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::TextBlock, |state| state.sequence(|state| state.restore_on_err(|state| self::TextElement(state)).or_else(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::LINE_SEPARATOR(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::LINE_SEPARATOR(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::LINE_SEPARATOR(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::LINE_SEPARATOR(state)))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::LINE_SEPARATOR(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::TextElement(state)).or_else(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::LINE_SEPARATOR(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::LINE_SEPARATOR(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::LINE_SEPARATOR(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::LINE_SEPARATOR(state)))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::LINE_SEPARATOR(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| self::TextElement(state)).or_else(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::LINE_SEPARATOR(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::LINE_SEPARATOR(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::LINE_SEPARATOR(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::LINE_SEPARATOR(state)))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::LINE_SEPARATOR(state)))))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TextElement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.restore_on_err(|state| self::StyleStatement(state)).or_else(|state| state.restore_on_err(|state| self::TildeStatement(state))).or_else(|state| state.restore_on_err(|state| self::MathStatement(state))).or_else(|state| state.restore_on_err(|state| self::RawStatement(state))).or_else(|state| self::CommandLine(state)).or_else(|state| self::CommandBlock(state)).or_else(|state| self::HTMLSelfClose(state)).or_else(|state| state.restore_on_err(|state| self::HTMLOpenClose(state))).or_else(|state| self::URL(state)).or_else(|state| self::Escaped(state)).or_else(|state| self::TextRest(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TextRest(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::TextRest, |state| state.sequence(|state| state.sequence(|state| state.lookahead(false, |state| self::LINE_SEPARATOR(state).or_else(|state| self::Escape(state)).or_else(|state| self::Vertical(state)).or_else(|state| self::Tilde(state)).or_else(|state| self::Asterisk(state)).or_else(|state| self::Dollar(state)).or_else(|state| self::Accent(state)).or_else(|state| self::Colon(state)).or_else(|state| state.match_string("<"))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.sequence(|state| state.lookahead(false, |state| self::LINE_SEPARATOR(state).or_else(|state| self::Escape(state)).or_else(|state| self::Vertical(state)).or_else(|state| self::Tilde(state)).or_else(|state| self::Asterisk(state)).or_else(|state| self::Dollar(state)).or_else(|state| self::Accent(state)).or_else(|state| self::Colon(state)).or_else(|state| state.match_string("<"))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| state.lookahead(false, |state| self::LINE_SEPARATOR(state).or_else(|state| self::Escape(state)).or_else(|state| self::Vertical(state)).or_else(|state| self::Tilde(state)).or_else(|state| self::Asterisk(state)).or_else(|state| self::Dollar(state)).or_else(|state| self::Accent(state)).or_else(|state| self::Colon(state)).or_else(|state| state.match_string("<"))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state))))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn URL(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::URL, |state| state.sequence(|state| state.sequence(|state| self::ASCII_ALPHA(state).and_then(|state| state.repeat(|state| self::ASCII_ALPHA(state)))).and_then(|state| state.match_string("://")).and_then(|state| state.sequence(|state| state.lookahead(false, |state| self::PATTERN_WHITE_SPACE(state)).and_then(|state| self::ANY(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| self::PATTERN_WHITE_SPACE(state)).and_then(|state| self::ANY(state))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Code(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Code, |state| state.sequence(|state| state.sequence(|state| state.optional(|state| self::WHITE_SPACE(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::WHITE_SPACE(state))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::CodeLevel(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::CodeAction(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::SYMBOL(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::arguments(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::WHITE_SPACE(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::WHITE_SPACE(state)))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::CodeText(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::POP(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::CodeMark(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::CodeMark(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::CodeMark(state))))
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
                    state.rule(Rule::Table, |state| state.sequence(|state| self::TableFirstLine(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| state.sequence(|state| self::LINE_SEPARATOR(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::TableRestLine(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| state.sequence(|state| self::LINE_SEPARATOR(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::TableRestLine(state)))))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TableFirstLine(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::TableFirstLine, |state| state.sequence(|state| state.sequence(|state| state.optional(|state| self::WHITE_SPACE(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::WHITE_SPACE(state))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Vertical(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.sequence(|state| self::Escape(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Vertical(state))).or_else(|state| self::Vertical(state)).or_else(|state| state.restore_on_err(|state| self::TextElement(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| self::Escape(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Vertical(state))).or_else(|state| self::Vertical(state)).or_else(|state| state.restore_on_err(|state| self::TextElement(state))))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TableRestLine(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::TableRestLine, |state| state.sequence(|state| state.optional(|state| state.sequence(|state| self::Escape(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Vertical(state))).or_else(|state| self::Vertical(state)).or_else(|state| state.restore_on_err(|state| self::TextElement(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| self::Escape(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Vertical(state))).or_else(|state| self::Vertical(state)).or_else(|state| state.restore_on_err(|state| self::TextElement(state))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn List(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::List, |state| state.sequence(|state| self::ListFirstLine(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| state.sequence(|state| self::LINE_SEPARATOR(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ListRestLine(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| state.sequence(|state| self::LINE_SEPARATOR(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ListRestLine(state)))))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ListFirstLine(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::ListFirstLine, |state| state.sequence(|state| state.sequence(|state| state.optional(|state| self::WHITE_SPACE(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::WHITE_SPACE(state))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ListMark(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::TextElement(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::TextElement(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| self::TextElement(state)))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ListRestLine(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::ListRestLine, |state| state.sequence(|state| state.sequence(|state| state.optional(|state| self::WHITE_SPACE(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::WHITE_SPACE(state))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ListMark(state).or_else(|state| self::Vertical(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::TextElement(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::TextElement(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| self::TextElement(state)))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ListMark(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::ListMark, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::Minus(state).or_else(|state| self::Plus(state)).or_else(|state| self::QuoteMark(state)).or_else(|state| state.sequence(|state| self::Integer(state).and_then(|state| self::Dot(state)))).and_then(|state| self::WHITE_SPACE(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn HorizontalRule(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::HorizontalRule, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| state.repeat(|state| self::WHITE_SPACE(state)).and_then(|state| state.sequence(|state| self::Minus(state).and_then(|state| self::Minus(state)).and_then(|state| self::Minus(state)).and_then(|state| state.repeat(|state| self::Minus(state)))).or_else(|state| state.sequence(|state| self::Set(state).and_then(|state| self::Set(state)).and_then(|state| self::Set(state)).and_then(|state| state.repeat(|state| self::Set(state)))))))))
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
                    state.rule(Rule::MathText, |state| state.sequence(|state| state.optional(|state| state.sequence(|state| self::Escape(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Dollar(state))).or_else(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::Dollar(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::PEEK(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.lookahead(false, |state| state.sequence(|state| self::LINE_SEPARATOR(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::LINE_SEPARATOR(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::LINE_SEPARATOR(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::LINE_SEPARATOR(state))))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| self::Escape(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Dollar(state))).or_else(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::Dollar(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::PEEK(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.lookahead(false, |state| state.sequence(|state| self::LINE_SEPARATOR(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::LINE_SEPARATOR(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::LINE_SEPARATOR(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::LINE_SEPARATOR(state))))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state)))))))))))
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
                    state.rule(Rule::RawText, |state| state.sequence(|state| state.optional(|state| state.sequence(|state| self::Escape(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Accent(state))).or_else(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::Accent(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::PEEK(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.lookahead(false, |state| state.sequence(|state| self::LINE_SEPARATOR(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::LINE_SEPARATOR(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::LINE_SEPARATOR(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::LINE_SEPARATOR(state))))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| self::Escape(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Accent(state))).or_else(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::Accent(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::PEEK(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.lookahead(false, |state| state.sequence(|state| self::LINE_SEPARATOR(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::LINE_SEPARATOR(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::LINE_SEPARATOR(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::LINE_SEPARATOR(state))))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state)))))))))))
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
                    state.rule(Rule::StyleText, |state| state.sequence(|state| state.optional(|state| state.sequence(|state| self::Escape(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Asterisk(state))).or_else(|state| state.restore_on_err(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::Asterisk(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::PEEK(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.lookahead(false, |state| state.sequence(|state| self::LINE_SEPARATOR(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::LINE_SEPARATOR(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::LINE_SEPARATOR(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::LINE_SEPARATOR(state))))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::TextElement(state))))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| self::Escape(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Asterisk(state))).or_else(|state| state.restore_on_err(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::Asterisk(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::PEEK(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.lookahead(false, |state| state.sequence(|state| self::LINE_SEPARATOR(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::LINE_SEPARATOR(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::LINE_SEPARATOR(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::LINE_SEPARATOR(state))))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::TextElement(state))))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StyleRest(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::StyleRest, |state| state.sequence(|state| self::Asterisk(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::Asterisk(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::Asterisk(state))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TildeStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.restore_on_err(|state| self::TildeLine(state)).or_else(|state| self::TildeRest(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TildeLine(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::TildeLine, |state| state.sequence(|state| self::TildeLevel(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::TildeText(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::POP(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Tilde(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TildeLevel(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::TildeLevel, |state| state.sequence(|state| self::Tilde(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.stack_push(|state| state.sequence(|state| state.optional(|state| self::Tilde(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::Tilde(state)))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TildeText(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::TildeText, |state| state.sequence(|state| state.optional(|state| state.sequence(|state| self::Escape(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Tilde(state))).or_else(|state| state.restore_on_err(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::Tilde(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::PEEK(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.lookahead(false, |state| state.sequence(|state| self::LINE_SEPARATOR(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::LINE_SEPARATOR(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::LINE_SEPARATOR(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::LINE_SEPARATOR(state))))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::TextElement(state))))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| self::Escape(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Tilde(state))).or_else(|state| state.restore_on_err(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::Tilde(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::PEEK(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.lookahead(false, |state| state.sequence(|state| self::LINE_SEPARATOR(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::LINE_SEPARATOR(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::LINE_SEPARATOR(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::LINE_SEPARATOR(state))))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::TextElement(state))))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TildeRest(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::TildeRest, |state| state.sequence(|state| self::Tilde(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::Tilde(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::Tilde(state))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Template(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| state.sequence(|state| state.optional(|state| self::WHITE_SPACE(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::WHITE_SPACE(state))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::HTMLSelfClose(state).or_else(|state| state.restore_on_err(|state| self::HTMLOpenClose(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn HTMLOpenClose(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::HTMLOpenClose, |state| state.sequence(|state| self::HTMLOpen(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::TextElement(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| self::TextElement(state))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::HTMLClose(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn HTMLOpen(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::HTMLOpen, |state| state.sequence(|state| state.match_string("<").and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::PATTERN_WHITE_SPACE(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::PATTERN_WHITE_SPACE(state)))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.stack_push(|state| self::SYMBOL(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.sequence(|state| state.sequence(|state| state.optional(|state| self::PATTERN_WHITE_SPACE(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::PATTERN_WHITE_SPACE(state))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::key_value(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::PATTERN_WHITE_SPACE(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::PATTERN_WHITE_SPACE(state))))))))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| state.sequence(|state| state.optional(|state| self::PATTERN_WHITE_SPACE(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::PATTERN_WHITE_SPACE(state))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::key_value(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::PATTERN_WHITE_SPACE(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::PATTERN_WHITE_SPACE(state)))))))))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string(">"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn HTMLClose(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::HTMLClose, |state| state.sequence(|state| state.match_string("</").and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::PATTERN_WHITE_SPACE(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::PATTERN_WHITE_SPACE(state)))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::PEEK(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::PATTERN_WHITE_SPACE(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::PATTERN_WHITE_SPACE(state)))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string(">"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn HTMLSelfClose(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::HTMLSelfClose, |state| state.sequence(|state| state.match_string("<").and_then(|state| state.repeat(|state| self::PATTERN_WHITE_SPACE(state))).and_then(|state| self::SYMBOL(state)).and_then(|state| state.repeat(|state| state.sequence(|state| state.repeat(|state| self::PATTERN_WHITE_SPACE(state)).and_then(|state| self::key_value(state)).and_then(|state| state.repeat(|state| self::PATTERN_WHITE_SPACE(state)))))).and_then(|state| state.match_string("/>")))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Command(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| state.sequence(|state| state.optional(|state| self::WHITE_SPACE(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::WHITE_SPACE(state))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::CommandLine(state).or_else(|state| self::CommandBlock(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn CommandLine(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::CommandLine, |state| state.sequence(|state| self::Escape(state).and_then(|state| self::SYMBOL(state)).and_then(|state| state.repeat(|state| self::WHITE_SPACE(state))).and_then(|state| self::Colon(state)).and_then(|state| self::RestOfLine(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn CommandBlock(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::CommandBlock, |state| state.sequence(|state| self::Escape(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::call(state).or_else(|state| state.sequence(|state| self::SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::WHITE_SPACE(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::WHITE_SPACE(state)))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.sequence(|state| state.match_string("[").and_then(|state| super::hidden::skip(state)).and_then(|state| self::argument_literal(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("]"))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| state.match_string("[").and_then(|state| super::hidden::skip(state)).and_then(|state| self::argument_literal(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("]"))))))))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn call(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| self::SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::WHITE_SPACE(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::WHITE_SPACE(state)))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::arguments(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn arguments(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| state.match_string("(").and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.sequence(|state| state.optional(|state| self::PATTERN_WHITE_SPACE(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::PATTERN_WHITE_SPACE(state))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string(")"))).or_else(|state| state.sequence(|state| state.sequence(|state| state.sequence(|state| state.sequence(|state| state.optional(|state| self::PATTERN_WHITE_SPACE(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::PATTERN_WHITE_SPACE(state))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::key_value(state).or_else(|state| self::argument(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::PATTERN_WHITE_SPACE(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::PATTERN_WHITE_SPACE(state))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.sequence(|state| state.sequence(|state| state.optional(|state| self::PATTERN_WHITE_SPACE(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::PATTERN_WHITE_SPACE(state))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::key_value(state).or_else(|state| self::argument(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::PATTERN_WHITE_SPACE(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::PATTERN_WHITE_SPACE(state))))))))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| state.sequence(|state| state.optional(|state| self::PATTERN_WHITE_SPACE(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::PATTERN_WHITE_SPACE(state))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::key_value(state).or_else(|state| self::argument(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::PATTERN_WHITE_SPACE(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::PATTERN_WHITE_SPACE(state))))))))))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string(")"))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn argument_literal(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::argument_literal, |state| state.sequence(|state| state.optional(|state| state.sequence(|state| state.lookahead(false, |state| state.match_string("]")).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| state.lookahead(false, |state| state.match_string("]")).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn argument(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::argument, |state| state.sequence(|state| self::value(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::PATTERN_WHITE_SPACE(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::PATTERN_WHITE_SPACE(state)))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::Comma(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn key_value(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::key_value, |state| state.sequence(|state| self::key(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::WHITE_SPACE(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::WHITE_SPACE(state)))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Set(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::WHITE_SPACE(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::WHITE_SPACE(state)))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::value(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::PATTERN_WHITE_SPACE(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::PATTERN_WHITE_SPACE(state)))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::Comma(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn key(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::key, |state| self::Integer(state).or_else(|state| self::String(state)).or_else(|state| self::SYMBOL(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn value(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::value, |state| self::Integer(state).or_else(|state| self::String(state)).or_else(|state| self::Keywords(state)).or_else(|state| self::call(state)).or_else(|state| self::SYMBOL(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Number(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::Number, |state| state.sequence(|state| state.optional(|state| self::Sign(state)).and_then(|state| self::Decimal(state).or_else(|state| self::DecimalBad(state)).or_else(|state| self::Integer(state))))))
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
                    state.rule(Rule::Integer, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("0").or_else(|state| state.sequence(|state| self::ASCII_NONZERO_DIGIT(state).and_then(|state| state.repeat(|state| self::ASCII_DIGIT(state)))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Sign(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Sign, |state| state.atomic(::pest::Atomicity::Atomic, |state| self::Plus(state).or_else(|state| self::Minus(state))))
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
                    state.rule(Rule::Escaped, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::Escape(state).and_then(|state| self::ANY(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Keywords(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Keywords, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("true").or_else(|state| state.match_string("false")).or_else(|state| state.match_string("null"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LINE_SEPARATOR(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::LINE_SEPARATOR, |state| state.atomic(::pest::Atomicity::Atomic, |state| self::NEWLINE(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn WHITE_SPACE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::WHITE_SPACE, |state| state.atomic(::pest::Atomicity::Atomic, |state| self::SPACE_SEPARATOR(state).or_else(|state| state.match_string("\t"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn PATTERN_WHITE_SPACE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::PATTERN_WHITE_SPACE, |state| state.atomic(::pest::Atomicity::Atomic, |state| self::NEWLINE(state).or_else(|state| self::WHITE_SPACE(state))))
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
                pub fn ANY(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.skip(1)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn EOI(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::EOI, |state| state.end_of_input())
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn SOI(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.start_of_input()
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn PEEK(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.stack_peek()
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn POP(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.stack_pop()
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_DIGIT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('0'..'9')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_NONZERO_DIGIT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('1'..'9')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_ALPHA(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('a'..'z').or_else(|state| state.match_range('A'..'Z'))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn NEWLINE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("\n").or_else(|state| state.match_string("\r\n")).or_else(|state| state.match_string("\r"))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn XID_CONTINUE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_char_by(::pest::unicode::XID_CONTINUE)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn XID_START(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_char_by(::pest::unicode::XID_START)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn SPACE_SEPARATOR(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_char_by(::pest::unicode::SPACE_SEPARATOR)
                }
            }
            pub use self::visible::*;
        }
        ::pest::state(input, |state| match rule {
            Rule::program => rules::program(state),
            Rule::statement => rules::statement(state),
            Rule::EmptyLine => rules::EmptyLine(state),
            Rule::BlockEnd => rules::BlockEnd(state),
            Rule::RestOfLine => rules::RestOfLine(state),
            Rule::Header => rules::Header(state),
            Rule::TextBlock => rules::TextBlock(state),
            Rule::TextElement => rules::TextElement(state),
            Rule::TextRest => rules::TextRest(state),
            Rule::URL => rules::URL(state),
            Rule::Code => rules::Code(state),
            Rule::CodeAction => rules::CodeAction(state),
            Rule::CodeLevel => rules::CodeLevel(state),
            Rule::CodeText => rules::CodeText(state),
            Rule::CodeMark => rules::CodeMark(state),
            Rule::Table => rules::Table(state),
            Rule::TableFirstLine => rules::TableFirstLine(state),
            Rule::TableRestLine => rules::TableRestLine(state),
            Rule::List => rules::List(state),
            Rule::ListFirstLine => rules::ListFirstLine(state),
            Rule::ListRestLine => rules::ListRestLine(state),
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
            Rule::TildeStatement => rules::TildeStatement(state),
            Rule::TildeLine => rules::TildeLine(state),
            Rule::TildeLevel => rules::TildeLevel(state),
            Rule::TildeText => rules::TildeText(state),
            Rule::TildeRest => rules::TildeRest(state),
            Rule::Template => rules::Template(state),
            Rule::HTMLOpenClose => rules::HTMLOpenClose(state),
            Rule::HTMLOpen => rules::HTMLOpen(state),
            Rule::HTMLClose => rules::HTMLClose(state),
            Rule::HTMLSelfClose => rules::HTMLSelfClose(state),
            Rule::Command => rules::Command(state),
            Rule::CommandLine => rules::CommandLine(state),
            Rule::CommandBlock => rules::CommandBlock(state),
            Rule::call => rules::call(state),
            Rule::arguments => rules::arguments(state),
            Rule::argument_literal => rules::argument_literal(state),
            Rule::argument => rules::argument(state),
            Rule::key_value => rules::key_value(state),
            Rule::key => rules::key(state),
            Rule::value => rules::value(state),
            Rule::Number => rules::Number(state),
            Rule::Decimal => rules::Decimal(state),
            Rule::DecimalBad => rules::DecimalBad(state),
            Rule::Integer => rules::Integer(state),
            Rule::Sign => rules::Sign(state),
            Rule::String => rules::String(state),
            Rule::S1 => rules::S1(state),
            Rule::S2 => rules::S2(state),
            Rule::SYMBOL => rules::SYMBOL(state),
            Rule::Escaped => rules::Escaped(state),
            Rule::Keywords => rules::Keywords(state),
            Rule::LINE_SEPARATOR => rules::LINE_SEPARATOR(state),
            Rule::WHITE_SPACE => rules::WHITE_SPACE(state),
            Rule::PATTERN_WHITE_SPACE => rules::PATTERN_WHITE_SPACE(state),
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
