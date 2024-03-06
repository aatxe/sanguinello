use nom::branch::*;
use nom::bytes::complete::{tag, take};
use nom::character::complete::{alpha1, alphanumeric1, digit1, multispace0};
use nom::combinator::{map, map_res, recognize};
use nom::multi::many0;
use nom::sequence::{delimited, pair};
use nom::*;

use std::str;
use std::str::FromStr;
use std::str::Utf8Error;

pub mod token;
use crate::lexer::token::*;

macro_rules! syntax {
    ($func_name: ident, $tag_string: literal, $output_token: expr) => {
        fn $func_name<'a>(s: &'a [u8]) -> IResult<&[u8], Token> {
            map(tag($tag_string), |_| $output_token)(s)
        }
    };
}

syntax! {equal_operator, "==", Token::Equal}
syntax! {not_equal_operator, "!=", Token::NotEqual}
syntax! {plus_operator, "+", Token::Plus}
syntax! {minus_operator, "-", Token::Minus}
syntax! {multiply_operator, "*", Token::Multiply}
syntax! {divide_operator, "/", Token::Divide}
syntax! {greater_operator_equal, ">=", Token::GreaterThanEqual}
syntax! {lesser_operator_equal, "<=", Token::LessThanEqual}
syntax! {greater_operator, ">", Token::GreaterThan}
syntax! {lesser_operator, "<", Token::LessThan}
syntax! {not_operator, "not", Token::ReservedNot}
syntax! {or_operator, "or", Token::ReservedOr}
syntax! {and_operator, "and", Token::ReservedAnd}

pub fn lex_operator(input: &[u8]) -> IResult<&[u8], Token> {
    alt((
        equal_operator,
        not_equal_operator,
        plus_operator,
        minus_operator,
        multiply_operator,
        divide_operator,
        not_operator,
        greater_operator_equal,
        lesser_operator_equal,
        greater_operator,
        lesser_operator,
        not_operator,
        or_operator,
        and_operator,
    ))(input)
}

// punctuations
syntax! {comment_punctuation, "--", Token::Comment}
syntax! {skinny_arrow_punctuation, "->", Token::SkinnyArrow}
syntax! {thick_arrow_punctuation, "=>", Token::ThickArrow}
syntax! {dot_punctuation, ".", Token::Dot}
syntax! {double_dot_punctuation, "..", Token::DoubleDot}
syntax! {triple_dot_punctuation, "...", Token::TripleDot}
syntax! {comma_punctuation, ",", Token::Comma}
syntax! {semicolon_punctuation, ";", Token::SemiColon}
syntax! {colon_punctuation, ":", Token::Colon}
syntax! {double_colon_punctuation, "::", Token::DoubleColon}
syntax! {lparen_punctuation, "(", Token::LParen}
syntax! {rparen_punctuation, ")", Token::RParen}
syntax! {lbrace_punctuation, "{", Token::LBrace}
syntax! {rbrace_punctuation, "}", Token::RBrace}
syntax! {lbracket_punctuation, "[", Token::LBracket}
syntax! {rbracket_punctuation, "]", Token::RBracket}

pub fn lex_punctuations(input: &[u8]) -> IResult<&[u8], Token> {
    alt((
        comment_punctuation,
        skinny_arrow_punctuation,
        thick_arrow_punctuation,
        dot_punctuation,
        double_dot_punctuation,
        triple_dot_punctuation,
        comma_punctuation,
        semicolon_punctuation,
        colon_punctuation,
        double_colon_punctuation,
        lparen_punctuation,
        rparen_punctuation,
        lbrace_punctuation,
        rbrace_punctuation,
        lbracket_punctuation,
        rbracket_punctuation,
    ))(input)
}

fn complete_byte_slice_str_from_utf8(c: &[u8]) -> Result<&str, Utf8Error> {
    str::from_utf8(c)
}

// Reserved or ident
fn lex_reserved_ident(input: &[u8]) -> IResult<&[u8], Token> {
    map_res(
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        )),
        |s| {
            let c = complete_byte_slice_str_from_utf8(s);
            c.map(|syntax| match syntax {
                // boolean literals
                "true" => Token::BooleanLiteral(true),
                "false" => Token::BooleanLiteral(false),

                // reserved words
                "and" => Token::ReservedAnd,
                "break" => Token::ReservedBreak,
                "continue" => Token::ReservedContinue,
                "do" => Token::ReservedDo,
                "else" => Token::ReservedElse,
                "elseif" => Token::ReservedElseif,
                "end" => Token::ReservedEnd,
                "for" => Token::ReservedFor,
                "function" => Token::ReservedFunction,
                "fn" => Token::ReservedFn,
                "if" => Token::ReservedIf,
                "in" => Token::ReservedIn,
                "let" => Token::ReservedLet,
                "nil" => Token::ReservedNil,
                "not" => Token::ReservedNot,
                "or" => Token::ReservedOr,
                "repeat" => Token::ReservedRepeat,
                "return" => Token::ReservedReturn,
                "then" => Token::ReservedThen,
                "until" => Token::ReservedUntil,
                "while" => Token::ReservedWhile,

                // identifiers
                _ => Token::Identifier(syntax.to_string()),
            })
        },
    )(input)
}

fn complete_str_from_str<F: FromStr>(c: &str) -> Result<F, F::Err> {
    FromStr::from_str(c)
}

// Integers parsing
fn lex_integer(input: &[u8]) -> IResult<&[u8], Token> {
    map(
        map_res(
            map_res(digit1, complete_byte_slice_str_from_utf8),
            complete_str_from_str,
        ),
        Token::NumberLiteral,
    )(input)
}

// Illegal tokens
fn lex_illegal(input: &[u8]) -> IResult<&[u8], Token> {
    map(take(1usize), |_| Token::Illegal)(input)
}

fn lex_token(input: &[u8]) -> IResult<&[u8], Token> {
    alt((
        lex_operator,
        lex_punctuations,
        lex_reserved_ident,
        lex_integer,
        lex_illegal,
    ))(input)
}

fn lex_tokens(input: &[u8]) -> IResult<&[u8], Vec<Token>> {
    many0(delimited(multispace0, lex_token, multispace0))(input)
}

pub struct Lexer;

impl Lexer {
    pub fn lex_tokens(bytes: &[u8]) -> IResult<&[u8], Vec<Token>> {
        lex_tokens(bytes)
            .map(|(slice, result)| (slice, [&result[..], &vec![Token::EOF][..]].concat()))
    }
}
