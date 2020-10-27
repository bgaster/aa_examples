#![feature(wasm_target_feature)]

extern crate wee_alloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern "C" {
    fn print(i: i32);
}

mod playhead;
use playhead::*;

const NUM_TRACKS: usize = 4;
const MAX_BUFFER_SIZE: usize = 1024;

#[no_mangle]
static mut IN_BUFFER0: [f32;MAX_BUFFER_SIZE] = [0.;MAX_BUFFER_SIZE];
#[no_mangle]
static mut IN_BUFFER1: [f32;MAX_BUFFER_SIZE] = [0.;MAX_BUFFER_SIZE];
#[no_mangle]
static mut OUT_BUFFER0: [f32;MAX_BUFFER_SIZE] = [0.;MAX_BUFFER_SIZE];
#[no_mangle]
static mut OUT_BUFFER1: [f32;MAX_BUFFER_SIZE] = [0.;MAX_BUFFER_SIZE];
static mut INPUTS: [* const f32;2] = [0 as * const f32; 2];
static mut OUTPUTS: [* mut f32;2] = [0 as * mut f32; 2];

static mut ENGINE : FourTrack = FourTrack {
    sample_rate: 0,
    active_track: 0,
    recording: false,
    playing: false,
    mutes: [1.0;NUM_TRACKS],
    playheads: Vec::new(),
    param_history: (-1, -1.),
};

// Notes and voices

type Note = i32;   
type Pitch = f32;

struct FourTrack {
    sample_rate: i32,
    active_track: usize,
    recording: bool,
    playing: bool,
    mutes: [f32;NUM_TRACKS],
    playheads: Vec<Playhead>,

    /// record last param set
    param_history: (i32, f32),
}

impl FourTrack {
    pub fn get_voices(&self) -> i32 { 
		0
	}

	pub fn get_input(&self, index: u32) -> u32 { 
		unsafe { INPUTS[index as usize] as u32 }
    }
    
    pub fn get_output(&self, index: u32) -> u32 { 
		unsafe { OUTPUTS[index as usize] as u32 }
	}

	pub fn set_input(&self, index: u32, offset: u32) { 
		unsafe { INPUTS[index as usize] = offset as * const f32; };
	}

	pub fn set_output(&self, index: u32, offset: u32) { 
		unsafe { OUTPUTS[index as usize] = offset as * mut f32; };
	}

	pub fn get_sample_rate(&self) -> i32 {
        return self.sample_rate;
    }
    
    pub fn get_num_inputs(&self) -> i32 {
		return 2;
    }
    
	pub fn get_num_outputs(&self) -> i32 {
		return 2;
    }
    
    pub fn init(&mut self, sample_rate: i32) {
        self.sample_rate = sample_rate;
        self.playheads = vec![
            Playhead::new(sample_rate as usize, Playhead::ONE_MINUTE_SECONDS),
            Playhead::new(sample_rate as usize, Playhead::ONE_MINUTE_SECONDS),
            Playhead::new(sample_rate as usize, Playhead::ONE_MINUTE_SECONDS),
            Playhead::new(sample_rate as usize, Playhead::ONE_MINUTE_SECONDS)];
        self.init_buffers();
    }

    pub fn handle_note_on(&mut self, _mn: Note, _vel: f32) {
        // no required here
        // really we want to move some of these functions to being optional
    }
    
    pub fn handle_note_off(&mut self, _mn: Note, _vel: f32) {
    }
    
    fn init_buffers(&self) {
		unsafe {
            INPUTS[0] = IN_BUFFER0.as_ptr();
            INPUTS[1] = IN_BUFFER1.as_ptr();
			OUTPUTS[0] = OUT_BUFFER0.as_mut_ptr();
			OUTPUTS[1] = OUT_BUFFER1.as_mut_ptr();
		};
    }
    
    pub fn get_param(&self, param: u32) -> f32 {
		match param {
            0 => self.active_track as f32,
            1 => self.recording as i32 as f32,
            // mute track 1
            2 => self.mutes[0] as f32,
            // mute track 2
            3 => self.mutes[1] as f32,
            // mute track 3
            4 => self.mutes[2] as f32,
            // mute track 4
            5 => self.mutes[3] as f32,
            6 => self.playing as i32 as f32,
			_ => 0.,
		}
    }
    
    pub fn set_param(&mut self, param: u32, value: f32) {
        // unsafe {
        // print(self.param_history.0 as i32);
        // print(self.param_history.1 as i32);
        // }
		match param {
            0 => { 
                let v = value as usize;
                if v <= NUM_TRACKS {
                    self.active_track = v;
                }
            },
            // set recording
            1 => { 
                self.recording = value != 0.0; 
            },            
            // mute track
            2 | 3 | 4 | 5 => { 
                self.mutes[(param-2) as usize] = if value == 0.0 { 1.0 } else { 0.0 }; 
            },
            // set playing
            6 => { 
                self.playing = value != 0.;
            },
            // set loop in
            7 => {
                for p in &mut self.playheads {
                    p.loop_in(value as usize);
                }
            },
            // set loop out
            8 => { 
                for p in &mut self.playheads {
                    p.loop_out(value as usize);
                }
            },
            // loop
            9 => { 
                for p in &mut self.playheads {
                    p.looping(value != 0.0);
                }
            },
            // stop
            10 => {
                self.playing = false;
                self.recording = false;
                // if stop is sent twice in a row, then reset playheads
                if self.param_history.0 == 10 && self.param_history.1 == 0. {
                    for p in &mut self.playheads {
                        p.gate();
                    }
                }
            },
			_ => {}
        }
        self.param_history = (param as i32, value);
    }
    
    #[target_feature(enable = "simd128")]
    unsafe fn compute(&mut self, count: i32, inputs: &[&[f32];2], outputs: &mut [&mut [f32];2]) {
        let [inputs0, inputs1] = inputs; //inputs[0][..count as usize].iter();
        let (inputs0, inputs1) = {
			let inputs0 = inputs0[..count as usize].iter();
			let inputs1 = inputs1[..count as usize].iter();
			(inputs0, inputs1)
        };
		let [outputs0, outputs1] = outputs;
		let (outputs0, outputs1) = {
			let outputs0 = outputs0[..count as usize].iter_mut();
			let outputs1 = outputs1[..count as usize].iter_mut();
			(outputs0, outputs1)
        };

        // TODO :-)
        if self.playing {
            let zipped_inputs = inputs0.zip(inputs1).zip(outputs0).zip(outputs1);
            for (((input0, input1), output0), output1) in zipped_inputs {
                *output0 = 0.;
                *output1 = 0.;
                let playhead_mute = self.playheads.iter_mut().zip(self.mutes.iter());
                for (i, ( p, mute)) in playhead_mute.enumerate() {
                    let (sample0, sample1) = 
                        // overdub if recording and the track is active 
                        if self.recording && self.active_track == i {
                            p.overdub(*input0, *input1)
                        }
                        else {
                            p.read()
                        };
                    *output0 += sample0 * mute;
                    *output1 += sample1 * mute;
                }
            }
            // if self.recording {
            //     // recoding input onto active track
            //     let zipped_inputs = inputs0.zip(inputs1).zip(outputs0).zip(outputs1);
            //     for (((input0, input1), output0), output1) in zipped_inputs {
            //         *output0 = 0.;
            //         *output1 = 0.;
            //         let playhead_mute = self.playheads.iter_mut().zip(self.mutes.iter());
            //         for (p, mute) in playhead_mute {
            //             let (sample0, sample1) = p.overdub(*input0, *input1);
            //             *output0 += sample0 * mute;
            //             *output1 += sample1 * mute;
            //         }
            //     }
            // }
            // else { 
            //     // only playing so add in output from non muted tracks to input
            //     let zipped_inputs = inputs0.zip(inputs1).zip(outputs0).zip(outputs1);
            //     for (((input0, input1), output0), output1) in zipped_inputs {
            //         *output0 = 0.;
            //         *output1 = 0.;
            //         let playhead_mute = self.playheads.iter_mut().zip(self.mutes.iter());
            //         for (p, mute) in playhead_mute {
            //             let (sample0, sample1) = p.read();
            //             *output0 = (sample0 + *input0) * mute;
            //             *output1 = (sample1 + *input1) * mute;
            //         }
            //     }

            //     // let zipped_iterators = outputs0.zip(outputs1);
            //     // for (output0, output1) in zipped_iterators {
            //     //     *output0 = 0.;
            //     //     *output1 = 0.;
            //     //     for p in &mut self.playheads {
            //     //         let (sample0, sample1) = p.read();
            //     //         *output0 = sample0;
            //     //         *output1 = sample1;
            //     //     } 
            //     // }
            // }
        }
        else {
            // not playing so pass input through
            let zipped_inputs = inputs0.zip(inputs1).zip(outputs0).zip(outputs1);
            for (((input0, input1), output0), output1) in zipped_inputs {
                *output0 = *input0;
                *output1 = *input1;
            }
        }
    }

    #[inline]
	pub fn compute_external(&mut self, count: i32) {
		let (input0, input1, output0, output1) = unsafe {
            (::std::slice::from_raw_parts(INPUTS[0], count as usize),
             ::std::slice::from_raw_parts(INPUTS[1], count as usize),
			 ::std::slice::from_raw_parts_mut(OUTPUTS[0], count as usize),
			 ::std::slice::from_raw_parts_mut(OUTPUTS[1], count as usize))
		};
		unsafe { self.compute(count, &[input0, input1], &mut [output0, output1]); }
	}
}

#[no_mangle]
pub fn get_input(index: u32) -> u32 { 
    unsafe { ENGINE.get_input(index) }
}

#[no_mangle]
pub fn get_output(index: u32) -> u32 { 
    unsafe { ENGINE.get_output(index) }
}

#[no_mangle]
pub fn set_input(index: u32, offset: u32) { 
    unsafe { ENGINE.set_input(index, offset); };
}

#[no_mangle]
pub fn set_output(index: u32, offset: u32) { 
    unsafe { ENGINE.set_output(index, offset); };
}

#[no_mangle]
pub fn handle_note_on(mn: i32, vel: f32) {
    unsafe { ENGINE.handle_note_on(mn, vel); }
}

#[no_mangle]
pub fn handle_note_off(mn: i32, vel: f32) {
    unsafe { ENGINE.handle_note_off(mn, vel); }
}

#[no_mangle]
pub fn get_voices() -> i32 {
    unsafe { ENGINE.get_voices() }
}

#[no_mangle]
pub fn get_param_index(length: i32) -> i32 {
   0
}

#[no_mangle]
pub fn get_sample_rate() -> f64 {
    unsafe { ENGINE.get_sample_rate() as f64 }
}

// number of input channels (currently max 2)
#[no_mangle]
pub fn get_num_input_channels() -> u32 {
    unsafe { ENGINE.get_num_inputs() as u32 }
}

// number of output channels (currently max 2)
#[no_mangle]
pub fn get_num_output_channels() -> u32 {
    unsafe { ENGINE.get_num_outputs() as u32 }
}

#[no_mangle]
pub fn init(sample_rate: f64) -> () {
    unsafe { ENGINE.init(sample_rate as i32); }
}

#[no_mangle]
pub fn set_param_float(index: u32, v: f32) {
    unsafe { ENGINE.set_param(index, v); }
}

#[no_mangle]
pub fn set_param_int(index: u32, v: i32) {
    unsafe { ENGINE.set_param(index, v as f32); }
}

#[no_mangle]
pub fn get_param_float(index: u32) -> f32 {
    unsafe { ENGINE.get_param(index) }
}

#[no_mangle]
pub fn get_param_int(index: u32) -> i32 {
    unsafe { ENGINE.get_param(index) as i32 }
}

#[no_mangle]
pub fn compute(frames: u32) -> () {
    unsafe { ENGINE.compute_external(frames as i32); }
}