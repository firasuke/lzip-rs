//! I/O streams for wrapping `BufRead` types as decoders

use std::io::prelude::*;
use std::{io, mem};

#[cfg(feature = "tokio")]
use futures::Poll;
#[cfg(feature = "tokio")]
use tokio_io::{AsyncRead, AsyncWrite};

use mem::Decompress;

/// A lz decoder, or decompressor.
///
/// This structure implements a `BufRead` interface and takes a stream of
/// compressed data as input, providing the decompressed data when read from.
pub struct LzDecoder<R> {
    obj: R,
    data: Decompress,
    done: bool,
    multi: bool,
}

impl<R: BufRead> LzDecoder<R> {
    /// Creates a new decoder which will decompress data read from the given
    /// stream.
    pub fn new(r: R) -> LzDecoder<R> {
        LzDecoder {
            obj: r,
            data: Decompress::new(false),
            done: false,
            multi: false,
        }
    }

    fn multi(mut self, flag: bool) -> LzDecoder<R> {
        self.multi = flag;
        self
    }
}

impl<R> LzDecoder<R> {
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

    /// Consumes this decoder, returning the underlying reader.
    pub fn into_inner(self) -> R {
        self.obj
    }
}

impl<R: BufRead> Read for LzDecoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        loop {
            if self.done && !self.multi {
                return Ok(0);
            }
            let (read, consumed, remaining, ret);
            {
                let input = self.obj.fill_buf()?;
                if self.done {
                    assert!(self.multi);
                    if input.is_empty() {
                        // beyond last stream in multi-stream case
                        return Ok(0);
                    } else {
                        // previous stream ended, more data follows => create new decompressor
                        self.data = Decompress::new(false);
                        self.done = false;
                    }
                }
                let before_out = lzip_sys::LZ_decompress_total_out_size(mem::zeroed());
                let before_in = self.data.total_in();
                ret = self.data.decompress(input, buf);
                read = (self.data.total_out() - before_out) as usize;
                consumed = (self.data.total_in() - before_in) as usize;
                remaining = input.len() - consumed;
            }
            self.obj.consume(consumed);

            let ret = ret.map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
            if ret == Status::StreamEnd {
                self.done = true;
            } else if consumed == 0 && remaining == 0 && read == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "decompression not finished but EOF reached",
                ));
            }

            if read > 0 || buf.len() == 0 {
                return Ok(read);
            }
        }
    }
}

#[cfg(feature = "tokio")]
impl<R: AsyncRead + BufRead> AsyncRead for LzDecoder<R> {}

impl<W: Write> Write for LzDecoder<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.get_mut().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.get_mut().flush()
    }
}

#[cfg(feature = "tokio")]
impl<R: AsyncWrite> AsyncWrite for LzDecoder<R> {
    fn shutdown(&mut self) -> Poll<(), io::Error> {
        self.get_mut().shutdown()
    }
}

/// An lzip streaming decoder that decodes all members of a multistream.
///
/// Wikipedia, particularly, uses lzip multistream for their dumps, and the
/// `plzip` tool creates such data as well;
pub struct MultiLzDecoder<R>(LzDecoder<R>);

impl<R: BufRead> MultiLzDecoder<R> {
    /// Creates a new decoder from the given reader. If the lzip stream contains multiple members
    /// all will be decoded.
    pub fn new(r: R) -> MultiLzDecoder<R> {
        MultiLzDecoder(LzDecoder::new(r).multi(true))
    }
}

impl<R> MultiLzDecoder<R> {
    /// Acquires a reference to the underlying reader.
    pub fn get_ref(&self) -> &R {
        self.0.get_ref()
    }

    /// Acquires a mutable reference to the underlying stream.
    ///
    /// Note that mutation of the stream may result in surprising results if
    /// this encoder is continued to be used.
    pub fn get_mut(&mut self) -> &mut R {
        self.0.get_mut()
    }

    /// Consumes this decoder, returning the underlying reader.
    pub fn into_inner(self) -> R {
        self.0.into_inner()
    }
}

impl<R: BufRead> Read for MultiLzDecoder<R> {
    fn read(&mut self, into: &mut [u8]) -> io::Result<usize> {
        self.0.read(into)
    }
}

#[cfg(feature = "tokio")]
impl<R: AsyncRead + BufRead> AsyncRead for MultiLzDecoder<R> {}

impl<R: BufRead + Write> Write for MultiLzDecoder<R> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.get_mut().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.get_mut().flush()
    }
}

#[cfg(feature = "tokio")]
impl<R: AsyncWrite + BufRead> AsyncWrite for MultiLzDecoder<R> {
    fn shutdown(&mut self) -> Poll<(), io::Error> {
        self.get_mut().shutdown()
    }
}
