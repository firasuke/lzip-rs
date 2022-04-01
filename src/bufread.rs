//! I/O streams for wrapping `BufRead` types as encoders/decoders

use std::io;
use std::io::prelude::*;

#[cfg(feature = "tokio")]
use futures::Poll;
#[cfg(feature = "tokio")]
use tokio_io::{AsyncRead, AsyncWrite};

use {Action, Compress, Compression, Decompress, Status};

/// An lzip encoder, or compressor.
///
/// This structure implements a `BufRead` interface and will read uncompressed
/// data from an underlying stream and emit a stream of compressed data.
pub struct LzEncoder<R> {
    obj: R,
    data: Compress,
    done: bool,
}

/// An lzip decoder, or decompressor.
///
/// This structure implements a `BufRead` interface and takes a stream of
/// compressed data as input, providing the decompressed data when read from.
pub struct LzDecoder<R> {
    obj: R,
    data: Decompress,
    done: bool,
    multi: bool,
}

impl<R: BufRead> XzEncoder<R> {
    /// Creates a new encoder which will read uncompressed data from the given
    /// stream and emit the compressed stream.
    ///
}