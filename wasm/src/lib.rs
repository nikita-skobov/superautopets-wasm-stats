fn get_image_width(bytes: &[u8]) -> Result<u32, String> {
    let img = image::load_from_memory(bytes).map_err(|e| e.to_string())?;
    Ok(img.width())
}

#[unsafe(no_mangle)]
extern "C" fn wasm_entrypoint(ptr: *const u8, len: usize) -> i32 {
    let bytes: &[u8] = unsafe { std::slice::from_raw_parts(ptr, len) };

    match get_image_width(bytes) {
        Ok(o) => o as _,
        Err(_) => -1,
    }
}

#[unsafe(no_mangle)]
extern "C" fn wasm_debug_sum(ptr: *const u8, len: usize) -> u32 {
    let bytes: &[u8] = unsafe { std::slice::from_raw_parts(ptr, len) };

    let mut out: u32 = 0;
    for b in bytes.iter() {
        let b = *b as u32;
        out += b;
    }
    out
}
