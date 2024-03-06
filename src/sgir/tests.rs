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
    let typ = Type::Instantiate { typ: Box::new(Type::ForAll { parameters: vec![TypeBinding { id: "a".to_owned(), kind: Kind::Star }],
                                                               typ: Box::new(Type::Function { arguments: vec![Type::Variable("a".to_owned())],
                                                                                              result: Box::new(Type::Variable("a".to_owned())) }) }),
                                  arguments: vec![Type::Number] };
    let kind = check_kinds(&HashMap::new(), typ);
    assert_eq!(kind, Ok(Kind::Star));
}

#[test]
fn test_kind_checking_unbound_identifier() {
    let typ = Type::Variable("foo".to_owned());
    let kind = check_kinds(&HashMap::new(), typ);
    assert_eq!(kind, Err(TypeError::UnboundIdentifier("foo".to_owned())));
}

#[test]
fn test_kind_checking_instantiated_monomorphic_type() {
    let quantified_type = Type::Function { arguments: vec![Type::Number],
                                           result: Box::new(Type::Number) };
    let typ = Type::Instantiate { typ: Box::new(quantified_type.clone()),
                                  arguments: vec![Type::Number] };
    let kind = check_kinds(&HashMap::new(), typ);
    assert_eq!(kind, Err(TypeError::ExpectedQuantifier { found: quantified_type }));
}

#[test]
fn test_kind_checking_instantiated_polymorphic_function_with_type_constructor() {
    let typ = Type::Instantiate { typ: Box::new(Type::ForAll { parameters: vec![TypeBinding { id: "a".to_owned(), kind: Kind::Star }],
                                                               typ: Box::new(Type::Function { arguments: vec![Type::Variable("a".to_owned())],
                                                                                              result: Box::new(Type::Variable("a".to_owned())) }) }),
                                  arguments: vec![Type::ForAll { parameters: vec![TypeBinding { id: "b".to_owned(), kind: Kind::Star }],
                                                                 typ: Box::new(Type::Variable("b".to_owned())) }] };
    let kind = check_kinds(&HashMap::new(), typ);
    assert_eq!(kind, Err(TypeError::KindMismatch { expected: Kind::Star, found: Kind::Arrow { from: vec![Kind::Star], to: Box::new(Kind::Star) }}));
}

#[test]
fn test_type_check_basic_number() {
    let expr = Expression::Number(42);

    let typ = check(expr);
    assert_eq!(typ, Ok(Type::Number));
}

#[test]
fn test_type_check_basic_boolean() {
    let expr = Expression::Boolean(true);

    let typ = check(expr);
    assert_eq!(typ, Ok(Type::Boolean));
}

#[test]
fn test_type_check_identity_function() {
    let expr = Expression::Function {
        parameters: vec![Binding { id: "x".to_owned(), typ: Type::Number }],
        body: Box::new(Expression::Variable("x".to_owned())),
    };

    let typ = check(expr);
    assert_eq!(typ, Ok(Type::Function { arguments: vec![Type::Number], result: Box::new(Type::Number) }));
}

#[test]
fn test_type_check_apply_identity_function() {
    let identity = Expression::Function {
        parameters: vec![Binding { id: "x".to_owned(), typ: Type::Number }],
        body: Box::new(Expression::Variable("x".to_owned())),
    };

    let expr = Expression::Application {
        function: Box::new(identity),
        arguments: vec![
            Expression::Number(42),
        ],
    };

    let typ = check(expr);
    assert_eq!(typ, Ok(Type::Number));
}

#[test]
fn test_type_check_apply_identity_function_mismatch() {
    let identity = Expression::Function {
        parameters: vec![Binding { id: "x".to_owned(), typ: Type::Number }],
        body: Box::new(Expression::Variable("x".to_owned())),
    };

    let expr = Expression::Application {
        function: Box::new(identity),
        arguments: vec![
            Expression::Boolean(false),
        ],
    };

    let typ = check(expr);
    assert_eq!(typ, Err(TypeError::TypeMismatch { expected: Type::Number, found: Type::Boolean }));
}

#[test]
fn test_type_checking_polymorphic_identity_function() {
    let identity = Expression::Quantify {
        parameters: vec![TypeBinding { id: "X".to_owned(), kind: Kind::Star }],
        body: Box::new(Expression::Function {
            parameters: vec![Binding { id: "x".to_owned(), typ: Type::Variable("X".to_owned()) }],
            body: Box::new(Expression::Variable("x".to_owned())),
        }),
    };

    let expected_type = Type::ForAll {
        parameters: vec![TypeBinding { id: "X".to_owned(), kind: Kind::Star }],
        typ: Box::new(Type::Function {
            arguments: vec![Type::Variable("X".to_owned())],
            result: Box::new(Type::Variable("X".to_owned())),
        }),
    };

    let typ = check(identity);
    assert_eq!(typ, Ok(expected_type));
}
