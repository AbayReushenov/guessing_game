# Создание нового проекта
Создать структуру директорий и Cargo.toml
Cargo может сделать это за нас
```bash
$ cd ~/projects
$ cargo new guessing_game --bin
$ cd guessing_game
```
# При помощи флага --bin , мы указали что хотим создать исполняемый файл, а не библиотеку.

```bash
cargo build
```

```bash
$ ./target/debug/guessing_game
```
>>Hello, world!


# Cargo: run
cargo run
похожа на cargo build ,
но после завершения компиляции, она запускает
получившийся исполняемый файл:

```bash
$ cargo run
```
>> Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
>> Running `target/debug/guessing_game`
>> Hello, world!

# добавили контейнер rand
```toml
[dependencies]

rand="0.9.2"

# Настройка VS Code с rust-analyzer

Обязательные расширения:

rust-analyzer - основное расширение для Rust

Even Better TOML - подсветка синтаксиса для Cargo.toml

Crates - автодополнение версий crates в Cargo.toml

Дополнительные расширения:

Error Lens - визуальное выделение ошибок

CodeLLDB - отладчик для Rust
