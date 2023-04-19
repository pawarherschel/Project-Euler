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
//        let answers_rev = answers.clone().into_iter().rev().collect::<Vec<_>>();
//        dbg!(&answers_rev);
//        answers_rev
//    }}
//}

macro_rules! answers {
    ($( $problem:ident ($args:expr); )*) => {{
        let mut answers: std::collections::HashMap<i32, String> = std::collections::HashMap::new();
        let mut _count = 1;
        $(
            let result = $problem($args);
            answers.insert(_count, result.to_string());
            _count += 1;
            )*
        dbg!(&answers);
        answers
    }}
}

fn main() {
    let _answers = answers![
        problem_1(1000);
        problem_2(4_000_000);
        problem_3(600851475143);
        problem_4(3);
        problem_5(20);
    ];
}
