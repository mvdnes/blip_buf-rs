//! blip_buf is a small waveform synthesis library meant for use in classic video game
//! sound chip emulation. It greatly simplifies sound chip emulation code by handling
//! all the details of resampling. The emulator merely sets the input clock rate and output
//! sample rate, adds waveforms by specifying the clock times where their amplitude changes,
//! then reads the resulting output samples.
//!
//! # Features
//!
//! * Several code examples, including simple sound chip emulator.
//! * Uses fast, high-quality band-limited resampling algorithm ([BLEP]).
//! * Output is low-pass and high-pass filtered and clamped to 16-bit range.
//! * Supports mono, stereo, and multi-channel synthesis.
//!
//! # Based upon
//!
//! This library is a very thin wrapper on the original C library, found here: https://code.google.com/p/blip-buf/
//!
//! [BLEP]: http://www.cs.cmu.edu/~eli/L/icmc01/hardsync.html

#![warn(missing_docs)]

extern crate blip_buf_sys as ffi;
extern crate libc;

use libc::{c_double, c_int, c_uint};

/// Maximum `clock_rate / sample_rate ratio`. For a given `sample_rate`,
/// `clock_rate` must not be greater than `sample_rate * MAX_RATIO`.
pub const MAX_RATIO : u32 = 1 << 20;

/// Maximum number of samples that can be generated from one time frame.
pub const MAX_FRAME : u32 = 4000;

/// Sample buffer that resamples from input clock rate to output sample rate
pub struct BlipBuf {
    ptr: *mut ffi::blip_t,
}

unsafe impl Send for BlipBuf {}

impl BlipBuf {
    /// Creates new buffer that can hold at most sample_count samples. Sets rates
    /// so that there are `MAX_RATIO` clocks per sample. Returns pointer to new
    /// buffer, or panics if insufficient memory.
    pub fn new(sample_count: u32) -> BlipBuf {
        unsafe {
            let ptr = ffi::blip_new(sample_count as c_int);
            assert!(!ptr.is_null());

            BlipBuf { ptr: ptr }
        }
    }

    /// Sets approximate input clock rate and output sample rate. For every
    /// `clock_rate` input clocks, approximately `sample_rate` samples are generated.
    pub fn set_rates(&mut self, clock_rate: f64, sample_rate: f64) {
        unsafe {
            ffi::blip_set_rates(self.ptr, clock_rate as c_double, sample_rate as c_double);
        }
    }

    /// Clears entire buffer. Afterwards, `samples_avail() == 0`.
    pub fn clear(&mut self) {
        unsafe {
            ffi::blip_clear(self.ptr);
        }
    }

    /// Adds positive/negative delta into buffer at specified clock time.
    pub fn add_delta(&mut self, clock_time: u32, delta: i32) {
        unsafe {
            ffi::blip_add_delta(self.ptr, clock_time as c_uint, delta as c_int);
        }
    }

    /// Same as `add_delta()`, but uses faster, lower-quality synthesis.
    pub fn add_delta_fast(&mut self, clock_time: u32, delta: i32) {
        unsafe {
            ffi::blip_add_delta_fast(self.ptr, clock_time as c_uint, delta as c_int);
        }
    }

    /// Length of time frame, in clocks, needed to make `sample_count` additional
    /// samples available.
    pub fn clocks_needed(&self, sample_count: u32) -> u32 {
        unsafe {
            ffi::blip_clocks_needed(self.ptr, sample_count as c_int) as u32
        }
    }

    /// Makes input clocks before `clock_duration` available for reading as output
    /// samples. Also begins new time frame at `clock_duration`, so that clock time 0 in
    /// the new time frame specifies the same clock as `clock_duration` in the old time
    /// frame specified. Deltas can have been added slightly past `clock_duration` (up to
    /// however many clocks there are in two output samples).
    pub fn end_frame(&mut self, clock_duration: u32) {
        unsafe {
            ffi::blip_end_frame(self.ptr, clock_duration as c_uint);
        }
    }

    /// Number of buffered samples available for reading.
    pub fn samples_avail(&self) -> u32 {
        unsafe {
            ffi::blip_samples_avail(self.ptr) as u32
        }
    }

    /// Reads and removes at most `buf.len()` samples and writes them to `buf`. If
    /// `stereo` is true, writes output to every other element of `buf`, allowing easy
    /// interleaving of two buffers into a stereo sample stream. Outputs 16-bit signed
    /// samples. Returns number of samples actually read.
    pub fn read_samples(&mut self, buf: &mut [i16], stereo: bool) -> usize {
        unsafe {
            let len = if stereo {
                buf.len() / 2
            }
            else {
                buf.len()
            };
            ffi::blip_read_samples(self.ptr, buf.as_mut_ptr(), len as c_int, stereo as c_int) as usize
        }
    }
}

impl Drop for BlipBuf {
    fn drop(&mut self) {
        unsafe {
            ffi::blip_delete(self.ptr);
        }
    }
}

#[cfg(test)]
mod test {
    use super::BlipBuf;

    #[test]
    fn basics() {
        let mut blipbuf = BlipBuf::new(44100);
        blipbuf.set_rates((1 << 22) as f64, 44100f64);
        drop(blipbuf);
    }
}
