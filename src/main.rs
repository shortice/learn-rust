mod hello_rustics; // mod как понял импортировать модуль
mod input_rustics;
mod struct_rustics;
mod enum_rustics;
mod ok_ne_ok_rusti;
mod collections;
mod template_rusti;
mod traits_rusti;

// fn - функция
// main - точка входа в программу
fn main() {
    hello_rustics::hello_world();
    // *:: - пространство имен?

    input_rustics::input_rustics();

    struct_rustics::struct_rustics();
    enum_rustics::enum_rustics();

    ok_ne_ok_rusti::is_okay(0);
    ok_ne_ok_rusti::is_okay(100);

    collections::tester();
    template_rusti::test_min();
    traits_rusti::testi();
}

