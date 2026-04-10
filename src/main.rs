mod shared;
mod sequential;
mod parallel_1;
mod parallel_2;

use shared::{TRAINING_SET_SIZE, load_dataset};
use sequential::predict_image_category;

fn main() {
    test_sequential(3, 0);

    // for i in 0..TRAINING_SET_SIZE {
    //     println!("{:#?}\n", train_images[i].data);
    //     println!("Label: {:#?}", train_images[i].label);
    // }
}

fn test_sequential(k: usize, test_img_index: usize) {
    let (train_images, test_images) = load_dataset();

    let test_image = &test_images[test_img_index];
    let result = predict_image_category(k, test_image, &train_images);

    println!("Predicted: {}", result);
    println!("Actual: {}", test_image.label);
    println!("{:#?}\n", test_image.data);
}
