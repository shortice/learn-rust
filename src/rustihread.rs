// rusti[t]hread ;D
use std::thread::{self, sleep};
use std::time::Duration;
// А тут что каналы делают? О как...
use std::sync::mpsc::channel;

pub fn test() {
    let th = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    th.join().unwrap();
    // Блокируем выполнение основного потока до тех пор
    // пока не будет выполнен поток, который мы создали

    let v = vec![1, 2, 3];

    // Без move тут не выйдет.
    // Ибо у потока совсем другая область видимости.
    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    // println!("{:?}", v);
    // ^ Не выйдет, потому что мы явно переместили v в владение к замыканию
    // (аноним. функции).

    handle.join().unwrap();

    // А теперь потоки!
    // Это средство для обмена информации между потоками.
    let (tx, rx) = channel::<String>();

    // Передаем через move владение переменной tx.
    thread::spawn(move || {
        sleep(Duration::from_secs(1));
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    println!("{}", rx.recv().unwrap());

    // Можно и иначе сделать все, например сделать отправку сообщений сразу 2 потокам:
    // https://doc.rust-lang.ru/book/ch16-02-message-passing.html#%D0%A1%D0%BE%D0%B7%D0%B4%D0%B0%D0%BD%D0%B8%D0%B5-%D0%BD%D0%B5%D1%81%D0%BA%D0%BE%D0%BB%D1%8C%D0%BA%D0%B8%D1%85-%D0%BE%D1%82%D0%BF%D1%80%D0%B0%D0%B2%D0%B8%D1%82%D0%B5%D0%BB%D0%B5%D0%B9-%D0%BF%D1%83%D1%82%D1%91%D0%BC-%D0%BA%D0%BB%D0%BE%D0%BD%D0%B8%D1%80%D0%BE%D0%B2%D0%B0%D0%BD%D0%B8%D1%8F-%D0%BF%D0%B5%D1%80%D0%B5%D0%B4%D0%B0%D1%82%D1%87%D0%B8%D0%BA%D0%B0
}
