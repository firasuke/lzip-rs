//! Lzip compression for Rust
//!
//! This library contains bindings to lzlib to support lzip compression and
//! decompression for Rust. The streams offered in this library are primarily
//! found in the `reader` and `writer` modules. Both compressors and
//! decompressors are available in each module depending on what operation you
//! need.
//!
//! # Example
//!
//! ```
//! use std::io::prelude::*;
//! use lzip::read::{LzEncoder, LzDecoder};
//!
//! // Round trip some bytes from a byte source, into a compressor, into a
//! // decompressor, and finally into a vector.
//! let data = "Hello, World!".as_bytes();
//! let compressor = LzEncoder::new(data, 9);
//! let mut decompressor = LzDecoder::new(compressor);
//!
//! let mut contents = String::new();
//! decompressor.read_to_string(&mut contents).unwrap();
//! assert_eq!(contents, "Hello, World!");
//! ```
//!
//! # Async I/O
//!
//! This crate optionally can support async I/O streams with the Tokio stack via
//! the `tokio` feature of this crate:
//!
//! ```toml
//! lzip = { version = "0.1", features = ["tokio"] }
//! ```
//!
//! All methods are internally capable of working with streams that may return
//! `ErrorKind::WouldBlock` when they're not ready to perform the particular
//! operation.
//!
//! Note that care needs to be taken when using these objects, however. The
//! Tokio runtime, in particular, requires that data is fully flushed before
//! dropping streams. For compatibility with blocking streams all streams are
//! flushed/written when they are dropped, and this is not always a suitable
//! time to perform I/O. If I/O streams are flushed before drop, however, then
//! these operations will be a noop.

#![deny(missing_docs)]
#![doc(html_root_url = "https://docs.rs/lzip/")]
