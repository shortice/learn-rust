pub fn test() {
    let m = vec![1, 2, 0, -23, 22, -1, -6, -1, 0, 20];

    /*
    Итак. В Rust итераторы ленивые. То есть, если мы сделаем
    какую-либо операцию например map в нем, то мы ничего не получим.

    Чтобы получить что-либо, придется использовать явно итераторы,
    это особенность языка.
    */

    println!("{:?}", m);
    let m: Vec<i32> = m.iter().map(|value| value * 10).collect();
    println!("{:?}", m);
    // Разница между iter и into_iter?
    /*
    А вот в чем.

    Смотрим сигнатуры функций.
    У iter &self.
    У into_iter просто self.

    То есть, первый берет по ссылке элемент итерирует, второй же
    по владению.

    И поэтому после into_iter:
    The vector connot used after calling this.
    */
    let ml: Vec<i32> = m.into_iter().filter(|value| *value > 100).collect();
    println!("{:?}", ml);
    // println!("{:?}", m); // <- тут будет ошибка ^^^
}
