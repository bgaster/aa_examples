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