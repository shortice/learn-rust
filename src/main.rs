mod closures_rusti;
mod collections;
mod enum_rustics;
mod hello_rustics;
mod input_rustics;
mod iter_rustics;
mod lifetime_rusti;
mod ok_ne_ok_rusti;
mod struct_rustics;
mod template_rusti;
mod traits_rusti;
mod use_module_rusti;
// mod как понял импортировать модуль

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
    lifetime_rusti::the_time();

    use_module_rusti::test();
    closures_rusti::test();
    iter_rustics::test();
}
