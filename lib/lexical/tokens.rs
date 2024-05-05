#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Illegal,
    EOF,

    Identifier(String),

    // literals
    NumberLiteral(i64),
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
    Equal,
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
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(C)]
pub struct Tokens<'a> {
    pub tok: &'a [Token],
    pub start: usize,
    pub end: usize,
}

impl<'a> Tokens<'a> {
    pub fn new(vec: &'a [Token]) -> Self {
        Tokens {
            tok: vec,
            start: 0,
            end: vec.len(),
        }
    }
}
