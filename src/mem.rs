//! Raw low-level manipulations of lzip streams.

use std::error;
use std::fmt;
use std::marker;
use std::mem;
use std::slice;

use libc::{c_int, c_uint};

/// Representation of an in-memory decompression stream.
///
/// An instance of `Decompress` can be used to inflate a stream of lz-encoded
/// data.
pub struct Decompress {
    inner: Stream<DirDecompress>,
}

struct Stream<D: Direction> {
    // lzlib requires a stable address for this stream.
    raw: Box<lzip_sys::LZ_Decoder>,
    _marker: marker::PhantomData<D>,
}

unsafe impl<D: Direction> Send for Stream<D> {}
unsafe impl<D: Direction> Sync for Stream<D> {}

trait Direction {
    unsafe fn destroy(stream: *mut lzip_sys::LZ_Decoder) -> c_int;
}

enum DirDecompress {}

/// Result of compression or decompression
#[derive(PartialEq, Eq, Copy, Debug, Clone)]
pub enum Status {
    /// Decompression went fine, nothing much to report.
    Ok,

    /// There was insulzip_syscient memory in the input or output buffer to complete
    /// the request, but otherwise everything went normally.
    MemNeeded,
}

/// Fatal errors encountered when compressing/decompressing bytes.
///
/// These errors indicate that progress could not be made in any form due to
/// input or output parameters.
#[derive(PartialEq, Eq, Copy, Debug, Clone)]
pub enum Error {
    /// The sequence of operations called on a decompression/compression stream
    /// were invalid. See methods for details.
    Sequence,

    /// The data being decompressed was invalid, or it was not a valid bz2
    /// stream.
    Data,

    /// The magic bz2 header wasn't present when decompressing.
    DataMagic,

    /// The parameters to this function were invalid.
    Param,
}

impl Decompress {
    /// Creates a new stream prepared for decompression.
    ///
    /// If `small` is true, then the library will use an alternative
    /// decompression algorithm which uses less memory but at the cost of
    /// decompressing more slowly (roughly speaking, half the speed, but the
    /// maximum memory requirement drops to around 2300k). See
    pub fn new(small: bool) -> Decompress {
        unsafe {
            let mut raw = Box::new(mem::zeroed());
            //assert_eq!(
            //lzip_sys::LZ_decompress_read(&mut *raw, , small as c_int),
            //0
            //);
            Decompress {
                inner: Stream {
                    raw: raw,
                    _marker: marker::PhantomData,
                },
            }
        }
    }

    /// Decompress a block of input into a block of output.
    pub fn decompress(&mut self, input: &[u8], output: &mut [u8]) -> Result<Status, Error> {
        //self.inner.raw.next_in = input.as_ptr() as *mut _;
        //self.inner.raw.avail_in = input.len() as c_uint;
        //self.inner.raw.next_out = output.as_mut_ptr() as *mut _;
        //self.inner.raw.avail_out = output.len() as c_uint;
        unsafe {
            match lzip_sys::LZ_decompress_errno(&mut *self.inner.raw) {
                lzip_sys::LZ_OK => Ok(Status::Ok),
                lzip_sys::LZ_MEM_ERROR => Ok(Status::MemNeeded),
                lzip_sys::LZ_BAD_ARGUMENT => Err(Error::Param),
                lzip_sys::LZ_DATA_ERROR => Err(Error::Data),
                lzip_sys::LZ_HEADER_ERROR => Err(Error::DataMagic),
                lzip_sys::LZ_SEQUENCE_ERROR => Err(Error::Sequence),
                c => panic!("wut: {}", c),
            }
        }
    }

    /// Decompress a block of input into an output vector.
    ///
    /// This function will not grow `output`, but it will fill the space after
    /// its current length up to its capacity. The length of the vector will be
    /// adjusted appropriately.
    pub fn decompress_vec(&mut self, input: &[u8], output: &mut Vec<u8>) -> Result<Status, Error> {
        let cap = output.capacity();
        let len = output.len();

        unsafe {
            let before = lzip_sys::LZ_decompress_total_out_size(mem::zeroed());
            let ret = {
                let ptr = output.as_mut_ptr().offset(len as isize);
                let out = slice::from_raw_parts_mut(ptr, cap - len);
                self.decompress(input, out)
            };
            output.set_len(
                (lzip_sys::LZ_decompress_total_out_size(mem::zeroed()) - before) as usize + len,
            );
            return ret;
        }
    }
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match self {
            Error::Sequence => "lzip: sequence of operations invalid",
            Error::Data => "lzip: invalid data",
            Error::DataMagic => "lzip: lz header missing",
            Error::Param => "lzip: invalid parameters",
        };
        f.write_str(description)
    }
}

impl From<Error> for std::io::Error {
    fn from(data: Error) -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::Other, data)
    }
}

impl Direction for DirDecompress {
    unsafe fn destroy(stream: *mut lzip_sys::LZ_Decoder) -> c_int {
        lzip_sys::LZ_decompress_finish(stream)
    }
}

impl<D: Direction> Drop for Stream<D> {
    fn drop(&mut self) {
        unsafe {
            let _ = D::destroy(&mut *self.raw);
        }
    }
}
