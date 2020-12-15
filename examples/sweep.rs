extern crate blip_buf;

use std::time::Duration;
use blip_buf::BlipBuf;

const CLOCK_RATE : f64 = 1000000.0;
const SAMPLE_RATE : u32 = 48000;

fn main() {
    let mut blip = BlipBuf::new(SAMPLE_RATE / 10);
    blip.set_rates(CLOCK_RATE, SAMPLE_RATE as f64 );

    let mut time  = 0;      // number of clocks until next wave delta
    let mut delta = 10000;  // amplitude of next delta
    let mut period = 400;   // clocks between deltas

    for _n in 0..60 {
        // Slowly lower pitch every frame
        period = period + 3;

        // Generate 1/60 second of input clocks. We could generate
        // any number of clocks here, all the way down to 1.
        let clocks = CLOCK_RATE as i32 / 60;
        while time < clocks
        {
            blip.add_delta( time as u32, delta );
            delta = -delta; // square wave deltas alternate sign
            time = time + period;
        }

        // Add those clocks to buffer and adjust time for next frame
        time = time - clocks;
        blip.end_frame( clocks as u32 );

        // Read and play any output samples now available
        while blip.samples_avail() > 0
        {
            let temp = &mut [0i16; 1024];
            let count = blip.read_samples( temp, false );
            play_samples( &temp[..count] );
        }
    }

    // wait until the sound finishes
    std::thread::sleep(Duration::from_secs(1));
}

fn play_samples(_buf: &[i16]) {
    // play contents of _buf
}
