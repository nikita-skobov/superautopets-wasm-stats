// #[link(wasm_import_module = "host")]
// extern "C" {
//     fn get_host_data_size() -> u32;
//     fn get_host_data(ptr: *const u8, len: u32);
//     fn set_host_data(ptr: *const u8, len: u32);
// }

// fn fill_data_from_host() -> Vec<u8> {
//     let data = unsafe {
//         let len = get_host_data_size() as usize;
//         let mut data: Vec<u8> = Vec::with_capacity(len);
//         data.set_len(len);
//         let ptr = data.as_ptr();
//         let len = data.len();
//         get_host_data(ptr, len as _);
//         data
//     };
//     data
// }

// fn set_data_for_host(out_data: Vec<u8>) {
//     unsafe {
//         let ptr = out_data.as_ptr();
//         let len = out_data.len();
//         set_host_data(ptr, len as _);
//     }
// }

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
