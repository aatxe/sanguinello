use crate::sgir::Binding;

mod sgir;

fn main() {
    use sgir::Expression::*;
    use sgir::Type;

    let prog = Application {
        function: Box::new(Function {
            parameters: vec![
                Binding { id: "chucc".to_owned(), typ: Type::Number },
                Binding { id: "awe".to_owned(), typ: Type::Number },
                Binding { id: "alex!".to_owned(), typ: Type::Number },
                Binding { id: "j".to_owned(), typ: Type::Number },
            ],
            body: Box::new(Variable("alex!".to_owned())),
        }),
        arguments: vec![
            Number(420),
            Number(11),
            Function {
                parameters: vec![Binding { id: "x".to_owned(), typ: Type::Number }],
                body: Box::new(Variable("x".to_owned())),
            },
            Number(694208008135),
        ]
    };

    let result = sgir::run(prog);
    println!("{:?}", result);
}
