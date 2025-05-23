extern crate rand;
use rand::Rng;
use colored::*;

fn знайти_індекс_мінімальної_пари(дані: &[u32]) -> Option<usize> {
    дані.windows(2)
        .enumerate()
        .min_by_key(|(_, пара)| пара[0] + пара[1])
        .map(|(індекс, _)| індекс)
}

fn згенерувати_випадковий_вектор(розмір: usize) -> Vec<u32> {
    let mut генератор = rand::thread_rng();
    (0..розмір).map(|_| генератор.gen_range(10..=99)).collect()
}

fn main() {
    const РОЗМІР: usize = 20;
    let числа = згенерувати_випадковий_вектор(РОЗМІР);

    println!("{}", "== Згенерований вектор ==".yellow().bold());
    println!("Індекси : {}", (0..РОЗМІР).map(|i| format!("{:2}", i)).collect::<Vec<_>>().join(", "));
    println!("Значення: {}", числа.iter().map(|n| format!("{:2}", n)).collect::<Vec<_>>().join(", "));

    if let Some(мін_індекс) = знайти_індекс_мінімальної_пари(&числа) {
        let сума = числа[мін_індекс] + числа[мін_індекс + 1];
        let стрілка = format!(
            "{}{}",
            " ".repeat(мін_індекс * 4),
            "^^----^^".green()
        );

        println!("\n{}", "== Мінімальна пара сусідів ==".cyan().bold());
        println!("{стрілка}");
        println!("Сума: {} (індекси {} і {})", сума.to_string().bright_green(), мін_індекс, мін_індекс + 1);
    } else {
        println!("{}", "Недостатньо елементів для аналізу.".red());
    }
}
