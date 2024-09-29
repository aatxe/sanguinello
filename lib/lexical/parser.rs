use std::ops::RangeFrom;

use bumpalo::Bump;
use nom::branch::alt;
use nom::bytes::complete::take;
use nom::combinator::peek;
use nom::error::{Error, ErrorKind};
use nom::sequence::{delimited, tuple};
use nom::*;

use super::tokens::{Token, Tokens};
use crate::sg::ast::{Expression, Identifier, Literal, Operator, Property};

pub fn token<'a, I>(t: Token<'a>) -> impl Fn(I) -> IResult<I, Token<'a>>
where
    I: nom::Slice<RangeFrom<usize>> + nom::InputIter<Item = &'a Token<'a>>,
{
    move |input: I| match input.iter_elements().next().map(|tt| {
        let b = *tt == t;
        (&t, b)
    }) {
        Some((t, true)) => Ok((input.slice(1..), t.clone())),

        _ => Err(Err::Error(Error {
            input,
            code: ErrorKind::Char,
        })),
    }
}

pub struct Parser {
    alloc: Bump,
}

impl Parser {
    pub fn literal<'tokens>(&self, input: Tokens<'tokens>) -> IResult<Tokens<'tokens>, Literal> {
        let (input, ret) = take(1usize)(input)?;
        match ret[0] {
            Token::ReservedNil => Ok((input, Literal::Nil)),
            Token::NumberLiteral(value) => Ok((input, Literal::Number(value))),
            Token::BooleanLiteral(value) => Ok((input, Literal::Boolean(value))),

            _ => Err(Err::Error(Error {
                input,
                code: ErrorKind::Tag,
            })),
        }
    }

    pub fn infix_operator<'tokens>(
        &self,
        input: Tokens<'tokens>,
    ) -> IResult<Tokens<'tokens>, Operator> {
        let (input, ret) = take(1usize)(input)?;
        match ret[0] {
            Token::Plus => Ok((input, Operator::Plus)),
            Token::Minus => Ok((input, Operator::Minus)),
            Token::Divide => Ok((input, Operator::Divide)),
            Token::Multiply => Ok((input, Operator::Multiply)),
            Token::FloorDivide => Ok((input, Operator::FloorDivide)),
            Token::DoubleEqual => Ok((input, Operator::Equal)),
            Token::NotEqual => Ok((input, Operator::NotEqual)),
            Token::GreaterThanEqual => Ok((input, Operator::GreaterThanEqual)),
            Token::LessThanEqual => Ok((input, Operator::LessThanEqual)),
            Token::GreaterThan => Ok((input, Operator::GreaterThan)),
            Token::LessThan => Ok((input, Operator::LessThan)),
            Token::ReservedAnd => Ok((input, Operator::And)),
            Token::ReservedOr => Ok((input, Operator::Or)),

            _ => Err(Err::Error(Error {
                input,
                code: ErrorKind::Tag,
            })),
        }
    }

    pub fn property<'tokens, 'parser>(
        &'parser self,
        input: Tokens<'tokens>,
    ) -> IResult<Tokens<'tokens>, Property<'parser>>
    where
        'tokens: 'parser,
    {
        if let Ok(_) = peek(token(Token::LBracket))(input) {
            let (input, (key_expr, _, value_expr)) = tuple((
                |i| self.brackets(i),
                token(Token::Equal),
                |i| self.expression(i),
            ))(input)?;

            return Ok((
                input,
                Property {
                    key: Some(self.alloc.alloc(key_expr)),
                    value: Some(self.alloc.alloc(value_expr)),
                },
            ));
        }

        let (input, (key_expr, _, value_expr)) = tuple((
            |i| self.identifier(i),
            token(Token::Equal),
            |i| self.expression(i),
        ))(input)?;

        Ok((
            input,
            Property {
                key: Some(self.alloc.alloc(key_expr)),
                value: Some(self.alloc.alloc(value_expr)),
            },
        ))
    }

    pub fn identifier<'tokens, 'parser>(
        &'parser self,
        input: Tokens<'tokens>,
    ) -> IResult<Tokens<'tokens>, Expression<'parser>>
    where
        'tokens: 'parser,
    {
        let (input, ret) = take(1usize)(input)?;
        match ret[0] {
            Token::Identifier(s) => Ok((input, Expression::Identifier(Identifier(s)))),

            _ => Err(Err::Error(Error {
                input,
                code: ErrorKind::Tag,
            })),
        }
    }

    pub fn parens<'tokens, 'parser>(
        &'parser self,
        input: Tokens<'tokens>,
    ) -> IResult<Tokens<'tokens>, Expression<'parser>>
    where
        'tokens: 'parser,
    {
        let (input, _) = peek(token(Token::LParen))(input)?;
        delimited(
            token(Token::LParen),
            |i| self.expression(i),
            token(Token::RParen),
        )(input)
    }

    pub fn brackets<'tokens, 'parser>(
        &'parser self,
        input: Tokens<'tokens>,
    ) -> IResult<Tokens<'tokens>, Expression<'parser>>
    where
        'tokens: 'parser,
    {
        let (input, _) = peek(token(Token::LBracket))(input)?;
        delimited(
            token(Token::LBracket),
            |i| self.expression(i),
            token(Token::RBracket),
        )(input)
    }

    pub fn braces<'tokens, 'parser>(
        &'parser self,
        input: Tokens<'tokens>,
    ) -> IResult<Tokens<'tokens>, Expression<'parser>>
    where
        'tokens: 'parser,
    {
        let (input, _) = peek(token(Token::LBrace))(input)?;
        delimited(
            token(Token::LBrace),
            |i| self.expression(i),
            token(Token::RBrace),
        )(input)
    }

    pub fn expression<'tokens, 'parser>(
        &'parser self,
        input: Tokens<'tokens>,
    ) -> IResult<Tokens<'tokens>, Expression<'parser>>
    where
        'tokens: 'parser,
    {
        alt((|i| self.parens(i), |i| self.identifier(i)))(input)
    }
}
