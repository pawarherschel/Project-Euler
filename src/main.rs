mod zaphkiel;

use project_euler::*;

macro_rules! answers {
    ($( $problem:ident ($args:expr); )*) => {{
        let mut answers: std::collections::HashMap<i32, String> = std::collections::HashMap::new();
        let mut _count = 1;
        $(
            let result = $problem($args);
            answers.insert(_count, result.to_string());
            _count += 1;
        )*
        let latest = answers.get(&(&_count - 1)).unwrap();
        dbg!(latest);
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
        problem_6(100);
        problem_7(10_001);
        problem_8(13);
        problem_9(1000);
        problem_10(2_000_000);
    ];
}
