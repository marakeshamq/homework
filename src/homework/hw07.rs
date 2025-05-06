fn swap_letter_case(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_uppercase() || c.is_uppercase() {
                c.to_lowercase().to_string()
            } else {
                c.to_uppercase().to_string()
            }
        })
        .collect()
}

fn main() {
    let samples = [
        "DATA",
        "Hello World!!!",
        "Tell me Your story",
        "Error 404 !$#@",
        "Вареники - їжа богів",
    ];

    for original in samples.iter() {
        let transformed = swap_letter_case(original);
        println!("ORIG {:25} => NEW {:25}", original, transformed);
    }
}
