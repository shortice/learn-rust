// Обощенные типы данных.

fn minimum<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut min = &list[0];

    for item in list {
        if item < min {
            min = item;
        }
    }

    min
}

pub fn test_min() {
    let m = vec![1, 2, -1];

    println!("The minimum value of m array: {}", minimum(&m));
}
