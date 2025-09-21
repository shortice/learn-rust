/*
Здесь библиотека с тестами. 

Интересная штука про тесты:
cargo test -- --test-threads=1 # 1 - это кол-во потоков

Можно тесты выполнять в многопотоке, но только тогда
когда эти тесты не зависят друг от друга и не имеют
общее состояние.
*/

pub mod rustics {

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

}

// О макросах здесь
// assert_eq! проверяет равенство
// assert_ne! проверяет неравенство
// assert! уже проверяет true или false, если false - то будет panic
#[cfg(test)]
mod tests {
    use super::rustics::*;

    #[test] // TODO: Это кстати атрибуты функций. Надо узнать лучше.
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic] // Этот тест будет успешным, если будет panic
    fn it_not_works_but_ok() {
        assert_eq!(add(2, 2), 5);
    }

    #[test]
    fn test_with_result() -> Result<(), String> {
        if add(2, 2) == 4 {
            Ok(())
        } else {
            Err(
                String::from("2 + 2 is always 4")
            )
        }
    }
}
