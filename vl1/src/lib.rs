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
	fEntry0: 0.0,
	fSampleRate: 0,
	fConst0: 0.0,
	fConst1: 0.0,
	fConst2: 0.0,
	fConst3: 0.0,
	fHslider0: 0.0,
	iVec0: [0;2],
	fRec0: [0.0;2],
	fConst4: 0.0,
	fConst5: 0.0,
	fHslider1: 0.0,
	fEntry1: 0.0,
	fVec1: [0.0;2],
	fRec3: [0.0;2],
	fVec2: [0.0;2],
	fRec2: [0.0;2],
	fVec3: [0.0;2],
	IOTA: 0,
	fVec4: [0.0;4096],
	fConst6: 0.0,
	fButton0: 0.0,
	fRec5: [0.0;2],
	fEntry2: 0.0,
	fRec4: [0.0;2],
	fEntry3: 0.0,
	fEntry4: 0.0,
	fRec7: [0.0;2],
	fEntry5: 0.0,
	fRec6: [0.0;2],
	fEntry6: 0.0,
	fVec5: [0.0;2],
	fRec8: [0.0;2],
	fConst7: 0.0,
	fConst8: 0.0,
	fRec1: [0.0;3],
	fVec6: [0.0;2],
	fRec10: [0.0;2],
	fVec7: [0.0;2],
	fVec8: [0.0;4096],
	fConst9: 0.0,
	fRec9: [0.0;3],
	fConst10: 0.0,
	fConst11: 0.0,
	fConst12: 0.0,
	fConst13: 0.0,
	fConst14: 0.0,
	fConst15: 0.0,
	fConst16: 0.0,
	fConst17: 0.0,
	fConst18: 0.0,
	fConst19: 0.0,
	fRec11: [0.0;3],
	fRec12: [0.0;3],
	fConst20: 0.0,
	fConst21: 0.0,
	fVec9: [0.0;2],
	fRec14: [0.0;2],
	fVec10: [0.0;2],
	fVec11: [0.0;4096],
	fConst22: 0.0,
	fRec13: [0.0;3],
	fConst23: 0.0,
	fConst24: 0.0,
	fConst25: 0.0,
	fRec15: [0.0;3],
	fRec16: [0.0;3],
	fEntry7: 0.0,
	fVec12: [0.0;2],
	fRec19: [0.0;2],
	fVec13: [0.0;2],
	fRec18: [0.0;2],
	fVec14: [0.0;2],
	fVec15: [0.0;4096],
	fRec17: [0.0;3],
	fVec16: [0.0;2],
	fRec21: [0.0;2],
	fVec17: [0.0;2],
	fVec18: [0.0;4096],
	fRec20: [0.0;3],
	fConst26: 0.0,
	fConst27: 0.0,
	fRec22: [0.0;3],
	next_allocated_voice_age: 1000000000,
	next_unallocated_voice_age: 0,
	voices: [VoiceInfo {active: false,note: 0,channel: 0,voice_age: 0,};1],
	voice_freq: [0;1],
	voice_gain: [0;1],
	voice_gate: [0;1],
};

type T = f32;

struct mydsp {
	fEntry0: f32,
	fSampleRate: i32,
	fConst0: f32,
	fConst1: f32,
	fConst2: f32,
	fConst3: f32,
	fHslider0: f32,
	iVec0: [i32;2],
	fRec0: [f32;2],
	fConst4: f32,
	fConst5: f32,
	fHslider1: f32,
	fEntry1: f32,
	fVec1: [f32;2],
	fRec3: [f32;2],
	fVec2: [f32;2],
	fRec2: [f32;2],
	fVec3: [f32;2],
	IOTA: i32,
	fVec4: [f32;4096],
	fConst6: f32,
	fButton0: f32,
	fRec5: [f32;2],
	fEntry2: f32,
	fRec4: [f32;2],
	fEntry3: f32,
	fEntry4: f32,
	fRec7: [f32;2],
	fEntry5: f32,
	fRec6: [f32;2],
	fEntry6: f32,
	fVec5: [f32;2],
	fRec8: [f32;2],
	fConst7: f32,
	fConst8: f32,
	fRec1: [f32;3],
	fVec6: [f32;2],
	fRec10: [f32;2],
	fVec7: [f32;2],
	fVec8: [f32;4096],
	fConst9: f32,
	fRec9: [f32;3],
	fConst10: f32,
	fConst11: f32,
	fConst12: f32,
	fConst13: f32,
	fConst14: f32,
	fConst15: f32,
	fConst16: f32,
	fConst17: f32,
	fConst18: f32,
	fConst19: f32,
	fRec11: [f32;3],
	fRec12: [f32;3],
	fConst20: f32,
	fConst21: f32,
	fVec9: [f32;2],
	fRec14: [f32;2],
	fVec10: [f32;2],
	fVec11: [f32;4096],
	fConst22: f32,
	fRec13: [f32;3],
	fConst23: f32,
	fConst24: f32,
	fConst25: f32,
	fRec15: [f32;3],
	fRec16: [f32;3],
	fEntry7: f32,
	fVec12: [f32;2],
	fRec19: [f32;2],
	fVec13: [f32;2],
	fRec18: [f32;2],
	fVec14: [f32;2],
	fVec15: [f32;4096],
	fRec17: [f32;3],
	fVec16: [f32;2],
	fRec21: [f32;2],
	fVec17: [f32;2],
	fVec18: [f32;4096],
	fRec20: [f32;3],
	fConst26: f32,
	fConst27: f32,
	fRec22: [f32;3],

	next_allocated_voice_age: i64,
	next_unallocated_voice_age: i64,
	voices: [VoiceInfo;1],
	voice_freq: [u32;1],
	voice_gain: [u32;1],
	voice_gate: [u32;1],
}

impl mydsp {
		
	fn new() -> mydsp { 
		mydsp {
			fEntry0: 0.0,
			fSampleRate: 0,
			fConst0: 0.0,
			fConst1: 0.0,
			fConst2: 0.0,
			fConst3: 0.0,
			fHslider0: 0.0,
			iVec0: [0;2],
			fRec0: [0.0;2],
			fConst4: 0.0,
			fConst5: 0.0,
			fHslider1: 0.0,
			fEntry1: 0.0,
			fVec1: [0.0;2],
			fRec3: [0.0;2],
			fVec2: [0.0;2],
			fRec2: [0.0;2],
			fVec3: [0.0;2],
			IOTA: 0,
			fVec4: [0.0;4096],
			fConst6: 0.0,
			fButton0: 0.0,
			fRec5: [0.0;2],
			fEntry2: 0.0,
			fRec4: [0.0;2],
			fEntry3: 0.0,
			fEntry4: 0.0,
			fRec7: [0.0;2],
			fEntry5: 0.0,
			fRec6: [0.0;2],
			fEntry6: 0.0,
			fVec5: [0.0;2],
			fRec8: [0.0;2],
			fConst7: 0.0,
			fConst8: 0.0,
			fRec1: [0.0;3],
			fVec6: [0.0;2],
			fRec10: [0.0;2],
			fVec7: [0.0;2],
			fVec8: [0.0;4096],
			fConst9: 0.0,
			fRec9: [0.0;3],
			fConst10: 0.0,
			fConst11: 0.0,
			fConst12: 0.0,
			fConst13: 0.0,
			fConst14: 0.0,
			fConst15: 0.0,
			fConst16: 0.0,
			fConst17: 0.0,
			fConst18: 0.0,
			fConst19: 0.0,
			fRec11: [0.0;3],
			fRec12: [0.0;3],
			fConst20: 0.0,
			fConst21: 0.0,
			fVec9: [0.0;2],
			fRec14: [0.0;2],
			fVec10: [0.0;2],
			fVec11: [0.0;4096],
			fConst22: 0.0,
			fRec13: [0.0;3],
			fConst23: 0.0,
			fConst24: 0.0,
			fConst25: 0.0,
			fRec15: [0.0;3],
			fRec16: [0.0;3],
			fEntry7: 0.0,
			fVec12: [0.0;2],
			fRec19: [0.0;2],
			fVec13: [0.0;2],
			fRec18: [0.0;2],
			fVec14: [0.0;2],
			fVec15: [0.0;4096],
			fRec17: [0.0;3],
			fVec16: [0.0;2],
			fRec21: [0.0;2],
			fVec17: [0.0;2],
			fVec18: [0.0;4096],
			fRec20: [0.0;3],
			fConst26: 0.0,
			fConst27: 0.0,
			fRec22: [0.0;3],
				next_allocated_voice_age: 1000000000,
				next_unallocated_voice_age: 0,
				voices: [VoiceInfo {active: false,note: 0,channel: 0,voice_age: 0,};1],
				voice_freq: [0;1],
				voice_gain: [0;1],
				voice_gate: [0;1],
		}
	}
	pub fn get_voices(&self) -> i32 { 
		1
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
		self.fEntry0 = 0.0;
		self.fHslider0 = 0.5;
		self.fHslider1 = 440.0;
		self.fEntry1 = 0.0;
		self.fButton0 = 0.0;
		self.fEntry2 = 0.178000003;
		self.fEntry3 = 0.00499999989;
		self.fEntry4 = 0.178000003;
		self.fEntry5 = 0.305999994;
		self.fEntry6 = 0.0;
		self.fEntry7 = 4.5;
	}
	fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.iVec0[l0 as usize] = 0;
		}
		for l1 in 0..2 {
			self.fRec0[l1 as usize] = 0.0;
		}
		for l2 in 0..2 {
			self.fVec1[l2 as usize] = 0.0;
		}
		for l3 in 0..2 {
			self.fRec3[l3 as usize] = 0.0;
		}
		for l4 in 0..2 {
			self.fVec2[l4 as usize] = 0.0;
		}
		for l5 in 0..2 {
			self.fRec2[l5 as usize] = 0.0;
		}
		for l6 in 0..2 {
			self.fVec3[l6 as usize] = 0.0;
		}
		self.IOTA = 0;
		for l7 in 0..4096 {
			self.fVec4[l7 as usize] = 0.0;
		}
		for l8 in 0..2 {
			self.fRec5[l8 as usize] = 0.0;
		}
		for l9 in 0..2 {
			self.fRec4[l9 as usize] = 0.0;
		}
		for l10 in 0..2 {
			self.fRec7[l10 as usize] = 0.0;
		}
		for l11 in 0..2 {
			self.fRec6[l11 as usize] = 0.0;
		}
		for l12 in 0..2 {
			self.fVec5[l12 as usize] = 0.0;
		}
		for l13 in 0..2 {
			self.fRec8[l13 as usize] = 0.0;
		}
		for l14 in 0..3 {
			self.fRec1[l14 as usize] = 0.0;
		}
		for l15 in 0..2 {
			self.fVec6[l15 as usize] = 0.0;
		}
		for l16 in 0..2 {
			self.fRec10[l16 as usize] = 0.0;
		}
		for l17 in 0..2 {
			self.fVec7[l17 as usize] = 0.0;
		}
		for l18 in 0..4096 {
			self.fVec8[l18 as usize] = 0.0;
		}
		for l19 in 0..3 {
			self.fRec9[l19 as usize] = 0.0;
		}
		for l20 in 0..3 {
			self.fRec11[l20 as usize] = 0.0;
		}
		for l21 in 0..3 {
			self.fRec12[l21 as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.fVec9[l22 as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.fRec14[l23 as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.fVec10[l24 as usize] = 0.0;
		}
		for l25 in 0..4096 {
			self.fVec11[l25 as usize] = 0.0;
		}
		for l26 in 0..3 {
			self.fRec13[l26 as usize] = 0.0;
		}
		for l27 in 0..3 {
			self.fRec15[l27 as usize] = 0.0;
		}
		for l28 in 0..3 {
			self.fRec16[l28 as usize] = 0.0;
		}
		for l29 in 0..2 {
			self.fVec12[l29 as usize] = 0.0;
		}
		for l30 in 0..2 {
			self.fRec19[l30 as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fVec13[l31 as usize] = 0.0;
		}
		for l32 in 0..2 {
			self.fRec18[l32 as usize] = 0.0;
		}
		for l33 in 0..2 {
			self.fVec14[l33 as usize] = 0.0;
		}
		for l34 in 0..4096 {
			self.fVec15[l34 as usize] = 0.0;
		}
		for l35 in 0..3 {
			self.fRec17[l35 as usize] = 0.0;
		}
		for l36 in 0..2 {
			self.fVec16[l36 as usize] = 0.0;
		}
		for l37 in 0..2 {
			self.fRec21[l37 as usize] = 0.0;
		}
		for l38 in 0..2 {
			self.fVec17[l38 as usize] = 0.0;
		}
		for l39 in 0..4096 {
			self.fVec18[l39 as usize] = 0.0;
		}
		for l40 in 0..3 {
			self.fRec20[l40 as usize] = 0.0;
		}
		for l41 in 0..3 {
			self.fRec22[l41 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		self.fConst0 = f32::min(192000.0, f32::max(1.0, (self.fSampleRate as f32)));
		self.fConst1 = f32::tan((9424.77832 / self.fConst0));
		self.fConst2 = (1.0 / self.fConst1);
		self.fConst3 = (1.0 / (((self.fConst2 + 2.0) / self.fConst1) + 1.0));
		self.fConst4 = (1.25 * self.fConst0);
		self.fConst5 = (1.0 / self.fConst0);
		self.fConst6 = (0.699999988 * self.fConst0);
		self.fConst7 = (((self.fConst2 + -2.0) / self.fConst1) + 1.0);
		self.fConst8 = (2.0 * (1.0 - (1.0 / mydsp_faustpower2_f(self.fConst1))));
		self.fConst9 = (0.5 * self.fConst0);
		self.fConst10 = f32::tan((10995.5742 / self.fConst0));
		self.fConst11 = (1.0 / self.fConst10);
		self.fConst12 = (1.0 / (((self.fConst11 + 0.200000003) / self.fConst10) + 1.0));
		self.fConst13 = (0.25 * self.fConst0);
		self.fConst14 = (0.400000006 * self.fConst0);
		self.fConst15 = (0.300000012 * self.fConst0);
		self.fConst16 = (0.200000003 * self.fConst0);
		self.fConst17 = (0.100000001 * self.fConst0);
		self.fConst18 = (((self.fConst11 + -0.200000003) / self.fConst10) + 1.0);
		self.fConst19 = (2.0 * (1.0 - (1.0 / mydsp_faustpower2_f(self.fConst10))));
		self.fConst20 = (1.0 / (((self.fConst11 + 1.0) / self.fConst10) + 1.0));
		self.fConst21 = (0.625 * self.fConst0);
		self.fConst22 = (1.0 - ((1.0 - self.fConst11) / self.fConst10));
		self.fConst23 = (1.0 / (((self.fConst2 + 0.5) / self.fConst1) + 1.0));
		self.fConst24 = (0.125 * self.fConst0);
		self.fConst25 = (((self.fConst2 + -0.5) / self.fConst1) + 1.0);
		self.fConst26 = (1.0 / (((self.fConst11 + 0.100000001) / self.fConst10) + 1.0));
		self.fConst27 = (((self.fConst11 + -0.100000001) / self.fConst10) + 1.0);
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
			"Attack" => Param { index: 0, range: ParamRange::new(0.0050000000000000001, 0.001, 0.5, 0.050000000000000003) },
			"Decay" => Param { index: 1, range: ParamRange::new(0.17799999999999999, 0.001, 2.0, 0.20000000000000001) },
			"PitchMod" => Param { index: 2, range: ParamRange::new(4.5, 2.7000000000000002, 10.0, 0.36499999999999999) },
			"Release" => Param { index: 3, range: ParamRange::new(0.17799999999999999, 0.001, 2.0, 0.20000000000000001) },
			"Sustain" => Param { index: 4, range: ParamRange::new(0.30599999999999999, 0.10000000000000001, 1.0, 0.10000000000000001) },
			"Tremelo" => Param { index: 5, range: ParamRange::new(0.0, 0.0, 10.0, 1.0) },
			"Vibrato" => Param { index: 6, range: ParamRange::new(0.0, 0.0, 10.0, 1.0) },
			"Waveform" => Param { index: 7, range: ParamRange::new(0.0, 0.0, 9.0, 1.0) },
			"freq_v0" => Param { index: 8, range: ParamRange::new(440.0, 50.0, 2000.0, 0.01) },
			"gain_v0" => Param { index: 9, range: ParamRange::new(0.5, 0.0, 1.0, 0.01) },
			"gate_v0" => Param { index: 10, range: ParamRange::new(0.0,0.0,0.0,0.0) },
			
			
			
			_ => Param { index: -1, range: ParamRange::new(0.0, 0.0, 0.0, 0.0)}
		}
	}
	fn init_voices(&mut self) {
		self.voice_freq[0] = self.get_param_info("freq_v0").index as u32;
		self.voice_gain[0] = self.get_param_info("gain_v0").index as u32;
		self.voice_gate[0] = self.get_param_info("gate_v0").index as u32;
	}
	pub fn handle_note_on(&mut self, mn: Note, vel: f32) {
		// set params for voice
		self.set_param(self.voice_gate[0], 1.0);
		self.set_param(self.voice_gain[0], vel);
		self.set_param(self.voice_freq[0], to_freq(mn));
	}
	pub fn handle_note_off(&mut self, mn: Note, vel: f32) {
		// set params for voice
		self.set_param(self.voice_gate[0], 0.0);
		self.set_param(self.voice_gain[0], vel);
			
	}
	fn init_buffers(&self) {
		unsafe {
			OUTPUTS[0] = OUT_BUFFER0.as_mut_ptr();
		};
	}
	
	pub fn get_param(&self, param: u32) -> T {
		match param {
			10 => self.fButton0,
			7 => self.fEntry0,
			6 => self.fEntry1,
			3 => self.fEntry2,
			0 => self.fEntry3,
			1 => self.fEntry4,
			4 => self.fEntry5,
			5 => self.fEntry6,
			2 => self.fEntry7,
			9 => self.fHslider0,
			8 => self.fHslider1,
			_ => 0.,
		}
	}
	
	pub fn set_param(&mut self, param: u32, value: T) {
		match param {
			10 => { self.fButton0 = value }
			7 => { self.fEntry0 = value }
			6 => { self.fEntry1 = value }
			3 => { self.fEntry2 = value }
			0 => { self.fEntry3 = value }
			1 => { self.fEntry4 = value }
			4 => { self.fEntry5 = value }
			5 => { self.fEntry6 = value }
			2 => { self.fEntry7 = value }
			9 => { self.fHslider0 = value }
			8 => { self.fHslider1 = value }
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
		let mut fSlow0: f32 = (self.fEntry0 as f32);
		let mut iSlow1: i32 = ((fSlow0 >= 5.0) as i32);
		let mut iSlow2: i32 = ((fSlow0 >= 3.0) as i32);
		let mut iSlow3: i32 = ((fSlow0 >= 2.0) as i32);
		let mut iSlow4: i32 = ((fSlow0 >= 1.0) as i32);
		let mut fSlow5: f32 = (0.00100000005 * (self.fHslider0 as f32));
		let mut fSlow6: f32 = (self.fHslider1 as f32);
		let mut fSlow7: f32 = (self.fEntry1 as f32);
		let mut fSlow8: f32 = (0.00100000005 * (self.fButton0 as f32));
		let mut fSlow9: f32 = f32::max(0.00100000005, (self.fConst0 * (self.fEntry2 as f32)));
		let mut fSlow10: f32 = (self.fEntry3 as f32);
		let mut fSlow11: f32 = (fSlow10 + (self.fEntry4 as f32));
		let mut fSlow12: f32 = (self.fConst0 * fSlow11);
		let mut fSlow13: f32 = (self.fConst0 * fSlow10);
		let mut fSlow14: f32 = (self.fEntry5 as f32);
		let mut fSlow15: f32 = (1.0 / (0.0 - (self.fConst0 * (fSlow10 - fSlow11))));
		let mut fSlow16: f32 = (self.fConst5 / fSlow10);
		let mut fSlow17: f32 = (1.0 / fSlow9);
		let mut fSlow18: f32 = (self.fEntry6 as f32);
		let mut fSlow19: f32 = (2.0 * fSlow6);
		let mut iSlow20: i32 = ((fSlow0 >= 4.0) as i32);
		let mut fSlow21: f32 = (0.5 * fSlow6);
		let mut iSlow22: i32 = ((fSlow0 >= 8.0) as i32);
		let mut iSlow23: i32 = ((fSlow0 >= 7.0) as i32);
		let mut iSlow24: i32 = ((fSlow0 >= 6.0) as i32);
		let mut fSlow25: f32 = (self.fEntry7 as f32);
		let mut iSlow26: i32 = ((fSlow0 >= 9.0) as i32);
		let zipped_iterators = outputs0;
		for output0 in zipped_iterators {
			self.iVec0[0] = 1;
			self.fRec0[0] = (fSlow5 + (0.999000013 * self.fRec0[1]));
			let mut fTemp0: f32 = (self.iVec0[1] as f32);
			self.fVec1[0] = 5.69999981;
			let mut fTemp1: f32 = ((self.fConst5 * self.fVec1[1]) + self.fRec3[1]);
			self.fRec3[0] = (fTemp1 - f32::floor(fTemp1));
			let mut fTemp2: f32 = (fSlow7 * ((2.0 * (1.0 - f32::abs(((2.0 * self.fRec3[0]) + -1.0)))) + -1.0));
			let mut fTemp3: f32 = f32::max((fSlow6 + fTemp2), 23.4489498);
			let mut fTemp4: f32 = f32::max(20.0, f32::abs(fTemp3));
			self.fVec2[0] = fTemp4;
			let mut fTemp5: f32 = (self.fRec2[1] + (self.fConst5 * self.fVec2[1]));
			self.fRec2[0] = (fTemp5 - f32::floor(fTemp5));
			let mut fTemp6: f32 = mydsp_faustpower2_f(((2.0 * self.fRec2[0]) + -1.0));
			self.fVec3[0] = fTemp6;
			let mut fTemp7: f32 = ((fTemp0 * (fTemp6 - self.fVec3[1])) / fTemp4);
			self.fVec4[(self.IOTA & 4095) as usize] = fTemp7;
			let mut fTemp8: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst6 / fTemp3)));
			let mut iTemp9: i32 = (fTemp8 as i32);
			let mut fTemp10: f32 = f32::floor(fTemp8);
			self.fRec5[0] = (fSlow8 + (0.999000013 * self.fRec5[1]));
			let mut iTemp11: i32 = ((self.fRec5[0] > 0.0) as i32);
			self.fRec4[0] = if (((iTemp11 > 0) as i32) as i32 != 0) { 0.0 } else { f32::min(fSlow9, (self.fRec4[1] + 1.0)) };
			self.fRec7[0] = if (((((self.fRec5[0] == 0.0) as i32) > 0) as i32) as i32 != 0) { 0.0 } else { f32::min(fSlow12, (self.fRec7[1] + 1.0)) };
			let mut iTemp12: i32 = ((self.fRec7[0] < fSlow13) as i32);
			let mut fTemp13: f32 = (fSlow14 * self.fRec5[0]);
			self.fRec6[0] = if (iTemp11 as i32 != 0) { ((iTemp11 as f32) * if (iTemp12 as i32 != 0) { if (((self.fRec7[0] < 0.0) as i32) as i32 != 0) { 0.0 } else { if (iTemp12 as i32 != 0) { (fSlow16 * self.fRec7[0]) } else { 1.0 } } } else { if (((self.fRec7[0] < fSlow12) as i32) as i32 != 0) { ((fSlow15 * ((self.fRec7[0] - fSlow13) * (fTemp13 + -1.0))) + 1.0) } else { fTemp13 } }) } else { self.fRec6[1] };
			let mut fTemp14: f32 = if (((self.fRec4[0] < 0.0) as i32) as i32 != 0) { self.fRec6[0] } else { if (((self.fRec4[0] < fSlow9) as i32) as i32 != 0) { (self.fRec6[0] + (fSlow17 * (0.0 - (self.fRec4[0] * self.fRec6[0])))) } else { 0.0 } };
			self.fVec5[0] = fSlow18;
			let mut fTemp15: f32 = (self.fRec8[1] + (self.fConst5 * self.fVec5[1]));
			self.fRec8[0] = (fTemp15 - f32::floor(fTemp15));
			let mut fTemp16: f32 = ((2.0 * (1.0 - f32::abs(((2.0 * self.fRec8[0]) + -1.0)))) + -1.0);
			self.fRec1[0] = ((self.fConst4 * (((self.fRec0[0] * ((fTemp7 - (self.fVec4[((self.IOTA - iTemp9) & 4095) as usize] * (fTemp10 + (1.0 - fTemp8)))) - ((fTemp8 - fTemp10) * self.fVec4[((self.IOTA - (iTemp9 + 1)) & 4095) as usize]))) * fTemp14) * fTemp16)) - (self.fConst3 * ((self.fConst7 * self.fRec1[2]) + (self.fConst8 * self.fRec1[1]))));
			let mut fTemp17: f32 = ((self.fRec0[0] * fTemp14) * fTemp16);
			let mut fTemp18: f32 = f32::max((fSlow19 + fTemp2), 23.4489498);
			let mut fTemp19: f32 = f32::max(20.0, f32::abs(fTemp18));
			self.fVec6[0] = fTemp19;
			let mut fTemp20: f32 = (self.fRec10[1] + (self.fConst5 * self.fVec6[1]));
			self.fRec10[0] = (fTemp20 - f32::floor(fTemp20));
			let mut fTemp21: f32 = mydsp_faustpower2_f(((2.0 * self.fRec10[0]) + -1.0));
			self.fVec7[0] = fTemp21;
			let mut fTemp22: f32 = ((fTemp0 * (fTemp21 - self.fVec7[1])) / fTemp19);
			self.fVec8[(self.IOTA & 4095) as usize] = fTemp22;
			let mut fTemp23: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst9 / fTemp18)));
			let mut iTemp24: i32 = (fTemp23 as i32);
			let mut fTemp25: f32 = f32::floor(fTemp23);
			self.fRec9[0] = ((self.fConst4 * (fTemp17 * ((fTemp22 - (self.fVec8[((self.IOTA - iTemp24) & 4095) as usize] * (fTemp25 + (1.0 - fTemp23)))) - ((fTemp23 - fTemp25) * self.fVec8[((self.IOTA - (iTemp24 + 1)) & 4095) as usize])))) - (self.fConst3 * ((self.fConst7 * self.fRec9[2]) + (self.fConst8 * self.fRec9[1]))));
			let mut fTemp26: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst9 / fTemp3)));
			let mut iTemp27: i32 = (fTemp26 as i32);
			let mut fTemp28: f32 = f32::floor(fTemp26);
			let mut fTemp29: f32 = ((fTemp7 - (self.fVec4[((self.IOTA - iTemp27) & 4095) as usize] * (fTemp28 + (1.0 - fTemp26)))) - ((fTemp26 - fTemp28) * self.fVec4[((self.IOTA - (iTemp27 + 1)) & 4095) as usize]));
			let mut fTemp30: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst14 / fTemp3)));
			let mut iTemp31: i32 = (fTemp30 as i32);
			let mut fTemp32: f32 = f32::floor(fTemp30);
			let mut fTemp33: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst15 / fTemp3)));
			let mut iTemp34: i32 = (fTemp33 as i32);
			let mut fTemp35: f32 = f32::floor(fTemp33);
			let mut fTemp36: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst16 / fTemp3)));
			let mut iTemp37: i32 = (fTemp36 as i32);
			let mut fTemp38: f32 = f32::floor(fTemp36);
			let mut fTemp39: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst17 / fTemp3)));
			let mut iTemp40: i32 = (fTemp39 as i32);
			let mut fTemp41: f32 = f32::floor(fTemp39);
			self.fRec11[0] = ((self.fConst13 * (fTemp17 * ((((fTemp29 + ((fTemp7 - (self.fVec4[((self.IOTA - iTemp31) & 4095) as usize] * (fTemp32 + (1.0 - fTemp30)))) - ((fTemp30 - fTemp32) * self.fVec4[((self.IOTA - (iTemp31 + 1)) & 4095) as usize]))) + ((fTemp7 - (self.fVec4[((self.IOTA - iTemp34) & 4095) as usize] * (fTemp35 + (1.0 - fTemp33)))) - ((fTemp33 - fTemp35) * self.fVec4[((self.IOTA - (iTemp34 + 1)) & 4095) as usize]))) + ((fTemp7 - (self.fVec4[((self.IOTA - iTemp37) & 4095) as usize] * (fTemp38 + (1.0 - fTemp36)))) - ((fTemp36 - fTemp38) * self.fVec4[((self.IOTA - (iTemp37 + 1)) & 4095) as usize]))) + ((fTemp7 - (self.fVec4[((self.IOTA - iTemp40) & 4095) as usize] * (fTemp41 + (1.0 - fTemp39)))) - ((fTemp39 - fTemp41) * self.fVec4[((self.IOTA - (iTemp40 + 1)) & 4095) as usize]))))) - (self.fConst12 * ((self.fConst18 * self.fRec11[2]) + (self.fConst19 * self.fRec11[1]))));
			self.fRec12[0] = ((self.fConst4 * (fTemp17 * fTemp29)) - (self.fConst3 * ((self.fConst7 * self.fRec12[2]) + (self.fConst8 * self.fRec12[1]))));
			let mut fTemp42: f32 = f32::max((fSlow21 + fTemp2), 23.4489498);
			let mut fTemp43: f32 = f32::max(20.0, f32::abs(fTemp42));
			self.fVec9[0] = fTemp43;
			let mut fTemp44: f32 = (self.fRec14[1] + (self.fConst5 * self.fVec9[1]));
			self.fRec14[0] = (fTemp44 - f32::floor(fTemp44));
			let mut fTemp45: f32 = mydsp_faustpower2_f(((2.0 * self.fRec14[0]) + -1.0));
			self.fVec10[0] = fTemp45;
			let mut fTemp46: f32 = ((fTemp0 * (fTemp45 - self.fVec10[1])) / fTemp43);
			self.fVec11[(self.IOTA & 4095) as usize] = fTemp46;
			let mut fTemp47: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst17 / fTemp42)));
			let mut iTemp48: i32 = (fTemp47 as i32);
			let mut fTemp49: f32 = f32::floor(fTemp47);
			let mut fTemp50: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst16 / fTemp42)));
			let mut iTemp51: i32 = (fTemp50 as i32);
			let mut fTemp52: f32 = f32::floor(fTemp50);
			self.fRec13[0] = ((self.fConst21 * (fTemp17 * (((fTemp46 - (self.fVec11[((self.IOTA - iTemp48) & 4095) as usize] * (fTemp49 + (1.0 - fTemp47)))) - ((fTemp47 - fTemp49) * self.fVec11[((self.IOTA - (iTemp48 + 1)) & 4095) as usize])) + ((fTemp46 - (self.fVec11[((self.IOTA - iTemp51) & 4095) as usize] * (fTemp52 + (1.0 - fTemp50)))) - ((fTemp50 - fTemp52) * self.fVec11[((self.IOTA - (iTemp51 + 1)) & 4095) as usize]))))) - (self.fConst20 * ((self.fConst22 * self.fRec13[2]) + (self.fConst19 * self.fRec13[1]))));
			let mut fTemp53: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst24 / fTemp42)));
			let mut fTemp54: f32 = f32::floor(fTemp53);
			let mut iTemp55: i32 = (fTemp53 as i32);
			let mut fTemp56: f32 = (0.0 - (self.fConst13 * (((fTemp53 - fTemp54) * self.fVec11[((self.IOTA - (iTemp55 + 1)) & 4095) as usize]) - (fTemp46 - (self.fVec11[((self.IOTA - iTemp55) & 4095) as usize] * (fTemp54 + (1.0 - fTemp53)))))));
			let mut fTemp57: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst9 / fTemp42)));
			let mut iTemp58: i32 = (fTemp57 as i32);
			let mut fTemp59: f32 = f32::floor(fTemp57);
			let mut fTemp60: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst13 / fTemp42)));
			let mut iTemp61: i32 = (fTemp60 as i32);
			let mut fTemp62: f32 = f32::floor(fTemp60);
			self.fRec15[0] = ((0.333333343 * (fTemp17 * ((5.0 * fTemp56) + (self.fConst4 * (((fTemp46 - (self.fVec11[((self.IOTA - iTemp58) & 4095) as usize] * (fTemp59 + (1.0 - fTemp57)))) - ((fTemp57 - fTemp59) * self.fVec11[((self.IOTA - (iTemp58 + 1)) & 4095) as usize])) + ((fTemp46 - (self.fVec11[((self.IOTA - iTemp61) & 4095) as usize] * (fTemp62 + (1.0 - fTemp60)))) - ((fTemp60 - fTemp62) * self.fVec11[((self.IOTA - (iTemp61 + 1)) & 4095) as usize]))))))) - (self.fConst23 * ((self.fConst25 * self.fRec15[2]) + (self.fConst8 * self.fRec15[1]))));
			self.fRec16[0] = ((5.0 * (fTemp17 * fTemp56)) - (self.fConst3 * ((self.fConst7 * self.fRec16[2]) + (self.fConst8 * self.fRec16[1]))));
			self.fVec12[0] = fSlow25;
			let mut fTemp63: f32 = ((self.fConst5 * self.fVec12[1]) + self.fRec19[1]);
			self.fRec19[0] = (fTemp63 - f32::floor(fTemp63));
			let mut fTemp64: f32 = (0.0 - (((self.fRec19[0] <= 0.5) as i32) as f32));
			let mut fTemp65: f32 = f32::max((fSlow6 + (fTemp2 + (220.0 * fTemp64))), 23.4489498);
			let mut fTemp66: f32 = f32::max(20.0, f32::abs(fTemp65));
			self.fVec13[0] = fTemp66;
			let mut fTemp67: f32 = (self.fRec18[1] + (self.fConst5 * self.fVec13[1]));
			self.fRec18[0] = (fTemp67 - f32::floor(fTemp67));
			let mut fTemp68: f32 = mydsp_faustpower2_f(((2.0 * self.fRec18[0]) + -1.0));
			self.fVec14[0] = fTemp68;
			let mut fTemp69: f32 = ((fTemp0 * (fTemp68 - self.fVec14[1])) / fTemp66);
			self.fVec15[(self.IOTA & 4095) as usize] = fTemp69;
			let mut fTemp70: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst6 / fTemp65)));
			let mut iTemp71: i32 = (fTemp70 as i32);
			let mut fTemp72: f32 = f32::floor(fTemp70);
			self.fRec17[0] = ((self.fConst4 * (fTemp17 * ((fTemp69 - (self.fVec15[((self.IOTA - iTemp71) & 4095) as usize] * (fTemp72 + (1.0 - fTemp70)))) - ((fTemp70 - fTemp72) * self.fVec15[((self.IOTA - (iTemp71 + 1)) & 4095) as usize])))) - (self.fConst3 * ((self.fConst7 * self.fRec17[2]) + (self.fConst8 * self.fRec17[1]))));
			let mut fTemp73: f32 = f32::max((fSlow19 + (fTemp2 + (440.0 * fTemp64))), 23.4489498);
			let mut fTemp74: f32 = f32::max(20.0, f32::abs(fTemp73));
			self.fVec16[0] = fTemp74;
			let mut fTemp75: f32 = (self.fRec21[1] + (self.fConst5 * self.fVec16[1]));
			self.fRec21[0] = (fTemp75 - f32::floor(fTemp75));
			let mut fTemp76: f32 = mydsp_faustpower2_f(((2.0 * self.fRec21[0]) + -1.0));
			self.fVec17[0] = fTemp76;
			let mut fTemp77: f32 = ((fTemp0 * (fTemp76 - self.fVec17[1])) / fTemp74);
			self.fVec18[(self.IOTA & 4095) as usize] = fTemp77;
			let mut fTemp78: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst9 / fTemp73)));
			let mut iTemp79: i32 = (fTemp78 as i32);
			let mut fTemp80: f32 = f32::floor(fTemp78);
			self.fRec20[0] = ((self.fConst4 * (fTemp17 * ((fTemp77 - (self.fVec18[((self.IOTA - iTemp79) & 4095) as usize] * (fTemp80 + (1.0 - fTemp78)))) - ((fTemp78 - fTemp80) * self.fVec18[((self.IOTA - (iTemp79 + 1)) & 4095) as usize])))) - (self.fConst3 * ((self.fConst7 * self.fRec20[2]) + (self.fConst8 * self.fRec20[1]))));
			let mut fTemp81: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst9 / fTemp65)));
			let mut iTemp82: i32 = (fTemp81 as i32);
			let mut fTemp83: f32 = f32::floor(fTemp81);
			let mut fTemp84: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst14 / fTemp65)));
			let mut iTemp85: i32 = (fTemp84 as i32);
			let mut fTemp86: f32 = f32::floor(fTemp84);
			let mut fTemp87: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst15 / fTemp65)));
			let mut iTemp88: i32 = (fTemp87 as i32);
			let mut fTemp89: f32 = f32::floor(fTemp87);
			let mut fTemp90: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst16 / fTemp65)));
			let mut iTemp91: i32 = (fTemp90 as i32);
			let mut fTemp92: f32 = f32::floor(fTemp90);
			let mut fTemp93: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst17 / fTemp65)));
			let mut iTemp94: i32 = (fTemp93 as i32);
			let mut fTemp95: f32 = f32::floor(fTemp93);
			self.fRec22[0] = ((self.fConst13 * (fTemp17 * ((((((fTemp69 - (self.fVec15[((self.IOTA - iTemp82) & 4095) as usize] * (fTemp83 + (1.0 - fTemp81)))) - ((fTemp81 - fTemp83) * self.fVec15[((self.IOTA - (iTemp82 + 1)) & 4095) as usize])) + ((fTemp69 - (self.fVec15[((self.IOTA - iTemp85) & 4095) as usize] * (fTemp86 + (1.0 - fTemp84)))) - ((fTemp84 - fTemp86) * self.fVec15[((self.IOTA - (iTemp85 + 1)) & 4095) as usize]))) + ((fTemp69 - (self.fVec15[((self.IOTA - iTemp88) & 4095) as usize] * (fTemp89 + (1.0 - fTemp87)))) - ((fTemp87 - fTemp89) * self.fVec15[((self.IOTA - (iTemp88 + 1)) & 4095) as usize]))) + ((fTemp69 - (self.fVec15[((self.IOTA - iTemp91) & 4095) as usize] * (fTemp92 + (1.0 - fTemp90)))) - ((fTemp90 - fTemp92) * self.fVec15[((self.IOTA - (iTemp91 + 1)) & 4095) as usize]))) + ((fTemp69 - (self.fVec15[((self.IOTA - iTemp94) & 4095) as usize] * (fTemp95 + (1.0 - fTemp93)))) - ((fTemp93 - fTemp95) * self.fVec15[((self.IOTA - (iTemp94 + 1)) & 4095) as usize]))))) - (self.fConst26 * ((self.fConst27 * self.fRec22[2]) + (self.fConst19 * self.fRec22[1]))));
			*output0 = (if (iSlow1 as i32 != 0) { if (iSlow22 as i32 != 0) { if (iSlow26 as i32 != 0) { (self.fConst26 * (self.fRec0[0] * (self.fRec22[2] + (self.fRec22[0] + (2.0 * self.fRec22[1]))))) } else { (self.fConst3 * (self.fRec0[0] * (self.fRec20[2] + (self.fRec20[0] + (2.0 * self.fRec20[1]))))) } } else { if (iSlow23 as i32 != 0) { (self.fConst3 * (self.fRec0[0] * (self.fRec17[2] + (self.fRec17[0] + (2.0 * self.fRec17[1]))))) } else { if (iSlow24 as i32 != 0) { (self.fConst3 * (self.fRec0[0] * (self.fRec16[2] + (self.fRec16[0] + (2.0 * self.fRec16[1]))))) } else { (self.fConst23 * (self.fRec0[0] * (self.fRec15[2] + (self.fRec15[0] + (2.0 * self.fRec15[1]))))) } } } } else { if (iSlow2 as i32 != 0) { if (iSlow20 as i32 != 0) { (self.fConst20 * (self.fRec0[0] * (self.fRec13[2] + (self.fRec13[0] + (2.0 * self.fRec13[1]))))) } else { (self.fConst3 * (self.fRec0[0] * (self.fRec12[2] + (self.fRec12[0] + (2.0 * self.fRec12[1]))))) } } else { if (iSlow3 as i32 != 0) { (self.fConst12 * (self.fRec0[0] * (self.fRec11[2] + (self.fRec11[0] + (2.0 * self.fRec11[1]))))) } else { if (iSlow4 as i32 != 0) { (self.fConst3 * (self.fRec0[0] * (self.fRec9[2] + (self.fRec9[0] + (2.0 * self.fRec9[1]))))) } else { (self.fConst3 * (self.fRec0[0] * (self.fRec1[2] + (self.fRec1[0] + (2.0 * self.fRec1[1]))))) } } } } as f32);
			self.iVec0[1] = self.iVec0[0];
			self.fRec0[1] = self.fRec0[0];
			self.fVec1[1] = self.fVec1[0];
			self.fRec3[1] = self.fRec3[0];
			self.fVec2[1] = self.fVec2[0];
			self.fRec2[1] = self.fRec2[0];
			self.fVec3[1] = self.fVec3[0];
			self.IOTA = (self.IOTA + 1);
			self.fRec5[1] = self.fRec5[0];
			self.fRec4[1] = self.fRec4[0];
			self.fRec7[1] = self.fRec7[0];
			self.fRec6[1] = self.fRec6[0];
			self.fVec5[1] = self.fVec5[0];
			self.fRec8[1] = self.fRec8[0];
			self.fRec1[2] = self.fRec1[1];
			self.fRec1[1] = self.fRec1[0];
			self.fVec6[1] = self.fVec6[0];
			self.fRec10[1] = self.fRec10[0];
			self.fVec7[1] = self.fVec7[0];
			self.fRec9[2] = self.fRec9[1];
			self.fRec9[1] = self.fRec9[0];
			self.fRec11[2] = self.fRec11[1];
			self.fRec11[1] = self.fRec11[0];
			self.fRec12[2] = self.fRec12[1];
			self.fRec12[1] = self.fRec12[0];
			self.fVec9[1] = self.fVec9[0];
			self.fRec14[1] = self.fRec14[0];
			self.fVec10[1] = self.fVec10[0];
			self.fRec13[2] = self.fRec13[1];
			self.fRec13[1] = self.fRec13[0];
			self.fRec15[2] = self.fRec15[1];
			self.fRec15[1] = self.fRec15[0];
			self.fRec16[2] = self.fRec16[1];
			self.fRec16[1] = self.fRec16[0];
			self.fVec12[1] = self.fVec12[0];
			self.fRec19[1] = self.fRec19[0];
			self.fVec13[1] = self.fVec13[0];
			self.fRec18[1] = self.fRec18[0];
			self.fVec14[1] = self.fVec14[0];
			self.fRec17[2] = self.fRec17[1];
			self.fRec17[1] = self.fRec17[0];
			self.fVec16[1] = self.fVec16[0];
			self.fRec21[1] = self.fRec21[0];
			self.fVec17[1] = self.fVec17[0];
			self.fRec20[2] = self.fRec20[1];
			self.fRec20[1] = self.fRec20[0];
			self.fRec22[2] = self.fRec22[1];
			self.fRec22[1] = self.fRec22[0];
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
