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
	fConst4: 0.0,
	fHslider1: 0.0,
	fVec0: [0.0;2],
	iVec1: [0;2],
	fConst5: 0.0,
	fRec1: [0.0;2],
	fVec2: [0.0;2],
	IOTA: 0,
	fVec3: [0.0;4096],
	fConst6: 0.0,
	fButton0: 0.0,
	fConst7: 0.0,
	fRec2: [0.0;2],
	fConst8: 0.0,
	fRec4: [0.0;2],
	fConst9: 0.0,
	fConst10: 0.0,
	fConst11: 0.0,
	fRec3: [0.0;2],
	fConst12: 0.0,
	fConst13: 0.0,
	fConst14: 0.0,
	fRec0: [0.0;3],
	fVec4: [0.0;2],
	fRec7: [0.0;2],
	fVec5: [0.0;2],
	fRec6: [0.0;2],
	fVec6: [0.0;2],
	fVec7: [0.0;4096],
	fConst15: 0.0,
	fConst16: 0.0,
	fRec8: [0.0;2],
	fConst17: 0.0,
	fRec10: [0.0;2],
	fConst18: 0.0,
	fRec9: [0.0;2],
	fConst19: 0.0,
	fRec5: [0.0;3],
	fConst20: 0.0,
	fConst21: 0.0,
	fConst22: 0.0,
	fConst23: 0.0,
	fConst24: 0.0,
	fRec12: [0.0;2],
	fConst25: 0.0,
	fRec14: [0.0;2],
	fRec13: [0.0;2],
	fConst26: 0.0,
	fVec8: [0.0;2],
	fRec15: [0.0;2],
	fVec9: [0.0;2],
	fVec10: [0.0;4096],
	fConst27: 0.0,
	fConst28: 0.0,
	fConst29: 0.0,
	fConst30: 0.0,
	fConst31: 0.0,
	fRec11: [0.0;3],
	fVec11: [0.0;2],
	fRec17: [0.0;2],
	fVec12: [0.0;2],
	fVec13: [0.0;4096],
	fRec16: [0.0;3],
	fConst32: 0.0,
	fConst33: 0.0,
	fRec20: [0.0;2],
	fConst34: 0.0,
	fConst35: 0.0,
	fRec19: [0.0;2],
	fVec14: [0.0;2],
	fRec21: [0.0;2],
	fVec15: [0.0;2],
	fVec16: [0.0;2048],
	fConst36: 0.0,
	fRec18: [0.0;3],
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
	fConst4: f32,
	fHslider1: f32,
	fVec0: [f32;2],
	iVec1: [i32;2],
	fConst5: f32,
	fRec1: [f32;2],
	fVec2: [f32;2],
	IOTA: i32,
	fVec3: [f32;4096],
	fConst6: f32,
	fButton0: f32,
	fConst7: f32,
	fRec2: [f32;2],
	fConst8: f32,
	fRec4: [f32;2],
	fConst9: f32,
	fConst10: f32,
	fConst11: f32,
	fRec3: [f32;2],
	fConst12: f32,
	fConst13: f32,
	fConst14: f32,
	fRec0: [f32;3],
	fVec4: [f32;2],
	fRec7: [f32;2],
	fVec5: [f32;2],
	fRec6: [f32;2],
	fVec6: [f32;2],
	fVec7: [f32;4096],
	fConst15: f32,
	fConst16: f32,
	fRec8: [f32;2],
	fConst17: f32,
	fRec10: [f32;2],
	fConst18: f32,
	fRec9: [f32;2],
	fConst19: f32,
	fRec5: [f32;3],
	fConst20: f32,
	fConst21: f32,
	fConst22: f32,
	fConst23: f32,
	fConst24: f32,
	fRec12: [f32;2],
	fConst25: f32,
	fRec14: [f32;2],
	fRec13: [f32;2],
	fConst26: f32,
	fVec8: [f32;2],
	fRec15: [f32;2],
	fVec9: [f32;2],
	fVec10: [f32;4096],
	fConst27: f32,
	fConst28: f32,
	fConst29: f32,
	fConst30: f32,
	fConst31: f32,
	fRec11: [f32;3],
	fVec11: [f32;2],
	fRec17: [f32;2],
	fVec12: [f32;2],
	fVec13: [f32;4096],
	fRec16: [f32;3],
	fConst32: f32,
	fConst33: f32,
	fRec20: [f32;2],
	fConst34: f32,
	fConst35: f32,
	fRec19: [f32;2],
	fVec14: [f32;2],
	fRec21: [f32;2],
	fVec15: [f32;2],
	fVec16: [f32;2048],
	fConst36: f32,
	fRec18: [f32;3],

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
			fConst4: 0.0,
			fHslider1: 0.0,
			fVec0: [0.0;2],
			iVec1: [0;2],
			fConst5: 0.0,
			fRec1: [0.0;2],
			fVec2: [0.0;2],
			IOTA: 0,
			fVec3: [0.0;4096],
			fConst6: 0.0,
			fButton0: 0.0,
			fConst7: 0.0,
			fRec2: [0.0;2],
			fConst8: 0.0,
			fRec4: [0.0;2],
			fConst9: 0.0,
			fConst10: 0.0,
			fConst11: 0.0,
			fRec3: [0.0;2],
			fConst12: 0.0,
			fConst13: 0.0,
			fConst14: 0.0,
			fRec0: [0.0;3],
			fVec4: [0.0;2],
			fRec7: [0.0;2],
			fVec5: [0.0;2],
			fRec6: [0.0;2],
			fVec6: [0.0;2],
			fVec7: [0.0;4096],
			fConst15: 0.0,
			fConst16: 0.0,
			fRec8: [0.0;2],
			fConst17: 0.0,
			fRec10: [0.0;2],
			fConst18: 0.0,
			fRec9: [0.0;2],
			fConst19: 0.0,
			fRec5: [0.0;3],
			fConst20: 0.0,
			fConst21: 0.0,
			fConst22: 0.0,
			fConst23: 0.0,
			fConst24: 0.0,
			fRec12: [0.0;2],
			fConst25: 0.0,
			fRec14: [0.0;2],
			fRec13: [0.0;2],
			fConst26: 0.0,
			fVec8: [0.0;2],
			fRec15: [0.0;2],
			fVec9: [0.0;2],
			fVec10: [0.0;4096],
			fConst27: 0.0,
			fConst28: 0.0,
			fConst29: 0.0,
			fConst30: 0.0,
			fConst31: 0.0,
			fRec11: [0.0;3],
			fVec11: [0.0;2],
			fRec17: [0.0;2],
			fVec12: [0.0;2],
			fVec13: [0.0;4096],
			fRec16: [0.0;3],
			fConst32: 0.0,
			fConst33: 0.0,
			fRec20: [0.0;2],
			fConst34: 0.0,
			fConst35: 0.0,
			fRec19: [0.0;2],
			fVec14: [0.0;2],
			fRec21: [0.0;2],
			fVec15: [0.0;2],
			fVec16: [0.0;2048],
			fConst36: 0.0,
			fRec18: [0.0;3],
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
		self.fButton0 = 0.0;
	}
	fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.fVec0[l0 as usize] = 0.0;
		}
		for l1 in 0..2 {
			self.iVec1[l1 as usize] = 0;
		}
		for l2 in 0..2 {
			self.fRec1[l2 as usize] = 0.0;
		}
		for l3 in 0..2 {
			self.fVec2[l3 as usize] = 0.0;
		}
		self.IOTA = 0;
		for l4 in 0..4096 {
			self.fVec3[l4 as usize] = 0.0;
		}
		for l5 in 0..2 {
			self.fRec2[l5 as usize] = 0.0;
		}
		for l6 in 0..2 {
			self.fRec4[l6 as usize] = 0.0;
		}
		for l7 in 0..2 {
			self.fRec3[l7 as usize] = 0.0;
		}
		for l8 in 0..3 {
			self.fRec0[l8 as usize] = 0.0;
		}
		for l9 in 0..2 {
			self.fVec4[l9 as usize] = 0.0;
		}
		for l10 in 0..2 {
			self.fRec7[l10 as usize] = 0.0;
		}
		for l11 in 0..2 {
			self.fVec5[l11 as usize] = 0.0;
		}
		for l12 in 0..2 {
			self.fRec6[l12 as usize] = 0.0;
		}
		for l13 in 0..2 {
			self.fVec6[l13 as usize] = 0.0;
		}
		for l14 in 0..4096 {
			self.fVec7[l14 as usize] = 0.0;
		}
		for l15 in 0..2 {
			self.fRec8[l15 as usize] = 0.0;
		}
		for l16 in 0..2 {
			self.fRec10[l16 as usize] = 0.0;
		}
		for l17 in 0..2 {
			self.fRec9[l17 as usize] = 0.0;
		}
		for l18 in 0..3 {
			self.fRec5[l18 as usize] = 0.0;
		}
		for l19 in 0..2 {
			self.fRec12[l19 as usize] = 0.0;
		}
		for l20 in 0..2 {
			self.fRec14[l20 as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.fRec13[l21 as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.fVec8[l22 as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.fRec15[l23 as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.fVec9[l24 as usize] = 0.0;
		}
		for l25 in 0..4096 {
			self.fVec10[l25 as usize] = 0.0;
		}
		for l26 in 0..3 {
			self.fRec11[l26 as usize] = 0.0;
		}
		for l27 in 0..2 {
			self.fVec11[l27 as usize] = 0.0;
		}
		for l28 in 0..2 {
			self.fRec17[l28 as usize] = 0.0;
		}
		for l29 in 0..2 {
			self.fVec12[l29 as usize] = 0.0;
		}
		for l30 in 0..4096 {
			self.fVec13[l30 as usize] = 0.0;
		}
		for l31 in 0..3 {
			self.fRec16[l31 as usize] = 0.0;
		}
		for l32 in 0..2 {
			self.fRec20[l32 as usize] = 0.0;
		}
		for l33 in 0..2 {
			self.fRec19[l33 as usize] = 0.0;
		}
		for l34 in 0..2 {
			self.fVec14[l34 as usize] = 0.0;
		}
		for l35 in 0..2 {
			self.fRec21[l35 as usize] = 0.0;
		}
		for l36 in 0..2 {
			self.fVec15[l36 as usize] = 0.0;
		}
		for l37 in 0..2048 {
			self.fVec16[l37 as usize] = 0.0;
		}
		for l38 in 0..3 {
			self.fRec18[l38 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		self.fConst0 = f32::min(192000.0, f32::max(1.0, (self.fSampleRate as f32)));
		self.fConst1 = f32::tan((9424.77832 / self.fConst0));
		self.fConst2 = (1.0 / self.fConst1);
		self.fConst3 = (1.0 / (((self.fConst2 + 2.0) / self.fConst1) + 1.0));
		self.fConst4 = (0.25 * self.fConst0);
		self.fConst5 = (1.0 / self.fConst0);
		self.fConst6 = (0.699999988 * self.fConst0);
		self.fConst7 = f32::max(0.00100000005, (0.178000003 * self.fConst0));
		self.fConst8 = (0.182999998 * self.fConst0);
		self.fConst9 = (0.00499999989 * self.fConst0);
		self.fConst10 = (5.61797762 / self.fConst0);
		self.fConst11 = (200.0 / self.fConst0);
		self.fConst12 = (1.0 / self.fConst7);
		self.fConst13 = (((self.fConst2 + -2.0) / self.fConst1) + 1.0);
		self.fConst14 = (2.0 * (1.0 - (1.0 / mydsp_faustpower2_f(self.fConst1))));
		self.fConst15 = (0.5 * self.fConst0);
		self.fConst16 = f32::max(0.00100000005, (1.30299997 * self.fConst0));
		self.fConst17 = (0.0549999997 * self.fConst0);
		self.fConst18 = (20.0 / self.fConst0);
		self.fConst19 = (1.0 / self.fConst16);
		self.fConst20 = f32::tan((10995.5742 / self.fConst0));
		self.fConst21 = (1.0 / self.fConst20);
		self.fConst22 = (1.0 / (((self.fConst21 + 0.200000003) / self.fConst20) + 1.0));
		self.fConst23 = (0.0500000007 * self.fConst0);
		self.fConst24 = f32::max(0.00100000005, self.fConst4);
		self.fConst25 = (0.100000001 * self.fConst0);
		self.fConst26 = (1.0 / self.fConst24);
		self.fConst27 = (0.400000006 * self.fConst0);
		self.fConst28 = (0.300000012 * self.fConst0);
		self.fConst29 = (0.200000003 * self.fConst0);
		self.fConst30 = (((self.fConst21 + -0.200000003) / self.fConst20) + 1.0);
		self.fConst31 = (2.0 * (1.0 - (1.0 / mydsp_faustpower2_f(self.fConst20))));
		self.fConst32 = (1.0 / (((self.fConst21 + 0.5) / self.fConst20) + 1.0));
		self.fConst33 = (0.0799999982 * self.fConst0);
		self.fConst34 = (0.0299999993 * self.fConst0);
		self.fConst35 = (33.3333321 / self.fConst0);
		self.fConst36 = (((self.fConst21 + -0.5) / self.fConst20) + 1.0);
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
			"Waveform" => Param { index: 0, range: ParamRange::new(0.0, 0.0, 4.0, 1.0) },
			"freq_v0" => Param { index: 1, range: ParamRange::new(440.0, 50.0, 2000.0, 0.01) },
			"gain_v0" => Param { index: 2, range: ParamRange::new(0.5, 0.0, 1.0, 0.01) },
			"gate_v0" => Param { index: 3, range: ParamRange::new(0.0,0.0,0.0,0.0) },
			
			
			
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
			3 => self.fButton0,
			0 => self.fEntry0,
			2 => self.fHslider0,
			1 => self.fHslider1,
			_ => 0.,
		}
	}
	
	pub fn set_param(&mut self, param: u32, value: T) {
		match param {
			3 => { self.fButton0 = value }
			0 => { self.fEntry0 = value }
			2 => { self.fHslider0 = value }
			1 => { self.fHslider1 = value }
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
		let mut iSlow1: i32 = ((fSlow0 >= 3.0) as i32);
		let mut iSlow2: i32 = ((fSlow0 >= 2.0) as i32);
		let mut iSlow3: i32 = ((fSlow0 >= 1.0) as i32);
		let mut fSlow4: f32 = (self.fHslider0 as f32);
		let mut fSlow5: f32 = (self.fConst3 * fSlow4);
		let mut fSlow6: f32 = (self.fHslider1 as f32);
		let mut fSlow7: f32 = f32::max(fSlow6, 23.4489498);
		let mut fSlow8: f32 = f32::max(20.0, f32::abs(fSlow7));
		let mut fSlow9: f32 = (self.fConst4 / fSlow8);
		let mut fSlow10: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst6 / fSlow7)));
		let mut fSlow11: f32 = f32::floor(fSlow10);
		let mut fSlow12: f32 = (fSlow11 + (1.0 - fSlow10));
		let mut iSlow13: i32 = (fSlow10 as i32);
		let mut fSlow14: f32 = (fSlow10 - fSlow11);
		let mut iSlow15: i32 = (iSlow13 + 1);
		let mut fSlow16: f32 = (self.fButton0 as f32);
		let mut iSlow17: i32 = ((fSlow16 > 0.0) as i32);
		let mut iSlow18: i32 = ((iSlow17 > 0) as i32);
		let mut fSlow19: f32 = (iSlow17 as f32);
		let mut iSlow20: i32 = ((((fSlow16 == 0.0) as i32) > 0) as i32);
		let mut fSlow21: f32 = (0.305999994 * fSlow16);
		let mut fSlow22: f32 = (self.fConst10 * (fSlow21 + -1.0));
		let mut fSlow23: f32 = (self.fConst4 * fSlow4);
		let mut fSlow24: f32 = (2.0 * fSlow6);
		let mut fSlow25: f32 = (0.829999983 * fSlow16);
		let mut fSlow26: f32 = (self.fConst18 * (fSlow25 + -1.0));
		let mut fSlow27: f32 = (self.fConst22 * fSlow4);
		let mut fSlow28: f32 = (self.fConst23 * fSlow4);
		let mut iSlow29: i32 = ((fSlow0 >= 4.0) as i32);
		let mut fSlow30: f32 = (self.fConst32 * fSlow4);
		let mut fSlow31: f32 = (0.5 * fSlow4);
		let mut fSlow32: f32 = f32::max((0.5 * fSlow6), 23.4489498);
		let mut fSlow33: f32 = f32::max(20.0, f32::abs(fSlow32));
		let mut fSlow34: f32 = (self.fConst4 / fSlow33);
		let mut fSlow35: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst25 / fSlow32)));
		let mut fSlow36: f32 = f32::floor(fSlow35);
		let mut fSlow37: f32 = (fSlow36 + (1.0 - fSlow35));
		let mut iSlow38: i32 = (fSlow35 as i32);
		let mut fSlow39: f32 = (fSlow35 - fSlow36);
		let mut iSlow40: i32 = (iSlow38 + 1);
		let mut fSlow41: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst29 / fSlow32)));
		let mut fSlow42: f32 = f32::floor(fSlow41);
		let mut fSlow43: f32 = (fSlow42 + (1.0 - fSlow41));
		let mut iSlow44: i32 = (fSlow41 as i32);
		let mut fSlow45: f32 = (fSlow41 - fSlow42);
		let mut iSlow46: i32 = (iSlow44 + 1);
		let zipped_iterators = outputs0;
		for output0 in zipped_iterators {
			self.fVec0[0] = fSlow8;
			self.iVec1[0] = 1;
			let mut fTemp0: f32 = (self.iVec1[1] as f32);
			let mut fTemp1: f32 = (self.fRec1[1] + (self.fConst5 * self.fVec0[1]));
			self.fRec1[0] = (fTemp1 - f32::floor(fTemp1));
			let mut fTemp2: f32 = mydsp_faustpower2_f(((2.0 * self.fRec1[0]) + -1.0));
			self.fVec2[0] = fTemp2;
			let mut fTemp3: f32 = (fSlow9 * (fTemp0 * (fTemp2 - self.fVec2[1])));
			self.fVec3[(self.IOTA & 4095) as usize] = fTemp3;
			self.fRec2[0] = if (iSlow18 as i32 != 0) { 0.0 } else { f32::min(self.fConst7, (self.fRec2[1] + 1.0)) };
			self.fRec4[0] = if (iSlow20 as i32 != 0) { 0.0 } else { f32::min(self.fConst8, (self.fRec4[1] + 1.0)) };
			let mut iTemp4: i32 = ((self.fRec4[0] < self.fConst9) as i32);
			self.fRec3[0] = if (iSlow17 as i32 != 0) { (fSlow19 * if (iTemp4 as i32 != 0) { if (((self.fRec4[0] < 0.0) as i32) as i32 != 0) { 0.0 } else { if (iTemp4 as i32 != 0) { (self.fConst11 * self.fRec4[0]) } else { 1.0 } } } else { if (((self.fRec4[0] < self.fConst8) as i32) as i32 != 0) { ((fSlow22 * (self.fRec4[0] - self.fConst9)) + 1.0) } else { fSlow21 } }) } else { self.fRec3[1] };
			self.fRec0[0] = ((fSlow4 * ((fTemp3 - ((fSlow12 * self.fVec3[((self.IOTA - iSlow13) & 4095) as usize]) + (fSlow14 * self.fVec3[((self.IOTA - iSlow15) & 4095) as usize]))) * if (((self.fRec2[0] < 0.0) as i32) as i32 != 0) { self.fRec3[0] } else { if (((self.fRec2[0] < self.fConst7) as i32) as i32 != 0) { (self.fRec3[0] + (self.fConst12 * (0.0 - (self.fRec2[0] * self.fRec3[0])))) } else { 0.0 } })) - (self.fConst3 * ((self.fConst13 * self.fRec0[2]) + (self.fConst14 * self.fRec0[1]))));
			self.fVec4[0] = 5.69999981;
			let mut fTemp5: f32 = ((self.fConst5 * self.fVec4[1]) + self.fRec7[1]);
			self.fRec7[0] = (fTemp5 - f32::floor(fTemp5));
			let mut fTemp6: f32 = ((2.0 * (1.0 - f32::abs(((2.0 * self.fRec7[0]) + -1.0)))) + -1.0);
			let mut fTemp7: f32 = f32::max((fSlow24 + (10.0 * fTemp6)), 23.4489498);
			let mut fTemp8: f32 = f32::max(20.0, f32::abs(fTemp7));
			self.fVec5[0] = fTemp8;
			let mut fTemp9: f32 = (self.fRec6[1] + (self.fConst5 * self.fVec5[1]));
			self.fRec6[0] = (fTemp9 - f32::floor(fTemp9));
			let mut fTemp10: f32 = mydsp_faustpower2_f(((2.0 * self.fRec6[0]) + -1.0));
			self.fVec6[0] = fTemp10;
			let mut fTemp11: f32 = ((fTemp0 * (fTemp10 - self.fVec6[1])) / fTemp8);
			self.fVec7[(self.IOTA & 4095) as usize] = fTemp11;
			let mut fTemp12: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst15 / fTemp7)));
			let mut iTemp13: i32 = (fTemp12 as i32);
			let mut fTemp14: f32 = f32::floor(fTemp12);
			self.fRec8[0] = if (iSlow18 as i32 != 0) { 0.0 } else { f32::min(self.fConst16, (self.fRec8[1] + 1.0)) };
			self.fRec10[0] = if (iSlow20 as i32 != 0) { 0.0 } else { f32::min(self.fConst17, (self.fRec10[1] + 1.0)) };
			let mut iTemp15: i32 = ((self.fRec10[0] < self.fConst9) as i32);
			self.fRec9[0] = if (iSlow17 as i32 != 0) { (fSlow19 * if (iTemp15 as i32 != 0) { if (((self.fRec10[0] < 0.0) as i32) as i32 != 0) { 0.0 } else { if (iTemp15 as i32 != 0) { (self.fConst11 * self.fRec10[0]) } else { 1.0 } } } else { if (((self.fRec10[0] < self.fConst17) as i32) as i32 != 0) { ((fSlow26 * (self.fRec10[0] - self.fConst9)) + 1.0) } else { fSlow25 } }) } else { self.fRec9[1] };
			self.fRec5[0] = ((fSlow23 * (((fTemp11 - (self.fVec7[((self.IOTA - iTemp13) & 4095) as usize] * (fTemp14 + (1.0 - fTemp12)))) - ((fTemp12 - fTemp14) * self.fVec7[((self.IOTA - (iTemp13 + 1)) & 4095) as usize])) * if (((self.fRec8[0] < 0.0) as i32) as i32 != 0) { self.fRec9[0] } else { if (((self.fRec8[0] < self.fConst16) as i32) as i32 != 0) { (self.fRec9[0] + (self.fConst19 * (0.0 - (self.fRec8[0] * self.fRec9[0])))) } else { 0.0 } })) - (self.fConst3 * ((self.fConst13 * self.fRec5[2]) + (self.fConst14 * self.fRec5[1]))));
			self.fRec12[0] = if (iSlow18 as i32 != 0) { 0.0 } else { f32::min(self.fConst24, (self.fRec12[1] + 1.0)) };
			let mut iTemp16: i32 = ((self.fRec12[0] < 0.0) as i32);
			let mut iTemp17: i32 = ((self.fRec12[0] < self.fConst24) as i32);
			self.fRec14[0] = if (iSlow20 as i32 != 0) { 0.0 } else { f32::min(self.fConst25, (self.fRec14[1] + 1.0)) };
			let mut iTemp18: i32 = ((self.fRec14[0] < self.fConst23) as i32);
			self.fRec13[0] = if (iSlow17 as i32 != 0) { (fSlow19 * if (iTemp18 as i32 != 0) { if (((self.fRec14[0] < 0.0) as i32) as i32 != 0) { 0.0 } else { if (iTemp18 as i32 != 0) { (self.fConst18 * self.fRec14[0]) } else { 1.0 } } } else { if (((self.fRec14[0] < self.fConst25) as i32) as i32 != 0) { ((fSlow26 * (self.fRec14[0] - self.fConst23)) + 1.0) } else { fSlow25 } }) } else { self.fRec13[1] };
			let mut fTemp19: f32 = if (iTemp16 as i32 != 0) { self.fRec13[0] } else { if (iTemp17 as i32 != 0) { (self.fRec13[0] + (self.fConst26 * (0.0 - (self.fRec12[0] * self.fRec13[0])))) } else { 0.0 } };
			let mut fTemp20: f32 = f32::max((fSlow6 + (5.0 * fTemp6)), 23.4489498);
			let mut fTemp21: f32 = f32::max(20.0, f32::abs(fTemp20));
			self.fVec8[0] = fTemp21;
			let mut fTemp22: f32 = (self.fRec15[1] + (self.fConst5 * self.fVec8[1]));
			self.fRec15[0] = (fTemp22 - f32::floor(fTemp22));
			let mut fTemp23: f32 = mydsp_faustpower2_f(((2.0 * self.fRec15[0]) + -1.0));
			self.fVec9[0] = fTemp23;
			let mut fTemp24: f32 = ((fTemp0 * (fTemp23 - self.fVec9[1])) / fTemp21);
			self.fVec10[(self.IOTA & 4095) as usize] = fTemp24;
			let mut fTemp25: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst15 / fTemp20)));
			let mut iTemp26: i32 = (fTemp25 as i32);
			let mut fTemp27: f32 = f32::floor(fTemp25);
			let mut fTemp28: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst27 / fTemp20)));
			let mut iTemp29: i32 = (fTemp28 as i32);
			let mut fTemp30: f32 = f32::floor(fTemp28);
			let mut fTemp31: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst28 / fTemp20)));
			let mut iTemp32: i32 = (fTemp31 as i32);
			let mut fTemp33: f32 = f32::floor(fTemp31);
			let mut fTemp34: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst29 / fTemp20)));
			let mut iTemp35: i32 = (fTemp34 as i32);
			let mut fTemp36: f32 = f32::floor(fTemp34);
			let mut fTemp37: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst25 / fTemp20)));
			let mut iTemp38: i32 = (fTemp37 as i32);
			let mut fTemp39: f32 = f32::floor(fTemp37);
			self.fRec11[0] = ((fSlow28 * (fTemp19 * ((((((fTemp24 - (self.fVec10[((self.IOTA - iTemp26) & 4095) as usize] * (fTemp27 + (1.0 - fTemp25)))) - ((fTemp25 - fTemp27) * self.fVec10[((self.IOTA - (iTemp26 + 1)) & 4095) as usize])) + ((fTemp24 - (self.fVec10[((self.IOTA - iTemp29) & 4095) as usize] * (fTemp30 + (1.0 - fTemp28)))) - ((fTemp28 - fTemp30) * self.fVec10[((self.IOTA - (iTemp29 + 1)) & 4095) as usize]))) + ((fTemp24 - (self.fVec10[((self.IOTA - iTemp32) & 4095) as usize] * (fTemp33 + (1.0 - fTemp31)))) - ((fTemp31 - fTemp33) * self.fVec10[((self.IOTA - (iTemp32 + 1)) & 4095) as usize]))) + ((fTemp24 - (self.fVec10[((self.IOTA - iTemp35) & 4095) as usize] * (fTemp36 + (1.0 - fTemp34)))) - ((fTemp34 - fTemp36) * self.fVec10[((self.IOTA - (iTemp35 + 1)) & 4095) as usize]))) + ((fTemp24 - (self.fVec10[((self.IOTA - iTemp38) & 4095) as usize] * (fTemp39 + (1.0 - fTemp37)))) - ((fTemp37 - fTemp39) * self.fVec10[((self.IOTA - (iTemp38 + 1)) & 4095) as usize]))))) - (self.fConst22 * ((self.fConst30 * self.fRec11[2]) + (self.fConst31 * self.fRec11[1]))));
			let mut fTemp40: f32 = f32::max((fSlow6 + (3.0 * fTemp6)), 23.4489498);
			let mut fTemp41: f32 = f32::max(20.0, f32::abs(fTemp40));
			self.fVec11[0] = fTemp41;
			let mut fTemp42: f32 = (self.fRec17[1] + (self.fConst5 * self.fVec11[1]));
			self.fRec17[0] = (fTemp42 - f32::floor(fTemp42));
			let mut fTemp43: f32 = mydsp_faustpower2_f(((2.0 * self.fRec17[0]) + -1.0));
			self.fVec12[0] = fTemp43;
			let mut fTemp44: f32 = ((fTemp0 * (fTemp43 - self.fVec12[1])) / fTemp41);
			self.fVec13[(self.IOTA & 4095) as usize] = fTemp44;
			let mut fTemp45: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst15 / fTemp40)));
			let mut iTemp46: i32 = (fTemp45 as i32);
			let mut fTemp47: f32 = f32::floor(fTemp45);
			self.fRec16[0] = ((fSlow23 * (fTemp19 * ((fTemp44 - (self.fVec13[((self.IOTA - iTemp46) & 4095) as usize] * (fTemp47 + (1.0 - fTemp45)))) - ((fTemp45 - fTemp47) * self.fVec13[((self.IOTA - (iTemp46 + 1)) & 4095) as usize])))) - (self.fConst3 * ((self.fConst13 * self.fRec16[2]) + (self.fConst14 * self.fRec16[1]))));
			self.fRec20[0] = if (iSlow20 as i32 != 0) { 0.0 } else { f32::min(self.fConst33, (self.fRec20[1] + 1.0)) };
			let mut iTemp48: i32 = ((self.fRec20[0] < self.fConst34) as i32);
			self.fRec19[0] = if (iSlow17 as i32 != 0) { (fSlow19 * if (iTemp48 as i32 != 0) { if (((self.fRec20[0] < 0.0) as i32) as i32 != 0) { 0.0 } else { if (iTemp48 as i32 != 0) { (self.fConst35 * self.fRec20[0]) } else { 1.0 } } } else { if (((self.fRec20[0] < self.fConst33) as i32) as i32 != 0) { ((fSlow26 * (self.fRec20[0] - self.fConst34)) + 1.0) } else { fSlow25 } }) } else { self.fRec19[1] };
			self.fVec14[0] = fSlow33;
			let mut fTemp49: f32 = (self.fRec21[1] + (self.fConst5 * self.fVec14[1]));
			self.fRec21[0] = (fTemp49 - f32::floor(fTemp49));
			let mut fTemp50: f32 = mydsp_faustpower2_f(((2.0 * self.fRec21[0]) + -1.0));
			self.fVec15[0] = fTemp50;
			let mut fTemp51: f32 = (fSlow34 * (fTemp0 * (fTemp50 - self.fVec15[1])));
			self.fVec16[(self.IOTA & 2047) as usize] = fTemp51;
			self.fRec18[0] = ((fSlow31 * (if (iTemp16 as i32 != 0) { self.fRec19[0] } else { if (iTemp17 as i32 != 0) { (self.fRec19[0] + (self.fConst26 * (0.0 - (self.fRec12[0] * self.fRec19[0])))) } else { 0.0 } } * ((fTemp51 - ((fSlow37 * self.fVec16[((self.IOTA - iSlow38) & 2047) as usize]) + (fSlow39 * self.fVec16[((self.IOTA - iSlow40) & 2047) as usize]))) + (fTemp51 - ((fSlow43 * self.fVec16[((self.IOTA - iSlow44) & 2047) as usize]) + (fSlow45 * self.fVec16[((self.IOTA - iSlow46) & 2047) as usize])))))) - (self.fConst32 * ((self.fConst36 * self.fRec18[2]) + (self.fConst31 * self.fRec18[1]))));
			*output0 = (if (iSlow1 as i32 != 0) { if (iSlow29 as i32 != 0) { (fSlow30 * (self.fRec18[2] + (self.fRec18[0] + (2.0 * self.fRec18[1])))) } else { (fSlow5 * (self.fRec16[2] + (self.fRec16[0] + (2.0 * self.fRec16[1])))) } } else { if (iSlow2 as i32 != 0) { (fSlow27 * (self.fRec11[2] + (self.fRec11[0] + (2.0 * self.fRec11[1])))) } else { if (iSlow3 as i32 != 0) { (fSlow5 * (self.fRec5[2] + (self.fRec5[0] + (2.0 * self.fRec5[1])))) } else { (fSlow5 * (self.fRec0[2] + (self.fRec0[0] + (2.0 * self.fRec0[1])))) } } } as f32);
			self.fVec0[1] = self.fVec0[0];
			self.iVec1[1] = self.iVec1[0];
			self.fRec1[1] = self.fRec1[0];
			self.fVec2[1] = self.fVec2[0];
			self.IOTA = (self.IOTA + 1);
			self.fRec2[1] = self.fRec2[0];
			self.fRec4[1] = self.fRec4[0];
			self.fRec3[1] = self.fRec3[0];
			self.fRec0[2] = self.fRec0[1];
			self.fRec0[1] = self.fRec0[0];
			self.fVec4[1] = self.fVec4[0];
			self.fRec7[1] = self.fRec7[0];
			self.fVec5[1] = self.fVec5[0];
			self.fRec6[1] = self.fRec6[0];
			self.fVec6[1] = self.fVec6[0];
			self.fRec8[1] = self.fRec8[0];
			self.fRec10[1] = self.fRec10[0];
			self.fRec9[1] = self.fRec9[0];
			self.fRec5[2] = self.fRec5[1];
			self.fRec5[1] = self.fRec5[0];
			self.fRec12[1] = self.fRec12[0];
			self.fRec14[1] = self.fRec14[0];
			self.fRec13[1] = self.fRec13[0];
			self.fVec8[1] = self.fVec8[0];
			self.fRec15[1] = self.fRec15[0];
			self.fVec9[1] = self.fVec9[0];
			self.fRec11[2] = self.fRec11[1];
			self.fRec11[1] = self.fRec11[0];
			self.fVec11[1] = self.fVec11[0];
			self.fRec17[1] = self.fRec17[0];
			self.fVec12[1] = self.fVec12[0];
			self.fRec16[2] = self.fRec16[1];
			self.fRec16[1] = self.fRec16[0];
			self.fRec20[1] = self.fRec20[0];
			self.fRec19[1] = self.fRec19[0];
			self.fVec14[1] = self.fVec14[0];
			self.fRec21[1] = self.fRec21[0];
			self.fVec15[1] = self.fVec15[0];
			self.fRec18[2] = self.fRec18[1];
			self.fRec18[1] = self.fRec18[0];
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