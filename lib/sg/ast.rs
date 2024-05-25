type Optional<T> = Option<Box<T>>;

#[derive(PartialEq, Debug, Eq, Clone)]
pub struct Identifier(pub String);

/// ```sg
/// @
/// @library
/// @/module/submodule
/// @library/module/submodule
/// ```
#[derive(PartialEq, Debug, Eq, Clone)]
pub struct Path {
    /// The unprefixed root of the path, `None` for `@`.
    root: Option<Identifier>,
    /// Fragments of the path separated by `/`
    fragments: Vec<Identifier>,
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

#[derive(PartialEq, Debug, Eq, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Divide,
    Multiply,
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

#[derive(PartialEq, Debug, Clone)]
pub struct TypeBinding {
    key: Optional<Identifier>,
    value: Optional<Type>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Binding {
    pub name: Identifier,
    pub typ: Option<Type>,
}

/// A `Block` is a series of `Statement`s followed by an optional `Expression`
/// that the `Block` takes the value of when evaluated.
#[derive(PartialEq, Debug, Clone)]
pub struct Block {
    statements: Vec<Statement>,
    expression: Optional<Expression>,
}

/// A `Property` is a key-value pair written either as `string_key = expression` or
/// as `[key_expression] = expression`.
#[derive(PartialEq, Debug, Clone)]
pub struct Property {
    key: Optional<Expression>,
    value: Optional<Expression>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expression {
    Identifier(Identifier),
    Literal(Literal),

    /// ```sg
    /// { key = value, key = value, key = value }
    /// ```
    Table {
        types: Vec<TypeBinding>,
        elements: Vec<Property>,
    },

    /// ```sg
    /// [e1, e2, e3]
    /// ```
    Array {
        elements: Vec<Expression>,
    },

    /// ```sg
    /// (e1, e2, e3)
    /// e1, e2, e3
    /// ```
    Tuple {
        elements: Vec<Expression>,
    },

    /// ```sg
    /// e1[e2]
    /// ```
    Index {
        value: Optional<Expression>,
        key: Optional<Expression>,
    },

    /// ```sg
    /// e1.string_key
    /// ```
    Project {
        value: Optional<Expression>,
        key: Identifier,
    },

    /// ```sg
    /// not a
    /// -a
    /// ```
    UnaryOperator {
        operator: Operator,
        operand: Optional<Expression>,
    },

    /// ```sg
    /// a or b
    /// a and b
    /// a ~= b
    /// ```
    BinaryOperator{
        operator: Operator,
        left_operand: Optional<Expression>,
        right_operand: Optional<Expression>,
    },

    /// ```sg
    /// function name(parameter)
    ///     body
    /// end
    /// ```
    Function {
        name: Option<Identifier>,
        parameter: Option<Binding>,
        body: Optional<Block>,
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
        function: Optional<Expression>,
        argument: Optional<Expression>,
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
        value: Optional<Expression>,
        key: Identifier,
        argument: Optional<Expression>,
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
        condition: Optional<Expression>,
        consequent: Block,
        antecedent: Block,
    },

    /// ```sg
    /// do
    ///     stmt1
    ///     stmt2
    ///     expr
    /// end
    /// ```
    Block(Block),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Statement {
    /// ```sg
    /// expr
    /// ```
    Expression(Expression),

    /// ```sg
    /// import @/module/submodule
    /// import @library/module/submodule
    /// ```
    Import(Path),

    /// ```sg
    /// export type Foo = Bar
    /// type Foo = Bar
    /// ```
    TypeAlias {
        exported: bool,
        name: Identifier,
        definition: Type,
    },

    /// ```sg
    /// local binding
    /// local binding = expr
    /// ```
    Local {
        binding: Binding,
        expression: Optional<Expression>,
    },

    /// ```sg
    /// export binding
    /// export binding = expr
    /// ```
    Export {
        binding: Binding,
        expression: Optional<Expression>,
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
        binding: Binding,
        types: Vec<TypeBinding>,
        elements: Vec<Property>,
    },

    /// ```sg
    /// for binding in expr do
    ///     block
    /// end
    /// ```
    ForIn {
        binding: Binding,
        iterator: Optional<Expression>,
        body: Optional<Block>,
    },

    /// ```sg
    /// while expr do
    ///     block
    /// end
    /// ```
    While {
        condition: Optional<Expression>,
        body: Optional<Block>,
    },

    /// ```sg
    /// repeat
    ///     block
    /// until expr
    /// ```
    RepeatUntil {
        body: Optional<Block>,
        condition: Optional<Expression>,
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
    Return(Optional<Expression>),
}
