fn main() -> Result<(), failure::Error> {
    let lib = libloading::Library::new("../dyn_c_lib/libwibble.so")?;
    let res = unsafe {
        let f: libloading::Symbol<unsafe extern "C" fn(i32, i32) -> i32> = lib.get(b"wibble_it")?;
        f(4, 20)
    };

    println!("Hello, world! res = {}", res);
    Ok(())
}
