fn is_palindrome(mut x: u32) -> bool {
    if x < 10 {
        return true;
    }

    let mut reversed = 0;
    let original = x;

    while x > 0 {
        let digit = x % 10;
        reversed = reversed * 10 + digit;
        x /= 10;
    }

    reversed == original
}

fn main() {
    let tests = [
        (123,    false),
        (121,    true),
        (1231,   false),
        (19391,  true),
        (1,      true),
        (785587, true),
        (374,    false),
        (77,     true),
        (300103, false),
    ];

    tests.iter().for_each(|(n, exp)| {
        assert_eq!(is_palindrome(*n), *exp);
        println!("{n:6}: {}", is_palindrome(*n));
    });
}
