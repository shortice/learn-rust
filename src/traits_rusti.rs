/*
Итак... типажи.

Мне удобнее будет говорить о них как о интерфейсах, не привычно
пока что такую терминологию вводить.
*/

// TODO: сделать потом более лучше типажи.
// Именно с where и `+`
// Референс: https://doc.rust-lang.ru/book/ch10-02-traits.html

use std::fmt::{Display, Formatter};

// Структура
pub struct DanilaGoodMindset {}

// Структура
pub struct DanilaBadMindset {}

// Интерфейс... (типаж, брр)
pub trait Printable {
    fn print(&self) {
        println!("How to print? Dont know.");
    }
}

// Реализация интерфейса (типажа) для DanilaGoodMindset
impl Printable for DanilaGoodMindset {
    fn print(&self) {
        println!("All its good!");
    }
}

// Реализация интерфейса (типажа) для DanilaBadMindset
impl Printable for DanilaBadMindset {
    fn print(&self) {
        println!("All its bad...");
    }
}

// Сделаем теперь реализации для Display типажа!

impl Display for DanilaGoodMindset {
    // Что это тут за кракозябра <'_>?
    // А это время жизни ссылки в Rust...
    // ' - время жизни
    // _ - время жизни определяется компилятором Rust на
    // основе данных, которые у него есть.
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "All its good!")
    }
}

impl Display for DanilaBadMindset {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "All its bad...")
    }
}

// А тут самое интересное.
// Параметр object принимает ссылку на одну из структур
// которая реализует типаж Printable
// Использует ссылку, чтобы не копировать, хотя что тут
// копировать то :-)
fn print(object: &impl Printable) {
    object.print();
}

// Можно еще вот так сделать
fn print_other_variant<T: Printable>(object: &T) {
    object.print();
}

pub fn testi() {
    let m = DanilaGoodMindset {};
    let m1 = DanilaBadMindset {};

    print(&m);
    print(&m1);

    print_other_variant(&m);
    print_other_variant(&m1);

    // Или лучше! Мы сделали же поддержку типажа Display
    println!("Через println!: {}, {}", m, m1);
}
