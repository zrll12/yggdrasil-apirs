use std::io::Cursor;
use std::path::PathBuf;

use base64::Engine;
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use image::{DynamicImage, EncodableLayout};
use image::codecs::png::{PngDecoder, PngEncoder};
use image::io::{Limits, Reader as ImageReader};
use image::ImageDecoder;
use image::ImageFormat::Png;
use sha2::{Digest, Sha256};
use tokio::fs::try_exists;
use crate::TEXTURE_CONFIG;

/// Write a file to disk and generate the id of the file
///
/// # Arguments
///
/// * `file_content`: The content of file
///
/// returns: Option<String>: Some(id) if the file is saved successfully, None if error occurs
pub async fn write_file(file_content: impl AsRef<[u8]>) -> Option<String> {
    let mut hasher = Sha256::new();
    hasher.update(file_content.as_ref());
    hasher.update(&file_content.as_ref().len().to_le_bytes());
    let hasher = hasher.finalize();

    let id = format!("{}", BASE64_URL_SAFE_NO_PAD.encode(hasher.as_bytes()));
    // let id = (&id[1..]).to_string();

    let mut path = PathBuf::from("./textures");
    path.push((&id[0..2]).to_string().to_ascii_lowercase());
    path.push((&id).to_string());
    
    // Recode the image
    let mut limit = Limits::default();
    limit.max_image_width = Some(TEXTURE_CONFIG.max_width);
    limit.max_image_height = Some(TEXTURE_CONFIG.max_height);
    let image = PngDecoder::with_limits(Cursor::new(file_content), limit).unwrap();
    let image = DynamicImage::from_decoder(image).unwrap();
    let mut image_bytes: Vec<u8> = Vec::new();
    image.write_to(&mut Cursor::new(&mut image_bytes), Png).unwrap();

    if try_exists(&path).await.unwrap() {
        return Some(id);
    }
    tokio::fs::create_dir_all(&path.parent().unwrap())
        .await
        .unwrap();
    tokio::fs::write(&path, image_bytes)
        .await
        .unwrap();

    Some(id)
}

/// Delete a file from disk
///
/// # Arguments
///
/// * `file_id`: The id of the file
///
/// returns: ()
async fn delete_file(file_id: &str) {
    let mut path = PathBuf::from("./textures");
    path.push((&file_id[0..2]).to_string().to_ascii_lowercase());
    path.push((&file_id).to_string());

    if try_exists(&path).await.unwrap() {
        tokio::fs::remove_file(&path).await.unwrap();
    }
}


/// Read a file from disk
///
/// # Arguments 
///
/// * `file_id`: The id of the file
///
/// returns: Option<DynamicImage>: Some(DynamicImage) if the file is read successfully, None if not exist or format not recognized
///   
/// # Examples 
///
/// ```
/// let file = read_image("test").unwrap();
/// ```
pub async fn read_image(file_id: &str) -> Option<DynamicImage> {
    let mut path = PathBuf::from("./textures");
    path.push((&file_id[0..2]).to_string().to_ascii_lowercase());
    path.push((&file_id).to_string());

    if !try_exists(&path).await.unwrap() {
        return None;
    }

    let file = ImageReader::open(path);
    if file.is_err() {
        return None;
    }
    let file = file.unwrap().with_guessed_format();
    if file.is_err() {
        return None;
    }
    let file = file.unwrap().decode();
    if file.is_err() {
        return None;
    }

    Some(file.unwrap())
}