use std::collections::HashMap;

type Identifier = String;

#[derive(Clone, Debug)]
pub enum Kind {
    Star,
    Arrow {
        from: Vec<Kind>,
        to: Box<Kind>,
    }
}

#[derive(Clone, Debug)]
pub struct TypeBinding {
    pub id: Identifier,
    pub kind: Kind,
}

#[derive(Clone, Debug)]
pub enum Type {
    TypeVariable(Identifier),
    ForAll {
        args: Vec<TypeBinding>,
        typ: Box<Type>,
    },
    Boolean,
    Number,
}

#[derive(Clone, Debug)]
pub struct Binding {
    pub id: Identifier,
    pub typ: Type,
}

#[derive(Clone, Debug)]
pub enum Expression {
    Variable(Identifier),

    // Primitives
    Boolean(bool),
    Number(i64), // haha, this should be a bignum

    Function {
        parameters: Vec<Binding>,
        body: Box<Expression>,
    },

    Application {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
}

#[derive(Clone, Debug)]
pub enum Value {
    // Primitives
    Boolean(bool),
    Number(i64), // haha, this should be a bignum

    Function {
        parameters: Vec<Binding>,
        body: Box<Expression>,
    },
}

type Substitution = HashMap<Identifier, Value>;

fn eval(subst: &Substitution, expr: Expression) -> Value {
    match expr {
        Expression::Variable(identifier) => subst[&identifier].clone(),
        Expression::Boolean(value) => Value::Boolean(value),
        Expression::Number(value) => Value::Number(value),
        Expression::Function { parameters, body } => Value::Function { parameters: parameters.clone(), body: body.clone() },
        Expression::Application { function, arguments } => match eval(subst, *function) {
            Value::Function { parameters, body } => {
                let mut extended_subst = subst.clone();
                extended_subst.extend(parameters.into_iter()
                                      .zip(arguments.into_iter())
                                      .map(|(param, arg)| (param.id, eval(subst, arg))));
                eval(&extended_subst, *body)
            },
            _ => panic!("this is not a function")
        },
    }
}

pub fn run(expr: Expression) -> Value {
    let subst = HashMap::new();
    eval(&subst, expr)
}
