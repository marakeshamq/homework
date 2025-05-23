fn gray(len: u8) -> Vec<String> {
    let size = 1 << len;
    let mut result = Vec::with_capacity(size);

    for i in 0..size {
        let gray_code = i ^ (i >> 1);
        let s = format!("{:0len$b}", gray_code, len = len as usize);
        result.push(s);
    }

    result
}

fn main() {
    let тестові_дані = [
        (0, vec!("")),
        (1, vec!("0", "1")),
        (2, vec!("00", "01", "11", "10")),
        (3, vec![
            "000", "001", "011", "010",
            "110", "111", "101", "100",
        ]),
        (4, vec![
            "0000", "0001", "0011", "0010",
            "0110", "0111", "0101", "0100",
            "1100", "1101", "1111", "1110",
            "1010", "1011", "1001", "1000",
        ]),
    ];

    for (n, expected) in &тестові_дані {
        let got = gray(*n);
        assert_eq!(&got, expected, "Помилка для довжини {}", n);
    }
    println!("Усі тести пройшли успішно!");
}
