//! Provides tools for HTTP parsing.
//!
//! This crate depends on *http-core* crate for core types such as
//! `Request` and `Response`. If you need more information about theses
//! types, refer to the *http-core* crate documentation.
//!
//! [`parse_req`] is used to parse bytes into useful HTTP `Response` representation.
//! ```no_run
//! # use std::io::Cursor;
//! # use std::io::{BufRead, BufReader, Read, Write};
//! # use http_parser::parse_req;
//! fn mock_tcp_stream(inner: &[u8]) -> Cursor<Vec<u8>> {
//!    let mut src = Cursor::new(inner.to_vec());
//!    src.set_position(0);
//!    
//!    src
//! }
//!
//! let mut stream = mock_tcp_stream(b"GET / HTTP/1.1");
//! parse_req(&mut stream);
//! ```

pub mod error;
mod helpers;
mod request;
pub mod response;

pub use request::parse_req;
