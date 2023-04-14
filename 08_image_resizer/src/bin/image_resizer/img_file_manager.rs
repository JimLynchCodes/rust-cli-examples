use image::imageops::FilterType;
use image::DynamicImage;
use std::error::Error;

#[cfg(not(test))]
pub fn read_image_file() -> DynamicImage {
    image::open("input_image/spacy_jim.png").expect("could not read image file")
}

#[cfg(test)]
pub fn read_image_file() -> DynamicImage {
    DynamicImage::new_rgb8(0, 0)
}

pub fn resize_image(
    mut img: &DynamicImage,
    new_width: u32,
    new_height: u32,
    filter_type: FilterType,
) -> DynamicImage {
    img.resize(new_width, new_height, FilterType::Lanczos3)
}

#[cfg(not(test))]
pub fn save_image_file(
    img: &DynamicImage,
    file_output_name: &str,
    multiplier_string: &str,
) -> Result<(), Box<dyn Error>> {
    Ok(img.save(format!(
        "output_images/{}_{}x.png",
        file_output_name, multiplier_string
    ))?)
}

#[cfg(test)]
pub fn save_image_file(
    img: &DynamicImage,
    file_output_name: &str,
    multiplier_string: &str,
) -> Result<(), Box<dyn Error>> {
    use img_file_manager_tests::saved_images;

    unsafe {
        saved_images.push(format!("{}_{}", file_output_name, multiplier_string));
    }

    Ok(())
}

#[cfg(test)]
pub mod img_file_manager_tests {
    pub static mut saved_images: Vec<String> = Vec::new();
}
