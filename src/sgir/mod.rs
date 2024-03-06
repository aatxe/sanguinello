//! This module implements `sgir`, an intermediate representation for sanguinello.
//!
//! It is based on System Fω with explicit typing.

use std::collections::HashMap;
use thiserror::Error;

#[cfg(test)]
mod tests;

type Identifier = String;

#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
    /// The type of types.
    Star,

    /// A type constructor, or type function.
    Arrow {
        from: Vec<Kind>,
        to: Box<Kind>,
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeBinding {
    pub id: Identifier,
    pub kind: Kind,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    /// a type variable, e.g. `T`
    Variable(Identifier),

    /// universal quantification, e.g. `forall<T...>. U`
    ForAll {
        parameters: Vec<TypeBinding>,
        typ: Box<Type>,
    },

    /// type instantiation, e.g. T<U...>
    Instantiate {
        typ: Box<Type>,
        arguments: Vec<Type>,
    },

    // TODO: existential quantification

    /// a function, e.g. (T...) -> U
    Function {
        arguments: Vec<Type>,
        result: Box<Type>,
    },

    /// a boolean
    Boolean,

    /// a number
    Number,
}

#[derive(Debug, Error, Clone, PartialEq)]
pub enum TypeError {
    #[error("kind mismatch: expected {expected:?}, found {found:?}")]
    KindMismatch {
        expected: Kind,
        found: Kind,
    },

    #[error("type mismatch: expected {expected:?}, found {found:?}")]
    TypeMismatch {
        expected: Type,
        found: Type,
    },

    #[error("cannot call a non-function: {found:?}")]
    CannotCallNonFunction {
        found: Type,
    },

    #[error("kind mismatch: expected a quantifier in type {found:?}")]
    ExpectedQuantifier {
        found: Type,
    },

    #[error("unbound identifier: {0}")]
    UnboundIdentifier(Identifier),
}

pub type TC<T> = Result<T, TypeError>;

type KindEnv = HashMap<Identifier, Kind>;

fn check_kinds(kenv: &KindEnv, typ: Type) -> TC<Kind> {
    match typ {
        Type::Variable(id) => match kenv.get(&id) {
            Some(kind) => Ok(kind.clone()),
            None => Err(TypeError::UnboundIdentifier(id.clone())),
        }

        Type::ForAll { parameters, typ } => {
            let from = parameters.iter()
                                 .map(|TypeBinding { kind, .. }| kind.clone())
                                 .collect();

            let mut extended_kenv = kenv.clone();
            extended_kenv.extend(parameters.into_iter()
                                 .map(|TypeBinding { id, kind }| (id, kind)));
            let to = Box::new(check_kinds(&extended_kenv, *typ)?);

            Ok(Kind::Arrow { from, to })
        }

        Type::Instantiate { typ, arguments } => match check_kinds(&kenv, *typ.clone())? {
            Kind::Arrow{ from, to } => {
                for (expected, argument) in from.into_iter().zip(arguments.into_iter()) {
                    let found = check_kinds(kenv, argument)?;
                    if expected != found {
                        return Err(TypeError::KindMismatch { expected, found })
                    }
                }
                Ok(*to)
            }
            Kind::Star => Err(TypeError::ExpectedQuantifier { found: *typ }),
        }

        _ => Ok(Kind::Star),
    }
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

type TypeEnv = HashMap<Identifier, Type>;

fn check_type(tenv: &TypeEnv, kenv: &KindEnv, expr: Expression) -> TC<Type> {
    match expr {
        Expression::Variable(id) =>  match tenv.get(&id) {
            Some(ty) => Ok(ty.clone()),
            None => Err(TypeError::UnboundIdentifier(id.clone())),
        },

        Expression::Boolean(_) => Ok(Type::Boolean),

        Expression::Number(_) => Ok(Type::Number),

        Expression::Function { parameters, body } => {
            for Binding { typ, .. } in parameters.iter() {
                if let kind@Kind::Arrow { .. } = check_kinds(kenv, typ.clone())? {
                    return Err(TypeError::KindMismatch { expected: Kind::Star, found: kind })
                }
            }

            let arguments = parameters.iter()
                                      .map(|Binding { typ, .. }| typ.clone())
                                      .collect();

            let mut extended_tenv = tenv.clone();
            extended_tenv.extend(parameters.into_iter()
                                 .map(|Binding { id, typ }| (id, typ)));
            let result = Box::new(check_type(&extended_tenv, &kenv, *body)?);

            Ok(Type::Function { arguments, result })
        },

        Expression::Application { function, arguments } => {
            match check_type(tenv, kenv, *function)? {
                Type::Function { arguments: argument_types, result: result_type } => {
                    for (argument, argument_type) in arguments.into_iter().zip(argument_types) {
                        let computed_type = check_type(tenv, kenv, argument)?;

                        if computed_type != argument_type {
                            return Err(TypeError::TypeMismatch { expected: argument_type, found: computed_type })
                        }
                    }

                    Ok(*result_type)
                },

                // Unexpected type here, it must be a function!
                found => Err(TypeError::CannotCallNonFunction { found })
            }
        },
    }
}

pub fn check(expr: Expression) -> TC<Type> {
    let type_env = HashMap::new();
    let kind_env = HashMap::new();
    check_type(&type_env, &kind_env, expr)
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
