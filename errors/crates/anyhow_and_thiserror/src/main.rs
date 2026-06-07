use anyhow::Context;

fn main() {
    println!("Hello, world!");
}

fn _foo(a: i32, b: i32) -> Result<i32, Box<dyn std::error::Error>> {

    let _s1 = match _sum_std(2, 2) {
        Ok(s) => s,
        Err(err) => return Err(format!("Функция sum({}, {}) вернула ошибку: {}", a, b, err).into()),
    };

    // Кратко, но
    // теряем возможность задать своё сообщение об ошибке:
    let _s2 = _sum_std(a, b)
        //.inspect_err(|e| eprintln!("Got error: {e}"))  // смотрим на ошибку
        
        // полный доступ к ошибке
        //.map_err(|err| Box::<dyn std::error::Error>::from(format!("Функция sum({}, {}) вернула ошибку: {}", a, b, err)))
        //.map_err(|err| -> Box<dyn std::error::Error> { format!("Функция sum({}, {}) вернула ошибку: {}", a, b, err).into() })

        ?;

    Ok(_s2)
}

fn _sum_std(a: i32, b: i32) -> Result<i32, Box<dyn std::error::Error>> {
    if a != 2 {
        return Err(format!("a = {}, а должно быть 2", a).into())
    }
    Ok(a + b)
}

fn _bar(a: i32, b: i32) -> anyhow::Result<i32> {

    let _s1 = match _sum_anyhow(2, 2) {
        Ok(s) => s,
        Err(err) => return Err(anyhow::anyhow!("Функция sum({}, {}) вернула ошибку: {}", a, b, err)),
    };

    // Кратко.
    // Есть возможность задать своё сообщение об ошибке.
    let _s2 = _sum_anyhow(a, b)
        //.inspect_err(|e| eprintln!("Got error: {e}"))  // смотрим на ошибку
        
        //.map_err(|err| anyhow::anyhow!("Функция sum({}, {}) вернула ошибку: {}", a, b, err))  // полный доступ к ошибке

        //.context(format!("Функция sum({}, {}) вернула ошибку", a, b))  // статически
        .with_context(|| format!("Функция sum({}, {}) вернула ошибку", a, b))  // динамически

        ?;

    Ok(_s2)
}

fn _sum_anyhow(a: i32, b: i32) -> anyhow::Result<i32> {
    if a != 2 {
        return Err(anyhow::anyhow!("a = {}, а должно быть 2", a))
    }
    Ok(a + b)
}
