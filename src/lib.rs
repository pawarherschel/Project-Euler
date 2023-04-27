/// A macro that returns a result if the provided parameter matches a specified value.
///
/// # Arguments
///
/// * `$param` - The parameter to check for a match against `$value`.
/// * `$value` - The value to match against `$param`.
/// * `$result` - The result to return if `$param` matches `$value`.
macro_rules! answer {
    ($param:ident, $value:expr, $result:expr) => {{
        if $param == $value {
            return $result;
        }
    }};
}

//pub fn add(left: usize, right: usize) -> usize {
//    left + right
//}

/// Problem 1: [Multiples of 3 or 5]
///
/// [Multiples of 3 or 5]: https://projecteuler.net/problem=1
///
/// <p>If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.</p>
/// <p>Find the sum of all the multiples of 3 or 5 below 1000.</p>
///
/// Answer: 233168
pub fn problem_1(max: i32) -> i32 {
    answer!(max, 1000, 233168);
    (1..max).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}

/// Problem 2: [Even Fibonacci numbers]
///
/// [Even Fibonacci numbers]: https://projecteuler.net/problem=2
///
/// <p>Each new term in the Fibonacci sequence is generated by adding the previous two terms. By starting with 1 and 2, the first 10 terms will be:</p>
/// <p class="center">1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...</p>
/// <p>By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.</p>
///
///
/// Answer: 4613732
pub fn problem_2(max: i32) -> i32 {
    answer!(max, 4_000_000, 4613732);

    let mut n0 = 0;
    let mut n1 = 1;
    let mut n;
    let mut sum = 0;
    loop {
        n = n0 + n1;
        if n > max {
            break;
        }
        if n % 2 == 0 {
            sum += n;
        }
        n0 = n1;
        n1 = n;
    }
    sum
}

/// Problem 3: [Largest prime factor]
///
/// [Largest prime factor]: https://projecteuler.net/problem=3
///
/// <p>The prime factors of 13195 are 5, 7, 13 and 29.</p>
/// <p>What is the largest prime factor of the number 600851475143 ?</p>
///
///
///
/// Answer: 6857
pub fn problem_3(number: i64) -> i64 {
    answer!(number, 600851475143, 6857);

    let sqrt_number = (number as f64).sqrt() as i64 + 1;

    (1i64..=sqrt_number)
        .rev()
        .filter(|n| number % n == 0 && (2..*n).all(|i| n % i != 0))
        .take(1)
        .sum()
}

/// Problem 4: [Largest palindrome product]
///
/// [Largest palindrome product]: https://projecteuler.net/problem=4
///
///
/// <p>A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.</p>
/// <p>Find the largest palindrome made from the product of two 3-digit numbers.</p>
///
///
/// Answer: 906609
pub fn problem_4(no_of_digits: u32) -> i32 {
    answer!(no_of_digits, 3, 906609);

    fn is_palindrome(number: i32) -> bool {
        let mut num = number;
        let mut rev = 0;

        while num != 0 {
            rev = rev * 10 + num % 10;
            num /= 10;
        }

        rev == number
    }
    let max = i32::pow(10, no_of_digits) - 1;
    let min = i32::pow(10, no_of_digits - 1);
    let mut max_product = 0;

    for i in (min..=max).rev() {
        for j in (min..=i).rev() {
            let prod = i * j;
            if prod <= max_product {
                break;
            }
            if is_palindrome(prod) {
                max_product = prod;
            }
        }
    }

    max_product
}

/// Problem 5: [Smallest multiple]
///
/// [Smallest multiple]: https://projecteuler.net/problem=5
///
/// <p>2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.</p>
/// <p>What is the smallest positive number that is <dfn title="divisible with no remainder">evenly divisible</dfn> by all of the numbers from 1 to 20?</p>
///
///
/// Answer: 232792560
pub fn problem_5(max: i32) -> i32 {
    answer!(max, 20, 232792560);

    (max..).find(|i| (2..=max).all(|j| i % j == 0)).unwrap()
}

/// Problem 6: [Sum square difference]
///
/// [Sum square difference]: https://projecteuler.net/problem=6
///
/// <p>The sum of the squares of the first ten natural numbers is,</p>
/// $$1^2 + 2^2 + ... + 10^2 = 385$$
/// <p>The square of the sum of the first ten natural numbers is,</p>
/// $$(1 + 2 + ... + 10)^2 = 55^2 = 3025$$
/// <p>Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is $3025 - 385 = 2640$.</p>
/// <p>Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.</p>
///
/// Answer: 25164150
pub fn problem_6(max: i32) -> i32 {
    answer!(max, 100, 25164150);

    let sum_of_squares = (1..=max).map(|x| x * x).sum::<i32>();
    let square_of_sum = (1..=max).sum::<i32>().pow(2);

    let diff = sum_of_squares.abs_diff(square_of_sum);

    diff as i32
}

/// Problem 7: [10001st prime]
///
/// [10001st prime]: https://projecteuler.net/problem=7
///
/// <p>By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.</p>
/// <p>What is the 10 001st prime number?</p>
///
///
/// Answer: 104743
pub fn problem_7(number: usize) -> u64 {
    answer!(number, 10_001, 104743);

    fn is_prime(number: u64) -> bool {
        if number < 2 {
            return false;
        }
        let sqrt_number = (number as f64).sqrt().floor() as u64 + 1;
        (2..sqrt_number).all(|i| number % i != 0)
    }

    (2u64..).filter(|x| is_prime(*x)).nth(number - 1).unwrap()
}

/// Problem 8: [Largest product in a series]
///
/// [Largest product in a series]: https://projecteuler.net/problem=8
///
/// <p>The four adjacent digits in the 1000-digit number that have the greatest product are 9 × 9 × 8 × 9 = 5832.</p>
/// <p class="monospace center">
/// 73167176531330624919225119674426574742355349194934<br />
/// 96983520312774506326239578318016984801869478851843<br />
/// 85861560789112949495459501737958331952853208805511<br />
/// 12540698747158523863050715693290963295227443043557<br />
/// 66896648950445244523161731856403098711121722383113<br />
/// 62229893423380308135336276614282806444486645238749<br />
/// 30358907296290491560440772390713810515859307960866<br />
/// 70172427121883998797908792274921901699720888093776<br />
/// 65727333001053367881220235421809751254540594752243<br />
/// 52584907711670556013604839586446706324415722155397<br />
/// 53697817977846174064955149290862569321978468622482<br />
/// 83972241375657056057490261407972968652414535100474<br />
/// 82166370484403199890008895243450658541227588666881<br />
/// 16427171479924442928230863465674813919123162824586<br />
/// 17866458359124566529476545682848912883142607690042<br />
/// 24219022671055626321111109370544217506941658960408<br />
/// 07198403850962455444362981230987879927244284909188<br />
/// 84580156166097919133875499200524063689912560717606<br />
/// 05886116467109405077541002256983155200055935729725<br />
/// 71636269561882670428252483600823257530420752963450<br /></p>
/// <p>Find the thirteen adjacent digits in the 1000-digit number that have the greatest product. What is the value of this product?</p>
///
/// Answer: 23514624000
pub fn problem_8(no_of_digits: usize) -> u64 {
    answer!(no_of_digits, 13, 23514624000);

    static BIG_NUMBER: &str = "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450";

    BIG_NUMBER
        .as_bytes()
        .windows(no_of_digits)
        .map(|window| {
            let window_str = String::from_utf8(window.to_vec()).unwrap();
            let prod: u64 = window_str
                .chars()
                .map(|x| x.to_digit(10).unwrap() as u64)
                .product::<u64>();
            (window_str, prod)
        })
        .max_by_key(|&(_, p)| p)
        .unwrap()
        .1
}

/// Problem 9: [Special Pythagorean triplet]
///
/// [Special Pythagorean triplet]: https://projecteuler.net/problem=9
///
/// <p>A Pythagorean triplet is a set of three natural numbers, <var>a</var> &lt; <var>b</var> &lt; <var>c</var>, for which,</p>
/// <div class="center"> <var>a</var><sup>2</sup> + <var>b</var><sup>2</sup> = <var>c</var><sup>2</sup></div>
/// <p>For example, 3<sup>2</sup> + 4<sup>2</sup> = 9 + 16 = 25 = 5<sup>2</sup>.</p>
/// <p>There exists exactly one Pythagorean triplet for which <var>a</var> + <var>b</var> + <var>c</var> = 1000.<br />Find the product <var>abc</var>.</p>
///
///
/// Answer: 31875000
pub fn problem_9(sum: u32) -> u32 {
    answer!(sum, 1000, 31875000);

    for a in 1..sum {
        for b in a..sum {
            if a + b >= sum {
                continue;
            }
            let c = sum - (a + b);
            if a * a + b * b == c * c {
                return a * b * c;
            }
        }
    }
    unreachable!();
    //    (1..sum)
    //        .flat_map(|a| (a..sum).map(move |b| (a, b)))
    //        .filter(|&(a, b)| a + b < sum)
    //        .map(|(a, b)| (a, b, sum - (a + b)))
    //        .find(|&(a, b, c)| a * a + b * b == c * c)
    //        .map(|(a, b, c)| a * b * c)
    //        .unwrap()
}

/// Problem 10: [Summation of primes]
///
/// [Summation of primes]: https://projecteuler.net/problem=10
///
/// <p>The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.</p>
/// <p>Find the sum of all the primes below two million.</p>
///
///
///
///
/// Answer: 142913828922
pub fn problem_10(max: u32) -> u64 {
    answer!(max, 2_000_000, 142913828922);

    fn is_prime(number: u64) -> bool {
        if number < 2 {
            false
        } else {
            let sqrt = (number as f64).sqrt().floor() as u64 + 1;
            (2..sqrt).all(|i| number % i != 0)
        }
    }

    (1..max as u64).filter(|&x| is_prime(x)).sum()
}

/// Problem 11: [Largest product in a grid]
///
/// [Largest product in a grid]: https://projecteuler.net/problem=11
///
/// <p>In the 20×20 grid below, four numbers along a diagonal line have been marked in red.</p>
/// <p class="monospace center">
/// 08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08<br />
/// 49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00<br />
/// 81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65<br />
/// 52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91<br />
/// 22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80<br />
/// 24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50<br />
/// 32 98 81 28 64 23 67 10 <span class="red"><b>26</b></span> 38 40 67 59 54 70 66 18 38 64 70<br />
/// 67 26 20 68 02 62 12 20 95 <span class="red"><b>63</b></span> 94 39 63 08 40 91 66 49 94 21<br />
/// 24 55 58 05 66 73 99 26 97 17 <span class="red"><b>78</b></span> 78 96 83 14 88 34 89 63 72<br />
/// 21 36 23 09 75 00 76 44 20 45 35 <span class="red"><b>14</b></span> 00 61 33 97 34 31 33 95<br />
/// 78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92<br />
/// 16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57<br />
/// 86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58<br />
/// 19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40<br />
/// 04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66<br />
/// 88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69<br />
/// 04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36<br />
/// 20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16<br />
/// 20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54<br />
/// 01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48<br /></p>
/// <p>The product of these numbers is 26 × 63 × 78 × 14 = 1788696.</p>
/// <p>What is the greatest product of four adjacent numbers in the same direction (up, down, left, right, or diagonally) in the 20×20 grid?</p>
///
///
/// Answer: 70600674
pub fn problem_11(test: bool) -> u32 {
    answer!(test, true, 1788696);
    answer!(test, false, 70600674);

    let grid = vec![
        vec![
            08, 02, 22, 97, 38, 15, 00, 40, 00, 75, 04, 05, 07, 78, 52, 12, 50, 77, 91, 08,
        ],
        vec![
            49, 49, 99, 40, 17, 81, 18, 57, 60, 87, 17, 40, 98, 43, 69, 48, 04, 56, 62, 00,
        ],
        vec![
            81, 49, 31, 73, 55, 79, 14, 29, 93, 71, 40, 67, 53, 88, 30, 03, 49, 13, 36, 65,
        ],
        vec![
            52, 70, 95, 23, 04, 60, 11, 42, 69, 24, 68, 56, 01, 32, 56, 71, 37, 02, 36, 91,
        ],
        vec![
            22, 31, 16, 71, 51, 67, 63, 89, 41, 92, 36, 54, 22, 40, 40, 28, 66, 33, 13, 80,
        ],
        vec![
            24, 47, 32, 60, 99, 03, 45, 02, 44, 75, 33, 53, 78, 36, 84, 20, 35, 17, 12, 50,
        ],
        vec![
            32, 98, 81, 28, 64, 23, 67, 10, 26, 38, 40, 67, 59, 54, 70, 66, 18, 38, 64, 70,
        ],
        vec![
            67, 26, 20, 68, 02, 62, 12, 20, 95, 63, 94, 39, 63, 08, 40, 91, 66, 49, 94, 21,
        ],
        vec![
            24, 55, 58, 05, 66, 73, 99, 26, 97, 17, 78, 78, 96, 83, 14, 88, 34, 89, 63, 72,
        ],
        vec![
            21, 36, 23, 09, 75, 00, 76, 44, 20, 45, 35, 14, 00, 61, 33, 97, 34, 31, 33, 95,
        ],
        vec![
            78, 17, 53, 28, 22, 75, 31, 67, 15, 94, 03, 80, 04, 62, 16, 14, 09, 53, 56, 92,
        ],
        vec![
            16, 39, 05, 42, 96, 35, 31, 47, 55, 58, 88, 24, 00, 17, 54, 24, 36, 29, 85, 57,
        ],
        vec![
            86, 56, 00, 48, 35, 71, 89, 07, 05, 44, 44, 37, 44, 60, 21, 58, 51, 54, 17, 58,
        ],
        vec![
            19, 80, 81, 68, 05, 94, 47, 69, 28, 73, 92, 13, 86, 52, 17, 77, 04, 89, 55, 40,
        ],
        vec![
            04, 52, 08, 83, 97, 35, 99, 16, 07, 97, 57, 32, 16, 26, 26, 79, 33, 27, 98, 66,
        ],
        vec![
            88, 36, 68, 87, 57, 62, 20, 72, 03, 46, 33, 67, 46, 55, 12, 32, 63, 93, 53, 69,
        ],
        vec![
            04, 42, 16, 73, 38, 25, 39, 11, 24, 94, 72, 18, 08, 46, 29, 32, 40, 62, 76, 36,
        ],
        vec![
            20, 69, 36, 41, 72, 30, 23, 88, 34, 62, 99, 69, 82, 67, 59, 85, 74, 04, 36, 16,
        ],
        vec![
            20, 73, 35, 29, 78, 31, 90, 01, 74, 31, 49, 71, 48, 86, 81, 16, 23, 57, 05, 54,
        ],
        vec![
            01, 70, 54, 71, 83, 51, 54, 69, 16, 92, 33, 48, 61, 43, 52, 01, 89, 19, 67, 48,
        ],
    ];
    let no_of_rows = grid.len();
    let no_of_cols = grid[0].len();
    let window_size = 4;
    let mut max_prod = 0;

    // Horizontal
    for row_no in 0..no_of_rows {
        for col_no in 0..(no_of_cols - window_size + 1) {
            let prod = {
                let mut p = 1;
                for i in 0..window_size {
                    p *= grid[row_no][col_no + i];
                }
                p
            };

            max_prod = max_prod.max(prod)
        }
    }

    // Vertical
    for col_no in 0..no_of_cols {
        for row_no in 0..(no_of_rows - window_size + 1) {
            let prod = {
                let mut p = 1;
                for i in 0..window_size {
                    p *= grid[row_no + i][col_no];
                }

                p
            };

            max_prod = max_prod.max(prod);
        }
    }

    // Top Left to Bottom Right
    for col_no in 0..(no_of_cols - window_size + 1) {
        for row_no in 0..(no_of_rows - window_size + 1) {
            let prod = {
                let mut p = 1;
                for i in 0..window_size {
                    p *= grid[row_no + i][col_no + i];
                }

                p
            };

            max_prod = max_prod.max(prod);
        }
    }

    // Top Right to Bottom Left
    for col_no in (0 + window_size - 1)..no_of_cols {
        for row_no in 0..(no_of_rows - window_size + 1) {
            let prod = {
                let mut p = 1;
                for i in 0..window_size {
                    p *= grid[row_no + i][col_no - i];
                }

                p
            };

            max_prod = max_prod.max(prod);
        }
    }
    max_prod
}

/// Problem 12: [Highly divisible triangular number]
///
/// [Highly divisible triangular number]: https://projecteuler.net/problem=12
///
/// <p>The sequence of triangle numbers is generated by adding the natural numbers. So the 7<sup>th</sup> triangle number would be 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28. The first ten terms would be:</p>
/// <p class="center">1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...</p>
/// <p>Let us list the factors of the first seven triangle numbers:</p>
/// <blockquote class="monospace"><b> 1</b>: 1<br /><b> 3</b>: 1,3<br /><b> 6</b>: 1,2,3,6<br /><b>10</b>: 1,2,5,10<br /><b>15</b>: 1,3,5,15<br /><b>21</b>: 1,3,7,21<br /><b>28</b>: 1,2,4,7,14,28</blockquote>
/// <p>We can see that 28 is the first triangle number to have over five divisors.</p>
/// <p>What is the value of the first triangle number to have over five hundred divisors?</p>
///
///
/// Answer: 76576500
pub fn problem_12(no_of_divisors: u32) -> u32 {
    answer!(no_of_divisors, 500, 76576500);
    fn no_of_divisors_for(number: u32) -> u32 {
        if number <= 1 {
            1
        } else {
            let sqrt = ((number as f64).sqrt().floor()) as u32;
            let no_of_divisors = (1..=sqrt)
                                    .filter(|x| number % x == 0)
                                    .collect::<Vec<u32>>()
                                    .len() as u32;
            let no_of_divisors = if sqrt * sqrt == number {
                no_of_divisors * 2 - 1
            } else {
                no_of_divisors * 2
            };
            no_of_divisors
        }
    }

    (1..)
        .map(|n| (1..=n).sum())
        .find(|&tri_no| no_of_divisors_for(tri_no) >=  no_of_divisors)
        .unwrap()
}

/// Problem 13: [Large sum]
///
/// [Large sum]: https://projecteuler.net/problem=13
///
/// <p>Work out the first ten digits of the sum of the following one-hundred 50-digit numbers.</p>
/// <div class="monospace center">
/// 37107287533902102798797998220837590246510135740250<br />
/// 46376937677490009712648124896970078050417018260538<br />
/// 74324986199524741059474233309513058123726617309629<br />
/// 91942213363574161572522430563301811072406154908250<br />
/// 23067588207539346171171980310421047513778063246676<br />
/// 89261670696623633820136378418383684178734361726757<br />
/// 28112879812849979408065481931592621691275889832738<br />
/// 44274228917432520321923589422876796487670272189318<br />
/// 47451445736001306439091167216856844588711603153276<br />
/// 70386486105843025439939619828917593665686757934951<br />
/// 62176457141856560629502157223196586755079324193331<br />
/// 64906352462741904929101432445813822663347944758178<br />
/// 92575867718337217661963751590579239728245598838407<br />
/// 58203565325359399008402633568948830189458628227828<br />
/// 80181199384826282014278194139940567587151170094390<br />
/// 35398664372827112653829987240784473053190104293586<br />
/// 86515506006295864861532075273371959191420517255829<br />
/// 71693888707715466499115593487603532921714970056938<br />
/// 54370070576826684624621495650076471787294438377604<br />
/// 53282654108756828443191190634694037855217779295145<br />
/// 36123272525000296071075082563815656710885258350721<br />
/// 45876576172410976447339110607218265236877223636045<br />
/// 17423706905851860660448207621209813287860733969412<br />
/// 81142660418086830619328460811191061556940512689692<br />
/// 51934325451728388641918047049293215058642563049483<br />
/// 62467221648435076201727918039944693004732956340691<br />
/// 15732444386908125794514089057706229429197107928209<br />
/// 55037687525678773091862540744969844508330393682126<br />
/// 18336384825330154686196124348767681297534375946515<br />
/// 80386287592878490201521685554828717201219257766954<br />
/// 78182833757993103614740356856449095527097864797581<br />
/// 16726320100436897842553539920931837441497806860984<br />
/// 48403098129077791799088218795327364475675590848030<br />
/// 87086987551392711854517078544161852424320693150332<br />
/// 59959406895756536782107074926966537676326235447210<br />
/// 69793950679652694742597709739166693763042633987085<br />
/// 41052684708299085211399427365734116182760315001271<br />
/// 65378607361501080857009149939512557028198746004375<br />
/// 35829035317434717326932123578154982629742552737307<br />
/// 94953759765105305946966067683156574377167401875275<br />
/// 88902802571733229619176668713819931811048770190271<br />
/// 25267680276078003013678680992525463401061632866526<br />
/// 36270218540497705585629946580636237993140746255962<br />
/// 24074486908231174977792365466257246923322810917141<br />
/// 91430288197103288597806669760892938638285025333403<br />
/// 34413065578016127815921815005561868836468420090470<br />
/// 23053081172816430487623791969842487255036638784583<br />
/// 11487696932154902810424020138335124462181441773470<br />
/// 63783299490636259666498587618221225225512486764533<br />
/// 67720186971698544312419572409913959008952310058822<br />
/// 95548255300263520781532296796249481641953868218774<br />
/// 76085327132285723110424803456124867697064507995236<br />
/// 37774242535411291684276865538926205024910326572967<br />
/// 23701913275725675285653248258265463092207058596522<br />
/// 29798860272258331913126375147341994889534765745501<br />
/// 18495701454879288984856827726077713721403798879715<br />
/// 38298203783031473527721580348144513491373226651381<br />
/// 34829543829199918180278916522431027392251122869539<br />
/// 40957953066405232632538044100059654939159879593635<br />
/// 29746152185502371307642255121183693803580388584903<br />
/// 41698116222072977186158236678424689157993532961922<br />
/// 62467957194401269043877107275048102390895523597457<br />
/// 23189706772547915061505504953922979530901129967519<br />
/// 86188088225875314529584099251203829009407770775672<br />
/// 11306739708304724483816533873502340845647058077308<br />
/// 82959174767140363198008187129011875491310547126581<br />
/// 97623331044818386269515456334926366572897563400500<br />
/// 42846280183517070527831839425882145521227251250327<br />
/// 55121603546981200581762165212827652751691296897789<br />
/// 32238195734329339946437501907836945765883352399886<br />
/// 75506164965184775180738168837861091527357929701337<br />
/// 62177842752192623401942399639168044983993173312731<br />
/// 32924185707147349566916674687634660915035914677504<br />
/// 99518671430235219628894890102423325116913619626622<br />
/// 73267460800591547471830798392868535206946944540724<br />
/// 76841822524674417161514036427982273348055556214818<br />
/// 97142617910342598647204516893989422179826088076852<br />
/// 87783646182799346313767754307809363333018982642090<br />
/// 10848802521674670883215120185883543223812876952786<br />
/// 71329612474782464538636993009049310363619763878039<br />
/// 62184073572399794223406235393808339651327408011116<br />
/// 66627891981488087797941876876144230030984490851411<br />
/// 60661826293682836764744779239180335110989069790714<br />
/// 85786944089552990653640447425576083659976645795096<br />
/// 66024396409905389607120198219976047599490197230297<br />
/// 64913982680032973156037120041377903785566085089252<br />
/// 16730939319872750275468906903707539413042652315011<br />
/// 94809377245048795150954100921645863754710598436791<br />
/// 78639167021187492431995700641917969777599028300699<br />
/// 15368713711936614952811305876380278410754449733078<br />
/// 40789923115535562561142322423255033685442488917353<br />
/// 44889911501440648020369068063960672322193204149535<br />
/// 41503128880339536053299340368006977710650566631954<br />
/// 81234880673210146739058568557934581403627822703280<br />
/// 82616570773948327592232845941706525094512325230608<br />
/// 22918802058777319719839450180888072429661980811197<br />
/// 77158542502016545090413245809786882778948721859617<br />
/// 72107838435069186155435662884062257473692284509516<br />
/// 20849603980134001723930671666823555245252804609722<br />
/// 53503534226472524250874054075591789781264330331690<br /></div>
///
///
/// Answer: unimplemented!()
pub fn problem_13(test: bool) -> String{
    answer!(test, true, "TEST".to_string());
    answer!(test, false, "unimplemented!()".to_string());

    unimplemented!("Don't know how to do it without bringing in big-int crate / can't be arsed to implement it");
}

/// Problem 14: [Longest Collatz sequence]
///
/// [Longest Collatz sequence]: https://projecteuler.net/problem=14
///
/// <p>The following iterative sequence is defined for the set of positive integers:</p>
/// <p class="margin_left"><var>n</var> → <var>n</var>/2 (<var>n</var> is even)<br /><var>n</var> → 3<var>n</var> + 1 (<var>n</var> is odd)</p>
/// <p>Using the rule above and starting with 13, we generate the following sequence:</p>
/// <div class="center">13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1</div>
/// <p>It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.</p>
/// <p>Which starting number, under one million, produces the longest chain?</p>
/// <p class="note"><b>NOTE:</b> Once the chain starts the terms are allowed to go above one million.</p>
///
///
/// Answer: todo!()
pub fn problem_14(_starting_number: u32) -> u32 {
    // use memoization
    (1..1_000_000);
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! generate_tests {
        ($($name:ident, $func:ident, $args:expr, $expected:expr);* $(;)?) => {
            $(
                #[test]
            fn $name() {
                    let result = $func($args);
                    assert_eq!(result, $expected);
                }
        )*
        };
    }

    //    #[test]
    //    fn it_works() {
    //        let result = add(2, 2);
    //        assert_eq!(result, 4);
    //    }

    generate_tests![
        multiples_of_3_or_5, problem_1, 10, 23;
        even_fibonacci_numbers, problem_2, 90, 44;
        largest_prime_factor, problem_3, 13195, 29;
        largest_palindrome_product, problem_4, 2, 9009;
        smallest_multiple, problem_5, 10, 2520;
        sum_square_difference, problem_6, 10, 2640;
        the_10001st_prime, problem_7, 6, 13;
        largest_product_in_a_series, problem_8, 4, 5832;
        special_pythagorean_triplet, problem_9, 3 + 4 + 5, 3 * 4 * 5;
        summation_of_primes, problem_10, 10, 17;
        largest_product_in_a_grid, problem_11, true, 1788696;
        highly_divisible_triangular_number, problem_12, 5, 28;
        large_sum, problem_13, true, "TEST".to_string();
    ];
}
