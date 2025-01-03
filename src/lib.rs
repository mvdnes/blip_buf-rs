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

/// Maximum `clock_rate / sample_rate ratio`. For a given `sample_rate`,
/// `clock_rate` must not be greater than `sample_rate * MAX_RATIO`.
pub const MAX_RATIO: usize = 1 << 20;

/// Maximum number of samples that can be generated from one time frame.
pub const MAX_FRAME: u32 = 4000;

#[allow(non_camel_case_types)]
type fixed_t = usize;

#[allow(non_camel_case_types)]
type enum_t = usize;

#[allow(non_camel_case_types)]
type buf_t = i32;

const PRE_SHIFT: enum_t = 32;
const TIME_BITS: enum_t = PRE_SHIFT + 20;
const TIME_UNIT: fixed_t = 1 << TIME_BITS;
const BASS_SHIFT: enum_t = 9;
const END_FRAME_EXTRA: usize = 2;

const HALF_WIDTH: usize = 8;
const BUF_EXTRA: usize = HALF_WIDTH * 2 + END_FRAME_EXTRA;
const PHASE_BITS: enum_t = 5;
const PHASE_COUNT: enum_t = 1 << PHASE_BITS;
const DELTA_BITS: enum_t = 15;
const DELTA_UNIT: enum_t = 1 << DELTA_BITS;
const FRAC_BITS: enum_t = TIME_BITS - PRE_SHIFT;

/// Sample buffer that resamples from input clock rate to output sample rate
pub struct BlipBuf {
    factor: fixed_t,
    offset: fixed_t,
    integrator: i32,
    avail: usize,
    samples: Vec<buf_t>,
}

unsafe impl Send for BlipBuf {}

const MAX_SAMPLE: enum_t = 32767;

impl BlipBuf {
    /// Creates new buffer that can hold at most sample_count samples. Sets rates
    /// so that there are `MAX_RATIO` clocks per sample. Returns pointer to new
    /// buffer, or panics if insu//fficient memory.
    pub fn new(sample_count: u32) -> BlipBuf {
        let sample_count = sample_count as usize;
        let mut blip = BlipBuf {
            factor: TIME_UNIT / MAX_RATIO,
            offset: 0,
            integrator: 0,
            avail: 0,
            samples: vec![0; sample_count + BUF_EXTRA],
        };
        blip.clear();
        blip
    }

    /// Sets approximate input clock rate and output sample rate. For every
    /// `clock_rate` input clocks, approximately `sample_rate` samples are generated.
    pub fn set_rates(&mut self, clock_rate: f64, sample_rate: f64) {
        let factor: f64 = (TIME_UNIT as f64) * sample_rate / clock_rate;
        self.factor = factor as fixed_t;

        /* Fails if clock_rate exceeds maximum, relative to sample_rate */
        // TODO: remove / return a result
        assert!(0.0 <= factor - self.factor as f64 && factor - (self.factor as f64) < 1.0);

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
        self.samples.fill(0);
    }

    /// Adds positive/negative delta into buffer at specified clock time.
    pub fn add_delta(&mut self, time: u32, mut delta: i32) {
        let time = time as usize;
        let fixed = ((time * self.factor + self.offset) >> PRE_SHIFT) as usize;
        let out_index = self.avail + (fixed >> FRAC_BITS);
        let phase_shift = FRAC_BITS - PHASE_BITS;
        let phase = fixed >> phase_shift & (PHASE_COUNT - 1);
        let mut index = BL_STEP[phase];
        let next = BL_STEP[phase + 1];
        let rev = BL_STEP[PHASE_COUNT - phase];
        let prev = BL_STEP[PHASE_COUNT - phase - 1];
        let interp = fixed >> (phase_shift - DELTA_BITS) & (DELTA_UNIT - 1);
        let delta2 = ((delta as i32) * (interp as i32)) >> DELTA_BITS;
        delta -= delta2;

        assert!(
            out_index <= self.samples.len() + END_FRAME_EXTRA,
            "buffer size was exceeded"
        );

        self.samples[out_index + 0] += index[0] as i32 * delta + next[0] as i32 * delta2;
        self.samples[out_index + 1] += index[1] as i32 * delta + next[1] as i32 * delta2;
        self.samples[out_index + 2] += index[2] as i32 * delta + next[2] as i32 * delta2;
        self.samples[out_index + 3] += index[3] as i32 * delta + next[3] as i32 * delta2;
        self.samples[out_index + 4] += index[4] as i32 * delta + next[4] as i32 * delta2;
        self.samples[out_index + 5] += index[5] as i32 * delta + next[5] as i32 * delta2;
        self.samples[out_index + 6] += index[6] as i32 * delta + next[6] as i32 * delta2;
        self.samples[out_index + 7] += index[7] as i32 * delta + next[7] as i32 * delta2;

        index = rev;
        self.samples[out_index + 8] += index[7] as i32 * delta + prev[7] as i32 * delta2;
        self.samples[out_index + 9] += index[6] as i32 * delta + prev[6] as i32 * delta2;
        self.samples[out_index + 10] += index[5] as i32 * delta + prev[5] as i32 * delta2;
        self.samples[out_index + 11] += index[4] as i32 * delta + prev[4] as i32 * delta2;
        self.samples[out_index + 12] += index[3] as i32 * delta + prev[3] as i32 * delta2;
        self.samples[out_index + 13] += index[2] as i32 * delta + prev[2] as i32 * delta2;
        self.samples[out_index + 14] += index[1] as i32 * delta + prev[1] as i32 * delta2;
        self.samples[out_index + 15] += index[0] as i32 * delta + prev[0] as i32 * delta2;
    }

    /// Same as `add_delta()`, but uses faster, lower-quality synthesis.
    pub fn add_delta_fast(&mut self, time: u32, delta: i16) {
        let time = time as usize;
        let fixed = time * self.factor + self.offset >> PRE_SHIFT;

        let out_index = self.avail + (fixed >> FRAC_BITS);

        let interp = fixed >> (FRAC_BITS - DELTA_BITS) & (DELTA_UNIT - 1);
        let delta2 = ((delta as usize) * interp) as i16;

        assert!(
            { out_index <= (self.samples.len()) },
            "buffer size was exceeded"
        );

        self.samples[out_index + 7] += (delta * (DELTA_UNIT as i16) - delta2) as i32;
        self.samples[out_index + 8] += delta2 as i32;
    }

    /// Length of time frame, in clocks, needed to make `sample_count` additional
    /// samples available.
    pub fn clocks_needed(&self, sample_count: u32) -> u32 {
        assert!(self.avail + sample_count as usize <= self.samples.len()); // TODO

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
        let off = (clock_duration as usize) * self.factor + self.offset;
        self.avail += off >> TIME_BITS;
        self.offset = off & (TIME_UNIT - 1);
        assert!(self.avail <= self.samples.len(), "buffer size was exceeded");
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
        for i in 0..self.samples.len() {
            if i < remain {
                self.samples[i] = self.samples[i + count];
            } else {
                self.samples[i] = 0;
            }
        }
    }

    /// Reads and removes at most `buf.len()` samples and writes them to `buf`. If
    /// `stereo` is true, writes output to every other element of `buf`, allowing easy
    /// interleaving of two buffers into a stereo sample stream. Outputs 16-bit signed
    /// samples. Returns number of samples actually read.
    pub fn read_samples(&mut self, buf: &mut [i16], stereo: bool) -> usize {
        let mut count = buf.len();
        if count > self.avail {
            count = self.avail;
        }

        if count != 0 {
            let step = if stereo { 2 } else { 1 };
            let mut in_index = 0;
            let end = in_index + count;
            let mut sum = self.integrator;
            let mut out_index = 0;
            while {
                let s = sum >> DELTA_BITS;
                sum += self.samples[in_index];
                in_index += 1;
                clamp(s);
                buf[out_index] = s as i16;
                out_index += step;
                sum -= s << (DELTA_BITS - BASS_SHIFT);
                in_index != end
            } {}
            self.integrator = sum;
            self.remove_samples(count);
        }
        count
    }
}

#[inline]
fn clamp(mut n: i32) -> i32 {
    if n as i16 as i32 != n {
        n = (n >> 16) ^ (MAX_SAMPLE as i32);
    }
    n
}

#[test]
fn check_assumptions() {
    const MIN_SAMPLE: i32 = -32768;

    let mut n: i32;

    assert!((-3 >> 1) == -2); /* right shift must preserve sign */
    n = MAX_SAMPLE * 2;
    n = clamp(n);
    assert!(n == MAX_SAMPLE);

    n = MIN_SAMPLE * 2;
    n = clamp(n);
    assert!(n == MIN_SAMPLE);

    assert!(MAX_RATIO as fixed_t <= TIME_UNIT);
    assert!(MAX_FRAME as fixed_t <= !1 >> TIME_BITS);
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

const BL_STEP: &'static [&'static [i16]] = &[
    &[43, -115, 350, -488, 1136, -914, 5861, 21022],
    &[44, -118, 348, -473, 1076, -799, 5274, 21001],
    &[45, -121, 344, -454, 1011, -677, 4706, 20936],
    &[46, -122, 336, -431, 942, -549, 4156, 20829],
    &[47, -123, 327, -404, 868, -418, 3629, 20679],
    &[47, -122, 316, -375, 792, -285, 3124, 20488],
    &[47, -120, 303, -344, 714, -151, 2644, 20256],
    &[46, -117, 289, -310, 634, -17, 2188, 19985],
    &[46, -114, 273, -275, 553, 117, 1758, 19675],
    &[44, -108, 255, -237, 471, 247, 1356, 19327],
    &[43, -103, 237, -199, 390, 373, 981, 18944],
    &[42, -98, 218, -160, 310, 495, 633, 18527],
    &[40, -91, 198, -121, 231, 611, 314, 18078],
    &[38, -84, 178, -81, 153, 722, 22, 17599],
    &[36, -76, 157, -43, 80, 824, -241, 17092],
    &[34, -68, 135, -3, 8, 919, -476, 16558],
    &[32, -61, 115, 34, -60, 1006, -683, 16001],
    &[29, -52, 94, 70, -123, 1083, -862, 15422],
    &[27, -44, 73, 106, -184, 1152, -1015, 14824],
    &[25, -36, 53, 139, -239, 1211, -1142, 14210],
    &[22, -27, 34, 170, -290, 1261, -1244, 13582],
    &[20, -20, 16, 199, -335, 1301, -1322, 12942],
    &[18, -12, -3, 226, -375, 1331, -1376, 12293],
    &[15, -4, -19, 250, -410, 1351, -1408, 11638],
    &[13, 3, -35, 272, -439, 1361, -1419, 10979],
    &[11, 9, -49, 292, -464, 1362, -1410, 10319],
    &[9, 16, -63, 309, -483, 1354, -1383, 9660],
    &[7, 22, -75, 322, -496, 1337, -1339, 9005],
    &[6, 26, -85, 333, -504, 1312, -1280, 8355],
    &[4, 31, -94, 341, -507, 1278, -1205, 7713],
    &[3, 35, -102, 347, -506, 1238, -1119, 7082],
    &[1, 40, -110, 350, -499, 1190, -1021, 6464],
    &[0, 43, -115, 350, -488, 1136, -914, 586],
];
