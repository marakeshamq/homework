fn greatest_common_divisor(a: u32, b: u32) -> u32 {
    let (min_val, max_val) = if a < b { (a, b) } else { (b, a) };

    if min_val == 0 {
        return 0;
    }
    if min_val == max_val {
        return min_val;
    }
    if max_val % min_val == 0 {
        return min_val;
    }

    for divisor in (1..=min_val / 2).rev() {
        if a % divisor == 0 && b % divisor == 0 {
            return divisor;
        }
    }

    1
}

fn test_gcd() {
    println!("=== Тестування GCD-функції ===");
    let test_cases = [
        (12, 17),
        (3, 27),
        (44, 57),
        (9, 3),
        (56, 56),
        (6790, 6797),
        (10, 2),
        (1000000, 12312),
        (2, 10),
        (5, 10),
        (15, 20),
    ];

    for &(x, y) in &test_cases {
        let result = greatest_common_divisor(x, y);
        println!("НСД({x}, {y}) = {result}");
    }
}

fn main() {
    const A: u32 = 62;
    const B: u32 = 64;

    println!("НСД для {A} і {B} дорівнює {}", greatest_common_divisor(A, B));

    test_gcd();
}
