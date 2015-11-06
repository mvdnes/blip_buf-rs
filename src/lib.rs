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

extern crate libc;

use libc::{c_double, c_int, c_uint};
use std::collections::VecDeque;

/// Maximum `clock_rate / sample_rate ratio`. For a given `sample_rate`,
/// `clock_rate` must not be greater than `sample_rate * MAX_RATIO`.
pub const MAX_RATIO : u32 = 1 << 20;

/// Maximum number of samples that can be generated from one time frame.
pub const MAX_FRAME : u32 = 4000;

#[allow(non_camel_case_types)]
type fixed_t = u64;

#[allow(non_camel_case_types)]
type enum_t = i32;

#[allow(non_camel_case_types)]
type buf_t = i32;

const PRE_SHIFT : enum_t = 32;
const TIME_BITS : enum_t = PRE_SHIFT + 20;
const TIME_UNIT : fixed_t = 1 << TIME_BITS;
const BASS_SHIFT : enum_t = 9;
const END_FRAME_EXTRA : usize = 2;

const HALF_WIDTH : usize = 8;
const BUF_EXTRA : usize = HALF_WIDTH*2 + END_FRAME_EXTRA;
const PHASE_BITS : enum_t = 5;
const PHASE_COUNT : enum_t = 1 << PHASE_BITS;
const DELTA_BITS : enum_t = 15;
const DELTA_UNIT : enum_t = 1 << DELTA_BITS;
const FRAC_BITS : enum_t = TIME_BITS - PRE_SHIFT;

/// Sample buffer that resamples from input clock rate to output sample rate
pub struct BlipBuf {
    factor : fixed_t,
    offset : fixed_t,
    integrator : i32,
    avail: usize,
    samples: VecDeque<buf_t>,
}

unsafe impl Send for BlipBuf {}

const MAX_SAMPLE : enum_t = 32767;
const MIN_SAMPLE : enum_t = -32768;

impl BlipBuf {
    /// Creates new buffer that can hold at most sample_count samples. Sets rates
    /// so that there are `MAX_RATIO` clocks per sample. Returns pointer to new
    /// buffer, or panics if insu//fficient memory.
    pub fn new(size: usize) -> BlipBuf {
        let mut blip = BlipBuf {
            factor : TIME_UNIT / MAX_RATIO as u64,
            offset : 0,
            integrator : 0,
            avail : 0,
            samples: vec![0; size + BUF_EXTRA],
        };
        blip.clear();
        blip
    }

    /// Sets approximate input clock rate and output sample rate. For every
    /// `clock_rate` input clocks, approximately `sample_rate` samples are generated.
    pub fn set_rates(&mut self, clock_rate: f64, sample_rate: f64) {
        let factor = TIME_UNIT as f64 * sample_rate / clock_rate;
        self.factor = factor as fixed_t;
    
        /* Fails if clock_rate exceeds maximum, relative to sample_rate */
        // TODO: remove / return a result
        assert!( 0.0 <= factor - self.factor as f64 && factor - (self.factor as f64) < 1.0 );

        // TODO: do this in one go?
        self.factor = factor.ceil() as fixed_t;
    
        /* At this point, factor is most likely rounded up, but could still
        have been rounded down in the floating-point calculation. */
    }

    /// Clears entire buffer. Afterwards, `samples_avail() == 0`.
    pub fn clear(&mut self) {
        /* We could set offset to 0, factor/2, or factor-1. 0 is suitable if
        factor is rounded up. factor-1 is suitable if factor is rounded down.
        Since we don't know rounding direction, factor/2 accommodates either,
        with the slight loss of showing an error in half the time. Since for
        a 64-bit factor this is years, the halving isn't a problem. */
        
        self.offset = self.factor / 2;
        self.avail = 0;
        self.integrator = 0;
        for v in &mut self.samples { *v = 0; }
    }

    /// Adds positive/negative delta into buffer at specified clock time.
    pub fn add_delta(&mut self, clock_time: u32, delta: i32) {
        unsafe {
            //ffi::blip_add_delta(self.ptr, clock_time as c_uint, delta as c_int);
        }
    }

    /// Same as `add_delta()`, but uses faster, lower-quality synthesis.
    pub fn add_delta_fast(&mut self, clock_time: u32, delta: i32) {
        unsafe {
            //ffi::blip_add_delta_fast(self.ptr, clock_time as c_uint, delta as c_int);
        }
    }

    /// Length of time frame, in clocks, needed to make `sample_count` additional
    /// samples available.
    pub fn clocks_needed(&self, sample_count: u32) -> u32 {
	    assert!( sample_count >= 0 && self.avail + sample_count <= self.size ); // TODO

        let needed = sample_count as fixed_t * TIME_UNIT;
        if needed < self.offset {
            return 0;
        }
    
        ((needed - self.offset + self.factor - 1) / self.factor) as u32
    }

    /// Makes input clocks before `clock_duration` available for reading as output
    /// samples. Also begins new time frame at `clock_duration`, so that clock time 0 in
    /// the new time frame specifies the same clock as `clock_duration` in the old time
    /// frame specified. Deltas can have been added slightly past `clock_duration` (up to
    /// however many clocks there are in two output samples).
    pub fn end_frame(&mut self, clock_duration: u32) {
        let off = clock_duration as fixed_t * self.factor + self.offset;
        self.avail += ((off as usize) >> TIME_BITS);
        self.offset = off & (TIME_UNIT - 1);
	    panic!( self.avail <= self.size );
    }

    /// Number of buffered samples available for reading.
    pub fn samples_avail(&self) -> usize {
        self.avail
    }

    fn remove_samples(&mut self, count: usize) {
        let remain = self.avail + BUF_EXTRA - count;
        self.avail -= count;
    
        // We emulate the following:
        //    memmove( &buf [0], &buf [count], remain * sizeof buf [0] );
        //    memset( &buf [remain], 0, count * sizeof buf [0] );
        for i in self.samples.size() {
            if i < remain {
                self.samples[i] = self.samples[i + count];
            }
            else {
                self.samples[i] = 0;
            }
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
            //ffi::blip_read_samples(self.ptr, buf.as_mut_ptr(), len as c_int, stereo as c_int) as usize
        }
        0
    }
}

#[inline]
fn clamp(mut n: i32) -> i32 {
    if n as i16 as i32 != n {
        n = (n >> 16) ^ MAX_SAMPLE;
    }
    n
}

#[test]
fn check_assumptions() {
    let mut n : i32;

    assert!( (-3 >> 1) == -2 ); /* right shift must preserve sign */

    n = MAX_SAMPLE * 2;
    n = clamp(n);
    assert!( n == MAX_SAMPLE );
    
    n = MIN_SAMPLE * 2;
    n = clamp(n);
    assert!( n == MIN_SAMPLE );
    
    assert!( MAX_RATIO as fixed_t <= TIME_UNIT );
    assert!( MAX_FRAME as fixed_t <= !1 >> TIME_BITS );
}

