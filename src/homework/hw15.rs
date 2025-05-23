use std::collections::HashMap;

fn вивести_рядок(рядок: &str, унікальні: &HashMap<char, u8>) {
    рядок.chars().for_each(|символ| {
        if let Some(число) = унікальні.get(&символ) {
            print!("{}", число);
        } else {
            print!(" ");
        }
    });
    println!();
}

fn знайти_і_вивести(масив: &[String]) {
    let mut унікальні: HashMap<char, u8> = HashMap::new();
    let mut лічильник: u8 = 1;

    for рядок in масив.iter() {
        for символ in рядок.chars() {
            if символ != ' ' && !унікальні.contains_key(&символ) {
                унікальні.insert(символ, лічильник);
                лічильник += 1;
            }
        }
    }

    вивести_рядок(&масив[0], &унікальні);
    вивести_рядок(&масив[1], &унікальні);

    for _ in 0..масив[0].len() {
        print!("-");
    }
    println!();

    вивести_рядок(&масив[2], &унікальні);
}

fn main() {
    let тестові_дані: [String; 3] = [
        String::from(" muxa"),
        String::from("x   a"),
        String::from(" slon"),
    ];

    знайти_і_вивести(&тестові_дані);

}
