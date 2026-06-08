use clock::Clock;

fn main() {
    // делимое = делитель * частное + остаток

    println!(" 25 %  24 = {}",  25 %  24); // 1
    println!(" 25 % -24 = {}",  25 % -24); // 1 потому что -24 * (-1 = 25 / -24) + 1 (остаток от деления) = 25

    println!("-25 %  24 = {}", -25 %  24); // -1 потому что 24 * (-1 = -25 / 24) + -1 (остаток от деления) = -25
    println!("-25 % -24 = {}", -25 % -24); // -1 потому что -24 * (1 = -25 / -24) + -1 (остаток от деления) = -25

    println!("-70 % 60 = {}", -70 % 60); // -10

    // let clock = Clock::new(14, 22);
    // let s = String::from(clock);
    // println!("now: {s}");

    // Если делимое и делитель оба положительные — разницы нет никакой.
    println!("61.div_euclid(60) = {}", (61 as i32).div_euclid(60)); // 1
    println!("50.div_euclid(60) = {}", (50 as i32).div_euclid(60)); // 0

    println!("50.rem_euclid(60) = {}", (50 as i32).rem_euclid(60)); // 50
    println!("61.rem_euclid(60) = {}", (61 as i32).rem_euclid(60)); // 1


    println!("-61.div_euclid(60) = {}", (-61 as i32).div_euclid(60)); // -2
    println!("-61.rem_euclid(60) = {}", (-61 as i32).rem_euclid(60)); // 59
}
