#![doc(html_root_url = "https://docs.rs/lzip-sys/")]

extern crate libc;

use libc::{c_char, c_int, c_uint, c_ulonglong};

pub const LZ_API_VERSION: u32 = 1013;
pub const LZ_VERSION_STRING: &[u8; 5usize] = b"1.13\0";

pub const LZ_ERRNO_LZ_OK: LzErrno = 0;
pub const LZ_ERRNO_LZ_BAD_ARGUMENT: LzErrno = 1;
pub const LZ_ERRNO_LZ_MEM_ERROR: LzErrno = 2;
pub const LZ_ERRNO_LZ_SEQUENCE_ERROR: LzErrno = 3;
pub const LZ_ERRNO_LZ_HEADER_ERROR: LzErrno = 4;
pub const LZ_ERRNO_LZ_UNEXPECTED_EOF: LzErrno = 5;
pub const LZ_ERRNO_LZ_DATA_ERROR: LzErrno = 6;
pub const LZ_ERRNO_LZ_LIBRARY_ERROR: LzErrno = 7;

pub type LzErrno = c_uint;

extern "C" {
    pub fn LZ_api_version() -> c_int;
    pub fn LZ_version() -> *const c_char;

    pub fn LZ_strerror(lz_errno: LzErrno) -> *const c_char;
    pub fn LZ_min_dictionary_bits() -> c_int;
    pub fn LZ_min_dictionary_size() -> c_int;
    pub fn LZ_max_dictionary_bits() -> c_int;
    pub fn LZ_max_dictionary_size() -> c_int;
    pub fn LZ_min_match_len_limit() -> c_int;
    pub fn LZ_max_match_len_limit() -> c_int;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LZ_Encoder {
    _unused: [u8; 0],
}

extern "C" {
    pub fn LZ_compress_open(
        dictionary_size: c_int,
        match_len_limit: c_int,
        member_size: c_ulonglong,
    ) -> *mut LZ_Encoder;
    pub fn LZ_compress_close(encoder: *mut LZ_Encoder) -> c_int;
    pub fn LZ_compress_finish(encoder: *mut LZ_Encoder) -> c_int;
    pub fn LZ_compress_restart_member(encoder: *mut LZ_Encoder, member_size: c_ulonglong) -> c_int;
    pub fn LZ_compress_sync_flush(encoder: *mut LZ_Encoder) -> c_int;
    pub fn LZ_compress_read(encoder: *mut LZ_Encoder, buffer: *mut u8, size: c_int) -> c_int;
    pub fn LZ_compress_write(encoder: *mut LZ_Encoder, buffer: *const u8, size: c_int) -> c_int;
    pub fn LZ_compress_write_size(encoder: *mut LZ_Encoder) -> c_int;
    pub fn LZ_compress_errno(encoder: *mut LZ_Encoder) -> LzErrno;
    pub fn LZ_compress_finished(encoder: *mut LZ_Encoder) -> c_int;
    pub fn LZ_compress_member_finished(encoder: *mut LZ_Encoder) -> c_int;
    pub fn LZ_compress_data_position(encoder: *mut LZ_Encoder) -> c_ulonglong;
    pub fn LZ_compress_member_position(encoder: *mut LZ_Encoder) -> c_ulonglong;
    pub fn LZ_compress_total_in_size(encoder: *mut LZ_Encoder) -> c_ulonglong;
    pub fn LZ_compress_total_out_size(encoder: *mut LZ_Encoder) -> c_ulonglong;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LZ_Decoder {
    _unused: [u8; 0],
}

extern "C" {
    pub fn LZ_decompress_open() -> *mut LZ_Decoder;
    pub fn LZ_decompress_close(decoder: *mut LZ_Decoder) -> c_int;
    pub fn LZ_decompress_finish(decoder: *mut LZ_Decoder) -> c_int;
    pub fn LZ_decompress_reset(decoder: *mut LZ_Decoder) -> c_int;
    pub fn LZ_decompress_sync_to_member(decoder: *mut LZ_Decoder) -> c_int;
    pub fn LZ_decompress_read(decoder: *mut LZ_Decoder, buffer: *mut u8, size: c_int) -> c_int;
    pub fn LZ_decompress_write(decoder: *mut LZ_Decoder, buffer: *const u8, size: c_int) -> c_int;
    pub fn LZ_decompress_write_size(decoder: *mut LZ_Decoder) -> c_int;
    pub fn LZ_decompress_errno(decoder: *mut LZ_Decoder) -> LzErrno;
    pub fn LZ_decompress_finished(decoder: *mut LZ_Decoder) -> c_int;
    pub fn LZ_decompress_member_finished(decoder: *mut LZ_Decoder) -> c_int;
    pub fn LZ_decompress_member_version(decoder: *mut LZ_Decoder) -> c_int;
    pub fn LZ_decompress_dictionary_size(decoder: *mut LZ_Decoder) -> c_int;
    pub fn LZ_decompress_data_crc(decoder: *mut LZ_Decoder) -> c_uint;
    pub fn LZ_decompress_data_position(decoder: *mut LZ_Decoder) -> c_ulonglong;
    pub fn LZ_decompress_member_position(decoder: *mut LZ_Decoder) -> c_ulonglong;
    pub fn LZ_decompress_total_in_size(decoder: *mut LZ_Decoder) -> c_ulonglong;
    pub fn LZ_decompress_total_out_size(decoder: *mut LZ_Decoder) -> c_ulonglong;
}
