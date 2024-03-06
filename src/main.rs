use crate::sgir::{Binding, TypeBinding};

mod lexer;
mod sgir;

fn main() {
    use sgir::Expression::*;
    use sgir::{Kind, Type};

    let identity = Quantify {
        parameters: vec![TypeBinding { id: "X".to_owned(), kind: Kind::Star }],
        body: Box::new(Function {
            parameters: vec![Binding { id: "x".to_owned(), typ: Type::Variable("X".to_owned()) }],
            body: Box::new(Variable("x".to_owned())),
        }),
    };

    let application = Application {
        function: Box::new(Instantiate {
            function: Box::new(identity),
            arguments: vec![Type::Number]
        }),
        arguments: vec![
            Number(11)
        ],
    };

    let prog = Application {
        function: Box::new(Function {
            parameters: vec![
                Binding { id: "chucc".to_owned(), typ: Type::Number },
                Binding { id: "awe".to_owned(), typ: Type::Boolean },
                Binding { id: "alex!".to_owned(), typ: Type::Number },
                Binding { id: "j".to_owned(), typ: Type::Number },
            ],
            body: Box::new(Variable("alex!".to_owned())),
        }), // (number, boolean, number, number) -> number
        arguments: vec![
            application, // : number
            Boolean(true), // : boolean
            Application {
                function: Box::new(Function {
                    parameters: vec![Binding { id: "x".to_owned(), typ: Type::Number }],
                    body: Box::new(Variable("x".to_owned())),
                }), // : (number) -> number
                arguments: vec![Number(42)], // : number
            }, // : number
            Number(694208008135), // : number
        ]
    };

    let typ = match sgir::check(prog.clone()) {
        Ok(typ) => typ,
        Err(type_error) => {
            eprintln!("[ERROR] {:?}", type_error);
            return
        },
    };
    println!("TYPE: {:?}", typ);

    let result = sgir::run(prog);
    println!("{:?}", result);
}
