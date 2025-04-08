use image::{DynamicImage, GenericImageView, SubImage};

const BLACK: [u8; 4] = [0,0,0,255];
const WHITE: [u8; 4] = [255,255,255,255];

fn get_image_width(bytes: &[u8]) -> Result<u32, String> {
    let img = image::load_from_memory(bytes).map_err(|e| e.to_string())?;
    Ok(img.width())
}

fn get_image_data(bytes: &[u8]) -> Result<HeartData, String> {
    let img = image::load_from_memory(bytes).map_err(|e| e.to_string())?;
    let probable_pets_img = probable_pets_image_check(&img).ok_or(format!("not a pets img"))?;
    let pets_img = PetsImage::new(&probable_pets_img).ok_or(format!("not a pets img"))?;
    Ok(pets_img.heart_data)
}


#[unsafe(no_mangle)]
extern "C" fn alloc(len: usize) -> *mut u8 {
    let mut buffer = Vec::with_capacity(len);
    let ptr = buffer.as_mut_ptr();
    std::mem::forget(buffer);
    ptr
}

#[unsafe(no_mangle)]
extern "C" fn wasm_entrypoint(ptr: *mut u8, len: usize) -> i32 {
    let bytes: Vec<u8> = unsafe { Vec::from_raw_parts(ptr, len, len) };

    match get_image_data(&bytes) {
        Ok(o) => {
            // wasm can return primitive types only, we transform our
            // data we're interested into a number

            // 3 least significant bits can represent 0-5
            // 1 bits to the left we use to denote has_bandage or not
            let has_bandage: i32 = 0b1000;
            let mut out = o.num_hearts as i32;
            if o.has_bandage {
                out += has_bandage;
            }
            out
        }
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

/// we can get reasonably close to knowing a screenshot is of SAP
/// by checking both margins at 131px and if they are 100% black on the sides
/// then its likely a pets image
pub struct ProbablyPetsImage<'a> {
    pub margin_size: u32,
    /// most will be on left. a few have this margin on the right. if false => right
    pub margin_is_on_left: bool,
    pub view: SubImage<&'a DynamicImage>,
}

fn probable_pets_image_check<'a>(img: &'a DynamicImage) -> Option<ProbablyPetsImage<'a>> {
    let margin_size = 131;
    let img_height = img.height();
    let img_width = img.width();
    if img_width != 2400 || img_height != 1080 {
        return None;
    }
    let mut margin_is_on_left = true;
    for y in 0..img_height {
        let margin_pixel = img.get_pixel(margin_size, y);
        if margin_pixel.0 != BLACK {
            margin_is_on_left = false;
            break;
        }
    }
    if !margin_is_on_left {
        for y in 0..img_height {
            let margin_pixel = img.get_pixel(img_width - margin_size, y);
            if margin_pixel.0 != BLACK {
                return None;
            }
        }
    }
    let view = if margin_is_on_left {
        img.view(margin_size, 0, img_width - margin_size, img_height)
    } else {
        img.view(0, 0, img_width - margin_size, img_height)
    };

    Some(ProbablyPetsImage { margin_size, margin_is_on_left, view })
}

struct PetsImage<'a> {
    pub view: SubImage<&'a DynamicImage>,
    pub heart_data: HeartData,
}

impl<'a> PetsImage<'a> {
    pub fn new(probpets: &'a ProbablyPetsImage<'a>) -> Option<Self> {
        let heart_data = handle_heart_check(probpets)?;
        Some(Self { view: probpets.view, heart_data })
    }
}

pub struct HeartData {
    pub valid_red_color: [u8; 4],
    pub num_hearts: usize,
    pub has_bandage: bool,
}

fn handle_heart_check<'a>(img: &'a ProbablyPetsImage<'a>) -> Option<HeartData> {
    let heart_view_x = 690;
    let heart_view_y = 892;
    let heart_view_width = 885;
    let heart_view_height = 133;

    let heart_bandage_x = 75;
    let heart_bandage_y = 60;

    let heart_view = img.view.view(heart_view_x, heart_view_y, heart_view_width, heart_view_height);

    let valid_red_heart_colors: [[u8; 4]; 2] = [
        [253, 6, 6, 255],
        [232, 52, 37, 255],
    ];
    let first_heart_pixel_x = 34;
    let pixel1 = heart_view.get_pixel(first_heart_pixel_x, first_heart_pixel_x);
    // return None if we are not looking at a pets screenshot:
    // if its not a red pixel where we expect there to be one
    let valid_red_color = valid_red_heart_colors.iter().find(|x| pixel1.0 == **x)?;
    let valid_red_color = *valid_red_color;
    let pixel_bandage = heart_view.get_pixel(heart_bandage_x, heart_bandage_y);
    let has_bandage = pixel_bandage.0 != valid_red_color;
    let heart_distance = 184;
    let mut num_hearts = 1;
    for i in 1..=4 {
        let heart_pixel_x = first_heart_pixel_x + heart_distance * i;
        let pixel_next = heart_view.get_pixel(heart_pixel_x, first_heart_pixel_x);
        if pixel_next.0 == valid_red_color {
            num_hearts += 1;
        }
    }
    Some(HeartData { valid_red_color, num_hearts, has_bandage })
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_probable_pets_img(filename: &str, t: impl Fn(ProbablyPetsImage)) {
        let data = std::fs::read(format!("./testfixtures/{}", filename)).expect("failed to read test fixture");
        let img = image::load_from_memory(&data).expect("failed to load test fixture img");
        let petsimg = probable_pets_image_check(&img).expect("it should be a pets img");
        t(petsimg);
    }

    #[test]
    fn can_detect_margin_position() {
        get_probable_pets_img("Screenshot_20240714-090957.png", |img| {
            assert!(img.margin_is_on_left);
        });

        get_probable_pets_img("Screenshot_20250308-073732.png", |img| {
            assert!(!img.margin_is_on_left);
        });
    }

    #[test]
    fn can_detect_heart_count() {
        get_probable_pets_img("Screenshot_20240714-090957.png", |img| {
            let petsimg = PetsImage::new(&img).expect("it should be a petsimg");
            assert_eq!(petsimg.heart_data.num_hearts, 3);
            assert_eq!(petsimg.heart_data.valid_red_color, [232, 52, 37, 255]);
            assert_eq!(petsimg.heart_data.has_bandage, false);
        });

        get_probable_pets_img("Screenshot_20250308-073732.png", |img| {
            let petsimg = PetsImage::new(&img).expect("it should be a petsimg");
            assert_eq!(petsimg.heart_data.num_hearts, 2);
            assert_eq!(petsimg.heart_data.valid_red_color, [253, 6, 6, 255]);
            assert_eq!(petsimg.heart_data.has_bandage, false);
        });

        get_probable_pets_img("Screenshot_20240629-104250.png", |img| {
            let petsimg = PetsImage::new(&img).expect("it should be a petsimg");
            assert_eq!(petsimg.heart_data.num_hearts, 2);
            assert_eq!(petsimg.heart_data.has_bandage, true);
        });
    }
}
