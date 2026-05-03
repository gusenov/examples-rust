use std::io;

fn main() {
    my();
    claude();
}

fn my() {
    let u : u8 = 255;
    println!("u = {u}");

    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    // 256

    // Rust uses the term panicking when a program exits with an error

    // thread 'main' (324258) panicked at src/main.rs:10:35:
    // Please type a number!: ParseIntError { kind: PosOverflow }
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    let u : u8 = s.trim().parse().expect("Please type a number!");
    println!("u = {u}");
}

fn claude() {
    let value: u8 = 255;

    // wrapping_* — когда поведение с обёрткой является намеренным (например, хэш-функции, работа с битами).
    // wrapping_*: wraps around on overflow
    let wrapping_result = value.wrapping_add(1);
    println!("wrapping_add: {}", wrapping_result); // 0

    // checked_* — когда переполнение недопустимо и нужно явно обработать ошибку (например, финансовые вычисления).
    // checked_*: returns None on overflow
    let checked_result = value.checked_add(1);
    match checked_result {
        Some(n) => println!("checked_add: {}", n),
        None    => println!("checked_add: overflow detected!"), // this runs
    }

    // overflowing_* — когда нужно и получить результат, и знать, было ли переполнение (редко используется напрямую).
    // overflowing_*: returns (value, did_overflow)
    let (overflowing_result, did_overflow) = value.overflowing_add(1);
    println!("overflowing_add: value = {}, overflowed = {}", overflowing_result, did_overflow);
    // value = 0, overflowed = true

    // saturating_* — когда нужно «ограничить» значение, не допуская выхода за пределы (например, уровень здоровья в игре: не может упасть ниже 0).
    // saturating_*: clamps to max/min
    let saturating_result = value.saturating_add(1);
    println!("saturating_add: {}", saturating_result); // 255 (stays at max)
}
// wrapping_add: 0
// checked_add: overflow detected!
// overflowing_add: value = 0, overflowed = true
// saturating_add: 255