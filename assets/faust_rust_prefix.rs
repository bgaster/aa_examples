#![feature(wasm_target_feature)]

// ALL OF THE FOLLOWING WARNINGS NEED TO BE ADDRESSED IN THE FAUST COMPILER
#![allow(non_snake_case)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]
// REMOVE SOMETIME SOON :-)

const MAX_PARAM_SIZE: usize = 1024;
#[no_mangle]
pub static mut PARAM_NAME: [u8;MAX_PARAM_SIZE] = [65;MAX_PARAM_SIZE];

const MAX_BUFFER_SIZE: usize = 1024;

// Everything following should not be visible to the outside world

#[derive(Clone)]
struct ParamRange {
    init: f32,
    min: f32,
    max: f32,
    step: f32,
}

impl ParamRange {
    pub fn new(init: f32, min: f32, max: f32, step: f32) -> Self {
        Self {
            init,
            min,
            max,
            step,
        }
    }
}

#[derive(Clone)]
struct Param {
    index: i32,
    range: ParamRange,
}

impl Param {
	pub fn new(name: String, index: i32, range: ParamRange) -> Self {
		Self {
            index,
            range
		}
	}
}

// Notes and voices

type Note = i32;   
type Pitch = f32;

/// convert midi note to its corresponding frequency
#[inline]
fn to_freq(n: i32) -> Pitch {
    2.0f32.powf( (n - 69) as Pitch / 12.0 ) * 440.0
}

/// convert midi note to an index within voices range, can then be used as 
/// sample index, for example.
#[inline]
fn to_index(n: i32, voices: u32) -> u32 {
    (12.0*(to_freq(n) / 130.81).log2()).round().abs() as u32 % voices
}

#[inline]
fn freq_to_index(freq: f32, voices: u32) -> u32 {
    (12.0*(freq / 130.81).log2()).round().abs() as u32 % voices
}

/// convert midi note to its corresponding frequency, with explicit base tuning
#[inline]
fn to_freq_tuning(n:i32, tuning: Pitch) -> Pitch {
    2.0f32.powf( (n - 69) as f32 / 12.0 ) * tuning
}

#[derive(Copy, Clone)]
struct VoiceInfo {
    pub active: bool,
    pub note: Note,
    pub channel: i32,
	pub voice_age: i64,
}

impl VoiceInfo {
	pub fn new() -> VoiceInfo {
		VoiceInfo {
			active: false,
			note: 0,
			channel: 0,
			voice_age: 0,
		}
	}
}