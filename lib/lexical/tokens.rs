use nom::{InputIter, InputLength, InputTake, Needed, Slice};
use std::iter::Enumerate;
use std::ops::{Index, Range, RangeFrom, RangeFull, RangeTo};
use std::slice::Iter;

#[derive(Clone, Debug, PartialEq)]
pub enum Token<'a> {
    Illegal,
    EOF,

    Identifier(&'a str),

    // literals
    NumberLiteral(f64),
    BooleanLiteral(bool),

    // reserved words
    /// `and`
    ReservedAnd,
    /// `break`
    ReservedBreak,
    /// `continue`
    ReservedContinue,
    /// `do`
    ReservedDo,
    /// `else`
    ReservedElse,
    /// `elseif`
    ReservedElseif,
    /// `end`
    ReservedEnd,
    /// `export`
    ReservedExport,
    /// `for`
    ReservedFor,
    /// `function`
    ReservedFunction,
    /// `fn`
    ReservedFn,
    /// `if`
    ReservedIf,
    /// `import`
    ReservedImport,
    /// `in`
    ReservedIn,
    /// `local`
    ReservedLocal,
    /// `match`
    ReservedMatch,
    /// `module`
    ReservedModule,
    /// `nil`
    ReservedNil,
    /// `not`
    ReservedNot,
    /// `or`
    ReservedOr,
    /// `repeat`
    ReservedRepeat,
    /// `return`
    ReservedReturn,
    /// `then`
    ReservedThen,
    /// `type`
    ReservedType,
    /// `until`
    ReservedUntil,
    /// `while`
    ReservedWhile,

    // operators
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `/`
    Divide,
    /// `*`
    Multiply,
    /// `//`
    FloorDivide,
    /// `==`
    DoubleEqual,
    /// `~=`
    NotEqual,
    /// `>=`
    GreaterThanEqual,
    /// `<=`
    LessThanEqual,
    /// `>`
    GreaterThan,
    /// `<`
    LessThan,

    // punctuation/symbols
    /// `--`
    Comment,
    /// `->`
    SkinnyArrow,
    /// `=>`
    ThickArrow,
    /// `.`
    Dot,
    /// `..`
    DoubleDot,
    /// `...`
    TripleDot,
    /// `,`
    Comma,
    /// `:`
    Colon,
    /// `::`
    DoubleColon, // maybe?
    /// `;`
    SemiColon,
    /// `(`
    LParen,
    /// `)`
    RParen,
    /// `{`
    LBrace,
    /// `}`
    RBrace,
    /// `[`
    LBracket,
    /// `]`
    RBracket,
    /// `@`
    AtSign,
    /// `=`
    Equal,
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(C)]
pub struct Tokens<'a> {
    pub tokens: &'a [Token<'a>],
    pub start: usize,
    pub end: usize,
}

impl<'a> Tokens<'a> {
    pub fn new(vec: &'a [Token]) -> Self {
        Tokens {
            tokens: vec,
            start: 0,
            end: vec.len(),
        }
    }
}

impl<'a> Index<usize> for Tokens<'a> {
    type Output = Token<'a>;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.tokens[idx]
    }
}

impl<'a> InputLength for Token<'a> {
    fn input_len(&self) -> usize {
        1
    }
}

impl<'a> InputLength for Tokens<'a> {
    fn input_len(&self) -> usize {
        self.tokens.len()
    }
}

impl<'a> InputTake for Tokens<'a> {
    fn take(&self, count: usize) -> Self {
        Tokens {
            tokens: &self.tokens[0..count],
            start: 0,
            end: count,
        }
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        let (prefix, suffix) = self.tokens.split_at(count);
        let prefix = Tokens {
            tokens: prefix,
            start: 0,
            end: prefix.len(),
        };
        let suffix = Tokens {
            tokens: suffix,
            start: 0,
            end: suffix.len(),
        };
        (suffix, prefix)
    }
}

impl<'a> InputIter for Tokens<'a> {
    type Item = &'a Token<'a>;
    type Iter = Enumerate<Iter<'a, Token<'a>>>;
    type IterElem = Iter<'a, Token<'a>>;

    fn iter_indices(&self) -> Self::Iter {
        self.tokens.iter().enumerate()
    }

    fn iter_elements(&self) -> Self::IterElem {
        self.tokens.iter()
    }

    fn position<P>(&self, pred: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.tokens.iter().position(pred)
    }

    fn slice_index(&self, count: usize) -> Result<usize, Needed> {
        if self.tokens.len() >= count {
            Ok(count)
        } else {
            Err(Needed::new(self.tokens.len()))
        }
    }
}

impl<'a> Slice<Range<usize>> for Tokens<'a> {
    fn slice(&self, range: Range<usize>) -> Self {
        let start = self.start + range.start;
        let end = self.start + range.end;
        let slice = &self.tokens[range];
        Tokens {
            tokens: slice,
            start,
            end,
        }
    }
}

impl<'a> Slice<RangeTo<usize>> for Tokens<'a> {
    fn slice(&self, range: RangeTo<usize>) -> Self {
        self.slice(0..range.end)
    }
}

impl<'a> Slice<RangeFrom<usize>> for Tokens<'a> {
    fn slice(&self, range: RangeFrom<usize>) -> Self {
        self.slice(range.start..self.end - self.start)
    }
}

impl<'a> Slice<RangeFull> for Tokens<'a> {
    fn slice(&self, _: RangeFull) -> Self {
        Tokens {
            tokens: &self.tokens,
            start: self.start,
            end: self.end,
        }
    }
}
