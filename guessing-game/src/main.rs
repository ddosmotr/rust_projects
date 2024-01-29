use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Угадайте число!");
    println!("Секретное число {}", secret_number);

    loop {
        println!("Пожалуйста введите свою догадку");

        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Не получилос прочитать строку");

//         let guess: u32 = guess.trim().parse().expect("Введите, пожалуйста, число");

        // take a look, this is match
        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Слишком малое число"),
            Ordering::Greater => println!("Слишком большое число"),
            Ordering::Equal => {
                println!("Вы выиграли");
                break;
            }
        }
    }
}
