fn print_rhombus(width: i32, height: i32) {
    let ratio: f32 = width as f32 / height as f32;

    for row in 0..height {
        for col in 0..width {
            let symbol: char;

            let dy = (row - height / 2).abs();
            let dx = ((col - width / 2).abs() as f32 / ratio) as i32;

            if dx + dy < (height as f32 / 2.0).round() as i32 {
                symbol = '*';
            } else {
                symbol = ' ';
            }

            print!("{symbol}");
        }
        println!();
    }
}

fn main() {
    const WIDTH: i32 = 11;
    const HEIGHT: i32 = 11;

    print_rhombus(WIDTH, HEIGHT);
}
