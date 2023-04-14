#[macro_use]
extern crate syn;

use project_euler::*;



//macro_rules! answers {
//    ($( $problem:ident ($args:expr); )*) => {{
//        let mut answers = vec![];
//        let mut _count = 1;
//        $(
//            let result = $problem($args);
//            answers.push(format!("Problem {}: {:?}", _count, result));
//            _count += 1;
//        )*
//        println!("{:?}", answers.iter().rev().collect::<Vec<_>>());
//    }}
//}
macro_rules! answers {
    ($( $problem:ident ($args:expr); )*) => {{
        let mut answers = vec![];
        let mut _count = 1;
        $(
            let result = { 
                let problem_ident = format!("problem_{}", _count);
                let problem_fn = ident!(problem_ident);
                problem_fn($args)
            };
            answers.push(format!("Problem {}: {:?}", _count, result));
            _count += 1;
            )*
        println!("{:?}", answers.iter().rev().collect::<Vec<_>>());
    }}
}

macro_rules! ident {
    ($ident:expr) => { 
        { 
            let id = $ident;
            syn::parse_str::<syn::Ident>(id).unwrap()
        } 
    };
}




fn main() {
    answers![
        problem_1(1000);
        problem_2(4_000_000);
        problem_3(600851475143);
        problem_4(3);
    ];
    
//    answers![
//        1000;
//        4_000_000;
//        600851475143;
//        3
//    ]
}
