use std::collections::HashMap;

pub fn tester() {
    let mut v: Vec<i64> = Vec::new();

    // можно еще так
    // let v = vec![1, 2, 3]
    // но тут тип данных будет i32

    v.push(10);
    v.push(-1);
    // Здесь требуется mut. Без него никак.
    // Полагаю, что тут все из-за того, что память инициализируется
    // в куче, а так как изменять память можно по логике Rust только
    // с указанием `mut`, то вполне логично.

    // Если бы мы просто сделали
    // for item in v { ... }
    // То тут скорее всего было бы копирование данных
    // В данном случае лучше сделать
    // for item in &v { ... }

    for item in v.iter().enumerate() {
        println!("{0} - {1}", item.0, item.1);
    }

    // Доступ по индексу

    let elem = &v[0]; // Можно и v[0]
    // Но в данном случае тогда будет копирование значения
    // Поэтому используем &, чтобы нам выдавалась ссылка

    println!("First element: {}", elem);

    // Но лучше всего использовать так

    let good_elem = v.get(0);

    if good_elem.is_none() {
        println!("Element at 0 index not found.");
    } else {
        println!("Element: {}", good_elem.unwrap());
    }

    let good_elem = v.get(100);

    if good_elem.is_none() {
        println!("Element at 0 index not found.");
    } else {
        println!("Element: {}", good_elem.unwrap());
        // Паники тут не будет, потому что мы сравнимаем
        // есть ли этот элемент вообще в массиве
    }

    // Строки

    let str = String::from("Hello ");
    let str2 = String::from("world!");

    // str больше не будет действительной ссылкой после
    // этой строчки.
    // Почему?
    // Мы передаем владение str в владение m, это особенность
    // Rust. А str2 будет действителен и после, так как
    // мы передаем ССЫЛКУ на строку.
    let m = str + &str2;

    let str = "Hello "; // Вернем обратно наш Hello

    println!("Sum result and the str2 var value: {}. {}", m, str2);

    // Можно еще вот так сделать через макрос
    println!("Result with format!: {}", format!("{str}{str2}"));

    // При этом format! не забирает в владение строки
    // так как использует ссылки.
    println!("Another format after format!: {}{}", str, str2);

    let str = "Hello";

    // Немного о том, почему нельзя взять и сделать так:
    // let m = &str[1]; // Так не сработает, Rust запрещает такое
    // но можно сделать так:
    let m = str.bytes().next(); // Берем тут лишь ПЕРВУЮ букву
    println!("Char at 0 index of string: {}", m.unwrap());
    // 72? Да, так и должно быть.

    // Таблица ascii (из https://www.ascii-code.com/):
    // 72 110	48	01001000	H	&#72;	 	Uppercase H

    // Я бы тут сказал подробнее, но... умолчу детали.
    // Если нужно больше - гуглите как хранятся строки
    // в виде массива, где хранятся не символы, как вам
    // покажется, а именно элементы по таблице ASCII.
    // И да, в данном случае тут тип u8, это 1 байт, есть символы
    // которые занимают более одного байта.

    // Ну и теперь HashMap!

    // HashMap.

    let mut ll: HashMap<String, String> = HashMap::new();

    ll.insert(String::from("Halo"), String::from("Дарова"));

    println!("{}", ll.get("Halo").unwrap());
}
