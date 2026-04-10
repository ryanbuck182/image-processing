use priority_queue::PriorityQueue;
use ndarray::{prelude::{Array3, Array2, s}};

struct Image {
    name: String,
}

pub fn calculate_distance_between_pixels(px1: u8, px2: u8) -> u8 {
    px1.abs_diff(px2)
}

fn calculate_distance_between_images(img1: &Image, img2: &Image) -> i8 {

}

fn find_closest_images(k: i32, pq: &PriorityQueue) {

}

fn predict_image_category(image: &Image) {

}

pub fn load_training_images(data: Vec<u8>) -> Array3<u8> {
    Array3::from_shape_vec((60_000, 28, 28), data)
        .expect("Error converting images to Array3 struct")
}

pub fn load_test_images(data: Vec<u8>) -> Array3<u8> {
    Array3::from_shape_vec((10_000, 28, 28), data)
        .expect("Error converting images to Array3 struct")
}
