use project_euler::*;

macro_rules! answers {
    ($( $problem:ident ($args:expr); )*) => {{
        let mut answers = vec![];
        let mut _count = 1;
        $(
            let result = $problem($args);
            answers.push(format!("Problem {}: {:?}", _count, result));
            _count += 1;
        )*
        println!("{:?}", answers.iter().rev().collect::<Vec<_>>());
    }}
}

fn main() {
    answers![
        problem_1(1000);
        problem_2(4_000_000);
        problem_3(600851475143);
        problem_4(3);
    ];
}
