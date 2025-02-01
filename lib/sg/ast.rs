use bumpalo::collections::Vec;

type Optional<'a, T> = Option<&'a T>;

#[derive(PartialEq, Debug, Eq, Clone)]
pub struct Identifier<'a>(pub &'a str);

/// ```sg
/// @
/// @library
/// @/module/submodule
/// @library/module/submodule
/// ```
#[derive(PartialEq, Debug, Eq, Clone)]
pub struct Path<'a> {
    /// The unprefixed root of the path, `None` for `@`.
    root: Option<Identifier<'a>>,
    /// Fragments of the path separated by `/`
    fragments: Vec<'a, Identifier<'a>>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Literal {
    /// ```sg
    /// nil
    /// ```
    Nil,

    /// ```sg
    /// 1
    /// 4
    /// 3.14
    /// ```
    Number(f64),

    /// ```sg
    /// true
    /// false
    /// ```
    Boolean(bool),
}

#[derive(PartialEq, Debug, Eq, Clone, Copy)]
pub enum Operator {
    Plus,
    Minus,
    Divide,
    Multiply,
    FloorDivide,
    Equal,
    NotEqual,
    GreaterThanEqual,
    LessThanEqual,
    GreaterThan,
    LessThan,
    Not,
    And,
    Or,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Type;

/// ```sg
/// type name = type
/// ```
#[derive(PartialEq, Debug)]
pub struct TypeBinding<'a> {
    key: Optional<'a, Identifier<'a>>,
    value: Optional<'a, Type>,
}

/// ```sg
/// local name: type
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct Binding<'a> {
    pub name: Identifier<'a>,
    pub typ: Option<Type>,
}

/// A `Block` is a series of `Statement`s followed by an optional `Expression`
/// that the `Block` takes the value of when evaluated.
#[derive(PartialEq, Debug)]
pub struct Block<'a> {
    statements: Vec<'a, Statement<'a>>,
    expression: Optional<'a, Expression<'a>>,
}

/// A `Property` is a key-value pair written either as `string_key = expression` or
/// as `[key_expression] = expression`.
#[derive(PartialEq, Debug)]
pub struct Property<'a> {
    pub(crate) key: Optional<'a, Expression<'a>>,
    pub(crate) value: Optional<'a, Expression<'a>>,
}

#[derive(PartialEq, Debug)]
pub enum Expression<'a> {
    Identifier(Identifier<'a>),
    Literal(Literal),

    /// ```sg
    /// { key = value, key = value, key = value }
    /// ```
    Table {
        types: Vec<'a, TypeBinding<'a>>,
        elements: Vec<'a, Property<'a>>,
    },

    /// ```sg
    /// [e1, e2, e3]
    /// ```
    Array {
        elements: Vec<'a, Expression<'a>>,
    },

    /// ```sg
    /// (e1, e2, e3)
    /// e1, e2, e3
    /// ```
    Tuple {
        elements: Vec<'a, Expression<'a>>,
    },

    /// ```sg
    /// e1[e2]
    /// ```
    Index {
        value: Optional<'a, Expression<'a>>,
        key: Optional<'a, Expression<'a>>,
    },

    /// ```sg
    /// e1.string_key
    /// ```
    Project {
        value: Optional<'a, Expression<'a>>,
        key: Identifier<'a>,
    },

    /// ```sg
    /// not a
    /// -a
    /// ```
    UnaryOperator {
        operator: Operator,
        operand: Optional<'a, Expression<'a>>,
    },

    /// ```sg
    /// a or b
    /// a and b
    /// a ~= b
    /// ```
    BinaryOperator {
        operator: Operator,
        left_operand: Optional<'a, Expression<'a>>,
        right_operand: Optional<'a, Expression<'a>>,
    },

    /// ```sg
    /// function name(parameter)
    ///     body
    /// end
    /// ```
    Function {
        name: Option<Identifier<'a>>,
        parameter: Option<Binding<'a>>,
        body: Optional<'a, Block<'a>>,
    },

    /// ```sg
    /// function argument
    /// function (argument)
    /// function(argument)
    /// function "argument"
    /// function"argument"
    /// function[argument]
    /// function [argument]
    /// function{argument}
    /// function {argument}
    /// ```
    Application {
        function: Optional<'a, Expression<'a>>,
        argument: Optional<'a, Expression<'a>>,
    },

    /// ```sg
    /// e1:e2 argument
    /// e1:e2 (argument)
    /// e1:e2(argument)
    /// e1:e2 "argument"
    /// e1:e2"argument"
    /// e1:e2[argument]
    /// e1:e2 [argument]
    /// e1:e2{argument}
    /// e1:e2 {argument}
    /// ```
    ProjectApplication {
        value: Optional<'a, Expression<'a>>,
        key: Identifier<'a>,
        argument: Optional<'a, Expression<'a>>,
    },

    /// ```sg
    /// if condition then
    ///     consequent
    /// else
    ///     antecedent
    /// end
    /// ```
    ///
    /// ```sg
    /// if condition then
    ///     consequent
    /// end
    /// ```
    If {
        condition: Optional<'a, Expression<'a>>,
        consequent: Block<'a>,
        antecedent: Block<'a>,
    },

    /// ```sg
    /// do
    ///     stmt1
    ///     stmt2
    ///     expr
    /// end
    /// ```
    Block(Block<'a>),
}

#[derive(PartialEq, Debug)]
pub enum Statement<'a> {
    /// ```sg
    /// expr
    /// ```
    Expression(Expression<'a>),

    /// ```sg
    /// import @/module/submodule
    /// import @library/module/submodule
    /// ```
    Import(Path<'a>),

    /// ```sg
    /// export type Foo = Bar
    /// type Foo = Bar
    /// ```
    TypeAlias {
        exported: bool,
        name: Identifier<'a>,
        definition: Type,
    },

    /// ```sg
    /// local binding
    /// local binding = expr
    /// ```
    Local {
        binding: Binding<'a>,
        expression: Optional<'a, Expression<'a>>,
    },

    /// ```sg
    /// export binding
    /// export binding = expr
    /// ```
    Export {
        binding: Binding<'a>,
        expression: Optional<'a, Expression<'a>>,
    },

    /// ```sg
    /// module binding do
    ///   type key = value
    ///   type key = value
    ///   type key = value
    ///
    ///   key = value,
    ///   key = value,
    ///   key = value,
    /// end
    /// ```
    /// which is syntactic sugar for
    /// ```sg
    /// local binding = {
    ///   type key = value
    ///   type key = value
    ///   type key = value
    ///
    ///   key = value,
    ///   key = value,
    ///   key = value,
    /// }
    /// ```
    /// or `export module name of type` which expands the same but with `export`.
    Module {
        exported: bool,
        binding: Binding<'a>,
        types: Vec<'a, TypeBinding<'a>>,
        elements: Vec<'a, Property<'a>>,
    },

    /// ```sg
    /// for binding in expr do
    ///     block
    /// end
    /// ```
    ForIn {
        binding: Binding<'a>,
        iterator: Optional<'a, Expression<'a>>,
        body: Optional<'a, Block<'a>>,
    },

    /// ```sg
    /// while expr do
    ///     block
    /// end
    /// ```
    While {
        condition: Optional<'a, Expression<'a>>,
        body: Optional<'a, Block<'a>>,
    },

    /// ```sg
    /// repeat
    ///     block
    /// until expr
    /// ```
    RepeatUntil {
        body: Optional<'a, Block<'a>>,
        condition: Optional<'a, Expression<'a>>,
    },

    /// ```sg
    /// break
    /// ```
    Break,

    /// ```sg
    /// continue
    /// ```
    Continue,

    /// ```sg
    /// return expr
    /// return
    /// ```
    Return(Optional<'a, Expression<'a>>),
}
