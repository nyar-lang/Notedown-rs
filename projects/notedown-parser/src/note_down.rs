pub struct NoteDownParser;
#[allow(dead_code, non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Rule {
    EOI,
    program,
    statement,
    EmptyLine,
    Command,
    Header,
    TextBlock,
    TextHeaderCharacter,
    TextBlockLine,
    Code,
    CodeAction,
    CodeLevel,
    CodeText,
    CodeMark,
    Table,
    TableEnd,
    TableMark,
    List,
    ListEnd,
    program_list,
    ListStatement,
    ListHeadLevel,
    ListRest,
    ListMark,
    HorizontalRule,
    CommandPart,
    CommandContent,
    Begin,
    End,
    CommandBlock,
    arguments,
    argument,
    key_value,
    key,
    value,
    CommandLine,
    command,
    RestOfLine,
    Number,
    Decimal,
    DecimalBad,
    Exponent,
    Integer,
    Zero,
    String,
    StringSingle,
    StringBlock,
    LiteralString,
    LiteralBlock,
    S1,
    S2,
    S3,
    S6,
    Word,
    SYMBOL,
    COMMENT,
    LineComment,
    BlockComment,
    MultiLineComment,
    Comment,
    NEWLINE,
    WHITESPACE,
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
                    if state.atomicity() == ::pest::Atomicity::NonAtomic { state.sequence(|state| state.repeat(|state| super::visible::WHITESPACE(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::visible::COMMENT(state).and_then(|state| state.repeat(|state| super::visible::WHITESPACE(state))))))) } else { Ok(state) }
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
                    self::Header(state).or_else(|state| self::HorizontalRule(state)).or_else(|state| state.restore_on_err(|state| self::Code(state))).or_else(|state| self::Table(state)).or_else(|state| self::List(state)).or_else(|state| self::Command(state)).or_else(|state| state.sequence(|state| self::EmptyLine(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::EmptyLine(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::EmptyLine(state)))))))))).or_else(|state| self::TextBlock(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn EmptyLine(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| state.sequence(|state| state.optional(|state| self::WHITESPACE(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::WHITESPACE(state))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::NEWLINE(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Command(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::CommandLine(state).or_else(|state| self::CommandPart(state)).or_else(|state| self::CommandBlock(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Header(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::Header, |state| state.sequence(|state| state.sequence(|state| self::Sharp(state).and_then(|state| state.optional(|state| self::Sharp(state))).and_then(|state| state.optional(|state| self::Sharp(state))).and_then(|state| state.optional(|state| self::Sharp(state))).and_then(|state| state.optional(|state| self::Sharp(state))).and_then(|state| state.optional(|state| self::Sharp(state)))).and_then(|state| state.optional(|state| self::arguments(state))).and_then(|state| self::RestOfLine(state)))))
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
                pub fn Code(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Code, |state| state.sequence(|state| self::CodeLevel(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::CodeAction(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::Word(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::arguments(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::CodeText(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::POP(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::CodeMark(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::CodeMark(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::CodeMark(state))))
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
                    state.rule(Rule::Table, |state| state.sequence(|state| self::TableMark(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.sequence(|state| state.lookahead(false, |state| self::TableEnd(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| state.lookahead(false, |state| self::TableEnd(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state)))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::TableEnd(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TableEnd(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::TableEnd, |state| state.sequence(|state| self::EmptyLine(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::EmptyLine(state))).or_else(|state| state.sequence(|state| self::EmptyLine(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::EOI(state)))).or_else(|state| self::EOI(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TableMark(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::TableMark, |state| state.atomic(::pest::Atomicity::Atomic, |state| self::Vertical(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn List(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::List, |state| state.sequence(|state| self::ListMark(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.sequence(|state| state.lookahead(false, |state| self::ListEnd(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| state.lookahead(false, |state| self::ListEnd(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state)))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ListEnd(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ListEnd(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::ListEnd, |state| state.sequence(|state| self::EmptyLine(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::EmptyLine(state))).or_else(|state| state.sequence(|state| self::EmptyLine(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::EOI(state)))).or_else(|state| self::EOI(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn program_list(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| self::SOI(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| self::ListStatement(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::ListStatement(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::ListStatement(state)))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::EOI(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ListStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::ListStatement, |state| state.sequence(|state| self::NEWLINE(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ListHeadLevel(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ListMark(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ListRest(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ListHeadLevel(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::ListHeadLevel, |state| state.sequence(|state| state.optional(|state| self::WHITESPACE(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::WHITESPACE(state))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ListRest(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::ListRest, |state| state.sequence(|state| state.optional(|state| state.sequence(|state| state.lookahead(false, |state| self::ListStatement(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| state.lookahead(false, |state| self::ListStatement(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ListMark(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::ListMark, |state| state.atomic(::pest::Atomicity::Atomic, |state| self::Plus(state).or_else(|state| self::Minus(state)).or_else(|state| self::Vertical(state)).or_else(|state| self::QuoteMark(state)).or_else(|state| state.sequence(|state| self::Integer(state).and_then(|state| self::Dot(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn HorizontalRule(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::HorizontalRule, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::Set(state).and_then(|state| self::Set(state)).and_then(|state| self::Set(state)).and_then(|state| state.repeat(|state| self::Set(state)))).or_else(|state| state.sequence(|state| self::Minus(state).and_then(|state| self::Minus(state)).and_then(|state| self::Minus(state)).and_then(|state| state.repeat(|state| self::Minus(state)))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn CommandPart(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::CommandPart, |state| state.sequence(|state| self::Begin(state).and_then(|state| self::arguments(state)).and_then(|state| self::CommandContent(state)).and_then(|state| self::End(state)).and_then(|state| state.optional(|state| self::arguments(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn CommandContent(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::CommandContent, |state| state.repeat(|state| self::CommandPart(state).or_else(|state| state.sequence(|state| state.lookahead(false, |state| self::End(state)).and_then(|state| self::ANY(state)))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Begin(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Begin, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::Escape(state).and_then(|state| state.match_string("begin")))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn End(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::End, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::Escape(state).and_then(|state| state.match_string("end")))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn CommandBlock(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::CommandBlock, |state| state.sequence(|state| self::command(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::arguments(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::arguments(state)))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn arguments(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::arguments, |state| state.sequence(|state| state.match_string("{").and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::key_value(state).or_else(|state| self::argument(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::key_value(state).or_else(|state| self::argument(state))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("}"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn argument(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::argument, |state| state.sequence(|state| self::value(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::Comma(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn key_value(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::key_value, |state| state.sequence(|state| self::key(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Set(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::value(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::Comma(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn key(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::key, |state| self::Word(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn value(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::value, |state| self::Word(state).or_else(|state| self::Integer(state)).or_else(|state| self::String(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn CommandLine(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::CommandLine, |state| state.sequence(|state| self::command(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::arguments(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Colon(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::RestOfLine(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn command(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::command, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::Escape(state).and_then(|state| self::Word(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn RestOfLine(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::RestOfLine, |state| state.sequence(|state| state.optional(|state| state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state)))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Number(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::Number, |state| self::Exponent(state).or_else(|state| self::Decimal(state)).or_else(|state| self::DecimalBad(state)).or_else(|state| self::Integer(state))))
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
                pub fn Exponent(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::Exponent, |state| state.sequence(|state| self::Decimal(state).or_else(|state| self::Integer(state)).and_then(|state| state.match_string("^^")).and_then(|state| self::Decimal(state).or_else(|state| self::Integer(state))))))
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
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::String, |state| self::StringSingle(state).or_else(|state| self::StringBlock(state)).or_else(|state| self::LiteralString(state)).or_else(|state| self::LiteralBlock(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StringSingle(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::StringSingle, |state| state.sequence(|state| self::S2(state).and_then(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| self::S2(state)).and_then(|state| self::ANY(state))))).and_then(|state| self::S2(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StringBlock(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::StringBlock, |state| state.sequence(|state| self::S6(state).and_then(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| self::S6(state)).and_then(|state| self::ANY(state))))).and_then(|state| self::S6(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LiteralString(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::LiteralString, |state| state.sequence(|state| self::S1(state).and_then(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| self::S1(state)).and_then(|state| self::ANY(state))))).and_then(|state| self::S1(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LiteralBlock(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::LiteralBlock, |state| state.sequence(|state| self::S3(state).and_then(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| self::S3(state)).and_then(|state| self::ANY(state))))).and_then(|state| self::S3(state)))))
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
                pub fn S3(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("'''")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn S6(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("\"\"\"")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Word(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Word, |state| state.sequence(|state| self::ASCII_ALPHA(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::ASCII_ALPHA(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::ASCII_ALPHA(state))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SYMBOL(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::SYMBOL, |state| state.atomic(::pest::Atomicity::Atomic, |state| self::ASCII_ALPHA(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn COMMENT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::COMMENT, |state| self::BlockComment(state).or_else(|state| self::LineComment(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LineComment(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::LineComment, |state| state.sequence(|state| self::Comment(state).and_then(|state| state.repeat(|state| self::WHITESPACE(state))).and_then(|state| self::Colon(state)).and_then(|state| self::RestOfLine(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn BlockComment(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::BlockComment, |state| state.sequence(|state| self::Comment(state).and_then(|state| state.repeat(|state| self::WHITESPACE(state))).and_then(|state| state.match_string("{")).and_then(|state| self::MultiLineComment(state)).and_then(|state| state.match_string("}")))))
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
                pub fn WHITESPACE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::Atomic, |state| self::SPACE_SEPARATOR(state).or_else(|state| self::TAB(state)))
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
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_DIGIT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('0'..'9')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_ALPHA(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('a'..'z').or_else(|state| state.match_range('A'..'Z'))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn SOI(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.start_of_input()
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_NONZERO_DIGIT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('1'..'9')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn POP(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.stack_pop()
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn PEEK(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.stack_peek()
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn EOI(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::EOI, |state| state.end_of_input())
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ANY(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.skip(1)
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
            Rule::Command => rules::Command(state),
            Rule::Header => rules::Header(state),
            Rule::TextBlock => rules::TextBlock(state),
            Rule::TextHeaderCharacter => rules::TextHeaderCharacter(state),
            Rule::TextBlockLine => rules::TextBlockLine(state),
            Rule::Code => rules::Code(state),
            Rule::CodeAction => rules::CodeAction(state),
            Rule::CodeLevel => rules::CodeLevel(state),
            Rule::CodeText => rules::CodeText(state),
            Rule::CodeMark => rules::CodeMark(state),
            Rule::Table => rules::Table(state),
            Rule::TableEnd => rules::TableEnd(state),
            Rule::TableMark => rules::TableMark(state),
            Rule::List => rules::List(state),
            Rule::ListEnd => rules::ListEnd(state),
            Rule::program_list => rules::program_list(state),
            Rule::ListStatement => rules::ListStatement(state),
            Rule::ListHeadLevel => rules::ListHeadLevel(state),
            Rule::ListRest => rules::ListRest(state),
            Rule::ListMark => rules::ListMark(state),
            Rule::HorizontalRule => rules::HorizontalRule(state),
            Rule::CommandPart => rules::CommandPart(state),
            Rule::CommandContent => rules::CommandContent(state),
            Rule::Begin => rules::Begin(state),
            Rule::End => rules::End(state),
            Rule::CommandBlock => rules::CommandBlock(state),
            Rule::arguments => rules::arguments(state),
            Rule::argument => rules::argument(state),
            Rule::key_value => rules::key_value(state),
            Rule::key => rules::key(state),
            Rule::value => rules::value(state),
            Rule::CommandLine => rules::CommandLine(state),
            Rule::command => rules::command(state),
            Rule::RestOfLine => rules::RestOfLine(state),
            Rule::Number => rules::Number(state),
            Rule::Decimal => rules::Decimal(state),
            Rule::DecimalBad => rules::DecimalBad(state),
            Rule::Exponent => rules::Exponent(state),
            Rule::Integer => rules::Integer(state),
            Rule::Zero => rules::Zero(state),
            Rule::String => rules::String(state),
            Rule::StringSingle => rules::StringSingle(state),
            Rule::StringBlock => rules::StringBlock(state),
            Rule::LiteralString => rules::LiteralString(state),
            Rule::LiteralBlock => rules::LiteralBlock(state),
            Rule::S1 => rules::S1(state),
            Rule::S2 => rules::S2(state),
            Rule::S3 => rules::S3(state),
            Rule::S6 => rules::S6(state),
            Rule::Word => rules::Word(state),
            Rule::SYMBOL => rules::SYMBOL(state),
            Rule::COMMENT => rules::COMMENT(state),
            Rule::LineComment => rules::LineComment(state),
            Rule::BlockComment => rules::BlockComment(state),
            Rule::MultiLineComment => rules::MultiLineComment(state),
            Rule::Comment => rules::Comment(state),
            Rule::NEWLINE => rules::NEWLINE(state),
            Rule::WHITESPACE => rules::WHITESPACE(state),
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
            Rule::EOI => rules::EOI(state),
        })
    }
}
