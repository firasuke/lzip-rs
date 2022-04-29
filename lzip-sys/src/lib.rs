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

pub type BitModel = c_int;
pub type LzErrno = c_uint;
pub type LzipHeader = [u8; 6usize];
pub type State = c_int;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LzmaOptions {
    pub dictionary_size: c_int,
    pub match_len_limit: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LenModel {
    pub choice1: BitModel,
    pub choice2: BitModel,
    pub bm_low: [[BitModel; 8usize]; 4usize],
    pub bm_mid: [[BitModel; 8usize]; 4usize],
    pub bm_high: [BitModel; 256usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CircularBuffer {
    pub buffer: *mut u8,
    pub buffer_size: c_uint,
    pub get: c_uint,
    pub put: c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RangeDecoder {
    pub cb: CircularBuffer,
    pub member_position: c_ulonglong,
    pub code: u32,
    pub range: u32,
    pub at_stream_end: bool,
    pub reload_pending: bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LZ_decoder {
    pub cb: CircularBuffer,
    pub partial_data_pos: c_ulonglong,
    pub rdec: *mut RangeDecoder,
    pub dictionary_size: c_uint,
    pub crc: u32,
    pub member_finished: bool,
    pub verify_trailer_pending: bool,
    pub pos_wrapped: bool,
    pub rep0: c_uint,
    pub rep1: c_uint,
    pub rep2: c_uint,
    pub rep3: c_uint,
    pub state: State,
    pub bm_literal: [[BitModel; 768usize]; 8usize],
    pub bm_match: [[BitModel; 4usize]; 12usize],
    pub bm_rep: [BitModel; 12usize],
    pub bm_rep0: [BitModel; 12usize],
    pub bm_rep1: [BitModel; 12usize],
    pub bm_rep2: [BitModel; 12usize],
    pub bm_len: [[BitModel; 4usize]; 12usize],
    pub bm_dis_slot: [[BitModel; 64usize]; 4usize],
    pub bm_dis: [BitModel; 115usize],
    pub bm_align: [BitModel; 16usize],
    pub match_len_model: LenModel,
    pub rep_len_model: LenModel,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MatchfinderBase {
    pub partial_data_pos: c_ulonglong,
    pub buffer: *mut u8,
    pub prev_positions: *mut i32,
    pub pos_array: *mut i32,
    pub before_size: c_int,
    pub after_size: c_int,
    pub buffer_size: c_int,
    pub dictionary_size: c_int,
    pub pos: c_int,
    pub cyclic_pos: c_int,
    pub stream_pos: c_int,
    pub pos_limit: c_int,
    pub key4_mask: c_int,
    pub num_prev_positions23: c_int,
    pub num_prev_positions: c_int,
    pub pos_array_size: c_int,
    pub saved_dictionary_size: c_int,
    pub at_stream_end: bool,
    pub sync_flush_pending: bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RangeEncoder {
    pub cb: CircularBuffer,
    pub min_free_bytes: c_uint,
    pub low: u64,
    pub partial_member_pos: c_ulonglong,
    pub range: u32,
    pub ff_count: c_uint,
    pub cache: u8,
    pub header: LzipHeader,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LzEncoderBase {
    pub mb: MatchfinderBase,
    pub member_size_limit: c_ulonglong,
    pub crc: u32,
    pub bm_literal: [[BitModel; 768usize]; 8usize],
    pub bm_match: [[BitModel; 4usize]; 12usize],
    pub bm_rep: [BitModel; 12usize],
    pub bm_rep0: [BitModel; 12usize],
    pub bm_rep1: [BitModel; 12usize],
    pub bm_rep2: [BitModel; 12usize],
    pub bm_len: [[BitModel; 4usize]; 12usize],
    pub bm_dis_slot: [[BitModel; 64usize]; 4usize],
    pub bm_dis: [BitModel; 115usize],
    pub bm_align: [BitModel; 16usize],
    pub match_len_model: LenModel,
    pub rep_len_model: LenModel,
    pub renc: RangeEncoder,
    pub reps: [c_int; 4usize],
    pub state: State,
    pub member_finished: bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LenPrices {
    pub lm: *const LenModel,
    pub len_symbols: c_int,
    pub count: c_int,
    pub prices: [[c_int; 272usize]; 4usize],
    pub counters: [c_int; 4usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Pair {
    pub dis: c_int,
    pub len: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Trial {
    pub state: State,
    pub price: c_int,
    pub dis4: c_int,
    pub prev_index: c_int,
    pub prev_index2: c_int,
    pub reps: [c_int; 4usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LZ_encoder {
    pub eb: LzEncoderBase,
    pub cycles: c_int,
    pub match_len_limit: c_int,
    pub match_len_prices: LenPrices,
    pub rep_len_prices: LenPrices,
    pub pending_num_pairs: c_int,
    pub pairs: [Pair; 274usize],
    pub trials: [Trial; 8192usize],
    pub dis_slot_prices: [[c_int; 58usize]; 4usize],
    pub dis_prices: [[c_int; 128usize]; 4usize],
    pub align_prices: [c_int; 16usize],
    pub num_dis_slots: c_int,
    pub price_counter: c_int,
    pub dis_price_counter: c_int,
    pub align_price_counter: c_int,
    pub been_flushed: bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FlzEncoder {
    pub eb: LzEncoderBase,
    pub key4: c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LZ_Encoder {
    pub partial_in_size: c_ulonglong,
    pub partial_out_size: c_ulonglong,
    pub lz_encoder_base: *mut LzEncoderBase,
    pub lz_encoder: *mut LZ_encoder,
    pub flz_encoder: *mut FlzEncoder,
    pub lz_errno: LzErrno,
    pub fatal: bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LZ_Decoder {
    pub partial_in_size: c_ulonglong,
    pub partial_out_size: c_ulonglong,
    pub rdec: *mut RangeDecoder,
    pub lz_decoder: *mut LZ_decoder,
    pub lz_errno: LzErrno,
    pub member_header: LzipHeader,
    pub fatal: bool,
    pub first_header: bool,
    pub seeking: bool,
}

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
