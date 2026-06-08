use anyhow::Context;
use std::io::Read;

fn main() {
    println!("Hello, world!");
}




// anyhow




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




// thiserror




#[derive(Debug)]
pub enum DivisionStdError {
    DividerIsZeroError { a: f32, b: f32, },
    DividerIsTooBigError(f32),

    SourceError { source: std::io::Error },
    
    WrappedError(std::num::ParseIntError),
    WrappedError2(std::num::ParseIntError),
}

impl std::fmt::Display for DivisionStdError {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DivisionStdError::DividerIsZeroError { a, b } => {
                write!(f, "could not divide {} / {}", a, b)
            },
            DivisionStdError::DividerIsTooBigError(d) => {
                write!(f, "divider is too big {d}")
            },
            
            DivisionStdError::SourceError { source} => {
                write!(f, "source error {}", source)
            },
            
            DivisionStdError::WrappedError (e) => {
                write!(f, "wrapped error {}", e)
            },
            DivisionStdError::WrappedError2 (e) => {
                write!(f, "wrapped error {}", e)
            },
        }
    }

}

impl std::error::Error for DivisionStdError {
    
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DivisionStdError::DividerIsZeroError {..} => None,
            DivisionStdError::DividerIsTooBigError(_) => None,
            
            DivisionStdError::SourceError { source } => Some(source),
            
            DivisionStdError::WrappedError(e) => Some(e),
            DivisionStdError::WrappedError2(e) => Some(e),
        }
    }

}

fn _div_std(a: f32, b: f32) -> Result<f32, DivisionStdError> {
    match b {
        0.0 => return Err(DivisionStdError::DividerIsZeroError { a, b }),
        100500. => return Err(DivisionStdError::DividerIsTooBigError(b)),
        _ => ()
    }

    let mut contents = String::new();
    std::fs::File::open("").unwrap().read_to_string(&mut contents)
        .map_err(|e| DivisionStdError::SourceError {
            source: e,
        })?;
    let _port: i32 = contents.trim().parse()
        .map_err(DivisionStdError::WrappedError)?;

    Ok(a / b)
}




#[derive(thiserror::Error, Debug)]
pub enum DivisionThisError {
    
    #[error("could not divide {a} / {b}")]
    DividerIsZeroError { a: f32, b: f32, },

    #[error("divider is too big {0}")]
    DividerIsTooBigError(f32),

    #[error("source error {source}")]
    SourceError { 
        
        #[source]
        source: std::io::Error 
    },

    #[error("wrapped error {0}")]
    WrappedError(#[from] std::num::ParseIntError),
    
    //#[error("wrapped error {0}")]
    //WrappedError2(#[from] std::num::ParseIntError),
    // conflicting implementations of trait `From<ParseIntError>` for type `DivisionThisError`

    #[error("wrapped error {0}")]
    WrappedError3(#[from] std::net::AddrParseError),
}

fn _div_thiserror(a: f32, b: f32) -> Result<f32, DivisionThisError> {
    match b {
        0.0 => return Err(DivisionThisError::DividerIsZeroError { a, b }),
        100500. => return Err(DivisionThisError::DividerIsTooBigError(b)),
        _ => ()
    }

    let mut contents = String::new();
    std::fs::File::open("").unwrap().read_to_string(&mut contents)
        .map_err(|e| DivisionThisError::SourceError {
            source: e,
        })?;
    let _port: i32 = contents.trim().parse()
        .map_err(DivisionThisError::WrappedError)?;

    Ok(a / b)
}
