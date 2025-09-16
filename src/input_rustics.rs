use std::io; // Системный ввод/вывод
use std::time; // Для получения времени
use std::cmp::Ordering; // Для сравнений

// Использует библиотеку io из std библиотеки.

// Генератор рандома простой
pub fn random(max: u64) -> u64 {
    let duraction = time::SystemTime::now().duration_since(
        time::UNIX_EPOCH
    );

    let mut seconds = duraction
    .expect("Get Time failed.")
    .as_secs();

    seconds ^= seconds << 13;
    seconds ^= seconds >> 17;
    seconds ^= seconds << 5;

    seconds % max
}

pub fn input_rustics() {
    println!("Guess the number! 0 - 1000.");

    println!("Please input your guess.");

    let secret_number = random(1000);

    // Создаем пустую строку, которой ВЛАДЕЕМ
    // А еще мы тут создаем переменную через let. 
    // mut используем, чтобы переменная была изменяемой, 
    // по умолчанию в Rust все неизменяемо.
    let mut guess = String::new(); 

    io::stdin() // Поток ввода
        .read_line(&mut guess) 
        // Читаем строку. &guess - НЕИЗМЕНЯЕМАЯ ссылка на объект
        // guess, когда как с mut она будет изменяемой.
        .expect("Failed to read line"); 
    // Если не получилось, то выводим Failed to read line

    // P.S Результата read_line - перечислением Result.
    // В нем хранится либо результат функции, либо Error.

    // trim удаляет \n и пробелы с строки
    // parse приводит строку к типу u64, который му тут указали.
    // except разумеется, если произошла ошибка.
    let guess: u64 = guess.trim().parse()
    .expect("Please input integer!");

    // cmp сравниваем число, тут объяснять ничего не надо
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

    println!("You typed {x}. Actual value {y}.", 
        x = guess,
        y = secret_number
    );
}