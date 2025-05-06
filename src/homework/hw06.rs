fn render_tree_layer(current: u32, total: u32) {
    if current == total {
        return;
    }

    let next = current + 1;

    for row in 1..=next {
        let padding = total - row;
        let stars = row * 2 - 1;

        print!("{:width$}", "", width = padding as usize);
        for _ in 0..stars {
            print!("*");
        }
        println!();
    }

    render_tree_layer(next, total);
}

fn show_tree(layers: u32) {
    render_tree_layer(0, layers);
}

fn main() {
    let total_layers = 4;
    show_tree(total_layers);
}
