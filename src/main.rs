use std::{error::Error, ffi::OsStr, fs::read_dir, path::{Path, PathBuf}};
use image::{imageops::FilterType::Triangle, EncodableLayout, ImageReader, DynamicImage::ImageLuma8};
use rayon::prelude::*;

const IMG_EXT: &[&str] = &["png", "img", "jpg", "webp", "jpeg"];

fn collect_images(dir: &Path, paths: &mut Vec<PathBuf>) -> Result<(), Box<dyn Error>> {
    for i in read_dir(dir)? {
        let i = i?;
        let path = i.path();

        if path.is_dir() {
            collect_images(&path, paths)?;
        } else {
            if let Some(ext) = path.extension().and_then(OsStr::to_str) {
                if IMG_EXT.contains(&ext) {
                    paths.push(path);
                }
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let sample_image = ImageReader::open("sample.png")?
        .with_guessed_format()?
        .decode()?
        .to_luma8();
    let sample_dimensions = sample_image.dimensions();
    let sample_raw = sample_image.as_bytes();
    let n = (sample_dimensions.0 * sample_dimensions.1) as usize;

    let mut paths: Vec<PathBuf> = Vec::new();
    collect_images(Path::new("./images"), &mut paths)?;

    let (best_path, best_score) = paths.par_iter().filter_map(|path| {
        let mut image = ImageReader::open(&path).ok()?
            .with_guessed_format().ok()?
            .decode().ok()?
            .resize_exact(sample_dimensions.0, sample_dimensions.1, Triangle);
        image = ImageLuma8(image.to_luma8());
        let raw_image = image.as_bytes();

        let mse = raw_image.iter()
            .zip(sample_raw.iter())
            .map(|(a, b)| {
                let c = *b as f64 - *a as f64;
                c * c
            })
            .sum::<f64>() / (n as f64);

        Some((path.clone(), mse))
    })
    .min_by(|a, b| a.1.total_cmp(&b.1))
    .unwrap();

    println!("the closest image is: {:?}, with score: {:?}", best_path, best_score);

    Ok(())
}
