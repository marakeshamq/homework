fn rotate_string(s: &str, shift: isize) -> String {
    let len = s.len();

    if len == 0 {
        return String::new();
    }

    // Нормализуем сдвиг в пределах длины строки
    let shift = ((shift % len as isize) + len as isize) % len as isize;
    let shift = shift as usize;

    // Разделяем строку на две части и переставляем их местами
    let mut chars: Vec<char> = s.chars().collect();
    chars.rotate_right(shift);
    chars.into_iter().collect()
}

fn main() {
    let source = "abcdefgh";
    let test_cases = [
        (0,  "abcdefgh"),
        (8,  "abcdefgh"),
        (-8, "abcdefgh"),
        (1,  "habcdefg"),
        (2,  "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10,"cdefghab"),
    ];

    for (shift, _) in &test_cases {
        let rotated = rotate_string(source, *shift);
        println!("{shift:3}: {rotated}");
    }
}
