use std::ops::BitAnd;

use image::{imageops::FilterType::Nearest, DynamicImage, GenericImageView, SubImage};

pub const BLACK: [u8; 4] = [0,0,0,255];
pub const WHITE: [u8; 4] = [255,255,255,255];

pub const VEC_0: [u8; 256] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub const VEC_1: [u8; 256] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub const VEC_2: [u8; 256] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub const VEC_3: [u8; 256] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub const VEC_4: [u8; 256] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub const VEC_5: [u8; 256] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0];
pub const VEC_6: [u8; 256] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub const VEC_7: [u8; 256] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub const VEC_8: [u8; 256] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0];
pub const VEC_9: [u8; 256] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

pub fn vector_distance(vec1: &[u8; 256], vec2: &[u8; 256]) -> f64 {
    vec1.iter().zip(vec2.iter())
        .map(|(a, b)| (*a as i8 - *b as i8).pow(2) as f64)
        .sum::<f64>()
        .sqrt()
}

fn get_image_width(bytes: &[u8]) -> Result<u32, String> {
    let img = image::load_from_memory(bytes).map_err(|e| e.to_string())?;
    Ok(img.width())
}

pub struct ImageData {
    pub heart_data: HeartData,
    pub turn_number: u8,
}

fn get_image_data(bytes: &[u8]) -> Result<ImageData, String> {
    let img = image::load_from_memory(bytes).map_err(|e| e.to_string())?;
    let probable_pets_img = probable_pets_image_check(&img).ok_or(format!("not a pets img"))?;
    let pets_img = PetsImage::new(&probable_pets_img).ok_or(format!("not a pets img"))?;
    let turn_number = pets_img.get_turn_number();
    Ok(ImageData {
        heart_data: pets_img.heart_data,
        turn_number,
    })
}

pub fn get_pets_img<T>(bytes: &[u8], cb: impl FnOnce(PetsImage) -> T) -> Result<T, String> {
    let img = image::load_from_memory(bytes).map_err(|e| e.to_string())?;
    let probable_pets_img = probable_pets_image_check(&img).ok_or(format!("not a pets img"))?;
    let pets_img = PetsImage::new(&probable_pets_img).ok_or(format!("not a pets img"))?;
    let t = cb(pets_img);

    Ok(t)
}


#[unsafe(no_mangle)]
extern "C" fn alloc(len: usize) -> *mut u8 {
    let mut buffer = Vec::with_capacity(len);
    let ptr = buffer.as_mut_ptr();
    std::mem::forget(buffer);
    ptr
}

#[unsafe(no_mangle)]
extern "C" fn wasm_entrypoint(ptr: *mut u8, len: usize) -> i64 {
    let bytes: Vec<u8> = unsafe { Vec::from_raw_parts(ptr, len, len) };

    match get_image_data(&bytes) {
        Ok(o) => {
            // wasm can return primitive types only, we transform our
            // data we're interested into a number

            // 3 least significant bits can represent 0-5
            // 1 bits to the left we use to denote has_bandage or not
            let has_bandage: i64 = 0b1000;
            let mut out = o.heart_data.num_hearts as i64;
            if o.heart_data.has_bandage {
                out += has_bandage;
            }
            // next, the turn number is a number usually between 10 and 30... it can be more
            // but highly unlikely.
            // we encode this in 5 bits. the first bit is whether its >= 20.
            // the next 4 bits encode the last digit 0 - 9.
            // 01000 => 18
            // 10000 => 20
            // 10001 => 21
            // 00001 => 11
            let over_20: i64 = 0b10000;
            let mut turn_count = (o.turn_number as i64 - 10).bitand(0b1111);
            if o.turn_number >= 20 {
                turn_count += over_20;
            }
            // shift the turn count to be left of the heart info
            // 0b1111 <- heart info
            //     0b11111 <- turn count
            out + (turn_count << 4)
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

pub struct PetsImage<'a> {
    pub view: SubImage<&'a DynamicImage>,
    pub heart_data: HeartData,
}

impl<'a> PetsImage<'a> {
    pub fn new(probpets: &'a ProbablyPetsImage<'a>) -> Option<Self> {
        let heart_data = handle_heart_check(probpets)?;
        Some(Self { view: probpets.view, heart_data })
    }
    pub fn get_sub_view(&self, view: (u32, u32, u32, u32)) -> SubImage<&DynamicImage> {
        self.view.view(view.0, view.1, view.2, view.3)
    }
    pub fn get_turn_number_view(&self) -> SubImage<&DynamicImage> {
        self.get_sub_view((1860, 790, 66, 49))
    }
    pub fn get_digits(&self) -> Option<(DynamicImage, DynamicImage)> {
        let view = self.get_turn_number_view();
        let (left, right) = extract_digit_clips(&view)?;
        let mut first_digit = left.to_image();
        let mut second_digit = right.to_image();
        first_digit.pixels_mut().for_each(|px| {
            if px.0 != WHITE {
                px.0 = BLACK;
            }
        });
        second_digit.pixels_mut().for_each(|px| {
            if px.0 != WHITE {
                px.0 = BLACK;
            }
        });
        let first_digit = image::imageops::resize(&first_digit, 16, 16, Nearest);
        let second_digit = image::imageops::resize(&second_digit, 16, 16, Nearest);
        let first_img = DynamicImage::ImageRgba8(first_digit);
        let second_img = DynamicImage::ImageRgba8(second_digit);
        Some((first_img, second_img))
    }
    pub fn get_digits_vectors_from_two_dyn_images(images: (DynamicImage, DynamicImage)) -> ([u8; 256], [u8; 256]) {
        let (left, right) = images;
        let mut first_digit_vector: [u8; 256] = [0; 256];
        for (i, (_, _, p)) in left.pixels().enumerate() {
            if p.0 == WHITE {
                first_digit_vector[i] = 1;
            }
        }
        let mut second_digit_vector: [u8; 256] = [0; 256];
        for (i, (_, _, p)) in right.pixels().enumerate() {
            if p.0 == WHITE {
                second_digit_vector[i] = 1;
            }
        }
        (first_digit_vector, second_digit_vector)
    }
    pub fn get_digits_vectors(&self) -> Option<([u8; 256], [u8; 256])> {
        self.get_digits().map(|(left, right)| {
            Self::get_digits_vectors_from_two_dyn_images((left, right))
        })
    }
    pub fn get_turn_number(&self) -> u8 {
        let (left, right) = match self.get_digits_vectors() {
            Some(x) => x,
            None => return 0,
        };
        let base = get_digit(&left);
        let second_digit = get_digit(&right);
        return base * 10 + second_digit;
    }
}

pub fn get_digit(digit: &[u8; 256]) -> u8 {
    let breakdown = get_digit_similarity(digit);
    let mut lowest_value = 9999.0;
    let mut lowest_index = 11;
    for (i, b) in breakdown.into_iter().enumerate() {
        if b < lowest_value {
            lowest_value = b;
            lowest_index = i as u8;
        }
    }
    lowest_index
}

/// returns an array of 10 floats, each representing the distance between
/// the current digit and the digit of the index.
/// for example [1,1,1,1,1,0,1,1,1,1]
/// would imply the current digit has a distance of 0 from digit 5 (index 5)
/// and a distance of 1 for the rest. this implies the current digit is most likely a 5
pub fn get_digit_similarity(current_digit: &[u8; 256]) -> [f64; 10] {
    [
        VEC_0,
        VEC_1,
        VEC_2,
        VEC_3,
        VEC_4,
        VEC_5,
        VEC_6,
        VEC_7,
        VEC_8,
        VEC_9
    ].map(|x| vector_distance(current_digit, &x))
}

pub fn extract_digit_zone<'a>(view: &'a SubImage<&'a DynamicImage>, start_x: u32) -> Option<(u32, SubImage<&'a DynamicImage>)> {
    // setup mapping function to treat anything non white as black
    let get_px = |x: u32, y: u32| {
        let mut px = view.get_pixel(x, y).0;
        if px != WHITE {
            px = BLACK;
        }
        px
    };
    // find the digit start by finding the first column that is not 100% black.
    let width = view.width();
    let height = view.height();
    let mut digit_start_x = None;

    'outter: for x in start_x..width {
        for y in 0..height {
            if get_px(x, y) == WHITE {
                digit_start_x = Some(x);
                break 'outter;
            }
        }
    }
    let digit_start = digit_start_x?;
    let mut digit_end = None;
    // find the end of the digit, defined as first column that is all black
    for x in digit_start..width {
        let mut column_all_black = true;
        for y in 0..height {
            if get_px(x, y) == WHITE {
                column_all_black = false;
                break;
            }
        }
        if column_all_black {
            digit_end = Some(x);
            break;
        }
    }
    let digit_end = digit_end?;
    let v = view.view(digit_start, 0, digit_end - digit_start, height);
    Some((digit_end, v))
}

pub fn extract_digit_clips<'a>(view: &'a SubImage<&'a DynamicImage>) -> Option<(SubImage<&'a DynamicImage>, SubImage<&'a DynamicImage>)> {
    let (next_digit_start_x, first_digit_view) = extract_digit_zone(&view, 0)?;
    let (_, second_digit_view) = extract_digit_zone(&view, next_digit_start_x)?;

    Some((first_digit_view, second_digit_view))
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

    fn get_pets_img(filename: &str, t: impl Fn(PetsImage)) {
        let data = std::fs::read(format!("./testfixtures/{}", filename)).expect("failed to read test fixture");
        let img = image::load_from_memory(&data).expect("failed to load test fixture img");
        let petsimg = probable_pets_image_check(&img).expect("it should be a pets img");
        let petsimg = PetsImage::new(&petsimg).expect("failed to get pets img");
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
