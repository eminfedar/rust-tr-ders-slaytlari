// fn hello(name: &str) -> String
// - fonksiyonu “Hello {name}!” metnini döndürecek
// fn double_if_even(num: i32) -> i32
// - fonksiyonu çift sayı verilirse 2 katını, tek sayı verilirse aynısını döndürecek.
// fn multiply_pi(num: f32) -> f32
// - fonksiyonu verilen sayıyı Pi sayısı ile çarpıp geri döndürecek.

use std::f32::consts;

fn hello(name: &str) -> String {
    format!("Hello {name}!")
}

fn double_if_even(num: i32) -> i32 {
    if num % 2 == 0 {
        num * 2
    } else {
        num
    }
}

fn multiply_pi(num: f32) -> f32 {
    num * consts::PI
}

fn main() {
    println!("{}", hello("Emin"));

    println!("{}", double_if_even(10));
    println!("{}", double_if_even(5));

    println!("{}", multiply_pi(2.0));
}
