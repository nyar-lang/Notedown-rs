#[allow(dead_code)]
#[grammar = "note_down.pest"]
pub struct Parser;
#[allow(dead_code, non_camel_case_types)]
#[structural_match]
#[rustc_copy_clone_marker]
pub enum Rule {
    EOI,
    program,
    statement,
    EmptyLines,
    EmptyLine,
    Command,
    Header,
    Sharp,
    TextBlock,
    TextHeaderCharacter,
    TextBlockLine,
    Text,
    CommandPart,
    CommandContent,
    Begin,
    End,
    Comma,
    CommandBlock,
    arguments,
    argument,
    key_value,
    key,
    value,
    Set,
    LB,
    RB,
    CommandLine,
    command,
    ROL,
    Colon,
    Escape,
    Number,
    Decimal,
    DecimalBad,
    Exponent,
    Complex,
    Integer,
    ComplexHandler,
    Dot,
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
    SYMBOL,
    NameCharacter,
    NameStartCharacter,
    Underline,
    NEWLINE,
    WHITESPACE,
    TAB,
    CR,
    LF,
    COMMENT,
    LineComment,
    BlockComment,
    MultiLineComment,
    Comment,
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(dead_code, non_camel_case_types)]
impl ::std::clone::Clone for Rule {
    #[inline]
    fn clone(&self) -> Rule {
        {
            *self
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(dead_code, non_camel_case_types)]
impl ::std::marker::Copy for Rule {}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(dead_code, non_camel_case_types)]
impl ::std::fmt::Debug for Rule {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self,) {
            (&Rule::EOI,) => {
                let mut debug_trait_builder = f.debug_tuple("EOI");
                debug_trait_builder.finish()
            }
            (&Rule::program,) => {
                let mut debug_trait_builder = f.debug_tuple("program");
                debug_trait_builder.finish()
            }
            (&Rule::statement,) => {
                let mut debug_trait_builder = f.debug_tuple("statement");
                debug_trait_builder.finish()
            }
            (&Rule::EmptyLines,) => {
                let mut debug_trait_builder = f.debug_tuple("EmptyLines");
                debug_trait_builder.finish()
            }
            (&Rule::EmptyLine,) => {
                let mut debug_trait_builder = f.debug_tuple("EmptyLine");
                debug_trait_builder.finish()
            }
            (&Rule::Command,) => {
                let mut debug_trait_builder = f.debug_tuple("Command");
                debug_trait_builder.finish()
            }
            (&Rule::Header,) => {
                let mut debug_trait_builder = f.debug_tuple("Header");
                debug_trait_builder.finish()
            }
            (&Rule::Sharp,) => {
                let mut debug_trait_builder = f.debug_tuple("Sharp");
                debug_trait_builder.finish()
            }
            (&Rule::TextBlock,) => {
                let mut debug_trait_builder = f.debug_tuple("TextBlock");
                debug_trait_builder.finish()
            }
            (&Rule::TextHeaderCharacter,) => {
                let mut debug_trait_builder = f.debug_tuple("TextHeaderCharacter");
                debug_trait_builder.finish()
            }
            (&Rule::TextBlockLine,) => {
                let mut debug_trait_builder = f.debug_tuple("TextBlockLine");
                debug_trait_builder.finish()
            }
            (&Rule::Text,) => {
                let mut debug_trait_builder = f.debug_tuple("Text");
                debug_trait_builder.finish()
            }
            (&Rule::CommandPart,) => {
                let mut debug_trait_builder = f.debug_tuple("CommandPart");
                debug_trait_builder.finish()
            }
            (&Rule::CommandContent,) => {
                let mut debug_trait_builder = f.debug_tuple("CommandContent");
                debug_trait_builder.finish()
            }
            (&Rule::Begin,) => {
                let mut debug_trait_builder = f.debug_tuple("Begin");
                debug_trait_builder.finish()
            }
            (&Rule::End,) => {
                let mut debug_trait_builder = f.debug_tuple("End");
                debug_trait_builder.finish()
            }
            (&Rule::Comma,) => {
                let mut debug_trait_builder = f.debug_tuple("Comma");
                debug_trait_builder.finish()
            }
            (&Rule::CommandBlock,) => {
                let mut debug_trait_builder = f.debug_tuple("CommandBlock");
                debug_trait_builder.finish()
            }
            (&Rule::arguments,) => {
                let mut debug_trait_builder = f.debug_tuple("arguments");
                debug_trait_builder.finish()
            }
            (&Rule::argument,) => {
                let mut debug_trait_builder = f.debug_tuple("argument");
                debug_trait_builder.finish()
            }
            (&Rule::key_value,) => {
                let mut debug_trait_builder = f.debug_tuple("key_value");
                debug_trait_builder.finish()
            }
            (&Rule::key,) => {
                let mut debug_trait_builder = f.debug_tuple("key");
                debug_trait_builder.finish()
            }
            (&Rule::value,) => {
                let mut debug_trait_builder = f.debug_tuple("value");
                debug_trait_builder.finish()
            }
            (&Rule::Set,) => {
                let mut debug_trait_builder = f.debug_tuple("Set");
                debug_trait_builder.finish()
            }
            (&Rule::LB,) => {
                let mut debug_trait_builder = f.debug_tuple("LB");
                debug_trait_builder.finish()
            }
            (&Rule::RB,) => {
                let mut debug_trait_builder = f.debug_tuple("RB");
                debug_trait_builder.finish()
            }
            (&Rule::CommandLine,) => {
                let mut debug_trait_builder = f.debug_tuple("CommandLine");
                debug_trait_builder.finish()
            }
            (&Rule::command,) => {
                let mut debug_trait_builder = f.debug_tuple("command");
                debug_trait_builder.finish()
            }
            (&Rule::ROL,) => {
                let mut debug_trait_builder = f.debug_tuple("ROL");
                debug_trait_builder.finish()
            }
            (&Rule::Colon,) => {
                let mut debug_trait_builder = f.debug_tuple("Colon");
                debug_trait_builder.finish()
            }
            (&Rule::Escape,) => {
                let mut debug_trait_builder = f.debug_tuple("Escape");
                debug_trait_builder.finish()
            }
            (&Rule::Number,) => {
                let mut debug_trait_builder = f.debug_tuple("Number");
                debug_trait_builder.finish()
            }
            (&Rule::Decimal,) => {
                let mut debug_trait_builder = f.debug_tuple("Decimal");
                debug_trait_builder.finish()
            }
            (&Rule::DecimalBad,) => {
                let mut debug_trait_builder = f.debug_tuple("DecimalBad");
                debug_trait_builder.finish()
            }
            (&Rule::Exponent,) => {
                let mut debug_trait_builder = f.debug_tuple("Exponent");
                debug_trait_builder.finish()
            }
            (&Rule::Complex,) => {
                let mut debug_trait_builder = f.debug_tuple("Complex");
                debug_trait_builder.finish()
            }
            (&Rule::Integer,) => {
                let mut debug_trait_builder = f.debug_tuple("Integer");
                debug_trait_builder.finish()
            }
            (&Rule::ComplexHandler,) => {
                let mut debug_trait_builder = f.debug_tuple("ComplexHandler");
                debug_trait_builder.finish()
            }
            (&Rule::Dot,) => {
                let mut debug_trait_builder = f.debug_tuple("Dot");
                debug_trait_builder.finish()
            }
            (&Rule::Zero,) => {
                let mut debug_trait_builder = f.debug_tuple("Zero");
                debug_trait_builder.finish()
            }
            (&Rule::String,) => {
                let mut debug_trait_builder = f.debug_tuple("String");
                debug_trait_builder.finish()
            }
            (&Rule::StringSingle,) => {
                let mut debug_trait_builder = f.debug_tuple("StringSingle");
                debug_trait_builder.finish()
            }
            (&Rule::StringBlock,) => {
                let mut debug_trait_builder = f.debug_tuple("StringBlock");
                debug_trait_builder.finish()
            }
            (&Rule::LiteralString,) => {
                let mut debug_trait_builder = f.debug_tuple("LiteralString");
                debug_trait_builder.finish()
            }
            (&Rule::LiteralBlock,) => {
                let mut debug_trait_builder = f.debug_tuple("LiteralBlock");
                debug_trait_builder.finish()
            }
            (&Rule::S1,) => {
                let mut debug_trait_builder = f.debug_tuple("S1");
                debug_trait_builder.finish()
            }
            (&Rule::S2,) => {
                let mut debug_trait_builder = f.debug_tuple("S2");
                debug_trait_builder.finish()
            }
            (&Rule::S3,) => {
                let mut debug_trait_builder = f.debug_tuple("S3");
                debug_trait_builder.finish()
            }
            (&Rule::S6,) => {
                let mut debug_trait_builder = f.debug_tuple("S6");
                debug_trait_builder.finish()
            }
            (&Rule::SYMBOL,) => {
                let mut debug_trait_builder = f.debug_tuple("SYMBOL");
                debug_trait_builder.finish()
            }
            (&Rule::NameCharacter,) => {
                let mut debug_trait_builder = f.debug_tuple("NameCharacter");
                debug_trait_builder.finish()
            }
            (&Rule::NameStartCharacter,) => {
                let mut debug_trait_builder = f.debug_tuple("NameStartCharacter");
                debug_trait_builder.finish()
            }
            (&Rule::Underline,) => {
                let mut debug_trait_builder = f.debug_tuple("Underline");
                debug_trait_builder.finish()
            }
            (&Rule::NEWLINE,) => {
                let mut debug_trait_builder = f.debug_tuple("NEWLINE");
                debug_trait_builder.finish()
            }
            (&Rule::WHITESPACE,) => {
                let mut debug_trait_builder = f.debug_tuple("WHITESPACE");
                debug_trait_builder.finish()
            }
            (&Rule::TAB,) => {
                let mut debug_trait_builder = f.debug_tuple("TAB");
                debug_trait_builder.finish()
            }
            (&Rule::CR,) => {
                let mut debug_trait_builder = f.debug_tuple("CR");
                debug_trait_builder.finish()
            }
            (&Rule::LF,) => {
                let mut debug_trait_builder = f.debug_tuple("LF");
                debug_trait_builder.finish()
            }
            (&Rule::COMMENT,) => {
                let mut debug_trait_builder = f.debug_tuple("COMMENT");
                debug_trait_builder.finish()
            }
            (&Rule::LineComment,) => {
                let mut debug_trait_builder = f.debug_tuple("LineComment");
                debug_trait_builder.finish()
            }
            (&Rule::BlockComment,) => {
                let mut debug_trait_builder = f.debug_tuple("BlockComment");
                debug_trait_builder.finish()
            }
            (&Rule::MultiLineComment,) => {
                let mut debug_trait_builder = f.debug_tuple("MultiLineComment");
                debug_trait_builder.finish()
            }
            (&Rule::Comment,) => {
                let mut debug_trait_builder = f.debug_tuple("Comment");
                debug_trait_builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(dead_code, non_camel_case_types)]
impl ::std::cmp::Eq for Rule {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {}
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(dead_code, non_camel_case_types)]
impl ::std::hash::Hash for Rule {
    fn hash<__H: ::std::hash::Hasher>(&self, state: &mut __H) -> () {
        match (&*self,) {
            _ => ::std::hash::Hash::hash(
                &unsafe { ::std::intrinsics::discriminant_value(self) },
                state,
            ),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(dead_code, non_camel_case_types)]
impl ::std::cmp::Ord for Rule {
    #[inline]
    fn cmp(&self, other: &Rule) -> ::std::cmp::Ordering {
        {
            let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
            let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    _ => ::std::cmp::Ordering::Equal,
                }
            } else {
                __self_vi.cmp(&__arg_1_vi)
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(dead_code, non_camel_case_types)]
impl ::std::cmp::PartialEq for Rule {
    #[inline]
    fn eq(&self, other: &Rule) -> bool {
        {
            let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
            let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    _ => true,
                }
            } else {
                false
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(dead_code, non_camel_case_types)]
impl ::std::cmp::PartialOrd for Rule {
    #[inline]
    fn partial_cmp(&self, other: &Rule) -> ::std::option::Option<::std::cmp::Ordering> {
        {
            let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
            let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    _ => ::std::option::Option::Some(::std::cmp::Ordering::Equal),
                }
            } else {
                __self_vi.partial_cmp(&__arg_1_vi)
            }
        }
    }
}
#[allow(clippy::all)]
impl ::pest::Parser<Rule> for Parser {
    fn parse<'i>(
        rule: Rule,
        input: &'i str,
    ) -> ::std::result::Result<::pest::iterators::Pairs<'i, Rule>, ::pest::error::Error<Rule>> {
        mod rules {
            pub mod hidden {
                use super::super::Rule;
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn skip(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    if state.atomicity() == ::pest::Atomicity::NonAtomic {
                        state.sequence(|state| {
                            state
                                .repeat(|state| super::visible::WHITESPACE(state))
                                .and_then(|state| {
                                    state.repeat(|state| {
                                        state.sequence(|state| {
                                            super::visible::COMMENT(state).and_then(|state| {
                                                state.repeat(|state| {
                                                    super::visible::WHITESPACE(state)
                                                })
                                            })
                                        })
                                    })
                                })
                        })
                    } else {
                        Ok(state)
                    }
                }
            }
            pub mod visible {
                use super::super::Rule;
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn program(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| {
                        self::SOI(state)
                            .and_then(|state| super::hidden::skip(state))
                            .and_then(|state| {
                                state.sequence(|state| {
                                    state.optional(|state| {
                                        self::EmptyLine(state).and_then(|state| {
                                            state.repeat(|state| {
                                                state.sequence(|state| {
                                                    super::hidden::skip(state)
                                                        .and_then(|state| self::EmptyLine(state))
                                                })
                                            })
                                        })
                                    })
                                })
                            })
                            .and_then(|state| super::hidden::skip(state))
                            .and_then(|state| {
                                state.sequence(|state| {
                                    state.optional(|state| {
                                        self::statement(state).and_then(|state| {
                                            state.repeat(|state| {
                                                state.sequence(|state| {
                                                    super::hidden::skip(state)
                                                        .and_then(|state| self::statement(state))
                                                })
                                            })
                                        })
                                    })
                                })
                            })
                            .and_then(|state| super::hidden::skip(state))
                            .and_then(|state| self::EOI(state))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn statement(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::EmptyLines(state)
                        .or_else(|state| {
                            state.sequence(|state| {
                                self::Header(state)
                                    .and_then(|state| super::hidden::skip(state))
                                    .and_then(|state| self::NEWLINE(state))
                            })
                        })
                        .or_else(|state| self::TextBlock(state))
                        .or_else(|state| self::Command(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn EmptyLines(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| {
                        self::EmptyLine(state)
                            .and_then(|state| super::hidden::skip(state))
                            .and_then(|state| self::EmptyLine(state))
                            .and_then(|state| super::hidden::skip(state))
                            .and_then(|state| {
                                state.sequence(|state| {
                                    state.optional(|state| {
                                        self::EmptyLine(state).and_then(|state| {
                                            state.repeat(|state| {
                                                state.sequence(|state| {
                                                    super::hidden::skip(state)
                                                        .and_then(|state| self::EmptyLine(state))
                                                })
                                            })
                                        })
                                    })
                                })
                            })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn EmptyLine(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| {
                        state
                            .sequence(|state| {
                                state.optional(|state| {
                                    self::WHITESPACE(state).and_then(|state| {
                                        state.repeat(|state| {
                                            state.sequence(|state| {
                                                super::hidden::skip(state)
                                                    .and_then(|state| self::WHITESPACE(state))
                                            })
                                        })
                                    })
                                })
                            })
                            .and_then(|state| super::hidden::skip(state))
                            .and_then(|state| self::NEWLINE(state))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Command(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::CommandLine(state)
                        .or_else(|state| self::CommandPart(state))
                        .or_else(|state| self::CommandBlock(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Header(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| {
                        state.rule(Rule::Header, |state| {
                            state.sequence(|state| {
                                state
                                    .sequence(|state| {
                                        self::Sharp(state)
                                            .and_then(|state| {
                                                state.optional(|state| self::Sharp(state))
                                            })
                                            .and_then(|state| {
                                                state.optional(|state| self::Sharp(state))
                                            })
                                            .and_then(|state| {
                                                state.optional(|state| self::Sharp(state))
                                            })
                                            .and_then(|state| {
                                                state.optional(|state| self::Sharp(state))
                                            })
                                            .and_then(|state| {
                                                state.optional(|state| self::Sharp(state))
                                            })
                                    })
                                    .and_then(|state| state.repeat(|state| self::WHITESPACE(state)))
                                    .and_then(|state| self::ROL(state))
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Sharp(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Sharp, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("#"))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TextBlock(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::TextBlock, |state| {
                        state.sequence(|state| {
                            self::TextHeaderCharacter(state)
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| {
                                    state.sequence(|state| {
                                        state.optional(|state| {
                                            self::TextBlockLine(state).and_then(|state| {
                                                state.repeat(|state| {
                                                    state.sequence(|state| {
                                                        super::hidden::skip(state).and_then(
                                                            |state| self::TextBlockLine(state),
                                                        )
                                                    })
                                                })
                                            })
                                        })
                                    })
                                })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TextHeaderCharacter(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| {
                        state
                            .lookahead(false, |state| {
                                self::Escape(state)
                                    .or_else(|state| self::NEWLINE(state))
                                    .or_else(|state| self::Sharp(state))
                            })
                            .and_then(|state| super::hidden::skip(state))
                            .and_then(|state| self::Text(state))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TextBlockLine(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| {
                        state
                            .lookahead(false, |state| {
                                self::EmptyLines(state).or_else(|state| self::Sharp(state))
                            })
                            .and_then(|state| super::hidden::skip(state))
                            .and_then(|state| self::Text(state))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Text(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Text, |state| self::ANY(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn CommandPart(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| {
                        state.rule(Rule::CommandPart, |state| {
                            state.sequence(|state| {
                                self::Begin(state)
                                    .and_then(|state| self::arguments(state))
                                    .and_then(|state| self::CommandContent(state))
                                    .and_then(|state| self::End(state))
                                    .and_then(|state| {
                                        state.optional(|state| self::arguments(state))
                                    })
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn CommandContent(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| {
                        state.rule(Rule::CommandContent, |state| {
                            state.repeat(|state| {
                                self::CommandPart(state).or_else(|state| {
                                    state.sequence(|state| {
                                        state
                                            .lookahead(false, |state| self::End(state))
                                            .and_then(|state| self::ANY(state))
                                    })
                                })
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Begin(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Begin, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| {
                            state.sequence(|state| {
                                self::Escape(state).and_then(|state| state.match_string("begin"))
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn End(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::End, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| {
                            state.sequence(|state| {
                                self::Escape(state).and_then(|state| state.match_string("end"))
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Comma(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Comma, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(","))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn CommandBlock(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| {
                        state.rule(Rule::CommandBlock, |state| {
                            state.sequence(|state| {
                                self::command(state)
                                    .and_then(|state| super::hidden::skip(state))
                                    .and_then(|state| {
                                        state.sequence(|state| {
                                            state.optional(|state| {
                                                self::arguments(state).and_then(|state| {
                                                    state.repeat(|state| {
                                                        state.sequence(|state| {
                                                            super::hidden::skip(state).and_then(
                                                                |state| self::arguments(state),
                                                            )
                                                        })
                                                    })
                                                })
                                            })
                                        })
                                    })
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn arguments(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::arguments, |state| {
                        state.sequence(|state| {
                            self::LB(state)
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| {
                                    state.sequence(|state| {
                                        state.optional(|state| {
                                            self::key_value(state)
                                                .or_else(|state| self::argument(state))
                                                .and_then(|state| {
                                                    state.repeat(|state| {
                                                        state.sequence(|state| {
                                                            super::hidden::skip(state).and_then(
                                                                |state| {
                                                                    self::key_value(state).or_else(
                                                                        |state| {
                                                                            self::argument(state)
                                                                        },
                                                                    )
                                                                },
                                                            )
                                                        })
                                                    })
                                                })
                                        })
                                    })
                                })
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::RB(state))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn argument(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::argument, |state| {
                        state.sequence(|state| {
                            self::value(state)
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.optional(|state| self::Comma(state)))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn key_value(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::key_value, |state| {
                        state.sequence(|state| {
                            self::key(state)
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::Set(state))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::value(state))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.optional(|state| self::Comma(state)))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn key(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::key, |state| self::SYMBOL(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn value(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::value, |state| {
                        self::SYMBOL(state)
                            .or_else(|state| self::Integer(state))
                            .or_else(|state| self::String(state))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Set(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Set, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("="))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LB(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::LB, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("{"))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn RB(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::RB, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("}"))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn CommandLine(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::CommandLine, |state| {
                        state.sequence(|state| {
                            self::command(state)
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::Colon(state))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::ROL(state))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn command(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::command, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| {
                            state.sequence(|state| {
                                self::Escape(state)
                                    .and_then(|state| state.repeat(|state| self::SYMBOL(state)))
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ROL(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::ROL, |state| {
                        state.sequence(|state| {
                            state.optional(|state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .lookahead(false, |state| self::NEWLINE(state))
                                            .and_then(|state| super::hidden::skip(state))
                                            .and_then(|state| self::ANY(state))
                                    })
                                    .and_then(|state| {
                                        state.repeat(|state| {
                                            state.sequence(|state| {
                                                super::hidden::skip(state).and_then(|state| {
                                                    state.sequence(|state| {
                                                        state
                                                            .lookahead(false, |state| {
                                                                self::NEWLINE(state)
                                                            })
                                                            .and_then(|state| {
                                                                super::hidden::skip(state)
                                                            })
                                                            .and_then(|state| self::ANY(state))
                                                    })
                                                })
                                            })
                                        })
                                    })
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Colon(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Colon, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(":"))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Escape(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Escape, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("\\"))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Number(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| {
                        state.rule(Rule::Number, |state| {
                            self::Exponent(state)
                                .or_else(|state| self::Complex(state))
                                .or_else(|state| self::Decimal(state))
                                .or_else(|state| self::DecimalBad(state))
                                .or_else(|state| self::Integer(state))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Decimal(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| {
                        state.rule(Rule::Decimal, |state| {
                            state.sequence(|state| {
                                self::Integer(state)
                                    .and_then(|state| self::Dot(state))
                                    .and_then(|state| self::ASCII_DIGIT(state))
                                    .and_then(|state| {
                                        state.repeat(|state| self::ASCII_DIGIT(state))
                                    })
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn DecimalBad(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| {
                        state.rule(Rule::DecimalBad, |state| {
                            state
                                .sequence(|state| {
                                    self::Integer(state).and_then(|state| self::Dot(state))
                                })
                                .or_else(|state| {
                                    state.sequence(|state| {
                                        self::Dot(state)
                                            .and_then(|state| self::ASCII_DIGIT(state))
                                            .and_then(|state| {
                                                state.repeat(|state| self::ASCII_DIGIT(state))
                                            })
                                    })
                                })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Exponent(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| {
                        state.rule(Rule::Exponent, |state| {
                            state.sequence(|state| {
                                self::Decimal(state)
                                    .or_else(|state| self::Integer(state))
                                    .and_then(|state| state.match_string("^^"))
                                    .and_then(|state| {
                                        self::Decimal(state).or_else(|state| self::Integer(state))
                                    })
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Complex(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| {
                        state.rule(Rule::Complex, |state| {
                            state.sequence(|state| {
                                self::Decimal(state)
                                    .or_else(|state| self::Integer(state))
                                    .and_then(|state| self::ComplexHandler(state))
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Integer(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Integer, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| {
                            self::Zero(state).or_else(|state| {
                                state.sequence(|state| {
                                    self::ASCII_DIGIT(state).and_then(|state| {
                                        state.repeat(|state| {
                                            state.sequence(|state| {
                                                state
                                                    .optional(|state| self::Underline(state))
                                                    .and_then(|state| {
                                                        self::ASCII_NONZERO_DIGIT(state)
                                                    })
                                            })
                                        })
                                    })
                                })
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ComplexHandler(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::ComplexHandler, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| self::SYMBOL(state))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Dot(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Dot, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("."))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Zero(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Zero, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("0"))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn String(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| {
                        state.rule(Rule::String, |state| {
                            self::StringSingle(state)
                                .or_else(|state| self::StringBlock(state))
                                .or_else(|state| self::LiteralString(state))
                                .or_else(|state| self::LiteralBlock(state))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StringSingle(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| {
                        state.rule(Rule::StringSingle, |state| {
                            state.sequence(|state| {
                                self::S2(state)
                                    .and_then(|state| {
                                        state.repeat(|state| {
                                            state.sequence(|state| {
                                                state
                                                    .lookahead(false, |state| self::S2(state))
                                                    .and_then(|state| self::ANY(state))
                                            })
                                        })
                                    })
                                    .and_then(|state| self::S2(state))
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StringBlock(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| {
                        state.rule(Rule::StringBlock, |state| {
                            state.sequence(|state| {
                                self::S6(state)
                                    .and_then(|state| {
                                        state.repeat(|state| {
                                            state.sequence(|state| {
                                                state
                                                    .lookahead(false, |state| self::S6(state))
                                                    .and_then(|state| self::ANY(state))
                                            })
                                        })
                                    })
                                    .and_then(|state| self::S6(state))
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LiteralString(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| {
                        state.rule(Rule::LiteralString, |state| {
                            state.sequence(|state| {
                                self::S1(state)
                                    .and_then(|state| {
                                        state.repeat(|state| {
                                            state.sequence(|state| {
                                                state
                                                    .lookahead(false, |state| self::S1(state))
                                                    .and_then(|state| self::ANY(state))
                                            })
                                        })
                                    })
                                    .and_then(|state| self::S1(state))
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LiteralBlock(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| {
                        state.rule(Rule::LiteralBlock, |state| {
                            state.sequence(|state| {
                                self::S3(state)
                                    .and_then(|state| {
                                        state.repeat(|state| {
                                            state.sequence(|state| {
                                                state
                                                    .lookahead(false, |state| self::S3(state))
                                                    .and_then(|state| self::ANY(state))
                                            })
                                        })
                                    })
                                    .and_then(|state| self::S3(state))
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn S1(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("\'")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn S2(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("\"")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn S3(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("\'\'\'")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn S6(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("\"\"\"")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SYMBOL(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::SYMBOL, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| {
                            state.sequence(|state| {
                                self::NameStartCharacter(state).and_then(|state| {
                                    state.repeat(|state| self::NameCharacter(state))
                                })
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn NameCharacter(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::ASCII_DIGIT(state).or_else(|state| self::NameStartCharacter(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn NameStartCharacter(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::Underline(state).or_else(|state| self::ASCII_ALPHA(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Underline(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Underline, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("_"))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn NEWLINE(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::NEWLINE, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| {
                            state
                                .sequence(|state| self::CR(state).and_then(|state| self::LF(state)))
                                .or_else(|state| self::CR(state))
                                .or_else(|state| self::LF(state))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn WHITESPACE(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::Atomic, |state| {
                        self::SPACE_SEPARATOR(state).or_else(|state| self::TAB(state))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TAB(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::TAB, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("\t"))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn CR(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::CR, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("\r"))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LF(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::LF, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("\n"))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn COMMENT(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| {
                        state.rule(Rule::COMMENT, |state| {
                            self::BlockComment(state).or_else(|state| self::LineComment(state))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LineComment(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| {
                        state.rule(Rule::LineComment, |state| {
                            state.sequence(|state| {
                                self::Comment(state)
                                    .and_then(|state| state.repeat(|state| self::WHITESPACE(state)))
                                    .and_then(|state| self::Colon(state))
                                    .and_then(|state| self::ROL(state))
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn BlockComment(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| {
                        state.rule(Rule::BlockComment, |state| {
                            state.sequence(|state| {
                                self::Comment(state)
                                    .and_then(|state| state.repeat(|state| self::WHITESPACE(state)))
                                    .and_then(|state| self::LB(state))
                                    .and_then(|state| self::MultiLineComment(state))
                                    .and_then(|state| self::RB(state))
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn MultiLineComment(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| {
                        state.rule(Rule::MultiLineComment, |state| {
                            state.repeat(|state| {
                                self::BlockComment(state).or_else(|state| {
                                    state.sequence(|state| {
                                        state
                                            .lookahead(false, |state| self::RB(state))
                                            .and_then(|state| self::ANY(state))
                                    })
                                })
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Comment(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Comment, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| {
                            state.sequence(|state| {
                                self::Escape(state).and_then(|state| state.match_string("comment"))
                            })
                        })
                    })
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_NONZERO_DIGIT(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('1'..'9')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ANY(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.skip(1)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_DIGIT(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('0'..'9')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn SOI(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.start_of_input()
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn SPACE_SEPARATOR(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_char_by(::pest::unicode::SPACE_SEPARATOR)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_ALPHA(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state
                        .match_range('a'..'z')
                        .or_else(|state| state.match_range('A'..'Z'))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn EOI(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::EOI, |state| state.end_of_input())
                }
            }
            pub use self::visible::*;
        }
        ::pest::state(input, |state| match rule {
            Rule::program => rules::program(state),
            Rule::statement => rules::statement(state),
            Rule::EmptyLines => rules::EmptyLines(state),
            Rule::EmptyLine => rules::EmptyLine(state),
            Rule::Command => rules::Command(state),
            Rule::Header => rules::Header(state),
            Rule::Sharp => rules::Sharp(state),
            Rule::TextBlock => rules::TextBlock(state),
            Rule::TextHeaderCharacter => rules::TextHeaderCharacter(state),
            Rule::TextBlockLine => rules::TextBlockLine(state),
            Rule::Text => rules::Text(state),
            Rule::CommandPart => rules::CommandPart(state),
            Rule::CommandContent => rules::CommandContent(state),
            Rule::Begin => rules::Begin(state),
            Rule::End => rules::End(state),
            Rule::Comma => rules::Comma(state),
            Rule::CommandBlock => rules::CommandBlock(state),
            Rule::arguments => rules::arguments(state),
            Rule::argument => rules::argument(state),
            Rule::key_value => rules::key_value(state),
            Rule::key => rules::key(state),
            Rule::value => rules::value(state),
            Rule::Set => rules::Set(state),
            Rule::LB => rules::LB(state),
            Rule::RB => rules::RB(state),
            Rule::CommandLine => rules::CommandLine(state),
            Rule::command => rules::command(state),
            Rule::ROL => rules::ROL(state),
            Rule::Colon => rules::Colon(state),
            Rule::Escape => rules::Escape(state),
            Rule::Number => rules::Number(state),
            Rule::Decimal => rules::Decimal(state),
            Rule::DecimalBad => rules::DecimalBad(state),
            Rule::Exponent => rules::Exponent(state),
            Rule::Complex => rules::Complex(state),
            Rule::Integer => rules::Integer(state),
            Rule::ComplexHandler => rules::ComplexHandler(state),
            Rule::Dot => rules::Dot(state),
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
            Rule::SYMBOL => rules::SYMBOL(state),
            Rule::NameCharacter => rules::NameCharacter(state),
            Rule::NameStartCharacter => rules::NameStartCharacter(state),
            Rule::Underline => rules::Underline(state),
            Rule::NEWLINE => rules::NEWLINE(state),
            Rule::WHITESPACE => rules::WHITESPACE(state),
            Rule::TAB => rules::TAB(state),
            Rule::CR => rules::CR(state),
            Rule::LF => rules::LF(state),
            Rule::COMMENT => rules::COMMENT(state),
            Rule::LineComment => rules::LineComment(state),
            Rule::BlockComment => rules::BlockComment(state),
            Rule::MultiLineComment => rules::MultiLineComment(state),
            Rule::Comment => rules::Comment(state),
            Rule::EOI => rules::EOI(state),
        })
    }
}
