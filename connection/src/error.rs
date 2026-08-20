use std::fmt::Display;

pub enum ConnectionError {
    InvalidEnv(String),
    Io(std::io::Error)
}

impl Display for ConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectionError::InvalidEnv(e) => write!(f, "{}", e),
            ConnectionError::Io(e) => write!(f, "{}", e.to_string())
        }
    }
}

impl  From<std::io::Error> for ConnectionError {
    fn from(value: std::io::Error) -> Self {
        ConnectionError::Io(value)
    }
} 