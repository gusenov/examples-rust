use std::{
    fs::File,
    io::Read,
    net::{AddrParseError, Ipv4Addr},
    num::ParseIntError,
    fmt,
};
use thiserror::Error;
use http::StatusCode;

// The traditional approach is to define an enum with a variant for each error case:
pub enum _AddressErrorV1 {
    FileOpenError,
    FileUnreadableError,
    PortParseError,
    PortTooLowError,
    LocalHostError,
}
// But this loses all the underlying error information. 

// We can wrap the original errors to preserve them:
#[derive(Debug)]
pub enum AddressErrorV2 {
    FileOpenError { filename: String, source: std::io::Error },
    FileUnreadableError { filename: String, source: std::io::Error },
    PortParseError(ParseIntError),
    PortTooLowError(i32),
    LocalHostError(AddrParseError),
}

// Now we need to implement the Display trait so that we can actually show the error to the caller. 
impl fmt::Display for AddressErrorV2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AddressErrorV2::FileOpenError { filename, source } => {
                write!(f, "could not open file '{}': {}", filename, source)
            }
            AddressErrorV2::FileUnreadableError { filename, source } => {
                write!(f, "could not read file '{}': {}", filename, source)
            }
            AddressErrorV2::PortParseError(e) => {
                write!(f, "could not parse port number: {}", e)
            }
            AddressErrorV2::PortTooLowError(port) => {
                write!(f, "expected a port over 1000, but got {}", port)
            }
            AddressErrorV2::LocalHostError(e) => {
                write!(f, "invalid localhost address: {}", e)
            }
        }
    }
}

// We also need to implement the std::error::Error trait so that rust knows our custom error type is a proper error. 
// This makes it compatible with both ? and Box<dyn Error>.
impl std::error::Error for AddressErrorV2 {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AddressErrorV2::FileOpenError { source, .. } => Some(source),
            AddressErrorV2::FileUnreadableError { source, .. } => Some(source),
            AddressErrorV2::PortParseError(e) => Some(e),
            AddressErrorV2::PortTooLowError(_) => None,
            AddressErrorV2::LocalHostError(e) => Some(e),
        }
    }
}

fn _read_addr_from_file_v1(filename: &str) -> Result<String, AddressErrorV2> {
    let mut file = File::open(filename)
        .map_err(|e| AddressErrorV2::FileOpenError {
            filename: filename.to_string(),
            source: e,
        })?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| AddressErrorV2::FileUnreadableError {
            filename: filename.to_string(),
            source: e,
        })?;
    
    let port: i32 = contents.trim().parse()
        .map_err(AddressErrorV2::PortParseError)?;

    if port < 1000 {
        return Err(AddressErrorV2::PortTooLowError(port));
    }

    let localhost: Ipv4Addr = "127.0.0.1".parse()
        .map_err(AddressErrorV2::LocalHostError)?;
    
    Ok(format!("{localhost}:{port}"))
}




// #[derive(Error, Debug)] - derives std::error::Error (via thiserror) and Debug (via the standard derive)
#[derive(Error, Debug)]
pub enum AddressError {
    
    // #[error("...")] - generates the Display implementation. 
    // You can reference fields by name ({filename}) or by position ({0}). 
    // To implement std::error::Error, a type must also implement Display and Debug. 
    // We are getting the Debug implementation for free via the derive, and Display via this attribute.
    #[error("could not open file '{filename}'")]
    FileOpenError {
        filename: String,
        
        // #[source] - tells thiserror which field contains the underlying error, so it can implement source() correctly
        #[source]
        source: std::io::Error,
    },
    
    #[error("could not read file '{filename}'")]
    FileUnreadableError {
        filename: String,
        #[source]
        source: std::io::Error,
    },
    
    // #[from] - this one’s magic. 
    // It implements From<ThatError> for your enum, which means ? works automatically without needing .map_err(). 
    // Notice how we just write .parse()? for the port and localhost lines now (it also marks the wrapped error as the source)
    // I have chosen not to use #[from] for the file errors because I want to capture the filename as extra context. 
    // But for simpler cases where the wrapped error is all you need, #[from] saves a lot of noise.
    #[error("could not parse port number")]
    PortParseError(#[from] ParseIntError),
    
    #[error("expected a port over 1000, but got {0}")]
    PortTooLowError(i32),
    
    #[error("invalid localhost address")]
    LocalHostError(#[from] AddrParseError),
}

fn _read_addr_from_file_v2(filename: &str) -> Result<String, AddressError> {
    let mut file = File::open(filename)
        .map_err(|e| AddressError::FileOpenError {
            filename: filename.to_string(),
            source: e,
        })?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| AddressError::FileUnreadableError {
            filename: filename.to_string(),
            source: e,
        })?;
    
    let port: i32 = contents.trim().parse()?;

    if port < 1000 {
        return Err(AddressError::PortTooLowError(port));
    }

    let localhost: Ipv4Addr = "127.0.0.1".parse()?;
    
    Ok(format!("{localhost}:{port}"))
}

// enum errors are just regular Rust types - which means you can add whatever methods you need to your toolbox.
impl AddressError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AddressError::FileOpenError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            AddressError::FileUnreadableError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            AddressError::PortParseError(_) => StatusCode::BAD_REQUEST,
            AddressError::PortTooLowError(_) => StatusCode::BAD_REQUEST,
            AddressError::LocalHostError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
