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
fn mydsp_faustpower2_f(value: f32) -> f32 {
	return (value * value);
}

#[no_mangle]
static mut OUT_BUFFER0: [f32;MAX_BUFFER_SIZE] = [0.;MAX_BUFFER_SIZE];
static mut INPUTS: [* const f32;0] = [0 as * const f32; 0];
static mut OUTPUTS: [* mut f32;1] = [0 as * mut f32; 1];
static mut ENGINE : mydsp = mydsp {
	fHslider0: 0.0,
	iVec0: [0;2],
	fSampleRate: 0,
	fConst0: 0.0,
	fConst1: 0.0,
	fButton0: 0.0,
	fHslider1: 0.0,
	iRec1: [0;2],
	fHslider2: 0.0,
	fHslider3: 0.0,
	fHslider4: 0.0,
	fRec0: [0.0;2],
	fHslider5: 0.0,
	fRec4: [0.0;2],
	fHslider6: 0.0,
	fRec5: [0.0;2],
	fVec1: [0.0;2],
	fConst2: 0.0,
	fHslider7: 0.0,
	fVec2: [0.0;2],
	fConst3: 0.0,
	fRec6: [0.0;2],
	fVec3: [0.0;2],
	IOTA: 0,
	fVec4: [0.0;4096],
	fConst4: 0.0,
	fConst5: 0.0,
	fRec7: [0.0;2],
	fConst6: 0.0,
	iRec9: [0;2],
	fRec8: [0.0;2],
	fVec5: [0.0;2],
	fHslider8: 0.0,
	fRec10: [0.0;2],
	iRec12: [0;2],
	fRec11: [0.0;2],
	fVec6: [0.0;2],
	fVec7: [0.0;2],
	fRec13: [0.0;2],
	fVec8: [0.0;2],
	fVec9: [0.0;4096],
	iRec15: [0;2],
	fRec14: [0.0;2],
	fVec10: [0.0;2],
	fVec11: [0.0;2],
	fRec16: [0.0;2],
	fVec12: [0.0;2],
	fVec13: [0.0;4096],
	fConst7: 0.0,
	fRec17: [0.0;2],
	fRec18: [0.0;2],
	fVec14: [0.0;2],
	fRec19: [0.0;2],
	fVec15: [0.0;2],
	fVec16: [0.0;4096],
	fRec20: [0.0;2],
	iRec22: [0;2],
	fRec21: [0.0;2],
	fVec17: [0.0;2],
	fConst8: 0.0,
	fHslider9: 0.0,
	fRec23: [0.0;2],
	fRec3: [0.0;3],
	fRec2: [0.0;3],
	fHslider10: 0.0,
	fButton1: 0.0,
	iRec25: [0;2],
	fRec24: [0.0;2],
	fHslider11: 0.0,
	fVec18: [0.0;2],
	fRec28: [0.0;2],
	fVec19: [0.0;2],
	fVec20: [0.0;4096],
	fRec29: [0.0;2],
	iRec31: [0;2],
	fRec30: [0.0;2],
	fVec21: [0.0;2],
	iRec33: [0;2],
	fRec32: [0.0;2],
	fVec22: [0.0;2],
	fVec23: [0.0;2],
	fRec34: [0.0;2],
	fVec24: [0.0;2],
	fVec25: [0.0;4096],
	iRec36: [0;2],
	fRec35: [0.0;2],
	fVec26: [0.0;2],
	fVec27: [0.0;2],
	fRec37: [0.0;2],
	fVec28: [0.0;2],
	fVec29: [0.0;4096],
	fRec38: [0.0;2],
	fRec39: [0.0;2],
	fVec30: [0.0;2],
	fRec40: [0.0;2],
	fVec31: [0.0;2],
	fVec32: [0.0;4096],
	fRec41: [0.0;2],
	iRec43: [0;2],
	fRec42: [0.0;2],
	fVec33: [0.0;2],
	fRec27: [0.0;3],
	fRec26: [0.0;3],
	next_allocated_voice_age: 1000000000,
	next_unallocated_voice_age: 0,
	voices: [VoiceInfo {active: false,note: 0,channel: 0,voice_age: 0,};2],
	voice_freq: [0;2],
	voice_gain: [0;2],
	voice_gate: [0;2],
};

type T = f32;

struct mydsp {
	fHslider0: f32,
	iVec0: [i32;2],
	fSampleRate: i32,
	fConst0: f32,
	fConst1: f32,
	fButton0: f32,
	fHslider1: f32,
	iRec1: [i32;2],
	fHslider2: f32,
	fHslider3: f32,
	fHslider4: f32,
	fRec0: [f32;2],
	fHslider5: f32,
	fRec4: [f32;2],
	fHslider6: f32,
	fRec5: [f32;2],
	fVec1: [f32;2],
	fConst2: f32,
	fHslider7: f32,
	fVec2: [f32;2],
	fConst3: f32,
	fRec6: [f32;2],
	fVec3: [f32;2],
	IOTA: i32,
	fVec4: [f32;4096],
	fConst4: f32,
	fConst5: f32,
	fRec7: [f32;2],
	fConst6: f32,
	iRec9: [i32;2],
	fRec8: [f32;2],
	fVec5: [f32;2],
	fHslider8: f32,
	fRec10: [f32;2],
	iRec12: [i32;2],
	fRec11: [f32;2],
	fVec6: [f32;2],
	fVec7: [f32;2],
	fRec13: [f32;2],
	fVec8: [f32;2],
	fVec9: [f32;4096],
	iRec15: [i32;2],
	fRec14: [f32;2],
	fVec10: [f32;2],
	fVec11: [f32;2],
	fRec16: [f32;2],
	fVec12: [f32;2],
	fVec13: [f32;4096],
	fConst7: f32,
	fRec17: [f32;2],
	fRec18: [f32;2],
	fVec14: [f32;2],
	fRec19: [f32;2],
	fVec15: [f32;2],
	fVec16: [f32;4096],
	fRec20: [f32;2],
	iRec22: [i32;2],
	fRec21: [f32;2],
	fVec17: [f32;2],
	fConst8: f32,
	fHslider9: f32,
	fRec23: [f32;2],
	fRec3: [f32;3],
	fRec2: [f32;3],
	fHslider10: f32,
	fButton1: f32,
	iRec25: [i32;2],
	fRec24: [f32;2],
	fHslider11: f32,
	fVec18: [f32;2],
	fRec28: [f32;2],
	fVec19: [f32;2],
	fVec20: [f32;4096],
	fRec29: [f32;2],
	iRec31: [i32;2],
	fRec30: [f32;2],
	fVec21: [f32;2],
	iRec33: [i32;2],
	fRec32: [f32;2],
	fVec22: [f32;2],
	fVec23: [f32;2],
	fRec34: [f32;2],
	fVec24: [f32;2],
	fVec25: [f32;4096],
	iRec36: [i32;2],
	fRec35: [f32;2],
	fVec26: [f32;2],
	fVec27: [f32;2],
	fRec37: [f32;2],
	fVec28: [f32;2],
	fVec29: [f32;4096],
	fRec38: [f32;2],
	fRec39: [f32;2],
	fVec30: [f32;2],
	fRec40: [f32;2],
	fVec31: [f32;2],
	fVec32: [f32;4096],
	fRec41: [f32;2],
	iRec43: [i32;2],
	fRec42: [f32;2],
	fVec33: [f32;2],
	fRec27: [f32;3],
	fRec26: [f32;3],

	next_allocated_voice_age: i64,
	next_unallocated_voice_age: i64,
	voices: [VoiceInfo;2],
	voice_freq: [u32;2],
	voice_gain: [u32;2],
	voice_gate: [u32;2],
}

impl mydsp {
		
	fn new() -> mydsp { 
		mydsp {
			fHslider0: 0.0,
			iVec0: [0;2],
			fSampleRate: 0,
			fConst0: 0.0,
			fConst1: 0.0,
			fButton0: 0.0,
			fHslider1: 0.0,
			iRec1: [0;2],
			fHslider2: 0.0,
			fHslider3: 0.0,
			fHslider4: 0.0,
			fRec0: [0.0;2],
			fHslider5: 0.0,
			fRec4: [0.0;2],
			fHslider6: 0.0,
			fRec5: [0.0;2],
			fVec1: [0.0;2],
			fConst2: 0.0,
			fHslider7: 0.0,
			fVec2: [0.0;2],
			fConst3: 0.0,
			fRec6: [0.0;2],
			fVec3: [0.0;2],
			IOTA: 0,
			fVec4: [0.0;4096],
			fConst4: 0.0,
			fConst5: 0.0,
			fRec7: [0.0;2],
			fConst6: 0.0,
			iRec9: [0;2],
			fRec8: [0.0;2],
			fVec5: [0.0;2],
			fHslider8: 0.0,
			fRec10: [0.0;2],
			iRec12: [0;2],
			fRec11: [0.0;2],
			fVec6: [0.0;2],
			fVec7: [0.0;2],
			fRec13: [0.0;2],
			fVec8: [0.0;2],
			fVec9: [0.0;4096],
			iRec15: [0;2],
			fRec14: [0.0;2],
			fVec10: [0.0;2],
			fVec11: [0.0;2],
			fRec16: [0.0;2],
			fVec12: [0.0;2],
			fVec13: [0.0;4096],
			fConst7: 0.0,
			fRec17: [0.0;2],
			fRec18: [0.0;2],
			fVec14: [0.0;2],
			fRec19: [0.0;2],
			fVec15: [0.0;2],
			fVec16: [0.0;4096],
			fRec20: [0.0;2],
			iRec22: [0;2],
			fRec21: [0.0;2],
			fVec17: [0.0;2],
			fConst8: 0.0,
			fHslider9: 0.0,
			fRec23: [0.0;2],
			fRec3: [0.0;3],
			fRec2: [0.0;3],
			fHslider10: 0.0,
			fButton1: 0.0,
			iRec25: [0;2],
			fRec24: [0.0;2],
			fHslider11: 0.0,
			fVec18: [0.0;2],
			fRec28: [0.0;2],
			fVec19: [0.0;2],
			fVec20: [0.0;4096],
			fRec29: [0.0;2],
			iRec31: [0;2],
			fRec30: [0.0;2],
			fVec21: [0.0;2],
			iRec33: [0;2],
			fRec32: [0.0;2],
			fVec22: [0.0;2],
			fVec23: [0.0;2],
			fRec34: [0.0;2],
			fVec24: [0.0;2],
			fVec25: [0.0;4096],
			iRec36: [0;2],
			fRec35: [0.0;2],
			fVec26: [0.0;2],
			fVec27: [0.0;2],
			fRec37: [0.0;2],
			fVec28: [0.0;2],
			fVec29: [0.0;4096],
			fRec38: [0.0;2],
			fRec39: [0.0;2],
			fVec30: [0.0;2],
			fRec40: [0.0;2],
			fVec31: [0.0;2],
			fVec32: [0.0;4096],
			fRec41: [0.0;2],
			iRec43: [0;2],
			fRec42: [0.0;2],
			fVec33: [0.0;2],
			fRec27: [0.0;3],
			fRec26: [0.0;3],
				next_allocated_voice_age: 1000000000,
				next_unallocated_voice_age: 0,
				voices: [VoiceInfo {active: false,note: 0,channel: 0,voice_age: 0,};2],
				voice_freq: [0;2],
				voice_gain: [0;2],
				voice_gate: [0;2],
		}
	}
	pub fn get_voices(&self) -> i32 { 
		2
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
		return self.fSampleRate;
	}
	pub fn get_num_inputs(&self) -> i32 {
		return 0;
	}
	pub fn get_num_outputs(&self) -> i32 {
		return 1;
	}
	pub fn get_input_rate(&self, channel: i32) -> i32 {
		let mut rate: i32;
		match (channel) {
			_ => {
				rate = -1;
			},
		} 
		return rate;
	}
	pub fn get_output_rate(&self, channel: i32) -> i32 {
		let mut rate: i32;
		match (channel) {
			0 => {
				rate = 1;
			},
			_ => {
				rate = -1;
			},
		} 
		return rate;
	}
	
	fn class_init(sample_rate: i32) {
	}
	fn instance_reset_params(&mut self) {
		self.fHslider0 = 1.0;
		self.fButton0 = 0.0;
		self.fHslider1 = 0.0;
		self.fHslider2 = 0.00100000005;
		self.fHslider3 = 0.0;
		self.fHslider4 = 1.0;
		self.fHslider5 = 0.0;
		self.fHslider6 = 1.0;
		self.fHslider7 = 440.0;
		self.fHslider8 = 2.0;
		self.fHslider9 = 0.5;
		self.fHslider10 = 1.0;
		self.fButton1 = 0.0;
		self.fHslider11 = 440.0;
	}
	fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.iVec0[l0 as usize] = 0;
		}
		for l1 in 0..2 {
			self.iRec1[l1 as usize] = 0;
		}
		for l2 in 0..2 {
			self.fRec0[l2 as usize] = 0.0;
		}
		for l3 in 0..2 {
			self.fRec4[l3 as usize] = 0.0;
		}
		for l4 in 0..2 {
			self.fRec5[l4 as usize] = 0.0;
		}
		for l5 in 0..2 {
			self.fVec1[l5 as usize] = 0.0;
		}
		for l6 in 0..2 {
			self.fVec2[l6 as usize] = 0.0;
		}
		for l7 in 0..2 {
			self.fRec6[l7 as usize] = 0.0;
		}
		for l8 in 0..2 {
			self.fVec3[l8 as usize] = 0.0;
		}
		self.IOTA = 0;
		for l9 in 0..4096 {
			self.fVec4[l9 as usize] = 0.0;
		}
		for l10 in 0..2 {
			self.fRec7[l10 as usize] = 0.0;
		}
		for l11 in 0..2 {
			self.iRec9[l11 as usize] = 0;
		}
		for l12 in 0..2 {
			self.fRec8[l12 as usize] = 0.0;
		}
		for l13 in 0..2 {
			self.fVec5[l13 as usize] = 0.0;
		}
		for l14 in 0..2 {
			self.fRec10[l14 as usize] = 0.0;
		}
		for l15 in 0..2 {
			self.iRec12[l15 as usize] = 0;
		}
		for l16 in 0..2 {
			self.fRec11[l16 as usize] = 0.0;
		}
		for l17 in 0..2 {
			self.fVec6[l17 as usize] = 0.0;
		}
		for l18 in 0..2 {
			self.fVec7[l18 as usize] = 0.0;
		}
		for l19 in 0..2 {
			self.fRec13[l19 as usize] = 0.0;
		}
		for l20 in 0..2 {
			self.fVec8[l20 as usize] = 0.0;
		}
		for l21 in 0..4096 {
			self.fVec9[l21 as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.iRec15[l22 as usize] = 0;
		}
		for l23 in 0..2 {
			self.fRec14[l23 as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.fVec10[l24 as usize] = 0.0;
		}
		for l25 in 0..2 {
			self.fVec11[l25 as usize] = 0.0;
		}
		for l26 in 0..2 {
			self.fRec16[l26 as usize] = 0.0;
		}
		for l27 in 0..2 {
			self.fVec12[l27 as usize] = 0.0;
		}
		for l28 in 0..4096 {
			self.fVec13[l28 as usize] = 0.0;
		}
		for l29 in 0..2 {
			self.fRec17[l29 as usize] = 0.0;
		}
		for l30 in 0..2 {
			self.fRec18[l30 as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fVec14[l31 as usize] = 0.0;
		}
		for l32 in 0..2 {
			self.fRec19[l32 as usize] = 0.0;
		}
		for l33 in 0..2 {
			self.fVec15[l33 as usize] = 0.0;
		}
		for l34 in 0..4096 {
			self.fVec16[l34 as usize] = 0.0;
		}
		for l35 in 0..2 {
			self.fRec20[l35 as usize] = 0.0;
		}
		for l36 in 0..2 {
			self.iRec22[l36 as usize] = 0;
		}
		for l37 in 0..2 {
			self.fRec21[l37 as usize] = 0.0;
		}
		for l38 in 0..2 {
			self.fVec17[l38 as usize] = 0.0;
		}
		for l39 in 0..2 {
			self.fRec23[l39 as usize] = 0.0;
		}
		for l40 in 0..3 {
			self.fRec3[l40 as usize] = 0.0;
		}
		for l41 in 0..3 {
			self.fRec2[l41 as usize] = 0.0;
		}
		for l42 in 0..2 {
			self.iRec25[l42 as usize] = 0;
		}
		for l43 in 0..2 {
			self.fRec24[l43 as usize] = 0.0;
		}
		for l44 in 0..2 {
			self.fVec18[l44 as usize] = 0.0;
		}
		for l45 in 0..2 {
			self.fRec28[l45 as usize] = 0.0;
		}
		for l46 in 0..2 {
			self.fVec19[l46 as usize] = 0.0;
		}
		for l47 in 0..4096 {
			self.fVec20[l47 as usize] = 0.0;
		}
		for l48 in 0..2 {
			self.fRec29[l48 as usize] = 0.0;
		}
		for l49 in 0..2 {
			self.iRec31[l49 as usize] = 0;
		}
		for l50 in 0..2 {
			self.fRec30[l50 as usize] = 0.0;
		}
		for l51 in 0..2 {
			self.fVec21[l51 as usize] = 0.0;
		}
		for l52 in 0..2 {
			self.iRec33[l52 as usize] = 0;
		}
		for l53 in 0..2 {
			self.fRec32[l53 as usize] = 0.0;
		}
		for l54 in 0..2 {
			self.fVec22[l54 as usize] = 0.0;
		}
		for l55 in 0..2 {
			self.fVec23[l55 as usize] = 0.0;
		}
		for l56 in 0..2 {
			self.fRec34[l56 as usize] = 0.0;
		}
		for l57 in 0..2 {
			self.fVec24[l57 as usize] = 0.0;
		}
		for l58 in 0..4096 {
			self.fVec25[l58 as usize] = 0.0;
		}
		for l59 in 0..2 {
			self.iRec36[l59 as usize] = 0;
		}
		for l60 in 0..2 {
			self.fRec35[l60 as usize] = 0.0;
		}
		for l61 in 0..2 {
			self.fVec26[l61 as usize] = 0.0;
		}
		for l62 in 0..2 {
			self.fVec27[l62 as usize] = 0.0;
		}
		for l63 in 0..2 {
			self.fRec37[l63 as usize] = 0.0;
		}
		for l64 in 0..2 {
			self.fVec28[l64 as usize] = 0.0;
		}
		for l65 in 0..4096 {
			self.fVec29[l65 as usize] = 0.0;
		}
		for l66 in 0..2 {
			self.fRec38[l66 as usize] = 0.0;
		}
		for l67 in 0..2 {
			self.fRec39[l67 as usize] = 0.0;
		}
		for l68 in 0..2 {
			self.fVec30[l68 as usize] = 0.0;
		}
		for l69 in 0..2 {
			self.fRec40[l69 as usize] = 0.0;
		}
		for l70 in 0..2 {
			self.fVec31[l70 as usize] = 0.0;
		}
		for l71 in 0..4096 {
			self.fVec32[l71 as usize] = 0.0;
		}
		for l72 in 0..2 {
			self.fRec41[l72 as usize] = 0.0;
		}
		for l73 in 0..2 {
			self.iRec43[l73 as usize] = 0;
		}
		for l74 in 0..2 {
			self.fRec42[l74 as usize] = 0.0;
		}
		for l75 in 0..2 {
			self.fVec33[l75 as usize] = 0.0;
		}
		for l76 in 0..3 {
			self.fRec27[l76 as usize] = 0.0;
		}
		for l77 in 0..3 {
			self.fRec26[l77 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		self.fConst0 = f32::min(192000.0, f32::max(1.0, (self.fSampleRate as f32)));
		self.fConst1 = (6.90999985 / self.fConst0);
		self.fConst2 = (0.25 * self.fConst0);
		self.fConst3 = (1.0 / self.fConst0);
		self.fConst4 = (3.0 / self.fConst0);
		self.fConst5 = (0.5 * self.fConst0);
		self.fConst6 = (2.0 * self.fConst0);
		self.fConst7 = (6.0 / self.fConst0);
		self.fConst8 = (3.14159274 / self.fConst0);
	}
	fn instance_init(&mut self, sample_rate: i32) {
		self.instance_constants(sample_rate);
		self.instance_reset_params();
		self.instance_clear();
	}
	pub fn init(&mut self, sample_rate: i32) {
		mydsp::class_init(sample_rate);
		self.instance_init(sample_rate);
		self.init_voices();
		self.init_buffers();
	}
	pub fn get_param_info(&mut self, name: &str) -> Param {
		match name {
			"Filter" => Param { index: 0, range: ParamRange::new(0.5, 0.0, 1.0, 0.01) },
			"Relation" => Param { index: 1, range: ParamRange::new(2.0, 0.0, 3.0009999999999999, 0.001) },
			"Sub" => Param { index: 2, range: ParamRange::new(0.0, 0.0, 1.0, 0.001) },
			"Wave" => Param { index: 3, range: ParamRange::new(1.0, 0.080000000000000002, 4.0, 0.01) },
			"Attack" => Param { index: 4, range: ParamRange::new(0.001, 0.001, 4.0, 0.001) },
			"Decay" => Param { index: 5, range: ParamRange::new(0.0, 0.0, 4.0, 0.001) },
			"Release" => Param { index: 6, range: ParamRange::new(0.0, 0.0, 4.0, 0.01) },
			"Sustain" => Param { index: 7, range: ParamRange::new(1.0, 0.0, 1.0, 0.01) },
			
			"freq_v0" => Param { index: 8, range: ParamRange::new(440.0, 20.0, 1000.0, 1.0) },
			"gain_v0" => Param { index: 9, range: ParamRange::new(1.0, 0.0, 1.0, 0.007874015748031496) },
			"gate_v0" => Param { index: 10, range: ParamRange::new(0.0,0.0,0.0,0.0) },
			
			
			"freq_v1" => Param { index: 11, range: ParamRange::new(440.0, 20.0, 1000.0, 1.0) },
			"gain_v1" => Param { index: 12, range: ParamRange::new(1.0, 0.0, 1.0, 0.007874015748031496) },
			"gate_v1" => Param { index: 13, range: ParamRange::new(0.0,0.0,0.0,0.0) },
			
			
			
			
			_ => Param { index: -1, range: ParamRange::new(0.0, 0.0, 0.0, 0.0)}
		}
	}
	fn init_voices(&mut self) {
		self.voice_freq[0] = self.get_param_info("freq_v0").index as u32;
		self.voice_gain[0] = self.get_param_info("gain_v0").index as u32;
		self.voice_gate[0] = self.get_param_info("gate_v0").index as u32;
		self.voice_freq[1] = self.get_param_info("freq_v1").index as u32;
		self.voice_gain[1] = self.get_param_info("gain_v1").index as u32;
		self.voice_gate[1] = self.get_param_info("gate_v1").index as u32;
	}
	pub fn handle_note_on(&mut self, mn: Note, vel: f32) {
		let mut allocated_voice = 0;
		let mut allocated_voice_age = self.voices[allocated_voice].voice_age;
		// find the oldest voice to reuse
		for i in 0..2 {
			let age = self.voices[i].voice_age;
			if age < allocated_voice_age {
				allocated_voice_age = age;
				allocated_voice = i;
			}
		}
		// update the VoiceInfo for our chosen voice
		self.voices[allocated_voice].channel   = 0;
		self.voices[allocated_voice].note      = mn;
		self.voices[allocated_voice].voice_age = self.next_allocated_voice_age;
		self.next_allocated_voice_age          = self.next_allocated_voice_age + 1;// set params for choosen voice
		self.set_param(self.voice_gate[allocated_voice], 1.0);
		self.set_param(self.voice_gain[allocated_voice], vel);
		self.set_param(self.voice_freq[allocated_voice], to_freq(mn));
	}

	pub fn handle_note_off(&mut self, mn: Note, vel: f32) {
		for voice in 0..2 {
			if self.voices[voice].note == mn {
				// mark voice as being unused
				self.voices[voice].voice_age = self.next_unallocated_voice_age;
				self.next_unallocated_voice_age = self.next_unallocated_voice_age + 1;
				// set params for choosen voice
				self.set_param(self.voice_gate[voice], 0.0);
				self.set_param(self.voice_gain[voice], vel);
			}
		}
	}

	fn init_buffers(&self) {
		unsafe {
			OUTPUTS[0] = OUT_BUFFER0.as_mut_ptr();
		};
	}
	
	pub fn get_param(&self, param: u32) -> T {
		match param {
			10 => self.fButton0,
			13 => self.fButton1,
			9 => self.fHslider0,
			6 => self.fHslider1,
			12 => self.fHslider10,
			11 => self.fHslider11,
			4 => self.fHslider2,
			5 => self.fHslider3,
			7 => self.fHslider4,
			2 => self.fHslider5,
			3 => self.fHslider6,
			8 => self.fHslider7,
			1 => self.fHslider8,
			0 => self.fHslider9,
			_ => 0.,
		}
	}
	
	pub fn set_param(&mut self, param: u32, value: T) {
		match param {
			10 => { self.fButton0 = value }
			13 => { self.fButton1 = value }
			9 => { self.fHslider0 = value }
			6 => { self.fHslider1 = value }
			12 => { self.fHslider10 = value }
			11 => { self.fHslider11 = value }
			4 => { self.fHslider2 = value }
			5 => { self.fHslider3 = value }
			7 => { self.fHslider4 = value }
			2 => { self.fHslider5 = value }
			3 => { self.fHslider6 = value }
			8 => { self.fHslider7 = value }
			1 => { self.fHslider8 = value }
			0 => { self.fHslider9 = value }
			_ => {}
		}
	}
	#[target_feature(enable = "simd128")]
	#[inline]
	unsafe fn compute(&mut self, count: i32, inputs: &[T], outputs: &mut [&mut [T];1]) {
		let [outputs0] = outputs;
		let (outputs0) = {
			let outputs0 = outputs0[..count as usize].iter_mut();
			(outputs0)
		};
		let mut fSlow0: f32 = (self.fHslider0 as f32);
		let mut iSlow1: i32 = (((self.fButton0 as f32) > 0.0) as i32);
		let mut fSlow2: f32 = (self.fHslider1 as f32);
		let mut fSlow3: f32 = (self.fHslider2 as f32);
		let mut iSlow4: i32 = ((self.fConst0 * fSlow3) as i32);
		let mut fSlow5: f32 = (self.fHslider3 as f32);
		let mut fSlow6: f32 = (6.90999985 * fSlow3);
		let mut fSlow7: f32 = (self.fHslider4 as f32);
		let mut fSlow8: f32 = (fSlow7 * (iSlow1 as f32));
		let mut fSlow9: f32 = (0.00100000005 * (self.fHslider5 as f32));
		let mut fSlow10: f32 = (0.00100000005 * (self.fHslider6 as f32));
		let mut fSlow11: f32 = (self.fHslider7 as f32);
		let mut fSlow12: f32 = f32::max((0.5 * fSlow11), 23.4489498);
		let mut fSlow13: f32 = f32::max(20.0, f32::abs(fSlow12));
		let mut fSlow14: f32 = (self.fConst2 / fSlow13);
		let mut fSlow15: f32 = (self.fConst0 / fSlow12);
		let mut fSlow16: f32 = (self.fConst4 * fSlow11);
		let mut fSlow17: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst5 / fSlow12)));
		let mut fSlow18: f32 = f32::floor(fSlow17);
		let mut fSlow19: f32 = (fSlow18 + (1.0 - fSlow17));
		let mut iSlow20: i32 = (fSlow17 as i32);
		let mut fSlow21: f32 = (fSlow17 - fSlow18);
		let mut iSlow22: i32 = (iSlow20 + 1);
		let mut fSlow23: f32 = (self.fConst0 / fSlow11);
		let mut iSlow24: i32 = ((self.fConst6 / fSlow11) as i32);
		let mut fSlow25: f32 = (self.fConst3 * fSlow11);
		let mut fSlow26: f32 = (0.00100000005 * (self.fHslider8 as f32));
		let mut fSlow27: f32 = (1.0 / fSlow11);
		let mut fSlow28: f32 = (self.fConst7 * fSlow11);
		let mut fSlow29: f32 = f32::max(fSlow11, 23.4489498);
		let mut fSlow30: f32 = f32::max(20.0, f32::abs(fSlow29));
		let mut fSlow31: f32 = (self.fConst2 / fSlow30);
		let mut fSlow32: f32 = (self.fConst0 / fSlow29);
		let mut fSlow33: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst5 / fSlow29)));
		let mut fSlow34: f32 = f32::floor(fSlow33);
		let mut fSlow35: f32 = (fSlow34 + (1.0 - fSlow33));
		let mut iSlow36: i32 = (fSlow33 as i32);
		let mut fSlow37: f32 = (fSlow33 - fSlow34);
		let mut iSlow38: i32 = (iSlow36 + 1);
		let mut iSlow39: i32 = (fSlow23 as i32);
		let mut fSlow40: f32 = (0.00100000005 * (self.fHslider9 as f32));
		let mut fSlow41: f32 = (self.fHslider10 as f32);
		let mut iSlow42: i32 = (((self.fButton1 as f32) > 0.0) as i32);
		let mut fSlow43: f32 = (fSlow7 * (iSlow42 as f32));
		let mut fSlow44: f32 = (self.fHslider11 as f32);
		let mut fSlow45: f32 = f32::max((0.5 * fSlow44), 23.4489498);
		let mut fSlow46: f32 = f32::max(20.0, f32::abs(fSlow45));
		let mut fSlow47: f32 = (self.fConst2 / fSlow46);
		let mut fSlow48: f32 = (self.fConst0 / fSlow45);
		let mut fSlow49: f32 = (self.fConst4 * fSlow44);
		let mut fSlow50: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst5 / fSlow45)));
		let mut fSlow51: f32 = f32::floor(fSlow50);
		let mut fSlow52: f32 = (fSlow51 + (1.0 - fSlow50));
		let mut iSlow53: i32 = (fSlow50 as i32);
		let mut fSlow54: f32 = (fSlow50 - fSlow51);
		let mut iSlow55: i32 = (iSlow53 + 1);
		let mut fSlow56: f32 = (self.fConst0 / fSlow44);
		let mut iSlow57: i32 = ((self.fConst6 / fSlow44) as i32);
		let mut fSlow58: f32 = (self.fConst3 * fSlow44);
		let mut fSlow59: f32 = (1.0 / fSlow44);
		let mut fSlow60: f32 = (self.fConst7 * fSlow44);
		let mut fSlow61: f32 = f32::max(fSlow44, 23.4489498);
		let mut fSlow62: f32 = f32::max(20.0, f32::abs(fSlow61));
		let mut fSlow63: f32 = (self.fConst2 / fSlow62);
		let mut fSlow64: f32 = (self.fConst0 / fSlow61);
		let mut fSlow65: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst5 / fSlow61)));
		let mut fSlow66: f32 = f32::floor(fSlow65);
		let mut fSlow67: f32 = (fSlow66 + (1.0 - fSlow65));
		let mut iSlow68: i32 = (fSlow65 as i32);
		let mut fSlow69: f32 = (fSlow65 - fSlow66);
		let mut iSlow70: i32 = (iSlow68 + 1);
		let mut iSlow71: i32 = (fSlow56 as i32);
		let zipped_iterators = outputs0;
		for output0 in zipped_iterators {
			self.iVec0[0] = 1;
			self.iRec1[0] = (iSlow1 * (self.iRec1[1] + 1));
			let mut iTemp0: i32 = ((self.iRec1[0] < iSlow4) as i32);
			let mut fTemp1: f32 = f32::exp((0.0 - (self.fConst1 / if (iSlow1 as i32 != 0) { if (iTemp0 as i32 != 0) { fSlow6 } else { fSlow5 } } else { fSlow2 })));
			self.fRec0[0] = ((self.fRec0[1] * fTemp1) + (if (iSlow1 as i32 != 0) { if (iTemp0 as i32 != 0) { 1.58730161 } else { fSlow8 } } else { 0.0 } * (1.0 - fTemp1)));
			self.fRec4[0] = (fSlow9 + (0.999000013 * self.fRec4[1]));
			self.fRec5[0] = (fSlow10 + (0.999000013 * self.fRec5[1]));
			let mut fTemp2: f32 = f32::min(1.0, f32::max(0.0, (2.0 - self.fRec5[0])));
			self.fVec1[0] = 0.25;
			self.fVec2[0] = fSlow13;
			let mut fTemp3: f32 = (self.iVec0[1] as f32);
			let mut fTemp4: f32 = (self.fRec6[1] + (self.fConst3 * self.fVec2[1]));
			self.fRec6[0] = (fTemp4 - f32::floor(fTemp4));
			let mut fTemp5: f32 = mydsp_faustpower2_f(((2.0 * self.fRec6[0]) + -1.0));
			self.fVec3[0] = fTemp5;
			let mut fTemp6: f32 = (fSlow14 * (fTemp3 * (fTemp5 - self.fVec3[1])));
			self.fVec4[(self.IOTA & 4095) as usize] = fTemp6;
			let mut fTemp7: f32 = f32::min(0.5, (0.5 * self.fRec5[0]));
			let mut fTemp8: f32 = f32::max(0.0, f32::min(2047.0, (fSlow15 * fTemp7)));
			let mut iTemp9: i32 = (fTemp8 as i32);
			let mut fTemp10: f32 = f32::floor(fTemp8);
			self.fRec7[0] = ((fTemp6 + (0.999000013 * self.fRec7[1])) - ((fSlow19 * self.fVec4[((self.IOTA - iSlow20) & 4095) as usize]) + (fSlow21 * self.fVec4[((self.IOTA - iSlow22) & 4095) as usize])));
			let mut fTemp11: f32 = f32::min(1.0, f32::max(0.0, (self.fRec5[0] + -2.0)));
			let mut fTemp12: f32 = (1.0 - (fTemp2 + fTemp11));
			let mut fTemp13: f32 = (self.fVec1[1] * fTemp11);
			self.iRec9[0] = ((self.iVec0[1] + self.iRec9[1]) % iSlow24);
			let mut fTemp14: f32 = (0.100000001 * (f32::max(3.0, self.fRec5[0]) + -3.0));
			let mut fTemp15: f32 = (fTemp14 + 0.5);
			let mut fTemp16: f32 = ((self.fRec8[1] * (1.0 - (((((self.iRec9[0] == 0) as i32) > 0) as i32) as f32))) + (fSlow25 * fTemp15));
			self.fRec8[0] = (fTemp16 - f32::floor(fTemp16));
			let mut fTemp17: f32 = mydsp_faustpower2_f(((2.0 * self.fRec8[0]) + -1.0));
			self.fVec5[0] = fTemp17;
			self.fRec10[0] = (fSlow26 + (0.999000013 * self.fRec10[1]));
			let mut fTemp18: f32 = (((self.fRec10[0] >= 2.0) as i32) as f32);
			let mut iTemp19: i32 = ((self.fRec10[0] >= 3.0) as i32);
			let mut fTemp20: f32 = if ((((self.fRec10[0] == 0.0) as i32) + iTemp19) as i32 != 0) { 1.0 } else { f32::max(f32::max(1.0, ((0.0199999996 * (self.fRec10[0] + -2.0999999)) + 1.0)), ((0.0199999996 * (1.0 - self.fRec10[0])) + 1.0)) };
			self.iRec12[0] = ((self.iVec0[1] + self.iRec12[1]) % ((fSlow23 / fTemp20) as i32));
			let mut fTemp21: f32 = (fTemp20 + fTemp14);
			let mut fTemp22: f32 = ((self.fRec11[1] * (1.0 - (((((self.iRec12[0] == 0) as i32) > 0) as i32) as f32))) + (fSlow25 * fTemp21));
			self.fRec11[0] = (fTemp22 - f32::floor(fTemp22));
			let mut fTemp23: f32 = mydsp_faustpower2_f(((2.0 * self.fRec11[0]) + -1.0));
			self.fVec6[0] = fTemp23;
			let mut fTemp24: f32 = f32::max((fSlow11 * fTemp20), 23.4489498);
			let mut fTemp25: f32 = f32::max(20.0, f32::abs(fTemp24));
			self.fVec7[0] = fTemp25;
			let mut fTemp26: f32 = (self.fRec13[1] + (self.fConst3 * self.fVec7[1]));
			self.fRec13[0] = (fTemp26 - f32::floor(fTemp26));
			let mut fTemp27: f32 = mydsp_faustpower2_f(((2.0 * self.fRec13[0]) + -1.0));
			self.fVec8[0] = fTemp27;
			let mut fTemp28: f32 = ((fTemp3 * (fTemp27 - self.fVec8[1])) / fTemp25);
			self.fVec9[(self.IOTA & 4095) as usize] = fTemp28;
			let mut fTemp29: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst0 * (fTemp7 / fTemp24))));
			let mut iTemp30: i32 = (fTemp29 as i32);
			let mut fTemp31: f32 = f32::floor(fTemp29);
			let mut fTemp32: f32 = ((fSlow27 * ((fTemp13 * (fTemp23 - self.fVec6[1])) / fTemp21)) + (0.25 * (fTemp2 * ((fTemp28 - (self.fVec9[((self.IOTA - iTemp30) & 4095) as usize] * (fTemp31 + (1.0 - fTemp29)))) - ((fTemp29 - fTemp31) * self.fVec9[((self.IOTA - (iTemp30 + 1)) & 4095) as usize])))));
			let mut fTemp33: f32 = if (iTemp19 as i32 != 0) { 1.49829996 } else { 1.0 };
			let mut fTemp34: f32 = (fTemp20 / fTemp33);
			self.iRec15[0] = ((self.iVec0[1] + self.iRec15[1]) % ((fSlow23 * fTemp34) as i32));
			let mut fTemp35: f32 = (fTemp33 / fTemp20);
			let mut fTemp36: f32 = (fTemp14 + fTemp35);
			let mut fTemp37: f32 = ((self.fRec14[1] * (1.0 - (((((self.iRec15[0] == 0) as i32) > 0) as i32) as f32))) + (fSlow25 * fTemp36));
			self.fRec14[0] = (fTemp37 - f32::floor(fTemp37));
			let mut fTemp38: f32 = mydsp_faustpower2_f(((2.0 * self.fRec14[0]) + -1.0));
			self.fVec10[0] = fTemp38;
			let mut fTemp39: f32 = f32::max((fSlow11 * fTemp35), 23.4489498);
			let mut fTemp40: f32 = f32::max(20.0, f32::abs(fTemp39));
			self.fVec11[0] = fTemp40;
			let mut fTemp41: f32 = (self.fRec16[1] + (self.fConst3 * self.fVec11[1]));
			self.fRec16[0] = (fTemp41 - f32::floor(fTemp41));
			let mut fTemp42: f32 = mydsp_faustpower2_f(((2.0 * self.fRec16[0]) + -1.0));
			self.fVec12[0] = fTemp42;
			let mut fTemp43: f32 = ((fTemp3 * (fTemp42 - self.fVec12[1])) / fTemp40);
			self.fVec13[(self.IOTA & 4095) as usize] = fTemp43;
			let mut fTemp44: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst0 * (fTemp7 / fTemp39))));
			let mut iTemp45: i32 = (fTemp44 as i32);
			let mut fTemp46: f32 = f32::floor(fTemp44);
			let mut fTemp47: f32 = ((fSlow27 * ((fTemp13 * (fTemp38 - self.fVec10[1])) / fTemp36)) + (0.25 * (fTemp2 * ((fTemp43 - (self.fVec13[((self.IOTA - iTemp45) & 4095) as usize] * (fTemp46 + (1.0 - fTemp44)))) - ((fTemp44 - fTemp46) * self.fVec13[((self.IOTA - (iTemp45 + 1)) & 4095) as usize])))));
			let mut fTemp48: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst5 / fTemp24)));
			let mut iTemp49: i32 = (fTemp48 as i32);
			let mut fTemp50: f32 = f32::floor(fTemp48);
			self.fRec17[0] = ((0.999000013 * self.fRec17[1]) + (self.fConst2 * ((fTemp28 - (self.fVec9[((self.IOTA - iTemp49) & 4095) as usize] * (fTemp50 + (1.0 - fTemp48)))) - ((fTemp48 - fTemp50) * self.fVec9[((self.IOTA - (iTemp49 + 1)) & 4095) as usize]))));
			let mut fTemp51: f32 = (self.fRec17[0] * fTemp20);
			let mut fTemp52: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst5 / fTemp39)));
			let mut iTemp53: i32 = (fTemp52 as i32);
			let mut fTemp54: f32 = f32::floor(fTemp52);
			self.fRec18[0] = ((0.999000013 * self.fRec18[1]) + (self.fConst2 * ((fTemp43 - (self.fVec13[((self.IOTA - iTemp53) & 4095) as usize] * (fTemp54 + (1.0 - fTemp52)))) - ((fTemp52 - fTemp54) * self.fVec13[((self.IOTA - (iTemp53 + 1)) & 4095) as usize]))));
			let mut fTemp55: f32 = (self.fRec18[0] * fTemp33);
			let mut fTemp56: f32 = (((self.fRec10[0] < 2.0) as i32) as f32);
			let mut fTemp57: f32 = f32::min(1.0, f32::max(0.0, (2.0 - self.fRec10[0])));
			let mut fTemp58: f32 = (1.0 - fTemp57);
			let mut fTemp59: f32 = (1.0 - self.fRec4[0]);
			self.fVec14[0] = fSlow30;
			let mut fTemp60: f32 = (self.fRec19[1] + (self.fConst3 * self.fVec14[1]));
			self.fRec19[0] = (fTemp60 - f32::floor(fTemp60));
			let mut fTemp61: f32 = mydsp_faustpower2_f(((2.0 * self.fRec19[0]) + -1.0));
			self.fVec15[0] = fTemp61;
			let mut fTemp62: f32 = (fSlow31 * (fTemp3 * (fTemp61 - self.fVec15[1])));
			self.fVec16[(self.IOTA & 4095) as usize] = fTemp62;
			let mut fTemp63: f32 = f32::max(0.0, f32::min(2047.0, (fSlow32 * fTemp7)));
			let mut iTemp64: i32 = (fTemp63 as i32);
			let mut fTemp65: f32 = f32::floor(fTemp63);
			self.fRec20[0] = ((fTemp62 + (0.999000013 * self.fRec20[1])) - ((fSlow35 * self.fVec16[((self.IOTA - iSlow36) & 4095) as usize]) + (fSlow37 * self.fVec16[((self.IOTA - iSlow38) & 4095) as usize])));
			self.iRec22[0] = ((self.iVec0[1] + self.iRec22[1]) % iSlow39);
			let mut fTemp66: f32 = (fTemp14 + 1.0);
			let mut fTemp67: f32 = ((self.fRec21[1] * (1.0 - (((((self.iRec22[0] == 0) as i32) > 0) as i32) as f32))) + (fSlow25 * fTemp66));
			self.fRec21[0] = (fTemp67 - f32::floor(fTemp67));
			let mut fTemp68: f32 = mydsp_faustpower2_f(((2.0 * self.fRec21[0]) + -1.0));
			self.fVec17[0] = fTemp68;
			self.fRec23[0] = (fSlow40 + (0.999000013 * self.fRec23[1]));
			let mut fTemp69: f32 = f32::tan((self.fConst8 * ((10000.0 * mydsp_faustpower2_f(self.fRec23[0])) + 100.0)));
			let mut fTemp70: f32 = (1.0 / fTemp69);
			let mut fTemp71: f32 = (((fTemp70 + -0.800000012) / fTemp69) + 1.0);
			let mut fTemp72: f32 = (1.0 - (1.0 / mydsp_faustpower2_f(fTemp69)));
			let mut fTemp73: f32 = (((fTemp70 + 0.800000012) / fTemp69) + 1.0);
			self.fRec3[0] = (((self.fRec4[0] * (((fTemp2 * (fTemp6 - ((self.fVec4[((self.IOTA - iTemp9) & 4095) as usize] * (fTemp10 + (1.0 - fTemp8))) + ((fTemp8 - fTemp10) * self.fVec4[((self.IOTA - (iTemp9 + 1)) & 4095) as usize])))) + (fSlow16 * (self.fRec7[0] * fTemp12))) + (fSlow23 * ((fTemp13 * (fTemp17 - self.fVec5[1])) / fTemp15)))) + (((fTemp18 * ((self.fConst0 * (fTemp32 + fTemp47)) + (fSlow28 * (fTemp12 * (fTemp51 + (fTemp55 / fTemp20)))))) + ((fTemp56 * ((fSlow28 * (fTemp51 * fTemp12)) + (self.fConst0 * fTemp32))) * (fTemp58 + (fTemp57 * ((fSlow28 * ((fTemp55 * fTemp12) / fTemp20)) + (self.fConst0 * fTemp47)))))) + (fTemp59 * (((fTemp2 * (fTemp62 - ((self.fVec16[((self.IOTA - iTemp64) & 4095) as usize] * (fTemp65 + (1.0 - fTemp63))) + ((fTemp63 - fTemp65) * self.fVec16[((self.IOTA - (iTemp64 + 1)) & 4095) as usize])))) + (fSlow28 * (self.fRec20[0] * fTemp12))) + (fSlow23 * ((fTemp13 * (fTemp68 - self.fVec17[1])) / fTemp66)))))) - (((self.fRec3[2] * fTemp71) + (2.0 * (self.fRec3[1] * fTemp72))) / fTemp73));
			self.fRec2[0] = ((((self.fRec3[1] + (0.5 * self.fRec3[0])) + (0.5 * self.fRec3[2])) - ((fTemp71 * self.fRec2[2]) + (2.0 * (fTemp72 * self.fRec2[1])))) / fTemp73);
			self.iRec25[0] = (iSlow42 * (self.iRec25[1] + 1));
			let mut iTemp74: i32 = ((self.iRec25[0] < iSlow4) as i32);
			let mut fTemp75: f32 = f32::exp((0.0 - (self.fConst1 / if (iSlow42 as i32 != 0) { if (iTemp74 as i32 != 0) { fSlow6 } else { fSlow5 } } else { fSlow2 })));
			self.fRec24[0] = ((self.fRec24[1] * fTemp75) + (if (iSlow42 as i32 != 0) { if (iTemp74 as i32 != 0) { 1.58730161 } else { fSlow43 } } else { 0.0 } * (1.0 - fTemp75)));
			self.fVec18[0] = fSlow46;
			let mut fTemp76: f32 = (self.fRec28[1] + (self.fConst3 * self.fVec18[1]));
			self.fRec28[0] = (fTemp76 - f32::floor(fTemp76));
			let mut fTemp77: f32 = mydsp_faustpower2_f(((2.0 * self.fRec28[0]) + -1.0));
			self.fVec19[0] = fTemp77;
			let mut fTemp78: f32 = (fSlow47 * (fTemp3 * (fTemp77 - self.fVec19[1])));
			self.fVec20[(self.IOTA & 4095) as usize] = fTemp78;
			let mut fTemp79: f32 = f32::max(0.0, f32::min(2047.0, (fSlow48 * fTemp7)));
			let mut iTemp80: i32 = (fTemp79 as i32);
			let mut fTemp81: f32 = f32::floor(fTemp79);
			self.fRec29[0] = ((fTemp78 + (0.999000013 * self.fRec29[1])) - ((fSlow52 * self.fVec20[((self.IOTA - iSlow53) & 4095) as usize]) + (fSlow54 * self.fVec20[((self.IOTA - iSlow55) & 4095) as usize])));
			self.iRec31[0] = ((self.iVec0[1] + self.iRec31[1]) % iSlow57);
			let mut fTemp82: f32 = ((self.fRec30[1] * (1.0 - (((((self.iRec31[0] == 0) as i32) > 0) as i32) as f32))) + (fSlow58 * fTemp15));
			self.fRec30[0] = (fTemp82 - f32::floor(fTemp82));
			let mut fTemp83: f32 = mydsp_faustpower2_f(((2.0 * self.fRec30[0]) + -1.0));
			self.fVec21[0] = fTemp83;
			self.iRec33[0] = ((self.iVec0[1] + self.iRec33[1]) % ((fSlow56 / fTemp20) as i32));
			let mut fTemp84: f32 = ((self.fRec32[1] * (1.0 - (((((self.iRec33[0] == 0) as i32) > 0) as i32) as f32))) + (fSlow58 * fTemp21));
			self.fRec32[0] = (fTemp84 - f32::floor(fTemp84));
			let mut fTemp85: f32 = mydsp_faustpower2_f(((2.0 * self.fRec32[0]) + -1.0));
			self.fVec22[0] = fTemp85;
			let mut fTemp86: f32 = f32::max((fSlow44 * fTemp20), 23.4489498);
			let mut fTemp87: f32 = f32::max(20.0, f32::abs(fTemp86));
			self.fVec23[0] = fTemp87;
			let mut fTemp88: f32 = (self.fRec34[1] + (self.fConst3 * self.fVec23[1]));
			self.fRec34[0] = (fTemp88 - f32::floor(fTemp88));
			let mut fTemp89: f32 = mydsp_faustpower2_f(((2.0 * self.fRec34[0]) + -1.0));
			self.fVec24[0] = fTemp89;
			let mut fTemp90: f32 = ((fTemp3 * (fTemp89 - self.fVec24[1])) / fTemp87);
			self.fVec25[(self.IOTA & 4095) as usize] = fTemp90;
			let mut fTemp91: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst0 * (fTemp7 / fTemp86))));
			let mut iTemp92: i32 = (fTemp91 as i32);
			let mut fTemp93: f32 = f32::floor(fTemp91);
			let mut fTemp94: f32 = ((fSlow59 * ((fTemp13 * (fTemp85 - self.fVec22[1])) / fTemp21)) + (0.25 * (fTemp2 * ((fTemp90 - (self.fVec25[((self.IOTA - iTemp92) & 4095) as usize] * (fTemp93 + (1.0 - fTemp91)))) - ((fTemp91 - fTemp93) * self.fVec25[((self.IOTA - (iTemp92 + 1)) & 4095) as usize])))));
			self.iRec36[0] = ((self.iVec0[1] + self.iRec36[1]) % ((fSlow56 * fTemp34) as i32));
			let mut fTemp95: f32 = ((self.fRec35[1] * (1.0 - (((((self.iRec36[0] == 0) as i32) > 0) as i32) as f32))) + (fSlow58 * fTemp36));
			self.fRec35[0] = (fTemp95 - f32::floor(fTemp95));
			let mut fTemp96: f32 = mydsp_faustpower2_f(((2.0 * self.fRec35[0]) + -1.0));
			self.fVec26[0] = fTemp96;
			let mut fTemp97: f32 = f32::max((fSlow44 * fTemp35), 23.4489498);
			let mut fTemp98: f32 = f32::max(20.0, f32::abs(fTemp97));
			self.fVec27[0] = fTemp98;
			let mut fTemp99: f32 = (self.fRec37[1] + (self.fConst3 * self.fVec27[1]));
			self.fRec37[0] = (fTemp99 - f32::floor(fTemp99));
			let mut fTemp100: f32 = mydsp_faustpower2_f(((2.0 * self.fRec37[0]) + -1.0));
			self.fVec28[0] = fTemp100;
			let mut fTemp101: f32 = ((fTemp3 * (fTemp100 - self.fVec28[1])) / fTemp98);
			self.fVec29[(self.IOTA & 4095) as usize] = fTemp101;
			let mut fTemp102: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst0 * (fTemp7 / fTemp97))));
			let mut iTemp103: i32 = (fTemp102 as i32);
			let mut fTemp104: f32 = f32::floor(fTemp102);
			let mut fTemp105: f32 = ((fSlow59 * ((fTemp13 * (fTemp96 - self.fVec26[1])) / fTemp36)) + (0.25 * (fTemp2 * ((fTemp101 - (self.fVec29[((self.IOTA - iTemp103) & 4095) as usize] * (fTemp104 + (1.0 - fTemp102)))) - ((fTemp102 - fTemp104) * self.fVec29[((self.IOTA - (iTemp103 + 1)) & 4095) as usize])))));
			let mut fTemp106: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst5 / fTemp86)));
			let mut iTemp107: i32 = (fTemp106 as i32);
			let mut fTemp108: f32 = f32::floor(fTemp106);
			self.fRec38[0] = ((0.999000013 * self.fRec38[1]) + (self.fConst2 * ((fTemp90 - (self.fVec25[((self.IOTA - iTemp107) & 4095) as usize] * (fTemp108 + (1.0 - fTemp106)))) - ((fTemp106 - fTemp108) * self.fVec25[((self.IOTA - (iTemp107 + 1)) & 4095) as usize]))));
			let mut fTemp109: f32 = (self.fRec38[0] * fTemp20);
			let mut fTemp110: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst5 / fTemp97)));
			let mut iTemp111: i32 = (fTemp110 as i32);
			let mut fTemp112: f32 = f32::floor(fTemp110);
			self.fRec39[0] = ((0.999000013 * self.fRec39[1]) + (self.fConst2 * ((fTemp101 - (self.fVec29[((self.IOTA - iTemp111) & 4095) as usize] * (fTemp112 + (1.0 - fTemp110)))) - ((fTemp110 - fTemp112) * self.fVec29[((self.IOTA - (iTemp111 + 1)) & 4095) as usize]))));
			let mut fTemp113: f32 = (self.fRec39[0] * fTemp33);
			self.fVec30[0] = fSlow62;
			let mut fTemp114: f32 = (self.fRec40[1] + (self.fConst3 * self.fVec30[1]));
			self.fRec40[0] = (fTemp114 - f32::floor(fTemp114));
			let mut fTemp115: f32 = mydsp_faustpower2_f(((2.0 * self.fRec40[0]) + -1.0));
			self.fVec31[0] = fTemp115;
			let mut fTemp116: f32 = (fSlow63 * (fTemp3 * (fTemp115 - self.fVec31[1])));
			self.fVec32[(self.IOTA & 4095) as usize] = fTemp116;
			let mut fTemp117: f32 = f32::max(0.0, f32::min(2047.0, (fSlow64 * fTemp7)));
			let mut iTemp118: i32 = (fTemp117 as i32);
			let mut fTemp119: f32 = f32::floor(fTemp117);
			self.fRec41[0] = ((fTemp116 + (0.999000013 * self.fRec41[1])) - ((fSlow67 * self.fVec32[((self.IOTA - iSlow68) & 4095) as usize]) + (fSlow69 * self.fVec32[((self.IOTA - iSlow70) & 4095) as usize])));
			self.iRec43[0] = ((self.iVec0[1] + self.iRec43[1]) % iSlow71);
			let mut fTemp120: f32 = ((self.fRec42[1] * (1.0 - (((((self.iRec43[0] == 0) as i32) > 0) as i32) as f32))) + (fSlow58 * fTemp66));
			self.fRec42[0] = (fTemp120 - f32::floor(fTemp120));
			let mut fTemp121: f32 = mydsp_faustpower2_f(((2.0 * self.fRec42[0]) + -1.0));
			self.fVec33[0] = fTemp121;
			self.fRec27[0] = (((self.fRec4[0] * (((fTemp2 * (fTemp78 - ((self.fVec20[((self.IOTA - iTemp80) & 4095) as usize] * (fTemp81 + (1.0 - fTemp79))) + ((fTemp79 - fTemp81) * self.fVec20[((self.IOTA - (iTemp80 + 1)) & 4095) as usize])))) + (fSlow49 * (self.fRec29[0] * fTemp12))) + (fSlow56 * ((fTemp13 * (fTemp83 - self.fVec21[1])) / fTemp15)))) + (((fTemp18 * ((self.fConst0 * (fTemp94 + fTemp105)) + (fSlow60 * (fTemp12 * (fTemp109 + (fTemp113 / fTemp20)))))) + ((fTemp56 * ((fSlow60 * (fTemp109 * fTemp12)) + (self.fConst0 * fTemp94))) * (fTemp58 + (fTemp57 * ((fSlow60 * ((fTemp113 * fTemp12) / fTemp20)) + (self.fConst0 * fTemp105)))))) + (fTemp59 * (((fTemp2 * (fTemp116 - ((self.fVec32[((self.IOTA - iTemp118) & 4095) as usize] * (fTemp119 + (1.0 - fTemp117))) + ((fTemp117 - fTemp119) * self.fVec32[((self.IOTA - (iTemp118 + 1)) & 4095) as usize])))) + (fSlow60 * (self.fRec41[0] * fTemp12))) + (fSlow56 * ((fTemp13 * (fTemp121 - self.fVec33[1])) / fTemp66)))))) - (((fTemp71 * self.fRec27[2]) + (2.0 * (fTemp72 * self.fRec27[1]))) / fTemp73));
			self.fRec26[0] = ((((self.fRec27[1] + (0.5 * self.fRec27[0])) + (0.5 * self.fRec27[2])) - ((fTemp71 * self.fRec26[2]) + (2.0 * (fTemp72 * self.fRec26[1])))) / fTemp73);
			*output0 = ((((fSlow0 * (f32::min(1.0, self.fRec0[0]) * ((self.fRec2[1] + (0.5 * self.fRec2[0])) + (0.5 * self.fRec2[2])))) + (fSlow41 * (f32::min(1.0, self.fRec24[0]) * ((self.fRec26[1] + (0.5 * self.fRec26[0])) + (0.5 * self.fRec26[2]))))) / fTemp73) as f32);
			self.iVec0[1] = self.iVec0[0];
			self.iRec1[1] = self.iRec1[0];
			self.fRec0[1] = self.fRec0[0];
			self.fRec4[1] = self.fRec4[0];
			self.fRec5[1] = self.fRec5[0];
			self.fVec1[1] = self.fVec1[0];
			self.fVec2[1] = self.fVec2[0];
			self.fRec6[1] = self.fRec6[0];
			self.fVec3[1] = self.fVec3[0];
			self.IOTA = (self.IOTA + 1);
			self.fRec7[1] = self.fRec7[0];
			self.iRec9[1] = self.iRec9[0];
			self.fRec8[1] = self.fRec8[0];
			self.fVec5[1] = self.fVec5[0];
			self.fRec10[1] = self.fRec10[0];
			self.iRec12[1] = self.iRec12[0];
			self.fRec11[1] = self.fRec11[0];
			self.fVec6[1] = self.fVec6[0];
			self.fVec7[1] = self.fVec7[0];
			self.fRec13[1] = self.fRec13[0];
			self.fVec8[1] = self.fVec8[0];
			self.iRec15[1] = self.iRec15[0];
			self.fRec14[1] = self.fRec14[0];
			self.fVec10[1] = self.fVec10[0];
			self.fVec11[1] = self.fVec11[0];
			self.fRec16[1] = self.fRec16[0];
			self.fVec12[1] = self.fVec12[0];
			self.fRec17[1] = self.fRec17[0];
			self.fRec18[1] = self.fRec18[0];
			self.fVec14[1] = self.fVec14[0];
			self.fRec19[1] = self.fRec19[0];
			self.fVec15[1] = self.fVec15[0];
			self.fRec20[1] = self.fRec20[0];
			self.iRec22[1] = self.iRec22[0];
			self.fRec21[1] = self.fRec21[0];
			self.fVec17[1] = self.fVec17[0];
			self.fRec23[1] = self.fRec23[0];
			self.fRec3[2] = self.fRec3[1];
			self.fRec3[1] = self.fRec3[0];
			self.fRec2[2] = self.fRec2[1];
			self.fRec2[1] = self.fRec2[0];
			self.iRec25[1] = self.iRec25[0];
			self.fRec24[1] = self.fRec24[0];
			self.fVec18[1] = self.fVec18[0];
			self.fRec28[1] = self.fRec28[0];
			self.fVec19[1] = self.fVec19[0];
			self.fRec29[1] = self.fRec29[0];
			self.iRec31[1] = self.iRec31[0];
			self.fRec30[1] = self.fRec30[0];
			self.fVec21[1] = self.fVec21[0];
			self.iRec33[1] = self.iRec33[0];
			self.fRec32[1] = self.fRec32[0];
			self.fVec22[1] = self.fVec22[0];
			self.fVec23[1] = self.fVec23[0];
			self.fRec34[1] = self.fRec34[0];
			self.fVec24[1] = self.fVec24[0];
			self.iRec36[1] = self.iRec36[0];
			self.fRec35[1] = self.fRec35[0];
			self.fVec26[1] = self.fVec26[0];
			self.fVec27[1] = self.fVec27[0];
			self.fRec37[1] = self.fRec37[0];
			self.fVec28[1] = self.fVec28[0];
			self.fRec38[1] = self.fRec38[0];
			self.fRec39[1] = self.fRec39[0];
			self.fVec30[1] = self.fVec30[0];
			self.fRec40[1] = self.fRec40[0];
			self.fVec31[1] = self.fVec31[0];
			self.fRec41[1] = self.fRec41[0];
			self.iRec43[1] = self.iRec43[0];
			self.fRec42[1] = self.fRec42[0];
			self.fVec33[1] = self.fVec33[0];
			self.fRec27[2] = self.fRec27[1];
			self.fRec27[1] = self.fRec27[0];
			self.fRec26[2] = self.fRec26[1];
			self.fRec26[1] = self.fRec26[0];
		}
	}

	#[inline]
	pub fn compute_external(&mut self, count: i32) {
		let (output0) = unsafe {
			(::std::slice::from_raw_parts_mut(OUTPUTS[0], count as usize))
		};
		unsafe { self.compute(count, &[], &mut [output0]); }
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
    if length < MAX_PARAM_SIZE as i32 {
        let mut param = String::new(); 
        for i in 0..length as usize {
            let c = unsafe { PARAM_NAME[i] } as char;
            param.push(c);
        }
        return unsafe { ENGINE.get_param_info(&param).index };
    }
    else {
        return -1;
    }
}

#[no_mangle]
pub fn get_gain_index() -> i32 {
	unsafe { ENGINE.get_param_info("gain").index }
}

#[no_mangle]
pub fn get_gate_index() -> i32 {
	unsafe { ENGINE.get_param_info("gate").index }
}

#[no_mangle]
pub fn get_freq_index() -> i32 {
	unsafe { ENGINE.get_param_info("freq").index }
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