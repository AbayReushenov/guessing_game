extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
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

    // Преобразуем ввод пользователя в число
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Пожалуйста, введите число!");
            return;
        }
    };
    // Метод cmp() может быть вызван у чего-либо, что может сравниваться, и получает
    // ссылку на то, с чем мы хотим его сравнить. Результатом сравнения будет тип Ordering ,
    // который мы добавили выше. Мы используем оператор match для определения Ordering —
    // результата сравнения. Ordering — перечисление.
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Слишком маленькое!"),
        Ordering::Greater => println!("Слишком большое!"),
        Ordering::Equal => println!("Вы выиграли!"),
    }
}
