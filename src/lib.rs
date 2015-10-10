extern crate blip_buf_sys as ffi;
extern crate libc;

use libc::{c_double, c_int, c_short, c_uint};

pub const MAX_RATIO : i32 = 1048576;
pub const MAX_FRAME : u32 = 4000;

pub struct BlipBuf {
    ptr: *mut ffi::blip_t,
}

impl BlipBuf {
    pub fn new(sample_count: i32) -> BlipBuf {
        unsafe {
            let ptr = ffi::blip_new(sample_count as c_int);
            assert!(!ptr.is_null());

            BlipBuf { ptr: ptr }
        }
    }

    pub fn set_rates(&mut self, clock_rate: f64, sample_rate: f64) {
        unsafe {
            ffi::blip_set_rates(self.ptr, clock_rate as c_double, sample_rate as c_double);
        }
    }

    pub fn clear(&mut self) {
        unsafe {
            ffi::blip_clear(self.ptr);
        }
    }

    pub fn add_delta(&mut self, clock_time: u32, delta: i32) {
        unsafe {
            ffi::blip_add_delta(self.ptr, clock_time as c_uint, delta as c_int);
        }
    }

    pub fn add_delta_fast(&mut self, clock_time: u32, delta: i32) {
        unsafe {
            ffi::blip_add_delta_fast(self.ptr, clock_time as c_uint, delta as c_int);
        }
    }

    pub fn clocks_needed(&self, sample_count: i32) -> i32 {
        unsafe {
            ffi::blip_clocks_needed(self.ptr, sample_count as c_int) as i32
        }
    }

    pub fn end_frame(&mut self, clock_duration: u32) {
        unsafe {
            ffi::blip_end_frame(self.ptr, clock_duration as c_uint);
        }
    }

    pub fn samples_avail(&self) -> i32 {
        unsafe {
            ffi::blip_samples_avail(self.ptr) as i32
        }
    }

    pub fn read_samples(&mut self, buf: &mut [i16], stereo: bool) -> i32 {
        unsafe {
            ffi::blip_read_samples(self.ptr, buf.as_ptr() as *mut c_short, buf.len() as c_int, stereo as c_int) as i32
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
