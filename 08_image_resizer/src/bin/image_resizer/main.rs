use image::GenericImageView;

mod walker;
mod img_file_manager;
mod new_sizes_calculator;
use img_file_manager::read_image_file;
use new_sizes_calculator::calculate_new_sizes;
use walker::walk_and_save;

pub enum StepDirection {
    Up,
    Down,
}

static FILE_OUTPUT_NAME: &str = "spacy_jim";
static UP_STEP_MULTIPLIER: f64 = 2.0;
static DOWN_STEP_MULTIPLIER: f64 = 2.0;
static STEPS_IN_UP_DIRECTION: u32 = 3;
static STEPS_IN_DOWN_DIRECTION: u32 = 3;

fn main() {
    let img = read_image_file();

    let (original_width, original_height) = img.dimensions();

    let new_sizes_scaled_down = calculate_new_sizes(
        StepDirection::Down,
        STEPS_IN_DOWN_DIRECTION,
        DOWN_STEP_MULTIPLIER,
        original_width,
        original_height,
    );
    let new_sizes_scaled_up = calculate_new_sizes(
        StepDirection::Up,
        STEPS_IN_UP_DIRECTION,
        UP_STEP_MULTIPLIER,
        original_width,
        original_height,
    );

    walk_and_save(new_sizes_scaled_up, &FILE_OUTPUT_NAME, img.clone());
    walk_and_save(new_sizes_scaled_down, &FILE_OUTPUT_NAME, img);
}
