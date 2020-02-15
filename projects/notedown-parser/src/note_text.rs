pub struct NoteTextParser;
#[allow(dead_code, non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Rule {
    EOI,
    TextMode,
    TextStatement,
    MathStatement,
    Math,
    MathLevel,
    MathText,
    MathRest,
    Dollar,
    CodeStatement,
    Code,
    CodeLevel,
    CodeText,
    CodeRest,
    Accent,
    StyleStatement,
    Style,
    StyleLevel,
    StyleText,
    StyleRest,
    Asterisk,
    LineStatement,
    Line,
    LineLevel,
    LineText,
    LineRest,
    Tilde,
    Command,
    Function,
    arguments,
    String,
    StringSingle,
    LiteralString,
    S1,
    S2,
    Word,
    English,
    Escaped,
    PUNCTUATION,
    SYMBOL,
    Escape,
    NonCharacter,
    SPACE_SEPARATOR,
    NEWLINE,
    CR,
    LF,
}
#[allow(clippy::all)]
impl ::pest::Parser<Rule> for NoteTextParser {
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
                pub fn TextMode(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| self::SOI(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::TextStatement(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| self::TextStatement(state))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::EOI(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TextStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.restore_on_err(|state| self::StyleStatement(state)).or_else(|state| state.restore_on_err(|state| self::LineStatement(state))).or_else(|state| state.restore_on_err(|state| self::MathStatement(state))).or_else(|state| state.restore_on_err(|state| self::CodeStatement(state))).or_else(|state| self::SPACE_SEPARATOR(state)).or_else(|state| self::NEWLINE(state)).or_else(|state| self::Escaped(state)).or_else(|state| self::Command(state)).or_else(|state| self::English(state)).or_else(|state| self::Word(state))
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
                pub fn Dollar(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("$")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn CodeStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.restore_on_err(|state| self::Code(state)).or_else(|state| self::CodeRest(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Code(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Code, |state| state.sequence(|state| self::CodeLevel(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::CodeText(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::POP(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Accent(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn CodeLevel(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::CodeLevel, |state| state.sequence(|state| self::Accent(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.stack_push(|state| state.sequence(|state| state.optional(|state| self::Accent(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::Accent(state)))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn CodeText(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::CodeText, |state| state.sequence(|state| state.optional(|state| state.sequence(|state| self::Escape(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Accent(state))).or_else(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::Accent(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::PEEK(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| self::Escape(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Accent(state))).or_else(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::Accent(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::PEEK(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state)))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn CodeRest(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::CodeRest, |state| state.sequence(|state| self::Accent(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::Accent(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::Accent(state))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Accent(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("`")
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
                pub fn Asterisk(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("*")
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
                pub fn Tilde(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("~")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Command(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Command, |state| state.sequence(|state| self::Function(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::arguments(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::arguments(state))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Function(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Function, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::Escape(state).and_then(|state| self::ASCII_ALPHA(state)).and_then(|state| state.repeat(|state| self::ASCII_ALPHA(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn arguments(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| state.match_string("{").and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::String(state).or_else(|state| state.sequence(|state| state.lookahead(false, |state| state.match_string("}")).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::String(state).or_else(|state| state.sequence(|state| state.lookahead(false, |state| state.match_string("}")).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state))))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("}")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn String(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::String, |state| self::StringSingle(state).or_else(|state| self::LiteralString(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StringSingle(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::StringSingle, |state| state.sequence(|state| self::S2(state).and_then(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| self::S2(state)).and_then(|state| self::ANY(state))))).and_then(|state| self::S2(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LiteralString(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::LiteralString, |state| state.sequence(|state| self::S1(state).and_then(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| self::S1(state)).and_then(|state| self::ANY(state))))).and_then(|state| self::S1(state)))))
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
                pub fn Word(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Word, |state| state.sequence(|state| state.sequence(|state| state.lookahead(false, |state| self::Escape(state).or_else(|state| self::English(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.sequence(|state| state.lookahead(false, |state| self::Escape(state).or_else(|state| self::English(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| state.lookahead(false, |state| self::Escape(state).or_else(|state| self::English(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state))))))))))).or_else(|state| self::Escape(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn English(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::English, |state| state.sequence(|state| self::ASCII_ALPHA(state).or_else(|state| self::ASCII_DIGIT(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::ASCII_ALPHA(state).or_else(|state| self::ASCII_DIGIT(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::ASCII_ALPHA(state).or_else(|state| self::ASCII_DIGIT(state)))))))))).or_else(|state| self::PUNCTUATION(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Escaped(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Escaped, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::Escape(state).and_then(|state| self::Escape(state).or_else(|state| self::Tilde(state)).or_else(|state| self::Asterisk(state)).or_else(|state| self::Dollar(state)).or_else(|state| self::Accent(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn PUNCTUATION(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::PUNCTUATION, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(",").or_else(|state| state.match_string(".")).or_else(|state| state.match_string("?")).or_else(|state| state.match_string("!"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SYMBOL(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| state.lookahead(false, |state| self::NonCharacter(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Escape(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("\\")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn NonCharacter(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::Asterisk(state).or_else(|state| self::Tilde(state)).or_else(|state| self::Dollar(state)).or_else(|state| self::Accent(state)).or_else(|state| self::SPACE_SEPARATOR(state)).or_else(|state| self::NEWLINE(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SPACE_SEPARATOR(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::SPACE_SEPARATOR, |state| state.match_string(" ").or_else(|state| state.match_string("\t")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn NEWLINE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::NEWLINE, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::CR(state).and_then(|state| self::LF(state))).or_else(|state| self::CR(state)).or_else(|state| self::LF(state))))
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
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn SOI(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.start_of_input()
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn EOI(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::EOI, |state| state.end_of_input())
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn POP(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.stack_pop()
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ANY(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.skip(1)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_DIGIT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('0'..'9')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn PEEK(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.stack_peek()
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_ALPHA(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('a'..'z').or_else(|state| state.match_range('A'..'Z'))
                }
            }
            pub use self::visible::*;
        }
        ::pest::state(input, |state| match rule {
            Rule::TextMode => rules::TextMode(state),
            Rule::TextStatement => rules::TextStatement(state),
            Rule::MathStatement => rules::MathStatement(state),
            Rule::Math => rules::Math(state),
            Rule::MathLevel => rules::MathLevel(state),
            Rule::MathText => rules::MathText(state),
            Rule::MathRest => rules::MathRest(state),
            Rule::Dollar => rules::Dollar(state),
            Rule::CodeStatement => rules::CodeStatement(state),
            Rule::Code => rules::Code(state),
            Rule::CodeLevel => rules::CodeLevel(state),
            Rule::CodeText => rules::CodeText(state),
            Rule::CodeRest => rules::CodeRest(state),
            Rule::Accent => rules::Accent(state),
            Rule::StyleStatement => rules::StyleStatement(state),
            Rule::Style => rules::Style(state),
            Rule::StyleLevel => rules::StyleLevel(state),
            Rule::StyleText => rules::StyleText(state),
            Rule::StyleRest => rules::StyleRest(state),
            Rule::Asterisk => rules::Asterisk(state),
            Rule::LineStatement => rules::LineStatement(state),
            Rule::Line => rules::Line(state),
            Rule::LineLevel => rules::LineLevel(state),
            Rule::LineText => rules::LineText(state),
            Rule::LineRest => rules::LineRest(state),
            Rule::Tilde => rules::Tilde(state),
            Rule::Command => rules::Command(state),
            Rule::Function => rules::Function(state),
            Rule::arguments => rules::arguments(state),
            Rule::String => rules::String(state),
            Rule::StringSingle => rules::StringSingle(state),
            Rule::LiteralString => rules::LiteralString(state),
            Rule::S1 => rules::S1(state),
            Rule::S2 => rules::S2(state),
            Rule::Word => rules::Word(state),
            Rule::English => rules::English(state),
            Rule::Escaped => rules::Escaped(state),
            Rule::PUNCTUATION => rules::PUNCTUATION(state),
            Rule::SYMBOL => rules::SYMBOL(state),
            Rule::Escape => rules::Escape(state),
            Rule::NonCharacter => rules::NonCharacter(state),
            Rule::SPACE_SEPARATOR => rules::SPACE_SEPARATOR(state),
            Rule::NEWLINE => rules::NEWLINE(state),
            Rule::CR => rules::CR(state),
            Rule::LF => rules::LF(state),
            Rule::EOI => rules::EOI(state),
        })
    }
}
