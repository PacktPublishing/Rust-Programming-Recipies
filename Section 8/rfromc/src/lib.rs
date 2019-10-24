use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn add_double(first: i32, second: i32) -> i32 {
    first + second * 2
}

#[no_mangle]
pub struct CPoint {
    x: i32,
    y: i32,
}

#[no_mangle]
pub extern "C" fn more_y(p: &mut CPoint) {
    p.y += p.x;
}

#[no_mangle]
pub extern "C" fn edit_string(s: &c_char) -> CString {
    println!("s = {}", s);
    let cs = unsafe { CStr::from_ptr(s) };
    println!("cs = {:?}", cs);
    let c_res = format!("You asked me {:?}", cs);

    CString::new(c_res).expect("Could not make CString")
}

#[no_mangle]
pub unsafe extern "C" fn s_to_space(s: *mut c_char) {
    for i in 0.. {
        let p = s.offset(i);
        println!("at {} = {}", i, (*p as u8) as char);
        if *p == 's' as i8 {
            *p = ' ' as i8;
        }
        if *p == 0 {
            return;
        }
    }
}
