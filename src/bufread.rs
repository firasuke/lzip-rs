//! I/O streams for wrapping `BufRead` types as encoders/decoders

use std::io;
use std::io::prelude::*;

#[cfg(feature = "tokio")]
use futures::Poll;
#[cfg(feature = "tokio")]
use tokio_io::{AsyncRead, AsyncWrite};

use stream::{Action, Check, Status, Stream};

/// An lzip encoder, or compressor.
///
/// This structure implements a `BufRead` interface and will read uncompressed
/// data from an underlying stream and emit a stream of compressed data.
pub struct LzEncoder<R> {
    obj: R,
    data: Stream,
}

/// An lzip decoder, or decompressor.
///
/// This structure implements a `BufRead` interface and takes a stream of
/// compressed data as input, providing the decompressed data when read from.
pub struct LzDecoder<R> {
    obj: R,
    data: Stream,
}

impl<R: BufRead> LzEncoder<R> {
    /// Creates a new encoder which will read uncompressed data from the given
    /// stream and emit the compressed stream.
    ///
    /// The `level` argument here is typically 0-9 with 6 being a good default.
    pub fn new(r: R, level: u32) -> LzEncoder<R> {
        let stream = Stream::new_easy_encoder(level, Check::Crc64).unwrap();
        LzEncoder::new_stream(r, stream)
    }

    /// Creates a new encoder with a custom `Stream`.
    ///
    /// The `Stream` can be pre-configured for multithreaded encoding, different
    /// compression options/tuning, etc.
    pub fn new_stream(r: R, stream: Stream) -> LzEncoder<R> {
        LzEncoder {
            obj: r,
            data: stream,
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

impl<R: BufRead> Read for LzEncoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        loop {
            let (read, consumed, eof, ret);
            {
                let input = self.obj.fill_buf()?;
                eof = input.is_empty();
                let before_out = self.data.total_out();
                let before_in = self.data.total_in();
                let action = if eof { Action::Finish } else { Action::Run };
                ret = self.data.process(input, buf, action);
                read = (self.data.total_out() - before_out) as usize;
                consumed = (self.data.total_in() - before_in) as usize;
            }
            self.obj.consume(consumed);

            ret.unwrap();

            // If we haven't ready any data and we haven't hit EOF yet, then we
            // need to keep asking for more data because if we return that 0
            // bytes of data have been read then it will be interpreted as EOF.
            if read == 0 && !eof && buf.len() > 0 {
                continue;
            }
            return Ok(read);
        }
    }
}

#[cfg(feature = "tokio")]
impl<R: AsyncRead + BufRead> AsyncRead for LzEncoder<R> {}

impl<W: Write> Write for LzEncoder<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.get_mut().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.get_mut().flush()
    }
}

#[cfg(feature = "tokio")]
impl<R: AsyncWrite> AsyncWrite for LzEncoder<R> {
    fn shutdown(&mut self) -> Poll<(), io::Error> {
        self.get_mut().shutdown()
    }
}

impl<R: BufRead> LzDecoder<R> {
    /// Creates a new decoder which will decompress data read from the given
    /// stream.
    pub fn new(r: R) -> LzDecoder<R> {
        let stream = Stream::new_stream_decoder(u64::max_value(), 0).unwrap();
        LzDecoder::new_stream(r, stream)
    }

    /// Creates a new decoder which will decompress data read from the given
    /// input. All the concatenated lzip streams from input will be consumed.
    // pub fn new_multi_decoder(r: R) -> LzDecoder<R> {
    //     let stream =
    //         Stream::new_auto_decoder(u64::max_value(), lzip_sys::LZMA_CONCATENATED).unwrap();
    //     LzDecoder::new_stream(r, stream)
    // }

    /// Creates a new decoder with a custom `Stream`.
    ///
    /// The `Stream` can be pre-configured for various checks, different
    /// decompression options/tuning, etc.
    pub fn new_stream(r: R, stream: Stream) -> LzDecoder<R> {
        LzDecoder {
            obj: r,
            data: stream,
        }
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

    /// Returns the number of bytes that the decompressor has consumed.
    ///
    /// Note that this will likely be smaller than what the decompressor
    /// actually read from the underlying stream due to buffering.
    pub fn total_in(&self) -> u64 {
        self.data.total_in()
    }

    /// Returns the number of bytes that the decompressor has produced.
    pub fn total_out(&self) -> u64 {
        self.data.total_out()
    }
}

impl<R: BufRead> Read for LzDecoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        loop {
            let (read, consumed, eof, ret);
            {
                let input = self.obj.fill_buf()?;
                eof = input.is_empty();
                let before_out = self.data.total_out();
                let before_in = self.data.total_in();
                ret = self
                    .data
                    .process(input, buf, if eof { Action::Finish } else { Action::Run });
                read = (self.data.total_out() - before_out) as usize;
                consumed = (self.data.total_in() - before_in) as usize;
            }
            self.obj.consume(consumed);

            let status = ret?;
            if read > 0 || eof || buf.len() == 0 {
                if read == 0 && status != Status::StreamEnd && buf.len() > 0 {
                    return Err(io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        "premature eof",
                    ));
                }
                return Ok(read);
            }
            if consumed == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "corrupt lzip stream",
                ));
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

#[cfg(test)]
mod tests {
    use bufread::{LzDecoder, LzEncoder};
    use std::io::Read;

    #[test]
    fn compressed_and_trailing_data() {
        // Make a vector with compressed data...
        let mut to_compress: Vec<u8> = Vec::new();
        const COMPRESSED_ORIG_SIZE: usize = 1024;
        for num in 0..COMPRESSED_ORIG_SIZE {
            to_compress.push(num as u8)
        }
        let mut encoder = LzEncoder::new(&to_compress[..], 6);

        let mut decoder_input = Vec::new();
        encoder.read_to_end(&mut decoder_input).unwrap();

        // ...plus additional unrelated trailing data
        const ADDITIONAL_SIZE: usize = 123;
        let mut additional_data = Vec::new();
        for num in 0..ADDITIONAL_SIZE {
            additional_data.push(((25 + num) % 256) as u8)
        }
        decoder_input.extend(&additional_data);

        // Decoder must be able to read the compressed lzip stream, and keep the trailing data.
        let mut decoder_reader = &decoder_input[..];
        {
            let mut decoder = LzDecoder::new(&mut decoder_reader);
            let mut decompressed_data = vec![0u8; to_compress.len()];

            assert_eq!(
                decoder.read(&mut decompressed_data).unwrap(),
                COMPRESSED_ORIG_SIZE
            );
            assert_eq!(decompressed_data, &to_compress[..]);
        }

        let mut remaining_data = Vec::new();
        let nb_read = decoder_reader.read_to_end(&mut remaining_data).unwrap();
        assert_eq!(nb_read, ADDITIONAL_SIZE);
        assert_eq!(remaining_data, &additional_data[..]);
    }
}
