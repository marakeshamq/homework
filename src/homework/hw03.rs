const WIDTH: usize = 7;
const HEIGHT: usize = 7;

pub fn draw_envelope() {
    let mut output = String::new();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if y == 0 || y == HEIGHT - 1 {
                output.push('#'); 
            } else if x == 0 || x == WIDTH - 1 {
                output.push('#'); 
            } else if x == y {
                output.push('\\'); 
            } else if x == WIDTH - y - 1 {
                output.push('/'); 
            } else {
                output.push(' '); 
            }
        }
        output.push('\n');
    }

    print!("{}", output);
}