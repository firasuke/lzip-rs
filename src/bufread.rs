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

impl<R: BufRead> LzEncoder<R> {
    /// Creates a new encoder which will read uncompressed data from the given
    /// stream and emit the compressed stream.
    pub fn new(r: R, level: Compression) -> LzEncoder<R> {
        LzEncoder {
            obj: r,
            data: Compress::new(level, 30),
            done: false,
        }
    }
}

impl<R> LzEncoder<R> {
    /// Acquires a reference to the underlying stream
    pub fn get_ref(&self) -> &R {
        &self.obj
    }

    /// Acquires a mutable reference to the underlying stream
    ///
    /// Note that mutation of the stream may result in surprising results if
    /// this encoder is continued to be used.
    pub fn get_mut(&mut self) -> &mut R {
        &mut self.obj
    }

    /// Consumes this encoder, returning the underlying reader.
    pub fn into_inner(self) -> R {
        self.obj
    }

    /// Returns the number of bytes produced by the compressor
    /// (e.g. the number of bytes read from this stream)
    ///
    /// Note that, due to buffering, this only bears any relation to
    /// total_in() when the compressor chooses to flush its data
    /// (unfortunately, this won't happen in general at the end of the
    /// stream, because the compressor doesn't know if there's more data
    /// to come).  At that point, `total_out() / total_in()` would be
    /// the compression ratio.
    pub fn total_out(&self) -> u64 {
        self.data.total_out()
    }

    /// Returns the number of bytes consumed by the compressor
    /// (e.g. the number of bytes read from the underlying stream)
    pub fn total_in(&self) -> u64 {
        self.data.total_in()
    }
}