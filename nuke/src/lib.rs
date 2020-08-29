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
	iVec0: [0;2],
	fSampleRate: 0,
	fConst0: 0.0,
	fConst1: 0.0,
	fButton0: 0.0,
	fRec1: [0.0;2],
	fHslider0: 0.0,
	iRec2: [0;2],
	fHslider1: 0.0,
	fHslider2: 0.0,
	fHslider3: 0.0,
	fRec0: [0.0;2],
	fHslider4: 0.0,
	fRec3: [0.0;2],
	fHslider5: 0.0,
	fRec6: [0.0;2],
	fHslider6: 0.0,
	fRec7: [0.0;2],
	fVec1: [0.0;2],
	fConst2: 0.0,
	fHslider7: 0.0,
	fVec2: [0.0;2],
	fConst3: 0.0,
	fRec8: [0.0;2],
	fVec3: [0.0;2],
	IOTA: 0,
	fVec4: [0.0;4096],
	fConst4: 0.0,
	fConst5: 0.0,
	fRec9: [0.0;2],
	fConst6: 0.0,
	iRec11: [0;2],
	fRec10: [0.0;2],
	fVec5: [0.0;2],
	fHslider8: 0.0,
	fRec12: [0.0;2],
	iRec14: [0;2],
	fRec13: [0.0;2],
	fVec6: [0.0;2],
	fVec7: [0.0;2],
	fRec15: [0.0;2],
	fVec8: [0.0;2],
	fVec9: [0.0;4096],
	iRec17: [0;2],
	fRec16: [0.0;2],
	fVec10: [0.0;2],
	fVec11: [0.0;2],
	fRec18: [0.0;2],
	fVec12: [0.0;2],
	fVec13: [0.0;4096],
	fConst7: 0.0,
	fRec19: [0.0;2],
	fRec20: [0.0;2],
	fVec14: [0.0;2],
	fRec21: [0.0;2],
	fVec15: [0.0;2],
	fVec16: [0.0;4096],
	fRec22: [0.0;2],
	iRec24: [0;2],
	fRec23: [0.0;2],
	fVec17: [0.0;2],
	fConst8: 0.0,
	fHslider9: 0.0,
	fRec25: [0.0;2],
	fRec5: [0.0;3],
	fRec4: [0.0;3],
	fButton1: 0.0,
	fRec27: [0.0;2],
	iRec28: [0;2],
	fRec26: [0.0;2],
	fHslider10: 0.0,
	fRec29: [0.0;2],
	fHslider11: 0.0,
	fVec18: [0.0;2],
	fRec32: [0.0;2],
	fVec19: [0.0;2],
	fVec20: [0.0;4096],
	fRec33: [0.0;2],
	iRec35: [0;2],
	fRec34: [0.0;2],
	fVec21: [0.0;2],
	iRec37: [0;2],
	fRec36: [0.0;2],
	fVec22: [0.0;2],
	fVec23: [0.0;2],
	fRec38: [0.0;2],
	fVec24: [0.0;2],
	fVec25: [0.0;4096],
	iRec40: [0;2],
	fRec39: [0.0;2],
	fVec26: [0.0;2],
	fVec27: [0.0;2],
	fRec41: [0.0;2],
	fVec28: [0.0;2],
	fVec29: [0.0;4096],
	fRec42: [0.0;2],
	fRec43: [0.0;2],
	fVec30: [0.0;2],
	fRec44: [0.0;2],
	fVec31: [0.0;2],
	fVec32: [0.0;4096],
	fRec45: [0.0;2],
	iRec47: [0;2],
	fRec46: [0.0;2],
	fVec33: [0.0;2],
	fRec31: [0.0;3],
	fRec30: [0.0;3],
	next_allocated_voice_age: 1000000000,
	next_unallocated_voice_age: 0,
	voices: [VoiceInfo {active: false,note: 0,channel: 0,voice_age: 0,};2],
	voice_freq: [0;2],
	voice_gain: [0;2],
	voice_gate: [0;2],
};

type T = f32;

struct mydsp {
	iVec0: [i32;2],
	fSampleRate: i32,
	fConst0: f32,
	fConst1: f32,
	fButton0: f32,
	fRec1: [f32;2],
	fHslider0: f32,
	iRec2: [i32;2],
	fHslider1: f32,
	fHslider2: f32,
	fHslider3: f32,
	fRec0: [f32;2],
	fHslider4: f32,
	fRec3: [f32;2],
	fHslider5: f32,
	fRec6: [f32;2],
	fHslider6: f32,
	fRec7: [f32;2],
	fVec1: [f32;2],
	fConst2: f32,
	fHslider7: f32,
	fVec2: [f32;2],
	fConst3: f32,
	fRec8: [f32;2],
	fVec3: [f32;2],
	IOTA: i32,
	fVec4: [f32;4096],
	fConst4: f32,
	fConst5: f32,
	fRec9: [f32;2],
	fConst6: f32,
	iRec11: [i32;2],
	fRec10: [f32;2],
	fVec5: [f32;2],
	fHslider8: f32,
	fRec12: [f32;2],
	iRec14: [i32;2],
	fRec13: [f32;2],
	fVec6: [f32;2],
	fVec7: [f32;2],
	fRec15: [f32;2],
	fVec8: [f32;2],
	fVec9: [f32;4096],
	iRec17: [i32;2],
	fRec16: [f32;2],
	fVec10: [f32;2],
	fVec11: [f32;2],
	fRec18: [f32;2],
	fVec12: [f32;2],
	fVec13: [f32;4096],
	fConst7: f32,
	fRec19: [f32;2],
	fRec20: [f32;2],
	fVec14: [f32;2],
	fRec21: [f32;2],
	fVec15: [f32;2],
	fVec16: [f32;4096],
	fRec22: [f32;2],
	iRec24: [i32;2],
	fRec23: [f32;2],
	fVec17: [f32;2],
	fConst8: f32,
	fHslider9: f32,
	fRec25: [f32;2],
	fRec5: [f32;3],
	fRec4: [f32;3],
	fButton1: f32,
	fRec27: [f32;2],
	iRec28: [i32;2],
	fRec26: [f32;2],
	fHslider10: f32,
	fRec29: [f32;2],
	fHslider11: f32,
	fVec18: [f32;2],
	fRec32: [f32;2],
	fVec19: [f32;2],
	fVec20: [f32;4096],
	fRec33: [f32;2],
	iRec35: [i32;2],
	fRec34: [f32;2],
	fVec21: [f32;2],
	iRec37: [i32;2],
	fRec36: [f32;2],
	fVec22: [f32;2],
	fVec23: [f32;2],
	fRec38: [f32;2],
	fVec24: [f32;2],
	fVec25: [f32;4096],
	iRec40: [i32;2],
	fRec39: [f32;2],
	fVec26: [f32;2],
	fVec27: [f32;2],
	fRec41: [f32;2],
	fVec28: [f32;2],
	fVec29: [f32;4096],
	fRec42: [f32;2],
	fRec43: [f32;2],
	fVec30: [f32;2],
	fRec44: [f32;2],
	fVec31: [f32;2],
	fVec32: [f32;4096],
	fRec45: [f32;2],
	iRec47: [i32;2],
	fRec46: [f32;2],
	fVec33: [f32;2],
	fRec31: [f32;3],
	fRec30: [f32;3],

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
			iVec0: [0;2],
			fSampleRate: 0,
			fConst0: 0.0,
			fConst1: 0.0,
			fButton0: 0.0,
			fRec1: [0.0;2],
			fHslider0: 0.0,
			iRec2: [0;2],
			fHslider1: 0.0,
			fHslider2: 0.0,
			fHslider3: 0.0,
			fRec0: [0.0;2],
			fHslider4: 0.0,
			fRec3: [0.0;2],
			fHslider5: 0.0,
			fRec6: [0.0;2],
			fHslider6: 0.0,
			fRec7: [0.0;2],
			fVec1: [0.0;2],
			fConst2: 0.0,
			fHslider7: 0.0,
			fVec2: [0.0;2],
			fConst3: 0.0,
			fRec8: [0.0;2],
			fVec3: [0.0;2],
			IOTA: 0,
			fVec4: [0.0;4096],
			fConst4: 0.0,
			fConst5: 0.0,
			fRec9: [0.0;2],
			fConst6: 0.0,
			iRec11: [0;2],
			fRec10: [0.0;2],
			fVec5: [0.0;2],
			fHslider8: 0.0,
			fRec12: [0.0;2],
			iRec14: [0;2],
			fRec13: [0.0;2],
			fVec6: [0.0;2],
			fVec7: [0.0;2],
			fRec15: [0.0;2],
			fVec8: [0.0;2],
			fVec9: [0.0;4096],
			iRec17: [0;2],
			fRec16: [0.0;2],
			fVec10: [0.0;2],
			fVec11: [0.0;2],
			fRec18: [0.0;2],
			fVec12: [0.0;2],
			fVec13: [0.0;4096],
			fConst7: 0.0,
			fRec19: [0.0;2],
			fRec20: [0.0;2],
			fVec14: [0.0;2],
			fRec21: [0.0;2],
			fVec15: [0.0;2],
			fVec16: [0.0;4096],
			fRec22: [0.0;2],
			iRec24: [0;2],
			fRec23: [0.0;2],
			fVec17: [0.0;2],
			fConst8: 0.0,
			fHslider9: 0.0,
			fRec25: [0.0;2],
			fRec5: [0.0;3],
			fRec4: [0.0;3],
			fButton1: 0.0,
			fRec27: [0.0;2],
			iRec28: [0;2],
			fRec26: [0.0;2],
			fHslider10: 0.0,
			fRec29: [0.0;2],
			fHslider11: 0.0,
			fVec18: [0.0;2],
			fRec32: [0.0;2],
			fVec19: [0.0;2],
			fVec20: [0.0;4096],
			fRec33: [0.0;2],
			iRec35: [0;2],
			fRec34: [0.0;2],
			fVec21: [0.0;2],
			iRec37: [0;2],
			fRec36: [0.0;2],
			fVec22: [0.0;2],
			fVec23: [0.0;2],
			fRec38: [0.0;2],
			fVec24: [0.0;2],
			fVec25: [0.0;4096],
			iRec40: [0;2],
			fRec39: [0.0;2],
			fVec26: [0.0;2],
			fVec27: [0.0;2],
			fRec41: [0.0;2],
			fVec28: [0.0;2],
			fVec29: [0.0;4096],
			fRec42: [0.0;2],
			fRec43: [0.0;2],
			fVec30: [0.0;2],
			fRec44: [0.0;2],
			fVec31: [0.0;2],
			fVec32: [0.0;4096],
			fRec45: [0.0;2],
			iRec47: [0;2],
			fRec46: [0.0;2],
			fVec33: [0.0;2],
			fRec31: [0.0;3],
			fRec30: [0.0;3],
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
		self.fButton0 = 0.0;
		self.fHslider0 = 0.0;
		self.fHslider1 = 0.00100000005;
		self.fHslider2 = 0.0;
		self.fHslider3 = 1.0;
		self.fHslider4 = 1.0;
		self.fHslider5 = 0.0;
		self.fHslider6 = 1.0;
		self.fHslider7 = 440.0;
		self.fHslider8 = 2.0;
		self.fHslider9 = 0.5;
		self.fButton1 = 0.0;
		self.fHslider10 = 1.0;
		self.fHslider11 = 440.0;
	}
	fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.iVec0[l0 as usize] = 0;
		}
		for l1 in 0..2 {
			self.fRec1[l1 as usize] = 0.0;
		}
		for l2 in 0..2 {
			self.iRec2[l2 as usize] = 0;
		}
		for l3 in 0..2 {
			self.fRec0[l3 as usize] = 0.0;
		}
		for l4 in 0..2 {
			self.fRec3[l4 as usize] = 0.0;
		}
		for l5 in 0..2 {
			self.fRec6[l5 as usize] = 0.0;
		}
		for l6 in 0..2 {
			self.fRec7[l6 as usize] = 0.0;
		}
		for l7 in 0..2 {
			self.fVec1[l7 as usize] = 0.0;
		}
		for l8 in 0..2 {
			self.fVec2[l8 as usize] = 0.0;
		}
		for l9 in 0..2 {
			self.fRec8[l9 as usize] = 0.0;
		}
		for l10 in 0..2 {
			self.fVec3[l10 as usize] = 0.0;
		}
		self.IOTA = 0;
		for l11 in 0..4096 {
			self.fVec4[l11 as usize] = 0.0;
		}
		for l12 in 0..2 {
			self.fRec9[l12 as usize] = 0.0;
		}
		for l13 in 0..2 {
			self.iRec11[l13 as usize] = 0;
		}
		for l14 in 0..2 {
			self.fRec10[l14 as usize] = 0.0;
		}
		for l15 in 0..2 {
			self.fVec5[l15 as usize] = 0.0;
		}
		for l16 in 0..2 {
			self.fRec12[l16 as usize] = 0.0;
		}
		for l17 in 0..2 {
			self.iRec14[l17 as usize] = 0;
		}
		for l18 in 0..2 {
			self.fRec13[l18 as usize] = 0.0;
		}
		for l19 in 0..2 {
			self.fVec6[l19 as usize] = 0.0;
		}
		for l20 in 0..2 {
			self.fVec7[l20 as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.fRec15[l21 as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.fVec8[l22 as usize] = 0.0;
		}
		for l23 in 0..4096 {
			self.fVec9[l23 as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.iRec17[l24 as usize] = 0;
		}
		for l25 in 0..2 {
			self.fRec16[l25 as usize] = 0.0;
		}
		for l26 in 0..2 {
			self.fVec10[l26 as usize] = 0.0;
		}
		for l27 in 0..2 {
			self.fVec11[l27 as usize] = 0.0;
		}
		for l28 in 0..2 {
			self.fRec18[l28 as usize] = 0.0;
		}
		for l29 in 0..2 {
			self.fVec12[l29 as usize] = 0.0;
		}
		for l30 in 0..4096 {
			self.fVec13[l30 as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fRec19[l31 as usize] = 0.0;
		}
		for l32 in 0..2 {
			self.fRec20[l32 as usize] = 0.0;
		}
		for l33 in 0..2 {
			self.fVec14[l33 as usize] = 0.0;
		}
		for l34 in 0..2 {
			self.fRec21[l34 as usize] = 0.0;
		}
		for l35 in 0..2 {
			self.fVec15[l35 as usize] = 0.0;
		}
		for l36 in 0..4096 {
			self.fVec16[l36 as usize] = 0.0;
		}
		for l37 in 0..2 {
			self.fRec22[l37 as usize] = 0.0;
		}
		for l38 in 0..2 {
			self.iRec24[l38 as usize] = 0;
		}
		for l39 in 0..2 {
			self.fRec23[l39 as usize] = 0.0;
		}
		for l40 in 0..2 {
			self.fVec17[l40 as usize] = 0.0;
		}
		for l41 in 0..2 {
			self.fRec25[l41 as usize] = 0.0;
		}
		for l42 in 0..3 {
			self.fRec5[l42 as usize] = 0.0;
		}
		for l43 in 0..3 {
			self.fRec4[l43 as usize] = 0.0;
		}
		for l44 in 0..2 {
			self.fRec27[l44 as usize] = 0.0;
		}
		for l45 in 0..2 {
			self.iRec28[l45 as usize] = 0;
		}
		for l46 in 0..2 {
			self.fRec26[l46 as usize] = 0.0;
		}
		for l47 in 0..2 {
			self.fRec29[l47 as usize] = 0.0;
		}
		for l48 in 0..2 {
			self.fVec18[l48 as usize] = 0.0;
		}
		for l49 in 0..2 {
			self.fRec32[l49 as usize] = 0.0;
		}
		for l50 in 0..2 {
			self.fVec19[l50 as usize] = 0.0;
		}
		for l51 in 0..4096 {
			self.fVec20[l51 as usize] = 0.0;
		}
		for l52 in 0..2 {
			self.fRec33[l52 as usize] = 0.0;
		}
		for l53 in 0..2 {
			self.iRec35[l53 as usize] = 0;
		}
		for l54 in 0..2 {
			self.fRec34[l54 as usize] = 0.0;
		}
		for l55 in 0..2 {
			self.fVec21[l55 as usize] = 0.0;
		}
		for l56 in 0..2 {
			self.iRec37[l56 as usize] = 0;
		}
		for l57 in 0..2 {
			self.fRec36[l57 as usize] = 0.0;
		}
		for l58 in 0..2 {
			self.fVec22[l58 as usize] = 0.0;
		}
		for l59 in 0..2 {
			self.fVec23[l59 as usize] = 0.0;
		}
		for l60 in 0..2 {
			self.fRec38[l60 as usize] = 0.0;
		}
		for l61 in 0..2 {
			self.fVec24[l61 as usize] = 0.0;
		}
		for l62 in 0..4096 {
			self.fVec25[l62 as usize] = 0.0;
		}
		for l63 in 0..2 {
			self.iRec40[l63 as usize] = 0;
		}
		for l64 in 0..2 {
			self.fRec39[l64 as usize] = 0.0;
		}
		for l65 in 0..2 {
			self.fVec26[l65 as usize] = 0.0;
		}
		for l66 in 0..2 {
			self.fVec27[l66 as usize] = 0.0;
		}
		for l67 in 0..2 {
			self.fRec41[l67 as usize] = 0.0;
		}
		for l68 in 0..2 {
			self.fVec28[l68 as usize] = 0.0;
		}
		for l69 in 0..4096 {
			self.fVec29[l69 as usize] = 0.0;
		}
		for l70 in 0..2 {
			self.fRec42[l70 as usize] = 0.0;
		}
		for l71 in 0..2 {
			self.fRec43[l71 as usize] = 0.0;
		}
		for l72 in 0..2 {
			self.fVec30[l72 as usize] = 0.0;
		}
		for l73 in 0..2 {
			self.fRec44[l73 as usize] = 0.0;
		}
		for l74 in 0..2 {
			self.fVec31[l74 as usize] = 0.0;
		}
		for l75 in 0..4096 {
			self.fVec32[l75 as usize] = 0.0;
		}
		for l76 in 0..2 {
			self.fRec45[l76 as usize] = 0.0;
		}
		for l77 in 0..2 {
			self.iRec47[l77 as usize] = 0;
		}
		for l78 in 0..2 {
			self.fRec46[l78 as usize] = 0.0;
		}
		for l79 in 0..2 {
			self.fVec33[l79 as usize] = 0.0;
		}
		for l80 in 0..3 {
			self.fRec31[l80 as usize] = 0.0;
		}
		for l81 in 0..3 {
			self.fRec30[l81 as usize] = 0.0;
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
			6 => self.fHslider0,
			4 => self.fHslider1,
			12 => self.fHslider10,
			11 => self.fHslider11,
			5 => self.fHslider2,
			7 => self.fHslider3,
			9 => self.fHslider4,
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
			6 => { self.fHslider0 = value }
			4 => { self.fHslider1 = value }
			12 => { self.fHslider10 = value }
			11 => { self.fHslider11 = value }
			5 => { self.fHslider2 = value }
			7 => { self.fHslider3 = value }
			9 => { self.fHslider4 = value }
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
		let mut fSlow0: f32 = (0.00100000005 * (self.fButton0 as f32));
		let mut fSlow1: f32 = (self.fHslider0 as f32);
		let mut fSlow2: f32 = (self.fHslider1 as f32);
		let mut iSlow3: i32 = ((self.fConst0 * fSlow2) as i32);
		let mut fSlow4: f32 = (self.fHslider2 as f32);
		let mut fSlow5: f32 = (6.90999985 * fSlow2);
		let mut fSlow6: f32 = (self.fHslider3 as f32);
		let mut fSlow7: f32 = (0.00100000005 * (self.fHslider4 as f32));
		let mut fSlow8: f32 = (0.00100000005 * (self.fHslider5 as f32));
		let mut fSlow9: f32 = (0.00100000005 * (self.fHslider6 as f32));
		let mut fSlow10: f32 = (self.fHslider7 as f32);
		let mut fSlow11: f32 = f32::max((0.5 * fSlow10), 23.4489498);
		let mut fSlow12: f32 = f32::max(20.0, f32::abs(fSlow11));
		let mut fSlow13: f32 = (self.fConst2 / fSlow12);
		let mut fSlow14: f32 = (self.fConst0 / fSlow11);
		let mut fSlow15: f32 = (self.fConst4 * fSlow10);
		let mut fSlow16: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst5 / fSlow11)));
		let mut fSlow17: f32 = f32::floor(fSlow16);
		let mut fSlow18: f32 = (fSlow17 + (1.0 - fSlow16));
		let mut iSlow19: i32 = (fSlow16 as i32);
		let mut fSlow20: f32 = (fSlow16 - fSlow17);
		let mut iSlow21: i32 = (iSlow19 + 1);
		let mut fSlow22: f32 = (self.fConst0 / fSlow10);
		let mut iSlow23: i32 = ((self.fConst6 / fSlow10) as i32);
		let mut fSlow24: f32 = (self.fConst3 * fSlow10);
		let mut fSlow25: f32 = (0.00100000005 * (self.fHslider8 as f32));
		let mut fSlow26: f32 = (1.0 / fSlow10);
		let mut fSlow27: f32 = (self.fConst7 * fSlow10);
		let mut fSlow28: f32 = f32::max(fSlow10, 23.4489498);
		let mut fSlow29: f32 = f32::max(20.0, f32::abs(fSlow28));
		let mut fSlow30: f32 = (self.fConst2 / fSlow29);
		let mut fSlow31: f32 = (self.fConst0 / fSlow28);
		let mut fSlow32: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst5 / fSlow28)));
		let mut fSlow33: f32 = f32::floor(fSlow32);
		let mut fSlow34: f32 = (fSlow33 + (1.0 - fSlow32));
		let mut iSlow35: i32 = (fSlow32 as i32);
		let mut fSlow36: f32 = (fSlow32 - fSlow33);
		let mut iSlow37: i32 = (iSlow35 + 1);
		let mut iSlow38: i32 = (fSlow22 as i32);
		let mut fSlow39: f32 = (0.00100000005 * (self.fHslider9 as f32));
		let mut fSlow40: f32 = (0.00100000005 * (self.fButton1 as f32));
		let mut fSlow41: f32 = (0.00100000005 * (self.fHslider10 as f32));
		let mut fSlow42: f32 = (self.fHslider11 as f32);
		let mut fSlow43: f32 = f32::max((0.5 * fSlow42), 23.4489498);
		let mut fSlow44: f32 = f32::max(20.0, f32::abs(fSlow43));
		let mut fSlow45: f32 = (self.fConst2 / fSlow44);
		let mut fSlow46: f32 = (self.fConst0 / fSlow43);
		let mut fSlow47: f32 = (self.fConst4 * fSlow42);
		let mut fSlow48: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst5 / fSlow43)));
		let mut fSlow49: f32 = f32::floor(fSlow48);
		let mut fSlow50: f32 = (fSlow49 + (1.0 - fSlow48));
		let mut iSlow51: i32 = (fSlow48 as i32);
		let mut fSlow52: f32 = (fSlow48 - fSlow49);
		let mut iSlow53: i32 = (iSlow51 + 1);
		let mut fSlow54: f32 = (self.fConst0 / fSlow42);
		let mut iSlow55: i32 = ((self.fConst6 / fSlow42) as i32);
		let mut fSlow56: f32 = (self.fConst3 * fSlow42);
		let mut fSlow57: f32 = (1.0 / fSlow42);
		let mut fSlow58: f32 = (self.fConst7 * fSlow42);
		let mut fSlow59: f32 = f32::max(fSlow42, 23.4489498);
		let mut fSlow60: f32 = f32::max(20.0, f32::abs(fSlow59));
		let mut fSlow61: f32 = (self.fConst2 / fSlow60);
		let mut fSlow62: f32 = (self.fConst0 / fSlow59);
		let mut fSlow63: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst5 / fSlow59)));
		let mut fSlow64: f32 = f32::floor(fSlow63);
		let mut fSlow65: f32 = (fSlow64 + (1.0 - fSlow63));
		let mut iSlow66: i32 = (fSlow63 as i32);
		let mut fSlow67: f32 = (fSlow63 - fSlow64);
		let mut iSlow68: i32 = (iSlow66 + 1);
		let mut iSlow69: i32 = (fSlow54 as i32);
		let zipped_iterators = outputs0;
		for output0 in zipped_iterators {
			self.iVec0[0] = 1;
			self.fRec1[0] = (fSlow0 + (0.999000013 * self.fRec1[1]));
			let mut iTemp0: i32 = ((self.fRec1[0] > 0.0) as i32);
			self.iRec2[0] = (iTemp0 * (self.iRec2[1] + 1));
			let mut iTemp1: i32 = ((self.iRec2[0] < iSlow3) as i32);
			let mut fTemp2: f32 = f32::exp((0.0 - (self.fConst1 / if (iTemp0 as i32 != 0) { if (iTemp1 as i32 != 0) { fSlow5 } else { fSlow4 } } else { fSlow1 })));
			self.fRec0[0] = ((self.fRec0[1] * fTemp2) + (if (iTemp0 as i32 != 0) { if (iTemp1 as i32 != 0) { 1.58730161 } else { (fSlow6 * (iTemp0 as f32)) } } else { 0.0 } * (1.0 - fTemp2)));
			self.fRec3[0] = (fSlow7 + (0.999000013 * self.fRec3[1]));
			self.fRec6[0] = (fSlow8 + (0.999000013 * self.fRec6[1]));
			self.fRec7[0] = (fSlow9 + (0.999000013 * self.fRec7[1]));
			let mut fTemp3: f32 = f32::min(1.0, f32::max(0.0, (2.0 - self.fRec7[0])));
			self.fVec1[0] = 0.25;
			self.fVec2[0] = fSlow12;
			let mut fTemp4: f32 = (self.iVec0[1] as f32);
			let mut fTemp5: f32 = (self.fRec8[1] + (self.fConst3 * self.fVec2[1]));
			self.fRec8[0] = (fTemp5 - f32::floor(fTemp5));
			let mut fTemp6: f32 = mydsp_faustpower2_f(((2.0 * self.fRec8[0]) + -1.0));
			self.fVec3[0] = fTemp6;
			let mut fTemp7: f32 = (fSlow13 * (fTemp4 * (fTemp6 - self.fVec3[1])));
			self.fVec4[(self.IOTA & 4095) as usize] = fTemp7;
			let mut fTemp8: f32 = f32::min(0.5, (0.5 * self.fRec7[0]));
			let mut fTemp9: f32 = f32::max(0.0, f32::min(2047.0, (fSlow14 * fTemp8)));
			let mut iTemp10: i32 = (fTemp9 as i32);
			let mut fTemp11: f32 = f32::floor(fTemp9);
			self.fRec9[0] = ((fTemp7 + (0.999000013 * self.fRec9[1])) - ((fSlow18 * self.fVec4[((self.IOTA - iSlow19) & 4095) as usize]) + (fSlow20 * self.fVec4[((self.IOTA - iSlow21) & 4095) as usize])));
			let mut fTemp12: f32 = f32::min(1.0, f32::max(0.0, (self.fRec7[0] + -2.0)));
			let mut fTemp13: f32 = (1.0 - (fTemp3 + fTemp12));
			let mut fTemp14: f32 = (self.fVec1[1] * fTemp12);
			self.iRec11[0] = ((self.iVec0[1] + self.iRec11[1]) % iSlow23);
			let mut fTemp15: f32 = (0.100000001 * (f32::max(3.0, self.fRec7[0]) + -3.0));
			let mut fTemp16: f32 = (fTemp15 + 0.5);
			let mut fTemp17: f32 = ((self.fRec10[1] * (1.0 - (((((self.iRec11[0] == 0) as i32) > 0) as i32) as f32))) + (fSlow24 * fTemp16));
			self.fRec10[0] = (fTemp17 - f32::floor(fTemp17));
			let mut fTemp18: f32 = mydsp_faustpower2_f(((2.0 * self.fRec10[0]) + -1.0));
			self.fVec5[0] = fTemp18;
			self.fRec12[0] = (fSlow25 + (0.999000013 * self.fRec12[1]));
			let mut fTemp19: f32 = (((self.fRec12[0] >= 2.0) as i32) as f32);
			let mut iTemp20: i32 = ((self.fRec12[0] >= 3.0) as i32);
			let mut fTemp21: f32 = if ((((self.fRec12[0] == 0.0) as i32) + iTemp20) as i32 != 0) { 1.0 } else { f32::max(f32::max(1.0, ((0.0199999996 * (self.fRec12[0] + -2.0999999)) + 1.0)), ((0.0199999996 * (1.0 - self.fRec12[0])) + 1.0)) };
			self.iRec14[0] = ((self.iVec0[1] + self.iRec14[1]) % ((fSlow22 / fTemp21) as i32));
			let mut fTemp22: f32 = (fTemp21 + fTemp15);
			let mut fTemp23: f32 = ((self.fRec13[1] * (1.0 - (((((self.iRec14[0] == 0) as i32) > 0) as i32) as f32))) + (fSlow24 * fTemp22));
			self.fRec13[0] = (fTemp23 - f32::floor(fTemp23));
			let mut fTemp24: f32 = mydsp_faustpower2_f(((2.0 * self.fRec13[0]) + -1.0));
			self.fVec6[0] = fTemp24;
			let mut fTemp25: f32 = f32::max((fSlow10 * fTemp21), 23.4489498);
			let mut fTemp26: f32 = f32::max(20.0, f32::abs(fTemp25));
			self.fVec7[0] = fTemp26;
			let mut fTemp27: f32 = (self.fRec15[1] + (self.fConst3 * self.fVec7[1]));
			self.fRec15[0] = (fTemp27 - f32::floor(fTemp27));
			let mut fTemp28: f32 = mydsp_faustpower2_f(((2.0 * self.fRec15[0]) + -1.0));
			self.fVec8[0] = fTemp28;
			let mut fTemp29: f32 = ((fTemp4 * (fTemp28 - self.fVec8[1])) / fTemp26);
			self.fVec9[(self.IOTA & 4095) as usize] = fTemp29;
			let mut fTemp30: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst0 * (fTemp8 / fTemp25))));
			let mut iTemp31: i32 = (fTemp30 as i32);
			let mut fTemp32: f32 = f32::floor(fTemp30);
			let mut fTemp33: f32 = ((fSlow26 * ((fTemp14 * (fTemp24 - self.fVec6[1])) / fTemp22)) + (0.25 * (fTemp3 * ((fTemp29 - (self.fVec9[((self.IOTA - iTemp31) & 4095) as usize] * (fTemp32 + (1.0 - fTemp30)))) - ((fTemp30 - fTemp32) * self.fVec9[((self.IOTA - (iTemp31 + 1)) & 4095) as usize])))));
			let mut fTemp34: f32 = if (iTemp20 as i32 != 0) { 1.49829996 } else { 1.0 };
			let mut fTemp35: f32 = (fTemp21 / fTemp34);
			self.iRec17[0] = ((self.iVec0[1] + self.iRec17[1]) % ((fSlow22 * fTemp35) as i32));
			let mut fTemp36: f32 = (fTemp34 / fTemp21);
			let mut fTemp37: f32 = (fTemp15 + fTemp36);
			let mut fTemp38: f32 = ((self.fRec16[1] * (1.0 - (((((self.iRec17[0] == 0) as i32) > 0) as i32) as f32))) + (fSlow24 * fTemp37));
			self.fRec16[0] = (fTemp38 - f32::floor(fTemp38));
			let mut fTemp39: f32 = mydsp_faustpower2_f(((2.0 * self.fRec16[0]) + -1.0));
			self.fVec10[0] = fTemp39;
			let mut fTemp40: f32 = f32::max((fSlow10 * fTemp36), 23.4489498);
			let mut fTemp41: f32 = f32::max(20.0, f32::abs(fTemp40));
			self.fVec11[0] = fTemp41;
			let mut fTemp42: f32 = (self.fRec18[1] + (self.fConst3 * self.fVec11[1]));
			self.fRec18[0] = (fTemp42 - f32::floor(fTemp42));
			let mut fTemp43: f32 = mydsp_faustpower2_f(((2.0 * self.fRec18[0]) + -1.0));
			self.fVec12[0] = fTemp43;
			let mut fTemp44: f32 = ((fTemp4 * (fTemp43 - self.fVec12[1])) / fTemp41);
			self.fVec13[(self.IOTA & 4095) as usize] = fTemp44;
			let mut fTemp45: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst0 * (fTemp8 / fTemp40))));
			let mut iTemp46: i32 = (fTemp45 as i32);
			let mut fTemp47: f32 = f32::floor(fTemp45);
			let mut fTemp48: f32 = ((fSlow26 * ((fTemp14 * (fTemp39 - self.fVec10[1])) / fTemp37)) + (0.25 * (fTemp3 * ((fTemp44 - (self.fVec13[((self.IOTA - iTemp46) & 4095) as usize] * (fTemp47 + (1.0 - fTemp45)))) - ((fTemp45 - fTemp47) * self.fVec13[((self.IOTA - (iTemp46 + 1)) & 4095) as usize])))));
			let mut fTemp49: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst5 / fTemp25)));
			let mut iTemp50: i32 = (fTemp49 as i32);
			let mut fTemp51: f32 = f32::floor(fTemp49);
			self.fRec19[0] = ((0.999000013 * self.fRec19[1]) + (self.fConst2 * ((fTemp29 - (self.fVec9[((self.IOTA - iTemp50) & 4095) as usize] * (fTemp51 + (1.0 - fTemp49)))) - ((fTemp49 - fTemp51) * self.fVec9[((self.IOTA - (iTemp50 + 1)) & 4095) as usize]))));
			let mut fTemp52: f32 = (self.fRec19[0] * fTemp21);
			let mut fTemp53: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst5 / fTemp40)));
			let mut iTemp54: i32 = (fTemp53 as i32);
			let mut fTemp55: f32 = f32::floor(fTemp53);
			self.fRec20[0] = ((0.999000013 * self.fRec20[1]) + (self.fConst2 * ((fTemp44 - (self.fVec13[((self.IOTA - iTemp54) & 4095) as usize] * (fTemp55 + (1.0 - fTemp53)))) - ((fTemp53 - fTemp55) * self.fVec13[((self.IOTA - (iTemp54 + 1)) & 4095) as usize]))));
			let mut fTemp56: f32 = (self.fRec20[0] * fTemp34);
			let mut fTemp57: f32 = (((self.fRec12[0] < 2.0) as i32) as f32);
			let mut fTemp58: f32 = f32::min(1.0, f32::max(0.0, (2.0 - self.fRec12[0])));
			let mut fTemp59: f32 = (1.0 - fTemp58);
			let mut fTemp60: f32 = (1.0 - self.fRec6[0]);
			self.fVec14[0] = fSlow29;
			let mut fTemp61: f32 = (self.fRec21[1] + (self.fConst3 * self.fVec14[1]));
			self.fRec21[0] = (fTemp61 - f32::floor(fTemp61));
			let mut fTemp62: f32 = mydsp_faustpower2_f(((2.0 * self.fRec21[0]) + -1.0));
			self.fVec15[0] = fTemp62;
			let mut fTemp63: f32 = (fSlow30 * (fTemp4 * (fTemp62 - self.fVec15[1])));
			self.fVec16[(self.IOTA & 4095) as usize] = fTemp63;
			let mut fTemp64: f32 = f32::max(0.0, f32::min(2047.0, (fSlow31 * fTemp8)));
			let mut iTemp65: i32 = (fTemp64 as i32);
			let mut fTemp66: f32 = f32::floor(fTemp64);
			self.fRec22[0] = ((fTemp63 + (0.999000013 * self.fRec22[1])) - ((fSlow34 * self.fVec16[((self.IOTA - iSlow35) & 4095) as usize]) + (fSlow36 * self.fVec16[((self.IOTA - iSlow37) & 4095) as usize])));
			self.iRec24[0] = ((self.iVec0[1] + self.iRec24[1]) % iSlow38);
			let mut fTemp67: f32 = (fTemp15 + 1.0);
			let mut fTemp68: f32 = ((self.fRec23[1] * (1.0 - (((((self.iRec24[0] == 0) as i32) > 0) as i32) as f32))) + (fSlow24 * fTemp67));
			self.fRec23[0] = (fTemp68 - f32::floor(fTemp68));
			let mut fTemp69: f32 = mydsp_faustpower2_f(((2.0 * self.fRec23[0]) + -1.0));
			self.fVec17[0] = fTemp69;
			self.fRec25[0] = (fSlow39 + (0.999000013 * self.fRec25[1]));
			let mut fTemp70: f32 = f32::tan((self.fConst8 * ((10000.0 * mydsp_faustpower2_f(self.fRec25[0])) + 100.0)));
			let mut fTemp71: f32 = (1.0 / fTemp70);
			let mut fTemp72: f32 = (((fTemp71 + -0.800000012) / fTemp70) + 1.0);
			let mut fTemp73: f32 = (1.0 - (1.0 / mydsp_faustpower2_f(fTemp70)));
			let mut fTemp74: f32 = (((fTemp71 + 0.800000012) / fTemp70) + 1.0);
			self.fRec5[0] = (((self.fRec6[0] * (((fTemp3 * (fTemp7 - ((self.fVec4[((self.IOTA - iTemp10) & 4095) as usize] * (fTemp11 + (1.0 - fTemp9))) + ((fTemp9 - fTemp11) * self.fVec4[((self.IOTA - (iTemp10 + 1)) & 4095) as usize])))) + (fSlow15 * (self.fRec9[0] * fTemp13))) + (fSlow22 * ((fTemp14 * (fTemp18 - self.fVec5[1])) / fTemp16)))) + (((fTemp19 * ((self.fConst0 * (fTemp33 + fTemp48)) + (fSlow27 * (fTemp13 * (fTemp52 + (fTemp56 / fTemp21)))))) + ((fTemp57 * ((fSlow27 * (fTemp52 * fTemp13)) + (self.fConst0 * fTemp33))) * (fTemp59 + (fTemp58 * ((fSlow27 * ((fTemp56 * fTemp13) / fTemp21)) + (self.fConst0 * fTemp48)))))) + (fTemp60 * (((fTemp3 * (fTemp63 - ((self.fVec16[((self.IOTA - iTemp65) & 4095) as usize] * (fTemp66 + (1.0 - fTemp64))) + ((fTemp64 - fTemp66) * self.fVec16[((self.IOTA - (iTemp65 + 1)) & 4095) as usize])))) + (fSlow27 * (self.fRec22[0] * fTemp13))) + (fSlow22 * ((fTemp14 * (fTemp69 - self.fVec17[1])) / fTemp67)))))) - (((self.fRec5[2] * fTemp72) + (2.0 * (self.fRec5[1] * fTemp73))) / fTemp74));
			self.fRec4[0] = ((((self.fRec5[1] + (0.5 * self.fRec5[0])) + (0.5 * self.fRec5[2])) - ((fTemp72 * self.fRec4[2]) + (2.0 * (fTemp73 * self.fRec4[1])))) / fTemp74);
			self.fRec27[0] = (fSlow40 + (0.999000013 * self.fRec27[1]));
			let mut iTemp75: i32 = ((self.fRec27[0] > 0.0) as i32);
			self.iRec28[0] = (iTemp75 * (self.iRec28[1] + 1));
			let mut iTemp76: i32 = ((self.iRec28[0] < iSlow3) as i32);
			let mut fTemp77: f32 = f32::exp((0.0 - (self.fConst1 / if (iTemp75 as i32 != 0) { if (iTemp76 as i32 != 0) { fSlow5 } else { fSlow4 } } else { fSlow1 })));
			self.fRec26[0] = ((self.fRec26[1] * fTemp77) + (if (iTemp75 as i32 != 0) { if (iTemp76 as i32 != 0) { 1.58730161 } else { (fSlow6 * (iTemp75 as f32)) } } else { 0.0 } * (1.0 - fTemp77)));
			self.fRec29[0] = (fSlow41 + (0.999000013 * self.fRec29[1]));
			self.fVec18[0] = fSlow44;
			let mut fTemp78: f32 = (self.fRec32[1] + (self.fConst3 * self.fVec18[1]));
			self.fRec32[0] = (fTemp78 - f32::floor(fTemp78));
			let mut fTemp79: f32 = mydsp_faustpower2_f(((2.0 * self.fRec32[0]) + -1.0));
			self.fVec19[0] = fTemp79;
			let mut fTemp80: f32 = (fSlow45 * (fTemp4 * (fTemp79 - self.fVec19[1])));
			self.fVec20[(self.IOTA & 4095) as usize] = fTemp80;
			let mut fTemp81: f32 = f32::max(0.0, f32::min(2047.0, (fSlow46 * fTemp8)));
			let mut iTemp82: i32 = (fTemp81 as i32);
			let mut fTemp83: f32 = f32::floor(fTemp81);
			self.fRec33[0] = ((fTemp80 + (0.999000013 * self.fRec33[1])) - ((fSlow50 * self.fVec20[((self.IOTA - iSlow51) & 4095) as usize]) + (fSlow52 * self.fVec20[((self.IOTA - iSlow53) & 4095) as usize])));
			self.iRec35[0] = ((self.iVec0[1] + self.iRec35[1]) % iSlow55);
			let mut fTemp84: f32 = ((self.fRec34[1] * (1.0 - (((((self.iRec35[0] == 0) as i32) > 0) as i32) as f32))) + (fSlow56 * fTemp16));
			self.fRec34[0] = (fTemp84 - f32::floor(fTemp84));
			let mut fTemp85: f32 = mydsp_faustpower2_f(((2.0 * self.fRec34[0]) + -1.0));
			self.fVec21[0] = fTemp85;
			self.iRec37[0] = ((self.iVec0[1] + self.iRec37[1]) % ((fSlow54 / fTemp21) as i32));
			let mut fTemp86: f32 = ((self.fRec36[1] * (1.0 - (((((self.iRec37[0] == 0) as i32) > 0) as i32) as f32))) + (fSlow56 * fTemp22));
			self.fRec36[0] = (fTemp86 - f32::floor(fTemp86));
			let mut fTemp87: f32 = mydsp_faustpower2_f(((2.0 * self.fRec36[0]) + -1.0));
			self.fVec22[0] = fTemp87;
			let mut fTemp88: f32 = f32::max((fSlow42 * fTemp21), 23.4489498);
			let mut fTemp89: f32 = f32::max(20.0, f32::abs(fTemp88));
			self.fVec23[0] = fTemp89;
			let mut fTemp90: f32 = (self.fRec38[1] + (self.fConst3 * self.fVec23[1]));
			self.fRec38[0] = (fTemp90 - f32::floor(fTemp90));
			let mut fTemp91: f32 = mydsp_faustpower2_f(((2.0 * self.fRec38[0]) + -1.0));
			self.fVec24[0] = fTemp91;
			let mut fTemp92: f32 = ((fTemp4 * (fTemp91 - self.fVec24[1])) / fTemp89);
			self.fVec25[(self.IOTA & 4095) as usize] = fTemp92;
			let mut fTemp93: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst0 * (fTemp8 / fTemp88))));
			let mut iTemp94: i32 = (fTemp93 as i32);
			let mut fTemp95: f32 = f32::floor(fTemp93);
			let mut fTemp96: f32 = ((fSlow57 * ((fTemp14 * (fTemp87 - self.fVec22[1])) / fTemp22)) + (0.25 * (fTemp3 * ((fTemp92 - (self.fVec25[((self.IOTA - iTemp94) & 4095) as usize] * (fTemp95 + (1.0 - fTemp93)))) - ((fTemp93 - fTemp95) * self.fVec25[((self.IOTA - (iTemp94 + 1)) & 4095) as usize])))));
			self.iRec40[0] = ((self.iVec0[1] + self.iRec40[1]) % ((fSlow54 * fTemp35) as i32));
			let mut fTemp97: f32 = ((self.fRec39[1] * (1.0 - (((((self.iRec40[0] == 0) as i32) > 0) as i32) as f32))) + (fSlow56 * fTemp37));
			self.fRec39[0] = (fTemp97 - f32::floor(fTemp97));
			let mut fTemp98: f32 = mydsp_faustpower2_f(((2.0 * self.fRec39[0]) + -1.0));
			self.fVec26[0] = fTemp98;
			let mut fTemp99: f32 = f32::max((fSlow42 * fTemp36), 23.4489498);
			let mut fTemp100: f32 = f32::max(20.0, f32::abs(fTemp99));
			self.fVec27[0] = fTemp100;
			let mut fTemp101: f32 = (self.fRec41[1] + (self.fConst3 * self.fVec27[1]));
			self.fRec41[0] = (fTemp101 - f32::floor(fTemp101));
			let mut fTemp102: f32 = mydsp_faustpower2_f(((2.0 * self.fRec41[0]) + -1.0));
			self.fVec28[0] = fTemp102;
			let mut fTemp103: f32 = ((fTemp4 * (fTemp102 - self.fVec28[1])) / fTemp100);
			self.fVec29[(self.IOTA & 4095) as usize] = fTemp103;
			let mut fTemp104: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst0 * (fTemp8 / fTemp99))));
			let mut iTemp105: i32 = (fTemp104 as i32);
			let mut fTemp106: f32 = f32::floor(fTemp104);
			let mut fTemp107: f32 = ((fSlow57 * ((fTemp14 * (fTemp98 - self.fVec26[1])) / fTemp37)) + (0.25 * (fTemp3 * ((fTemp103 - (self.fVec29[((self.IOTA - iTemp105) & 4095) as usize] * (fTemp106 + (1.0 - fTemp104)))) - ((fTemp104 - fTemp106) * self.fVec29[((self.IOTA - (iTemp105 + 1)) & 4095) as usize])))));
			let mut fTemp108: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst5 / fTemp88)));
			let mut iTemp109: i32 = (fTemp108 as i32);
			let mut fTemp110: f32 = f32::floor(fTemp108);
			self.fRec42[0] = ((0.999000013 * self.fRec42[1]) + (self.fConst2 * ((fTemp92 - (self.fVec25[((self.IOTA - iTemp109) & 4095) as usize] * (fTemp110 + (1.0 - fTemp108)))) - ((fTemp108 - fTemp110) * self.fVec25[((self.IOTA - (iTemp109 + 1)) & 4095) as usize]))));
			let mut fTemp111: f32 = (self.fRec42[0] * fTemp21);
			let mut fTemp112: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst5 / fTemp99)));
			let mut iTemp113: i32 = (fTemp112 as i32);
			let mut fTemp114: f32 = f32::floor(fTemp112);
			self.fRec43[0] = ((0.999000013 * self.fRec43[1]) + (self.fConst2 * ((fTemp103 - (self.fVec29[((self.IOTA - iTemp113) & 4095) as usize] * (fTemp114 + (1.0 - fTemp112)))) - ((fTemp112 - fTemp114) * self.fVec29[((self.IOTA - (iTemp113 + 1)) & 4095) as usize]))));
			let mut fTemp115: f32 = (self.fRec43[0] * fTemp34);
			self.fVec30[0] = fSlow60;
			let mut fTemp116: f32 = (self.fRec44[1] + (self.fConst3 * self.fVec30[1]));
			self.fRec44[0] = (fTemp116 - f32::floor(fTemp116));
			let mut fTemp117: f32 = mydsp_faustpower2_f(((2.0 * self.fRec44[0]) + -1.0));
			self.fVec31[0] = fTemp117;
			let mut fTemp118: f32 = (fSlow61 * (fTemp4 * (fTemp117 - self.fVec31[1])));
			self.fVec32[(self.IOTA & 4095) as usize] = fTemp118;
			let mut fTemp119: f32 = f32::max(0.0, f32::min(2047.0, (fSlow62 * fTemp8)));
			let mut iTemp120: i32 = (fTemp119 as i32);
			let mut fTemp121: f32 = f32::floor(fTemp119);
			self.fRec45[0] = ((fTemp118 + (0.999000013 * self.fRec45[1])) - ((fSlow65 * self.fVec32[((self.IOTA - iSlow66) & 4095) as usize]) + (fSlow67 * self.fVec32[((self.IOTA - iSlow68) & 4095) as usize])));
			self.iRec47[0] = ((self.iVec0[1] + self.iRec47[1]) % iSlow69);
			let mut fTemp122: f32 = ((self.fRec46[1] * (1.0 - (((((self.iRec47[0] == 0) as i32) > 0) as i32) as f32))) + (fSlow56 * fTemp67));
			self.fRec46[0] = (fTemp122 - f32::floor(fTemp122));
			let mut fTemp123: f32 = mydsp_faustpower2_f(((2.0 * self.fRec46[0]) + -1.0));
			self.fVec33[0] = fTemp123;
			self.fRec31[0] = (((self.fRec6[0] * (((fTemp3 * (fTemp80 - ((self.fVec20[((self.IOTA - iTemp82) & 4095) as usize] * (fTemp83 + (1.0 - fTemp81))) + ((fTemp81 - fTemp83) * self.fVec20[((self.IOTA - (iTemp82 + 1)) & 4095) as usize])))) + (fSlow47 * (self.fRec33[0] * fTemp13))) + (fSlow54 * ((fTemp14 * (fTemp85 - self.fVec21[1])) / fTemp16)))) + (((fTemp19 * ((self.fConst0 * (fTemp96 + fTemp107)) + (fSlow58 * (fTemp13 * (fTemp111 + (fTemp115 / fTemp21)))))) + ((fTemp57 * ((fSlow58 * (fTemp111 * fTemp13)) + (self.fConst0 * fTemp96))) * (fTemp59 + (fTemp58 * ((fSlow58 * ((fTemp115 * fTemp13) / fTemp21)) + (self.fConst0 * fTemp107)))))) + (fTemp60 * (((fTemp3 * (fTemp118 - ((self.fVec32[((self.IOTA - iTemp120) & 4095) as usize] * (fTemp121 + (1.0 - fTemp119))) + ((fTemp119 - fTemp121) * self.fVec32[((self.IOTA - (iTemp120 + 1)) & 4095) as usize])))) + (fSlow58 * (self.fRec45[0] * fTemp13))) + (fSlow54 * ((fTemp14 * (fTemp123 - self.fVec33[1])) / fTemp67)))))) - (((fTemp72 * self.fRec31[2]) + (2.0 * (fTemp73 * self.fRec31[1]))) / fTemp74));
			self.fRec30[0] = ((((self.fRec31[1] + (0.5 * self.fRec31[0])) + (0.5 * self.fRec31[2])) - ((fTemp72 * self.fRec30[2]) + (2.0 * (fTemp73 * self.fRec30[1])))) / fTemp74);
			*output0 = (((((f32::min(1.0, self.fRec0[0]) * self.fRec3[0]) * ((self.fRec4[1] + (0.5 * self.fRec4[0])) + (0.5 * self.fRec4[2]))) + ((f32::min(1.0, self.fRec26[0]) * self.fRec29[0]) * ((self.fRec30[1] + (0.5 * self.fRec30[0])) + (0.5 * self.fRec30[2])))) / fTemp74) as f32);
			self.iVec0[1] = self.iVec0[0];
			self.fRec1[1] = self.fRec1[0];
			self.iRec2[1] = self.iRec2[0];
			self.fRec0[1] = self.fRec0[0];
			self.fRec3[1] = self.fRec3[0];
			self.fRec6[1] = self.fRec6[0];
			self.fRec7[1] = self.fRec7[0];
			self.fVec1[1] = self.fVec1[0];
			self.fVec2[1] = self.fVec2[0];
			self.fRec8[1] = self.fRec8[0];
			self.fVec3[1] = self.fVec3[0];
			self.IOTA = (self.IOTA + 1);
			self.fRec9[1] = self.fRec9[0];
			self.iRec11[1] = self.iRec11[0];
			self.fRec10[1] = self.fRec10[0];
			self.fVec5[1] = self.fVec5[0];
			self.fRec12[1] = self.fRec12[0];
			self.iRec14[1] = self.iRec14[0];
			self.fRec13[1] = self.fRec13[0];
			self.fVec6[1] = self.fVec6[0];
			self.fVec7[1] = self.fVec7[0];
			self.fRec15[1] = self.fRec15[0];
			self.fVec8[1] = self.fVec8[0];
			self.iRec17[1] = self.iRec17[0];
			self.fRec16[1] = self.fRec16[0];
			self.fVec10[1] = self.fVec10[0];
			self.fVec11[1] = self.fVec11[0];
			self.fRec18[1] = self.fRec18[0];
			self.fVec12[1] = self.fVec12[0];
			self.fRec19[1] = self.fRec19[0];
			self.fRec20[1] = self.fRec20[0];
			self.fVec14[1] = self.fVec14[0];
			self.fRec21[1] = self.fRec21[0];
			self.fVec15[1] = self.fVec15[0];
			self.fRec22[1] = self.fRec22[0];
			self.iRec24[1] = self.iRec24[0];
			self.fRec23[1] = self.fRec23[0];
			self.fVec17[1] = self.fVec17[0];
			self.fRec25[1] = self.fRec25[0];
			self.fRec5[2] = self.fRec5[1];
			self.fRec5[1] = self.fRec5[0];
			self.fRec4[2] = self.fRec4[1];
			self.fRec4[1] = self.fRec4[0];
			self.fRec27[1] = self.fRec27[0];
			self.iRec28[1] = self.iRec28[0];
			self.fRec26[1] = self.fRec26[0];
			self.fRec29[1] = self.fRec29[0];
			self.fVec18[1] = self.fVec18[0];
			self.fRec32[1] = self.fRec32[0];
			self.fVec19[1] = self.fVec19[0];
			self.fRec33[1] = self.fRec33[0];
			self.iRec35[1] = self.iRec35[0];
			self.fRec34[1] = self.fRec34[0];
			self.fVec21[1] = self.fVec21[0];
			self.iRec37[1] = self.iRec37[0];
			self.fRec36[1] = self.fRec36[0];
			self.fVec22[1] = self.fVec22[0];
			self.fVec23[1] = self.fVec23[0];
			self.fRec38[1] = self.fRec38[0];
			self.fVec24[1] = self.fVec24[0];
			self.iRec40[1] = self.iRec40[0];
			self.fRec39[1] = self.fRec39[0];
			self.fVec26[1] = self.fVec26[0];
			self.fVec27[1] = self.fVec27[0];
			self.fRec41[1] = self.fRec41[0];
			self.fVec28[1] = self.fVec28[0];
			self.fRec42[1] = self.fRec42[0];
			self.fRec43[1] = self.fRec43[0];
			self.fVec30[1] = self.fVec30[0];
			self.fRec44[1] = self.fRec44[0];
			self.fVec31[1] = self.fVec31[0];
			self.fRec45[1] = self.fRec45[0];
			self.iRec47[1] = self.iRec47[0];
			self.fRec46[1] = self.fRec46[0];
			self.fVec33[1] = self.fVec33[0];
			self.fRec31[2] = self.fRec31[1];
			self.fRec31[1] = self.fRec31[0];
			self.fRec30[2] = self.fRec30[1];
			self.fRec30[1] = self.fRec30[0];
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
