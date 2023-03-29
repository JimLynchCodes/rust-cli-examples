use image::imageops::FilterType;
use image::DynamicImage;
use image::GenericImageView;

use crate::img_file_manager::{resize_image, save_image_file};

pub fn walk_and_save(
    new_sizes: Vec<(f64, f64, String)>,
    file_output_name: &str,
    img: DynamicImage,
) {
    for (new_width, new_height, multiplier_string) in new_sizes {
        let new_img = resize_image(
            &img,
            new_width as u32,
            new_height as u32,
            FilterType::Lanczos3,
        );

        println!(
            "Saving w:{}, h:{}, f:{}, m:{}",
            new_width, new_height, &file_output_name, multiplier_string
        );

        save_image_file(new_img, &file_output_name, &multiplier_string);
    }
}

#[test]
pub fn walks_and_saves() {
    use crate::walker::walk_and_save;
    use crate::img_file_manager::img_file_manager_tests::SAVED_IMAGES;

    let mock_sizes = vec![
        (1.0, 2.0, "foo".to_string()),
        (1.1, 2.2, "bar".to_string()),
        (3.33, 4.44, "a".to_string()),
    ];

    let mock_output_file_name = "baz";

    walk_and_save(
        mock_sizes.clone(),
        mock_output_file_name,
        DynamicImage::new_rgb8(0, 0),
    );

    for (index, (mock_width, mock_height, mock_multiplier_string)) in mock_sizes.iter().enumerate()
    {
        unsafe {
            assert_eq!(
                format!("w:{},h:{},f:{},m:{}", 
                mock_width, mock_height, 
                mock_output_file_name, mock_multiplier_string),
                SAVED_IMAGES[index]
            );
        }
    }
}
