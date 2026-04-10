use priority_queue::PriorityQueue;
use ndarray::{prelude::{Array3, Array2, s}};

pub const MNIST_TRAINING_IMAGES: &str = "archive/train-images.idx3-ubyte";
pub const MNIST_TRAINING_LABELS: &str = "archive/train-labels.idx1-ubyte";
pub const MNIST_TEST_IMAGES: &str = "archive/t10k-images.idx3-ubyte";
pub const MNIST_TEST_LABELS: &str = "archive/t10k-labels.idx1-ubyte";

pub const TRAINING_SET_SIZE: usize = 60_000;
pub const TEST_SET_SIZE: usize = 10_000;
pub const IMAGE_SIDE_SIZE: usize = 28;

pub struct Image {
    label: String,
    data: Array2<u8>,
}

pub fn calculate_distance_between_pixels(px1: u8, px2: u8) -> u8 {
    px1.abs_diff(px2)
}

pub fn calculate_distance_between_images(img1: &Image, img2: &Image) -> u8 {

}

pub fn find_closest_images(k: i32, pq: &PriorityQueue) {

}

pub fn predict_image_category(image: &Image) {

}

pub fn load_training_images(data: Vec<u8>) -> Array3<u8> {
    Array3::from_shape_vec((TRAINING_SET_SIZE, IMAGE_SIDE_SIZE, IMAGE_SIDE_SIZE), data)
        .expect("Error converting images to Array3 struct")
}

pub fn load_test_images(data: Vec<u8>) -> Array3<u8> {
    Array3::from_shape_vec((TEST_SET_SIZE, IMAGE_SIDE_SIZE, IMAGE_SIDE_SIZE), data)
        .expect("Error converting images to Array3 struct")
}
