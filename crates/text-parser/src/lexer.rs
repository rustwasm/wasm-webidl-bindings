use regex::{Regex, RegexSet};
use std::{cmp, fmt};

#[derive(Debug, Clone, PartialEq)]
pub enum Token<'input> {
    LeftParenthesis,
    RightParenthesis,
    ArrayBuffer,
    ByteString,
    DOMString,
    DataView,
    Float32Array,
    Float64Array,
    Int16Array,
    Int32Array,
    Int8Array,
    USVString,
    Uint16Array,
    Uint32Array,
    Uint8Array,
    Uint8ClampedArray,
    AllocCopy,
    AllocUtf8CStr,
    AllocUtf8Str,
    Any,
    Anyref,
    As,
    Bind,
    BindExport,
    BindImport,
    Boolean,
    Byte,
    Constructor,
    Copy,
    DefaultNewTarget,
    Dict,
    Double,
    Enum,
    EnumToI32,
    Export,
    F32,
    F64,
    Field,
    Float,
    Func,
    FuncBinding,
    Get,
    I32,
    I32ToEnum,
    I64,
    Import,
    LongLong,
    Long,
    Method,
    Object,
    Octet,
    Param,
    Result,
    Short,
    Symbol,
    Type,
    Union,
    UnrestrictedDouble,
    UnrestrictedFloat,
    UnsignedLongLong,
    UnsignedLong,
    UnsignedShort,
    Utf8CStr,
    Utf8Str,
    V128,
    View,
    Unsigned(&'input str),
    Identifier(&'input str),
    QuotedString(&'input str),
}

impl<'input> fmt::Display for Token<'input> {
    fn fmt<'formatter>(&self, f: &mut fmt::Formatter<'formatter>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct LexerBuilder {
    regex_set: RegexSet,
    regex_vec: Vec<Regex>,
    skip_regex: Regex,
}

impl LexerBuilder {
    pub fn new() -> Self {
        let lexemes: &[&str] = &[
            "^\\(",
            "^\\)",
            "^ArrayBuffer",
            "^ByteString",
            "^DOMString",
            "^DataView",
            "^Float32Array",
            "^Float64Array",
            "^Int16Array",
            "^Int32Array",
            "^Int8Array",
            "^USVString",
            "^Uint16Array",
            "^Uint32Array",
            "^Uint8Array",
            "^Uint8ClampedArray",
            "^alloc\\-copy",
            "^alloc\\-utf8\\-cstr",
            "^alloc\\-utf8\\-str",
            "^any",
            "^anyref",
            "^as",
            "^bind",
            "^bind\\-export",
            "^bind\\-import",
            "^boolean",
            "^byte",
            "^constructor",
            "^copy",
            "^default\\-new\\-target",
            "^dict",
            "^double",
            "^enum",
            "^enum\\-to\\-i32",
            "^export",
            "^f32",
            "^f64",
            "^field",
            "^float",
            "^func",
            "^func\\-binding",
            "^get",
            "^i32",
            "^i32\\-to\\-enum",
            "^i64",
            "^import",
            "^long long",
            "^long",
            "^method",
            "^object",
            "^octet",
            "^param",
            "^result",
            "^short",
            "^symbol",
            "^type",
            "^union",
            "^unrestricted double",
            "^unrestricted float",
            "^unsigned long long",
            "^unsigned long",
            "^unsigned short",
            "^utf8\\-cstr",
            "^utf8\\-str",
            "^v128",
            "^view",
            r"^([0-9]+)",
            r"^([a-zA-Z$][a-zA-Z0-9$_]*)",
            r#"^"(([^\\"]|\\.)*)""#,
        ];

        Self {
            regex_set: RegexSet::new(lexemes).unwrap(),
            regex_vec: lexemes
                .iter()
                .map(|lexeme| Regex::new(lexeme).unwrap())
                .collect(),
            skip_regex: Regex::new(r#"^(\p{White_Space}+|;[^;]+;)+"#).unwrap(),
        }
    }

    pub fn lexer<'input, 'builder>(&'builder self, input: &'input str) -> Lexer<'input, 'builder> {
        Lexer {
            input: input,
            consumed: 0,
            regex_set: &self.regex_set,
            regex_vec: &self.regex_vec,
            skip_regex: &self.skip_regex,
        }
    }
}

pub struct Lexer<'input, 'builder> {
    input: &'input str,
    consumed: usize,
    regex_set: &'builder RegexSet,
    regex_vec: &'builder Vec<Regex>,
    skip_regex: &'builder Regex,
}

impl<'input, 'builder> Iterator for Lexer<'input, 'builder> {
    type Item = Result<(usize, Token<'input>, usize), String>;

    fn next(&mut self) -> Option<Self::Item> {
        let (input, input_offset) = match self.skip_regex.find(self.input) {
            Some(match_) => {
                let offset = match_.end();

                (&self.input[offset..], offset)
            }

            None => (self.input, 0),
        };
        let start_offset = self.consumed + input_offset;

        if input.is_empty() {
            self.input = input;
            self.consumed = start_offset;

            None
        } else {
            let matches = self.regex_set.matches(input);

            if !matches.matched_any() {
                Some(Err(format!(
                    "Invalid token at {}: `{}…`",
                    start_offset,
                    &input[..cmp::min(input.len(), 10)]
                )))
            } else {
                let mut longest_match = 0;
                let mut token_by_regex = &self.regex_set.patterns()[0];

                for i in 0..self.regex_set.len() {
                    if matches.matched(i) {
                        let this_match = self.regex_vec[i].find(input).unwrap();
                        let length = this_match.end();

                        if length > longest_match {
                            longest_match = length;
                            token_by_regex = &self.regex_set.patterns()[i];
                        }
                    }
                }

                let result = &input[..longest_match];
                let remaining = &input[longest_match..];
                let end_offset = start_offset + longest_match;

                self.input = remaining;
                self.consumed = end_offset;

                Some(Ok((
                    start_offset,
                    match token_by_regex.as_ref() {
                        "^\\(" => Token::LeftParenthesis,
                        "^\\)" => Token::RightParenthesis,
                        "^ArrayBuffer" => Token::ArrayBuffer,
                        "^ByteString" => Token::ByteString,
                        "^DOMString" => Token::DOMString,
                        "^DataView" => Token::DataView,
                        "^Float32Array" => Token::Float32Array,
                        "^Float64Array" => Token::Float64Array,
                        "^Int16Array" => Token::Int16Array,
                        "^Int32Array" => Token::Int32Array,
                        "^Int8Array" => Token::Int8Array,
                        "^USVString" => Token::USVString,
                        "^Uint16Array" => Token::Uint16Array,
                        "^Uint32Array" => Token::Uint32Array,
                        "^Uint8Array" => Token::Uint8Array,
                        "^Uint8ClampedArray" => Token::Uint8ClampedArray,
                        "^alloc\\-copy" => Token::AllocCopy,
                        "^alloc\\-utf8\\-cstr" => Token::AllocUtf8CStr,
                        "^alloc\\-utf8\\-str" => Token::AllocUtf8Str,
                        "^any" => Token::Any,
                        "^anyref" => Token::Anyref,
                        "^as" => Token::As,
                        "^bind" => Token::Bind,
                        "^bind\\-export" => Token::BindExport,
                        "^bind\\-import" => Token::BindImport,
                        "^boolean" => Token::Boolean,
                        "^byte" => Token::Byte,
                        "^constructor" => Token::Constructor,
                        "^copy" => Token::Copy,
                        "^default\\-new\\-target" => Token::DefaultNewTarget,
                        "^dict" => Token::Dict,
                        "^double" => Token::Double,
                        "^enum" => Token::Enum,
                        "^enum\\-to\\-i32" => Token::EnumToI32,
                        "^export" => Token::Export,
                        "^f32" => Token::F32,
                        "^f64" => Token::F64,
                        "^field" => Token::Field,
                        "^float" => Token::Float,
                        "^func" => Token::Func,
                        "^func\\-binding" => Token::FuncBinding,
                        "^get" => Token::Get,
                        "^i32" => Token::I32,
                        "^i32\\-to\\-enum" => Token::I32ToEnum,
                        "^i64" => Token::I64,
                        "^import" => Token::Import,
                        "^long long" => Token::LongLong,
                        "^long" => Token::Long,
                        "^method" => Token::Method,
                        "^object" => Token::Object,
                        "^octet" => Token::Octet,
                        "^param" => Token::Param,
                        "^result" => Token::Result,
                        "^short" => Token::Short,
                        "^symbol" => Token::Symbol,
                        "^type" => Token::Type,
                        "^union" => Token::Union,
                        "^unrestricted double" => Token::UnrestrictedDouble,
                        "^unrestricted float" => Token::UnrestrictedFloat,
                        "^unsigned long long" => Token::UnsignedLongLong,
                        "^unsigned long" => Token::UnsignedLong,
                        "^unsigned short" => Token::UnsignedShort,
                        "^utf8\\-cstr" => Token::Utf8CStr,
                        "^utf8\\-str" => Token::Utf8Str,
                        "^v128" => Token::V128,
                        "^view" => Token::View,
                        r"^([0-9]+)" => Token::Unsigned(result),
                        r"^([a-zA-Z$][a-zA-Z0-9$_]*)" => Token::Identifier(result),
                        r#"^"(([^\\"]|\\.)*)""# => Token::QuotedString(result),
                        _ => panic!("oh no"),
                    },
                    end_offset,
                )))
            }
        }
    }
}
