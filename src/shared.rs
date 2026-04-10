use mnist::{Mnist, MnistBuilder};
use ndarray::prelude::*;

pub const MNIST_TRAINING_IMAGES: &str = "train-images.idx3-ubyte";
pub const MNIST_TRAINING_LABELS: &str = "train-labels.idx1-ubyte";
pub const MNIST_TEST_IMAGES: &str = "t10k-images.idx3-ubyte";
pub const MNIST_TEST_LABELS: &str = "t10k-labels.idx1-ubyte";

pub const TRAINING_SET_SIZE: usize = 60_000;
pub const TEST_SET_SIZE: usize = 10_000;
pub const IMAGE_SIDE_SIZE: usize = 28;

pub struct Image {
    pub label: u8,
    pub data: Array2<u8>,
}

pub fn calculate_accuracy(predicted_labels: &Vec<u8>, test_images: &Vec<Image>) -> (usize, f64) {
    let actual_labels: Vec<u8> = test_images.iter().map(|img| img.label).collect();

    let actual_predicted_iter = predicted_labels.iter().zip(actual_labels.iter());
    let accurate_predictions = actual_predicted_iter.filter(|(predicted, actual)| predicted == actual).count();
    let accuracy = accurate_predictions as f64 / predicted_labels.len() as f64;

    ( accurate_predictions, accuracy )
}

pub fn calculate_distance_between_images(img1: &Image, img2: &Image) -> u32 {
    let mut distance: u32 = 0;
    for i in 0..IMAGE_SIDE_SIZE {
        for j in 0..IMAGE_SIDE_SIZE {
            distance +=
                calculate_distance_between_pixels(img1.data[[i, j]], img2.data[[i, j]]) as u32;
        }
    }
    distance
}

fn calculate_distance_between_pixels(px1: u8, px2: u8) -> u8 {
    px1.abs_diff(px2)
}

pub fn load_dataset() -> (Vec<Image>, Vec<Image>) {
    let (trn_img, trn_lbl, tst_img, tst_lbl) = load_raw_data();
    parse_data(trn_img, trn_lbl, tst_img, tst_lbl)
}

fn load_raw_data() -> (Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>) {
    let Mnist {
        trn_img,
        trn_lbl,
        tst_img,
        tst_lbl,
        ..
    } = MnistBuilder::new()
        .training_images_filename(MNIST_TRAINING_IMAGES)
        .training_labels_filename(MNIST_TRAINING_LABELS)
        .test_images_filename(MNIST_TEST_IMAGES)
        .test_labels_filename(MNIST_TEST_LABELS)
        .label_format_digit()
        .training_set_length(TRAINING_SET_SIZE as u32)
        .test_set_length(TEST_SET_SIZE as u32)
        .finalize();
    (trn_img, trn_lbl, tst_img, tst_lbl)
}

fn parse_data(
    trn_img: Vec<u8>,
    trn_lbl: Vec<u8>,
    tst_img: Vec<u8>,
    tst_lbl: Vec<u8>,
) -> (Vec<Image>, Vec<Image>) {
    let train_images = load_images(trn_img, trn_lbl, TRAINING_SET_SIZE);
    let test_images = load_images(tst_img, tst_lbl, TEST_SET_SIZE);

    (train_images, test_images)
}

fn load_images(imgs: Vec<u8>, labels: Vec<u8>, set_size: usize) -> Vec<Image> {
    let array3d = Array3::from_shape_vec((set_size, IMAGE_SIDE_SIZE, IMAGE_SIDE_SIZE), imgs)
        .expect("Error converting images to Array3 struct");

    let mut images: Vec<Image> = Vec::new();
    for i in 0..set_size {
        images.push(Image {
            label: labels[i],
            data: array3d.index_axis(Axis(0), i).to_owned(),
        });
    }
    images
}
