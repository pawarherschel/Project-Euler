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
    ];
}
