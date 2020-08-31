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


pub struct mydspSIG0 {
	iRec19: [i32;2],
}

impl mydspSIG0 {
	
	pub fn get_num_inputsmydspSIG0(&self) -> i32 {
		return 0;
	}
	pub fn get_num_outputsmydspSIG0(&self) -> i32 {
		return 1;
	}
	pub fn get_input_ratemydspSIG0(&self, channel: i32) -> i32 {
		let mut rate: i32;
		match (channel) {
			_ => {
				rate = -1;
			},
		} 
		return rate;
	}
	pub fn get_output_ratemydspSIG0(&self, channel: i32) -> i32 {
		let mut rate: i32;
		match (channel) {
			0 => {
				rate = 0;
			},
			_ => {
				rate = -1;
			},
		} 
		return rate;
	}
	
	fn instance_initmydspSIG0(&mut self, sample_rate: i32) {
		let mut l4: i32 = 0;
		loop {
			self.iRec19[l4 as usize] = 0;
			l4 = (l4 + 1);
			if (l4 < 2) { continue; } else { break; }
		}
	}
	
	fn fillmydspSIG0(&mut self, count: i32, table: &mut[f32]) {
		for i in 0..count {
			self.iRec19[0] = (self.iRec19[1] + 1);
			table[i as usize] = f32::sin((9.58738019e-05 * ((self.iRec19[0] + -1) as f32)));
			self.iRec19[1] = self.iRec19[0];
		}
	}

}


pub fn newmydspSIG0() -> mydspSIG0 { 
	mydspSIG0 {
		iRec19: [0;2],
	}
}
fn mydsp_faustpower2_f(value: f32) -> f32 {
	return (value * value);
}
static mut ftbl0mydspSIG0: [f32;65536] = [0.0;65536];

#[no_mangle]
static mut IN_BUFFER0: [f32;MAX_BUFFER_SIZE] = [0.;MAX_BUFFER_SIZE];
#[no_mangle]
static mut OUT_BUFFER0: [f32;MAX_BUFFER_SIZE] = [0.;MAX_BUFFER_SIZE];
#[no_mangle]
static mut OUT_BUFFER1: [f32;MAX_BUFFER_SIZE] = [0.;MAX_BUFFER_SIZE];
static mut INPUTS: [* const f32;1] = [0 as * const f32; 1];
static mut OUTPUTS: [* mut f32;2] = [0 as * mut f32; 2];
static mut ENGINE : mydsp = mydsp {
	fHslider0: 0.0,
	fHslider1: 0.5,
	fSampleRate: 0,
	fConst0: 0.0,
	fConst1: 0.0,
	fConst2: 0.0,
	fHslider2: 0.0,
	fRec4: [0.0;2],
	fConst3: 0.0,
	fConst4: 0.0,
	fConst5: 0.0,
	fHslider3: 0.0,
	fRec18: [0.0;2],
	fConst6: 0.0,
	fConst7: 0.0,
	fRec17: [0.0;3],
	IOTA: 0,
	fVec0: [0.0;512],
	fConst8: 0.0,
	fRec20: [0.0;2],
	fRec15: [0.0;2],
	fVec1: [0.0;1024],
	fRec13: [0.0;2],
	fVec2: [0.0;4096],
	fRec11: [0.0;2],
	fVec3: [0.0;4096],
	fRec9: [0.0;2],
	fVec4: [0.0;2048],
	fRec5: [0.0;2],
	fRec29: [0.0;3],
	fVec5: [0.0;512],
	fRec27: [0.0;2],
	fVec6: [0.0;1024],
	fRec25: [0.0;2],
	fVec7: [0.0;4096],
	fRec23: [0.0;2],
	fVec8: [0.0;4096],
	fRec21: [0.0;2],
	fVec9: [0.0;2048],
	fRec6: [0.0;2],
	fConst9: 0.0,
	fConst10: 0.0,
	fConst11: 0.0,
	fConst12: 0.0,
	fConst13: 0.0,
	fRec39: [0.0;3],
	fVec10: [0.0;512],
	fRec37: [0.0;2],
	fVec11: [0.0;1024],
	fRec35: [0.0;2],
	fVec12: [0.0;4096],
	fRec33: [0.0;2],
	fVec13: [0.0;4096],
	fRec31: [0.0;2],
	fVec14: [0.0;4096],
	fConst14: 0.0,
	fConst15: 0.0,
	fRec30: [0.0;3],
	fConst16: 0.0,
	fRec7: [0.0;2],
	fHslider4: 0.0,
	fRec48: [0.0;3],
	fVec15: [0.0;512],
	fRec46: [0.0;2],
	fVec16: [0.0;1024],
	fRec44: [0.0;2],
	fVec17: [0.0;4096],
	fRec42: [0.0;2],
	fVec18: [0.0;4096],
	fRec40: [0.0;2],
	fVec19: [0.0;131072],
	fRec49: [0.0;2],
	fRec8: [0.0;2],
	fRec3: [0.0;2],
	fHslider5: 0.0,
	fRec50: [0.0;2],
	iVec20: [0;2],
	iConst17: 0,
	iRec51: [0;2],
	fConst18: 0.0,
	fRec1: [0.0;2],
	fRec0: [0.0;2],
};

type T = f32;

struct mydsp {
	fHslider0: f32,
	fHslider1: f32,
	fSampleRate: i32,
	fConst0: f32,
	fConst1: f32,
	fConst2: f32,
	fHslider2: f32,
	fRec4: [f32;2],
	fConst3: f32,
	fConst4: f32,
	fConst5: f32,
	fHslider3: f32,
	fRec18: [f32;2],
	fConst6: f32,
	fConst7: f32,
	fRec17: [f32;3],
	IOTA: i32,
	fVec0: [f32;512],
	fConst8: f32,
	fRec20: [f32;2],
	fRec15: [f32;2],
	fVec1: [f32;1024],
	fRec13: [f32;2],
	fVec2: [f32;4096],
	fRec11: [f32;2],
	fVec3: [f32;4096],
	fRec9: [f32;2],
	fVec4: [f32;2048],
	fRec5: [f32;2],
	fRec29: [f32;3],
	fVec5: [f32;512],
	fRec27: [f32;2],
	fVec6: [f32;1024],
	fRec25: [f32;2],
	fVec7: [f32;4096],
	fRec23: [f32;2],
	fVec8: [f32;4096],
	fRec21: [f32;2],
	fVec9: [f32;2048],
	fRec6: [f32;2],
	fConst9: f32,
	fConst10: f32,
	fConst11: f32,
	fConst12: f32,
	fConst13: f32,
	fRec39: [f32;3],
	fVec10: [f32;512],
	fRec37: [f32;2],
	fVec11: [f32;1024],
	fRec35: [f32;2],
	fVec12: [f32;4096],
	fRec33: [f32;2],
	fVec13: [f32;4096],
	fRec31: [f32;2],
	fVec14: [f32;4096],
	fConst14: f32,
	fConst15: f32,
	fRec30: [f32;3],
	fConst16: f32,
	fRec7: [f32;2],
	fHslider4: f32,
	fRec48: [f32;3],
	fVec15: [f32;512],
	fRec46: [f32;2],
	fVec16: [f32;1024],
	fRec44: [f32;2],
	fVec17: [f32;4096],
	fRec42: [f32;2],
	fVec18: [f32;4096],
	fRec40: [f32;2],
	fVec19: [f32;131072],
	fRec49: [f32;2],
	fRec8: [f32;2],
	fRec3: [f32;2],
	fHslider5: f32,
	fRec50: [f32;2],
	iVec20: [i32;2],
	iConst17: i32,
	iRec51: [i32;2],
	fConst18: f32,
	fRec1: [f32;2],
	fRec0: [f32;2],

}

impl mydsp {
		
	fn new() -> mydsp { 
		mydsp {
			fHslider0: 0.0,
			fHslider1: 0.5,
			fSampleRate: 0,
			fConst0: 0.0,
			fConst1: 0.0,
			fConst2: 0.0,
			fHslider2: 0.0,
			fRec4: [0.0;2],
			fConst3: 0.0,
			fConst4: 0.0,
			fConst5: 0.0,
			fHslider3: 0.0,
			fRec18: [0.0;2],
			fConst6: 0.0,
			fConst7: 0.0,
			fRec17: [0.0;3],
			IOTA: 0,
			fVec0: [0.0;512],
			fConst8: 0.0,
			fRec20: [0.0;2],
			fRec15: [0.0;2],
			fVec1: [0.0;1024],
			fRec13: [0.0;2],
			fVec2: [0.0;4096],
			fRec11: [0.0;2],
			fVec3: [0.0;4096],
			fRec9: [0.0;2],
			fVec4: [0.0;2048],
			fRec5: [0.0;2],
			fRec29: [0.0;3],
			fVec5: [0.0;512],
			fRec27: [0.0;2],
			fVec6: [0.0;1024],
			fRec25: [0.0;2],
			fVec7: [0.0;4096],
			fRec23: [0.0;2],
			fVec8: [0.0;4096],
			fRec21: [0.0;2],
			fVec9: [0.0;2048],
			fRec6: [0.0;2],
			fConst9: 0.0,
			fConst10: 0.0,
			fConst11: 0.0,
			fConst12: 0.0,
			fConst13: 0.0,
			fRec39: [0.0;3],
			fVec10: [0.0;512],
			fRec37: [0.0;2],
			fVec11: [0.0;1024],
			fRec35: [0.0;2],
			fVec12: [0.0;4096],
			fRec33: [0.0;2],
			fVec13: [0.0;4096],
			fRec31: [0.0;2],
			fVec14: [0.0;4096],
			fConst14: 0.0,
			fConst15: 0.0,
			fRec30: [0.0;3],
			fConst16: 0.0,
			fRec7: [0.0;2],
			fHslider4: 0.0,
			fRec48: [0.0;3],
			fVec15: [0.0;512],
			fRec46: [0.0;2],
			fVec16: [0.0;1024],
			fRec44: [0.0;2],
			fVec17: [0.0;4096],
			fRec42: [0.0;2],
			fVec18: [0.0;4096],
			fRec40: [0.0;2],
			fVec19: [0.0;131072],
			fRec49: [0.0;2],
			fRec8: [0.0;2],
			fRec3: [0.0;2],
			fHslider5: 0.0,
			fRec50: [0.0;2],
			iVec20: [0;2],
			iConst17: 0,
			iRec51: [0;2],
			fConst18: 0.0,
			fRec1: [0.0;2],
			fRec0: [0.0;2],
		}
	}
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
		return self.fSampleRate;
	}
	pub fn get_num_inputs(&self) -> i32 {
		return 1;
	}
	pub fn get_num_outputs(&self) -> i32 {
		return 2;
	}
	pub fn get_input_rate(&self, channel: i32) -> i32 {
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
	pub fn get_output_rate(&self, channel: i32) -> i32 {
		let mut rate: i32;
		match (channel) {
			0 => {
				rate = 1;
			},
			1 => {
				rate = 1;
			},
			_ => {
				rate = -1;
			},
		} 
		return rate;
	}
	
	fn class_init(sample_rate: i32) {
		let mut sig0: mydspSIG0 = newmydspSIG0();
		sig0.instance_initmydspSIG0(sample_rate);
		sig0.fillmydspSIG0(65536, unsafe { &mut ftbl0mydspSIG0 });
	}
	fn instance_reset_params(&mut self) {
		self.fHslider0 = 1.0;
		self.fHslider1 = 0.5;
		self.fHslider2 = 0.0;
		self.fHslider3 = 0.5;
		self.fHslider4 = 0.0;
		self.fHslider5 = 1.0;
	}
	fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.fRec4[l0 as usize] = 0.0;
		}
		for l1 in 0..2 {
			self.fRec18[l1 as usize] = 0.0;
		}
		for l2 in 0..3 {
			self.fRec17[l2 as usize] = 0.0;
		}
		self.IOTA = 0;
		for l3 in 0..512 {
			self.fVec0[l3 as usize] = 0.0;
		}
		for l5 in 0..2 {
			self.fRec20[l5 as usize] = 0.0;
		}
		for l6 in 0..2 {
			self.fRec15[l6 as usize] = 0.0;
		}
		for l7 in 0..1024 {
			self.fVec1[l7 as usize] = 0.0;
		}
		for l8 in 0..2 {
			self.fRec13[l8 as usize] = 0.0;
		}
		for l9 in 0..4096 {
			self.fVec2[l9 as usize] = 0.0;
		}
		for l10 in 0..2 {
			self.fRec11[l10 as usize] = 0.0;
		}
		for l11 in 0..4096 {
			self.fVec3[l11 as usize] = 0.0;
		}
		for l12 in 0..2 {
			self.fRec9[l12 as usize] = 0.0;
		}
		for l13 in 0..2048 {
			self.fVec4[l13 as usize] = 0.0;
		}
		for l14 in 0..2 {
			self.fRec5[l14 as usize] = 0.0;
		}
		for l15 in 0..3 {
			self.fRec29[l15 as usize] = 0.0;
		}
		for l16 in 0..512 {
			self.fVec5[l16 as usize] = 0.0;
		}
		for l17 in 0..2 {
			self.fRec27[l17 as usize] = 0.0;
		}
		for l18 in 0..1024 {
			self.fVec6[l18 as usize] = 0.0;
		}
		for l19 in 0..2 {
			self.fRec25[l19 as usize] = 0.0;
		}
		for l20 in 0..4096 {
			self.fVec7[l20 as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.fRec23[l21 as usize] = 0.0;
		}
		for l22 in 0..4096 {
			self.fVec8[l22 as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.fRec21[l23 as usize] = 0.0;
		}
		for l24 in 0..2048 {
			self.fVec9[l24 as usize] = 0.0;
		}
		for l25 in 0..2 {
			self.fRec6[l25 as usize] = 0.0;
		}
		for l26 in 0..3 {
			self.fRec39[l26 as usize] = 0.0;
		}
		for l27 in 0..512 {
			self.fVec10[l27 as usize] = 0.0;
		}
		for l28 in 0..2 {
			self.fRec37[l28 as usize] = 0.0;
		}
		for l29 in 0..1024 {
			self.fVec11[l29 as usize] = 0.0;
		}
		for l30 in 0..2 {
			self.fRec35[l30 as usize] = 0.0;
		}
		for l31 in 0..4096 {
			self.fVec12[l31 as usize] = 0.0;
		}
		for l32 in 0..2 {
			self.fRec33[l32 as usize] = 0.0;
		}
		for l33 in 0..4096 {
			self.fVec13[l33 as usize] = 0.0;
		}
		for l34 in 0..2 {
			self.fRec31[l34 as usize] = 0.0;
		}
		for l35 in 0..4096 {
			self.fVec14[l35 as usize] = 0.0;
		}
		for l36 in 0..3 {
			self.fRec30[l36 as usize] = 0.0;
		}
		for l37 in 0..2 {
			self.fRec7[l37 as usize] = 0.0;
		}
		for l38 in 0..3 {
			self.fRec48[l38 as usize] = 0.0;
		}
		for l39 in 0..512 {
			self.fVec15[l39 as usize] = 0.0;
		}
		for l40 in 0..2 {
			self.fRec46[l40 as usize] = 0.0;
		}
		for l41 in 0..1024 {
			self.fVec16[l41 as usize] = 0.0;
		}
		for l42 in 0..2 {
			self.fRec44[l42 as usize] = 0.0;
		}
		for l43 in 0..4096 {
			self.fVec17[l43 as usize] = 0.0;
		}
		for l44 in 0..2 {
			self.fRec42[l44 as usize] = 0.0;
		}
		for l45 in 0..4096 {
			self.fVec18[l45 as usize] = 0.0;
		}
		for l46 in 0..2 {
			self.fRec40[l46 as usize] = 0.0;
		}
		for l47 in 0..131072 {
			self.fVec19[l47 as usize] = 0.0;
		}
		for l48 in 0..2 {
			self.fRec49[l48 as usize] = 0.0;
		}
		for l49 in 0..2 {
			self.fRec8[l49 as usize] = 0.0;
		}
		for l50 in 0..2 {
			self.fRec3[l50 as usize] = 0.0;
		}
		for l51 in 0..2 {
			self.fRec50[l51 as usize] = 0.0;
		}
		for l52 in 0..2 {
			self.iVec20[l52 as usize] = 0;
		}
		for l53 in 0..2 {
			self.iRec51[l53 as usize] = 0;
		}
		for l54 in 0..2 {
			self.fRec1[l54 as usize] = 0.0;
		}
		for l55 in 0..2 {
			self.fRec0[l55 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		self.fConst0 = f32::min(192000.0, f32::max(1.0, (self.fSampleRate as f32)));
		self.fConst1 = f32::exp((0.0 - (1000.0 / self.fConst0)));
		self.fConst2 = (1.0 - self.fConst1);
		self.fConst3 = f32::tan((31415.9258 / self.fConst0));
		self.fConst4 = (1.0 / self.fConst3);
		self.fConst5 = (1.0 / (((self.fConst4 + 1.41421354) / self.fConst3) + 1.0));
		self.fConst6 = (((self.fConst4 + -1.41421354) / self.fConst3) + 1.0);
		self.fConst7 = (2.0 * (1.0 - (1.0 / mydsp_faustpower2_f(self.fConst3))));
		self.fConst8 = (0.25 / self.fConst0);
		self.fConst9 = f32::tan((314.159271 / self.fConst0));
		self.fConst10 = (1.0 / self.fConst9);
		self.fConst11 = (1.0 / (((self.fConst10 + 1.41421354) / self.fConst9) + 1.0));
		self.fConst12 = mydsp_faustpower2_f(self.fConst9);
		self.fConst13 = (1.0 / self.fConst12);
		self.fConst14 = (((self.fConst10 + -1.41421354) / self.fConst9) + 1.0);
		self.fConst15 = (2.0 * (1.0 - self.fConst13));
		self.fConst16 = (0.0 - (2.0 / self.fConst12));
		self.iConst17 = ((0.00999999978 * self.fConst0) as i32);
		self.fConst18 = f32::exp((0.0 - (25.0 / self.fConst0)));
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
			"Drywet" => Param { index: 0, range: ParamRange::new(0.0, 0.0, 1.0, 0.01) },
			"Length" => Param { index: 1, range: ParamRange::new(0.5, 0.0, 1.5, 0.01) },
			"Outputgain" => Param { index: 2, range: ParamRange::new(1.0, 0.0, 1.0, 0.01) },
			"Shape" => Param { index: 3, range: ParamRange::new(1.0, 0.0, 2.0, 0.01) },
			"Shimmer" => Param { index: 4, range: ParamRange::new(0.0, 0.0, 1.2, 0.01) },
			"Spread" => Param { index: 5, range: ParamRange::new(0.0, 0.0, 1.0, 0.01) },
			
			_ => Param { index: -1, range: ParamRange::new(0.0, 0.0, 0.0, 0.0)}
		}
	}
	fn init_voices(&mut self) {
	}
	pub fn handle_note_on(&mut self, _mn: Note, _vel: f32) {
	}pub fn handle_note_off(&mut self, _mn: Note, _vel: f32) {
	}
	fn init_buffers(&self) {
		unsafe {
			INPUTS[0] = IN_BUFFER0.as_ptr();
			OUTPUTS[0] = OUT_BUFFER0.as_mut_ptr();
			OUTPUTS[1] = OUT_BUFFER1.as_mut_ptr();
		};
	}
	
	pub fn get_param(&self, param: u32) -> T {
		match param {
			2 => self.fHslider0,
			0 => self.fHslider1,
			5 => self.fHslider2,
			1 => self.fHslider3,
			4 => self.fHslider4,
			3 => self.fHslider5,
			_ => 0.,
		}
	}
	
	pub fn set_param(&mut self, param: u32, value: T) {
		match param {
			2 => { self.fHslider0 = value }
			0 => { self.fHslider1 = value }
			5 => { self.fHslider2 = value }
			1 => { self.fHslider3 = value }
			4 => { self.fHslider4 = value }
			3 => { self.fHslider5 = value }
			_ => {}
		}
	}
	#[target_feature(enable = "simd128")]
	#[inline]
	unsafe fn compute(&mut self, count: i32, inputs: &[&[T];1], outputs: &mut [&mut [T];2]) {
		let inputs0 = inputs[0][..count as usize].iter();
		let [outputs0, outputs1] = outputs;
		let (outputs0, outputs1) = {
			let outputs0 = outputs0[..count as usize].iter_mut();
			let outputs1 = outputs1[..count as usize].iter_mut();
			(outputs0, outputs1)
		};
		let mut fSlow0: f32 = (self.fHslider0 as f32);
		let mut fSlow1: f32 = (self.fHslider1 as f32);
		let mut fSlow2: f32 = (1.0 - fSlow1);
		let mut fSlow3: f32 = (0.00100000005 * (self.fHslider2 as f32));
		let mut fSlow4: f32 = (0.00100000005 * (self.fHslider3 as f32));
		let mut fSlow5: f32 = (self.fHslider4 as f32);
		let mut fSlow6: f32 = (1.0 - fSlow5);
		let mut fSlow7: f32 = (0.00100000005 * (self.fHslider5 as f32));
		let zipped_iterators = inputs0.zip(outputs0).zip(outputs1);
		for ((input0, output0), output1) in zipped_iterators {
			let mut fTemp0: f32 = (*input0 as f32);
			let mut fTemp1: f32 = (fSlow2 * fTemp0);
			self.fRec4[0] = (fSlow3 + (0.999000013 * self.fRec4[1]));
			let mut fTemp2: f32 = (1.0 - self.fRec4[0]);
			self.fRec18[0] = (fSlow4 + (0.999000013 * self.fRec18[1]));
			let mut fTemp3: f32 = (self.fRec5[1] + self.fRec7[1]);
			self.fRec17[0] = ((self.fRec18[0] * (self.fRec8[1] + (fTemp3 + self.fRec6[1]))) - (self.fConst5 * ((self.fConst6 * self.fRec17[2]) + (self.fConst7 * self.fRec17[1]))));
			let mut fTemp4: f32 = (fTemp0 + ((0.600000024 * self.fRec15[1]) + (self.fConst5 * (self.fRec17[2] + (self.fRec17[0] + (2.0 * self.fRec17[1]))))));
			self.fVec0[(self.IOTA & 511) as usize] = fTemp4;
			self.fRec20[0] = (self.fConst8 + (self.fRec20[1] - f32::floor((self.fConst8 + self.fRec20[1]))));
			let mut fTemp5: f32 = ((0.00100000005 * unsafe { ftbl0mydspSIG0[((65536.0 * self.fRec20[0]) as i32) as usize] }) + 1.0);
			let mut fTemp6: f32 = (441.0 * fTemp5);
			let mut fTemp7: f32 = (fTemp6 + -1.0);
			let mut iTemp8: i32 = (fTemp7 as i32);
			let mut iTemp9: i32 = std::cmp::min(2049, std::cmp::max(0, iTemp8));
			let mut fTemp10: f32 = f32::floor(fTemp7);
			let mut fTemp11: f32 = (fTemp10 + (2.0 - fTemp6));
			let mut fTemp12: f32 = (fTemp6 + (-1.0 - fTemp10));
			let mut iTemp13: i32 = std::cmp::min(2049, std::cmp::max(0, (iTemp8 + 1)));
			self.fRec15[0] = ((self.fVec0[((self.IOTA - iTemp9) & 511) as usize] * fTemp11) + (fTemp12 * self.fVec0[((self.IOTA - iTemp13) & 511) as usize]));
			let mut fRec16: f32 = (0.0 - (0.600000024 * fTemp4));
			let mut fTemp14: f32 = (fRec16 + ((0.600000024 * self.fRec13[1]) + self.fRec15[1]));
			self.fVec1[(self.IOTA & 1023) as usize] = fTemp14;
			let mut fTemp15: f32 = (727.0 * fTemp5);
			let mut fTemp16: f32 = (fTemp15 + -1.0);
			let mut iTemp17: i32 = (fTemp16 as i32);
			let mut iTemp18: i32 = std::cmp::min(2049, std::cmp::max(0, iTemp17));
			let mut fTemp19: f32 = f32::floor(fTemp16);
			let mut fTemp20: f32 = (fTemp19 + (2.0 - fTemp15));
			let mut fTemp21: f32 = (fTemp15 + (-1.0 - fTemp19));
			let mut iTemp22: i32 = std::cmp::min(2049, std::cmp::max(0, (iTemp17 + 1)));
			self.fRec13[0] = ((self.fVec1[((self.IOTA - iTemp18) & 1023) as usize] * fTemp20) + (fTemp21 * self.fVec1[((self.IOTA - iTemp22) & 1023) as usize]));
			let mut fRec14: f32 = (0.0 - (0.600000024 * fTemp14));
			let mut fTemp23: f32 = (fRec14 + ((0.600000024 * self.fRec11[1]) + self.fRec13[1]));
			self.fVec2[(self.IOTA & 4095) as usize] = fTemp23;
			let mut fTemp24: f32 = (2403.0 * fTemp5);
			let mut fTemp25: f32 = (fTemp24 + -1.0);
			let mut iTemp26: i32 = (fTemp25 as i32);
			let mut iTemp27: i32 = std::cmp::min(2049, std::cmp::max(0, iTemp26));
			let mut fTemp28: f32 = f32::floor(fTemp25);
			let mut fTemp29: f32 = (fTemp28 + (2.0 - fTemp24));
			let mut fTemp30: f32 = (fTemp24 + (-1.0 - fTemp28));
			let mut iTemp31: i32 = std::cmp::min(2049, std::cmp::max(0, (iTemp26 + 1)));
			self.fRec11[0] = ((self.fVec2[((self.IOTA - iTemp27) & 4095) as usize] * fTemp29) + (fTemp30 * self.fVec2[((self.IOTA - iTemp31) & 4095) as usize]));
			let mut fRec12: f32 = (0.0 - (0.600000024 * fTemp23));
			let mut fTemp32: f32 = (fRec12 + ((0.600000024 * self.fRec9[1]) + self.fRec11[1]));
			self.fVec3[(self.IOTA & 4095) as usize] = fTemp32;
			let mut fTemp33: f32 = (3119.0 * fTemp5);
			let mut fTemp34: f32 = (fTemp33 + -1.0);
			let mut iTemp35: i32 = (fTemp34 as i32);
			let mut iTemp36: i32 = std::cmp::min(2049, std::cmp::max(0, iTemp35));
			let mut fTemp37: f32 = f32::floor(fTemp34);
			let mut fTemp38: f32 = (fTemp37 + (2.0 - fTemp33));
			let mut fTemp39: f32 = (fTemp33 + (-1.0 - fTemp37));
			let mut iTemp40: i32 = std::cmp::min(2049, std::cmp::max(0, (iTemp35 + 1)));
			self.fRec9[0] = ((self.fVec3[((self.IOTA - iTemp36) & 4095) as usize] * fTemp38) + (fTemp39 * self.fVec3[((self.IOTA - iTemp40) & 4095) as usize]));
			let mut fRec10: f32 = (0.0 - (0.600000024 * fTemp32));
			let mut fTemp41: f32 = (fRec10 + self.fRec9[1]);
			self.fVec4[(self.IOTA & 2047) as usize] = fTemp41;
			let mut fTemp42: f32 = (1422.0 * fTemp5);
			let mut iTemp43: i32 = (fTemp42 as i32);
			let mut fTemp44: f32 = f32::floor(fTemp42);
			self.fRec5[0] = (0.5 * ((self.fVec4[((self.IOTA - std::cmp::min(8193, std::cmp::max(0, iTemp43))) & 2047) as usize] * (fTemp44 + (1.0 - fTemp42))) + ((fTemp42 - fTemp44) * self.fVec4[((self.IOTA - std::cmp::min(8193, std::cmp::max(0, (iTemp43 + 1)))) & 2047) as usize])));
			self.fRec29[0] = ((self.fRec18[0] * (fTemp3 - (self.fRec6[1] + self.fRec8[1]))) - (self.fConst5 * ((self.fConst6 * self.fRec29[2]) + (self.fConst7 * self.fRec29[1]))));
			let mut fTemp45: f32 = (fTemp0 + ((0.600000024 * self.fRec27[1]) + (self.fConst5 * (self.fRec29[2] + (self.fRec29[0] + (2.0 * self.fRec29[1]))))));
			self.fVec5[(self.IOTA & 511) as usize] = fTemp45;
			self.fRec27[0] = ((fTemp11 * self.fVec5[((self.IOTA - iTemp9) & 511) as usize]) + (fTemp12 * self.fVec5[((self.IOTA - iTemp13) & 511) as usize]));
			let mut fRec28: f32 = (0.0 - (0.600000024 * fTemp45));
			let mut fTemp46: f32 = (fRec28 + ((0.600000024 * self.fRec25[1]) + self.fRec27[1]));
			self.fVec6[(self.IOTA & 1023) as usize] = fTemp46;
			self.fRec25[0] = ((fTemp20 * self.fVec6[((self.IOTA - iTemp18) & 1023) as usize]) + (fTemp21 * self.fVec6[((self.IOTA - iTemp22) & 1023) as usize]));
			let mut fRec26: f32 = (0.0 - (0.600000024 * fTemp46));
			let mut fTemp47: f32 = (fRec26 + ((0.600000024 * self.fRec23[1]) + self.fRec25[1]));
			self.fVec7[(self.IOTA & 4095) as usize] = fTemp47;
			self.fRec23[0] = ((fTemp29 * self.fVec7[((self.IOTA - iTemp27) & 4095) as usize]) + (fTemp30 * self.fVec7[((self.IOTA - iTemp31) & 4095) as usize]));
			let mut fRec24: f32 = (0.0 - (0.600000024 * fTemp47));
			let mut fTemp48: f32 = (fRec24 + ((0.600000024 * self.fRec21[1]) + self.fRec23[1]));
			self.fVec8[(self.IOTA & 4095) as usize] = fTemp48;
			self.fRec21[0] = ((fTemp38 * self.fVec8[((self.IOTA - iTemp36) & 4095) as usize]) + (fTemp39 * self.fVec8[((self.IOTA - iTemp40) & 4095) as usize]));
			let mut fRec22: f32 = (0.0 - (0.600000024 * fTemp48));
			let mut fTemp49: f32 = (fRec22 + self.fRec21[1]);
			self.fVec9[(self.IOTA & 2047) as usize] = fTemp49;
			let mut fTemp50: f32 = (1617.0 * fTemp5);
			let mut iTemp51: i32 = (fTemp50 as i32);
			let mut fTemp52: f32 = f32::floor(fTemp50);
			self.fRec6[0] = (0.5 * ((self.fVec9[((self.IOTA - std::cmp::min(8193, std::cmp::max(0, iTemp51))) & 2047) as usize] * (fTemp52 + (1.0 - fTemp50))) + ((fTemp50 - fTemp52) * self.fVec9[((self.IOTA - std::cmp::min(8193, std::cmp::max(0, (iTemp51 + 1)))) & 2047) as usize])));
			self.fRec39[0] = ((self.fRec18[0] * ((self.fRec5[1] + self.fRec6[1]) - (self.fRec7[1] + self.fRec8[1]))) - (self.fConst5 * ((self.fConst6 * self.fRec39[2]) + (self.fConst7 * self.fRec39[1]))));
			let mut fTemp53: f32 = (fTemp0 + ((0.600000024 * self.fRec37[1]) + (self.fConst5 * (self.fRec39[2] + (self.fRec39[0] + (2.0 * self.fRec39[1]))))));
			self.fVec10[(self.IOTA & 511) as usize] = fTemp53;
			self.fRec37[0] = ((fTemp11 * self.fVec10[((self.IOTA - iTemp9) & 511) as usize]) + (fTemp12 * self.fVec10[((self.IOTA - iTemp13) & 511) as usize]));
			let mut fRec38: f32 = (0.0 - (0.600000024 * fTemp53));
			let mut fTemp54: f32 = (fRec38 + ((0.600000024 * self.fRec35[1]) + self.fRec37[1]));
			self.fVec11[(self.IOTA & 1023) as usize] = fTemp54;
			self.fRec35[0] = ((fTemp20 * self.fVec11[((self.IOTA - iTemp18) & 1023) as usize]) + (fTemp21 * self.fVec11[((self.IOTA - iTemp22) & 1023) as usize]));
			let mut fRec36: f32 = (0.0 - (0.600000024 * fTemp54));
			let mut fTemp55: f32 = (fRec36 + ((0.600000024 * self.fRec33[1]) + self.fRec35[1]));
			self.fVec12[(self.IOTA & 4095) as usize] = fTemp55;
			self.fRec33[0] = ((fTemp29 * self.fVec12[((self.IOTA - iTemp27) & 4095) as usize]) + (fTemp30 * self.fVec12[((self.IOTA - iTemp31) & 4095) as usize]));
			let mut fRec34: f32 = (0.0 - (0.600000024 * fTemp55));
			let mut fTemp56: f32 = (fRec34 + ((0.600000024 * self.fRec31[1]) + self.fRec33[1]));
			self.fVec13[(self.IOTA & 4095) as usize] = fTemp56;
			self.fRec31[0] = ((fTemp38 * self.fVec13[((self.IOTA - iTemp36) & 4095) as usize]) + (fTemp39 * self.fVec13[((self.IOTA - iTemp40) & 4095) as usize]));
			let mut fRec32: f32 = (0.0 - (0.600000024 * fTemp56));
			let mut fTemp57: f32 = (fRec32 + self.fRec31[1]);
			self.fVec14[(self.IOTA & 4095) as usize] = fTemp57;
			let mut fTemp58: f32 = (2401.0 * fTemp5);
			let mut iTemp59: i32 = (fTemp58 as i32);
			let mut fTemp60: f32 = f32::floor(fTemp58);
			self.fRec30[0] = ((0.5 * ((self.fVec14[((self.IOTA - std::cmp::min(8193, std::cmp::max(0, iTemp59))) & 4095) as usize] * (fTemp60 + (1.0 - fTemp58))) + ((fTemp58 - fTemp60) * self.fVec14[((self.IOTA - std::cmp::min(8193, std::cmp::max(0, (iTemp59 + 1)))) & 4095) as usize]))) - (self.fConst11 * ((self.fConst14 * self.fRec30[2]) + (self.fConst15 * self.fRec30[1]))));
			self.fRec7[0] = (self.fConst11 * (((self.fConst13 * self.fRec30[0]) + (self.fConst16 * self.fRec30[1])) + (self.fConst13 * self.fRec30[2])));
			self.fRec48[0] = ((self.fRec18[0] * ((self.fRec5[1] + self.fRec8[1]) - (self.fRec7[1] + self.fRec6[1]))) - (self.fConst5 * ((self.fConst6 * self.fRec48[2]) + (self.fConst7 * self.fRec48[1]))));
			let mut fTemp61: f32 = ((self.fConst5 * (self.fRec48[2] + (self.fRec48[0] + (2.0 * self.fRec48[1])))) + (fTemp0 + (0.600000024 * self.fRec46[1])));
			self.fVec15[(self.IOTA & 511) as usize] = fTemp61;
			self.fRec46[0] = ((fTemp11 * self.fVec15[((self.IOTA - iTemp9) & 511) as usize]) + (fTemp12 * self.fVec15[((self.IOTA - iTemp13) & 511) as usize]));
			let mut fRec47: f32 = (0.0 - (0.600000024 * fTemp61));
			let mut fTemp62: f32 = (self.fRec46[1] + (fRec47 + (0.600000024 * self.fRec44[1])));
			self.fVec16[(self.IOTA & 1023) as usize] = fTemp62;
			self.fRec44[0] = ((fTemp20 * self.fVec16[((self.IOTA - iTemp18) & 1023) as usize]) + (fTemp21 * self.fVec16[((self.IOTA - iTemp22) & 1023) as usize]));
			let mut fRec45: f32 = (0.0 - (0.600000024 * fTemp62));
			let mut fTemp63: f32 = (self.fRec44[1] + (fRec45 + (0.600000024 * self.fRec42[1])));
			self.fVec17[(self.IOTA & 4095) as usize] = fTemp63;
			self.fRec42[0] = ((fTemp29 * self.fVec17[((self.IOTA - iTemp27) & 4095) as usize]) + (fTemp30 * self.fVec17[((self.IOTA - iTemp31) & 4095) as usize]));
			let mut fRec43: f32 = (0.0 - (0.600000024 * fTemp63));
			let mut fTemp64: f32 = (self.fRec42[1] + (fRec43 + (0.600000024 * self.fRec40[1])));
			self.fVec18[(self.IOTA & 4095) as usize] = fTemp64;
			self.fRec40[0] = ((fTemp38 * self.fVec18[((self.IOTA - iTemp36) & 4095) as usize]) + (fTemp39 * self.fVec18[((self.IOTA - iTemp40) & 4095) as usize]));
			let mut fRec41: f32 = (0.0 - (0.600000024 * fTemp64));
			let mut fTemp65: f32 = (fRec41 + self.fRec40[1]);
			self.fVec19[(self.IOTA & 131071) as usize] = fTemp65;
			let mut fTemp66: f32 = (6561.0 * fTemp5);
			let mut fTemp67: f32 = (fTemp66 + 8192.0);
			self.fRec49[0] = libm::fmodf(((fTemp66 + self.fRec49[1]) + 8191.0), fTemp67);
			let mut iTemp68: i32 = (self.fRec49[0] as i32);
			let mut fTemp69: f32 = f32::floor(self.fRec49[0]);
			let mut fTemp70: f32 = f32::min((self.fRec49[0] / fTemp67), 1.0);
			let mut fTemp71: f32 = (self.fRec49[0] + fTemp66);
			let mut fTemp72: f32 = (fTemp71 + 8192.0);
			let mut iTemp73: i32 = (fTemp72 as i32);
			let mut fTemp74: f32 = f32::floor(fTemp72);
			self.fRec8[0] = (0.5 * ((fSlow6 * self.fVec19[((self.IOTA - (f32::min(8192.0, f32::max(0.0, fTemp67)) as i32)) & 131071) as usize]) + (fSlow5 * ((((self.fVec19[((self.IOTA - std::cmp::min(65537, std::cmp::max(0, iTemp68))) & 131071) as usize] * (fTemp69 + (1.0 - self.fRec49[0]))) + ((self.fRec49[0] - fTemp69) * self.fVec19[((self.IOTA - std::cmp::min(65537, std::cmp::max(0, (iTemp68 + 1)))) & 131071) as usize])) * fTemp70) + (((self.fVec19[((self.IOTA - std::cmp::min(65537, std::cmp::max(0, iTemp73))) & 131071) as usize] * (fTemp74 + (-8191.0 - fTemp71))) + ((fTemp71 + (8192.0 - fTemp74)) * self.fVec19[((self.IOTA - std::cmp::min(65537, std::cmp::max(0, (iTemp73 + 1)))) & 131071) as usize])) * (1.0 - fTemp70))))));
			let mut fTemp75: f32 = (self.fRec5[0] + self.fRec7[0]);
			let mut fTemp76: f32 = (self.fRec4[0] + 1.0);
			let mut fTemp77: f32 = (self.fRec8[0] + self.fRec6[0]);
			let mut fTemp78: f32 = ((fTemp2 * fTemp75) + (fTemp76 * fTemp77));
			let mut fTemp79: f32 = ((fTemp2 * fTemp77) + (fTemp75 * fTemp76));
			self.fRec3[0] = ((self.fConst1 * self.fRec3[1]) + (self.fConst2 * f32::abs((f32::abs(fTemp78) + f32::abs(fTemp79)))));
			let mut fRec2: f32 = self.fRec3[0];
			self.fRec50[0] = (fSlow7 + (0.999000013 * self.fRec50[1]));
			let mut iTemp80: i32 = ((fRec2 > f32::powf(10.0, (0.0500000007 * ((64.0 * f32::max(0.0, (self.fRec50[0] + -1.0))) + -60.0)))) as i32);
			self.iVec20[0] = iTemp80;
			self.iRec51[0] = std::cmp::max(((self.iConst17 * ((iTemp80 < self.iVec20[1]) as i32)) as i32), ((self.iRec51[1] + -1) as i32));
			let mut fTemp81: f32 = f32::abs(f32::max((iTemp80 as f32), (((self.iRec51[0] > 0) as i32) as f32)));
			let mut fTemp82: f32 = if (((self.fRec0[1] > fTemp81) as i32) as i32 != 0) { self.fConst18 } else { self.fConst1 };
			self.fRec1[0] = ((self.fRec1[1] * fTemp82) + (fTemp81 * (1.0 - fTemp82)));
			self.fRec0[0] = self.fRec1[0];
			*output0 = ((fSlow0 * (fTemp1 + (fSlow1 * (self.fRec0[0] * fTemp78)))) as f32);
			*output1 = ((fSlow0 * (fTemp1 + (fSlow1 * (self.fRec0[0] * fTemp79)))) as f32);
			self.fRec4[1] = self.fRec4[0];
			self.fRec18[1] = self.fRec18[0];
			self.fRec17[2] = self.fRec17[1];
			self.fRec17[1] = self.fRec17[0];
			self.IOTA = (self.IOTA + 1);
			self.fRec20[1] = self.fRec20[0];
			self.fRec15[1] = self.fRec15[0];
			self.fRec13[1] = self.fRec13[0];
			self.fRec11[1] = self.fRec11[0];
			self.fRec9[1] = self.fRec9[0];
			self.fRec5[1] = self.fRec5[0];
			self.fRec29[2] = self.fRec29[1];
			self.fRec29[1] = self.fRec29[0];
			self.fRec27[1] = self.fRec27[0];
			self.fRec25[1] = self.fRec25[0];
			self.fRec23[1] = self.fRec23[0];
			self.fRec21[1] = self.fRec21[0];
			self.fRec6[1] = self.fRec6[0];
			self.fRec39[2] = self.fRec39[1];
			self.fRec39[1] = self.fRec39[0];
			self.fRec37[1] = self.fRec37[0];
			self.fRec35[1] = self.fRec35[0];
			self.fRec33[1] = self.fRec33[0];
			self.fRec31[1] = self.fRec31[0];
			self.fRec30[2] = self.fRec30[1];
			self.fRec30[1] = self.fRec30[0];
			self.fRec7[1] = self.fRec7[0];
			self.fRec48[2] = self.fRec48[1];
			self.fRec48[1] = self.fRec48[0];
			self.fRec46[1] = self.fRec46[0];
			self.fRec44[1] = self.fRec44[0];
			self.fRec42[1] = self.fRec42[0];
			self.fRec40[1] = self.fRec40[0];
			self.fRec49[1] = self.fRec49[0];
			self.fRec8[1] = self.fRec8[0];
			self.fRec3[1] = self.fRec3[0];
			self.fRec50[1] = self.fRec50[0];
			self.iVec20[1] = self.iVec20[0];
			self.iRec51[1] = self.iRec51[0];
			self.fRec1[1] = self.fRec1[0];
			self.fRec0[1] = self.fRec0[0];
		}
	}

	#[inline]
	pub fn compute_external(&mut self, count: i32) {
		let (input0, output0, output1) = unsafe {
			(::std::slice::from_raw_parts(INPUTS[0], count as usize),
			::std::slice::from_raw_parts_mut(OUTPUTS[0], count as usize),
			::std::slice::from_raw_parts_mut(OUTPUTS[1], count as usize))
		};
		unsafe { self.compute(count, &[input0], &mut [output0, output1]); }
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
