#![deny(improper_ctypes, improper_ctypes_definitions)]

use wallee::wallee;

#[no_mangle]
pub extern "C" fn wallee1(err: wallee::Error) {
    println!("{err:?}");
}

#[no_mangle]
pub extern "C" fn wallee2(err: &mut Option<wallee::Error>) {
    *err = Some(wallee!("ffi error"));
}

#[no_mangle]
pub extern "C" fn wallee3() -> Option<wallee::Error> {
    Some(wallee!("ffi error"))
}
