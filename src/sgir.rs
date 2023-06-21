use std::collections::HashMap;

type Identifier = String;

#[derive(Clone, Debug, PartialEq)]
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

type KindEnv = HashMap<Identifier, Kind>;

#[derive(Clone, Debug, PartialEq)]
struct KindMismatch {
    expected: Kind,
    found: Kind,
}

type KC<T> = Result<T, KindMismatch>;

fn check_kinds(kenv: &KindEnv, typ: Type) -> KC<Kind> {
    match typ {
        Type::Variable(id) => Ok(kenv[&id].clone()),

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

        Type::Instantiate { typ, arguments } => match check_kinds(&kenv, *typ)? {
            Kind::Arrow{ from, to } => {
                for (expected, argument) in from.into_iter().zip(arguments.into_iter()) {
                    let found = check_kinds(kenv, argument)?;
                    if expected != found {
                        return Err(KindMismatch { expected, found })
                    }
                }
                Ok(*to)
            }
            Kind::Star => panic!("this is not quantified!"),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kind_checking_trivial() {
        let typ = Type::Boolean;
        let kind = check_kinds(&HashMap::new(), typ);
        assert_eq!(kind, Ok(Kind::Star));
    }

    #[test]
    fn test_kind_checking_polymorphic_identity_function() {
        let typ = Type::ForAll { parameters: vec![TypeBinding { id: "a".to_owned(), kind: Kind::Star }],
                                 typ: Box::new(Type::Function { arguments: vec![Type::Variable("a".to_owned())],
                                                                result: Box::new(Type::Variable("a".to_owned())) }) };
        let kind = check_kinds(&HashMap::new(), typ);
        assert_eq!(kind, Ok(Kind::Arrow { from: vec![Kind::Star], to: Box::new(Kind::Star) }));
    }

    #[test]
    fn test_kind_checking_instantiated_polymorphic_identity_function() {
        let typ =
        Type::Instantiate { typ: Box::new(Type::ForAll { parameters: vec![TypeBinding { id: "a".to_owned(), kind: Kind::Star }],
                                                         typ: Box::new(Type::Function { arguments: vec![Type::Variable("a".to_owned())],
                                                                                        result: Box::new(Type::Variable("a".to_owned())) }) }),
                            arguments: vec![Type::Number] };
        let kind = check_kinds(&HashMap::new(), typ);
        assert_eq!(kind, Ok(Kind::Star));
    }
}
