### Сломанный FizzBuzz

Цель:

-Применить на практике утилиты fmt и clippy  
-Научиться использовать систему сборки cargo  
-Установить и настроить среду разработки  

Результатом является:

-Отформатированный и исправленый код программы FizzBuzz  
-Установленная среда разработки  

```
$ cargo run
   Compiling fizzbuzz v0.1.0 ()
error: expected type, found `"FizzBuzz"`
 --> src/main.rs:8:35
  |
8 |             (0, 0) => String:from("FizzBuzz"), (0, _) => String::from("Fizz"),
  |                    --       -     ^^^^^^^^^^ expected type
  |                    |        |
  |                    |        help: maybe write a path separator here: `::`
  |                    while parsing the `match` arm starting here

warning: unreachable statement
  --> src/main.rs:11:9
   |
7  |           let s = match (i % 3, i % 5) {
   |  _________________-
8  | |             (0, 0) => String:from("FizzBuzz"), (0, _) => String::from("Fizz"),
9  | |             (_, 0) => String::from("Buzz"), (_, _) => format!("{}", i)
10 | |         };
   | |_________- any code following this expression is unreachable
11 |           Result.push(s);
   |           ^^^^^^^^^^^^^^^ unreachable statement
   |
   = note: `#[warn(unreachable_code)]` on by default

warning: `fizzbuzz` (bin "fizzbuzz") generated 1 warning
error: could not compile `fizzbuzz` due to previous error; 1 warning emitted
```
```
$ cargo clippy
    Checking fizzbuzz v0.1.0 ()
warning: writing `&Vec<_>` instead of `&[_]` involves one more reference and cannot be used with non-Vec-based slices
  --> src/main.rs:16:25
   |
16 | fn print_result(result: &Vec<String>) {
   |                         ^^^^^^^^^^^^ help: change this to: `&[String]`
   |
   = note: `#[warn(clippy::ptr_arg)]` on by default
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#ptr_arg

warning: `fizzbuzz` (bin "fizzbuzz") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.25s
```
```
$ cargo fmt -- --check
Diff in main.rs at line 5:
     let mut result = Vec::with_capacity(COUNT);
     for i in 1..=COUNT {
         let s = match (i % 3, i % 5) {
-            (0, 0) => String::from("FizzBuzz"), (0, _) => String::from("Fizz"),
-            (_, 0) => String::from("Buzz"), (_, _) => format!("{}", i)
+            (0, 0) => String::from("FizzBuzz"),
+            (0, _) => String::from("Fizz"),
+            (_, 0) => String::from("Buzz"),
+            (_, _) => format!("{}", i),
         };
         result.push(s);
     }
```
```
$ cargo run
   Compiling fizzbuzz v0.1.0 ()
    Finished dev [unoptimized + debuginfo] target(s) in 0.24s
     Running `target/debug/fizzbuzz`
```