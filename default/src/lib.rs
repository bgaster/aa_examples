#![feature(wasm_target_feature)]

const MAX_BUFFER_SIZE: usize = 1024;

#[no_mangle]
pub static mut IN_BUFFER: [f32;MAX_BUFFER_SIZE] = [0.;MAX_BUFFER_SIZE];

#[no_mangle]
pub static mut OUT_BUFFER: [f32;MAX_BUFFER_SIZE] = [0.;MAX_BUFFER_SIZE];

static mut INPUTS: [* const f32;1] = [0 as * const f32; 1];
static mut OUTPUTS: [* mut f32;1] = [0 as * mut f32; 1];

static mut SAMPLE_RATE: f64 = 0.;

#[no_mangle]
pub fn get_input(index: u32) -> u32 { 
    unsafe { INPUTS[index as usize] as u32 }
}

#[no_mangle]
pub fn get_output(index: u32) -> u32 { 
    unsafe { OUTPUTS[index as usize] as u32 }
}

#[no_mangle]
pub fn set_input(index: u32, offset: u32) { 
    unsafe { INPUTS[index as usize] = offset as * const f32; };
}

#[no_mangle]
pub fn set_output(index: u32, offset: u32) { 
    unsafe { OUTPUTS[index as usize] = offset as * mut f32; };
}

#[no_mangle]
pub fn handle_note_on(_mn: i32, _vel: f32) {
}

#[no_mangle]
pub fn handle_note_off(_mn: i32, _vel: f32) {
}

#[no_mangle]
pub fn get_voices() -> i32 {
    0
}

#[no_mangle]
pub fn get_param_index(_length: i32) -> i32 {
    -1
}

#[no_mangle]
pub fn get_sample_rate() -> f64 {
    unsafe { SAMPLE_RATE }
}

#[inline]
fn set_sample_rate(sample_rate: f64) {
    unsafe { SAMPLE_RATE = sample_rate; }
}

#[no_mangle]
pub fn get_num_input_channels() -> u32 {
    0
}

// number of output channels (currently max 2)
#[no_mangle]
pub fn get_num_output_channels() -> u32 {
    1
}

#[no_mangle]
pub fn init(sample_rate: f64) -> () {
    set_sample_rate(sample_rate);
    unsafe {
        INPUTS[0] = IN_BUFFER.as_ptr();
        OUTPUTS[0] = OUT_BUFFER.as_mut_ptr();
    };
}

#[no_mangle]
pub fn set_param_float(_index: u32, _v: f32) {
    // avoid bounds checking (NOTE: WASM will do this for us)
    unsafe { core::arch::wasm32::unreachable(); }
}

#[no_mangle]
pub fn set_param_int(_index: u32, _v: i32) {
   unsafe { core::arch::wasm32::unreachable(); }
}

/// set bool parameter 
/// panics if param not defined
#[no_mangle]
pub fn set_param_bool(_index: u32, _v: u8) {
    unsafe { core::arch::wasm32::unreachable(); }
}

// float parameter at index
// panics if param not defined
#[no_mangle]
pub fn get_param_float(_index: u32) -> f32 {
    unsafe { core::arch::wasm32::unreachable(); }
}

// int parameter at index
// panics if param not defined
#[no_mangle]
pub fn get_param_int(_index: u32) -> i32 {
    unsafe { core::arch::wasm32::unreachable(); }
}

// bool parameter at index
// panics if param not defined
#[no_mangle]
pub fn get_param_bool(_index: u32) -> bool {
    unsafe { core::arch::wasm32::unreachable(); }
}

#[no_mangle]
pub fn compute(frames: u32) -> () {
    let outputs = unsafe { ::std::slice::from_raw_parts_mut(OUT_BUFFER.as_mut_ptr(), frames as usize) };
    // write 0 to output buffer
    for o in outputs[..frames as usize].iter_mut() {
        *o = 0.;
    }
}
