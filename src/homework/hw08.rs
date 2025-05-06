fn check_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }

    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }

    true
}

fn main() {
    println!(" Список простих чисел до 30:");
    for num in 0..30 {
        let mark = if check_prime(num) { "так" } else { "ні " };
        println!("{:2}: просте? → {}", num, mark);
    }
}
