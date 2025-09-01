extern crate rand;
use rand::Rng;
use std::io;
fn main() {
    println!("Угадайте число!");
    let secret_number = rand::rng().random_range(1..101);
    println!("Загаданное число: {}", secret_number);
    println!("Пожалуйста, введите предположение.");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .ok()
        .expect("Не удалось прочитать строку");
    println!("Ваша попытка: {}", guess);
}
