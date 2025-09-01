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
```

```bash
$ cargo build
   Compiling libc v0.2.175
   Compiling cfg-if v1.0.3
   Compiling zerocopy v0.8.26
   Compiling getrandom v0.3.3
   Compiling rand_core v0.9.3
   Compiling ppv-lite86 v0.2.21
   Compiling rand_chacha v0.9.0
   Compiling rand v0.9.2
   Compiling guessing_game v0.1.0 (/home/aaaaa/rust/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.59s
```
# page_28
