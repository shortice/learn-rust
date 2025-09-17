mod hello_rustics; // mod как понял импортировать модуль
mod input_rustics;
mod struct_rustics;
mod enum_rustics;

// fn - функция
// main - точка входа в программу
fn main() {
    hello_rustics::hello_world();
    // *:: - пространство имен?

    input_rustics::input_rustics();

    struct_rustics::struct_rustics();
    enum_rustics::enum_rustics();
}

