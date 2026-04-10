mod shared;
use shared::{
    MNIST_TEST_IMAGES, MNIST_TEST_LABELS, MNIST_TRAINING_IMAGES,
    MNIST_TRAINING_LABELS, TEST_SET_SIZE, TRAINING_SET_SIZE
};

use mnist::{Mnist, MnistBuilder};
use ndarray::prelude::s;

fn main() {
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
    
    let train_images = shared::load_training_images(trn_img);
    let train_labels = trn_lbl;

    let test_images = shared::load_test_images(tst_img);
    let test_labels = tst_lbl;

    for i in 0..TRAINING_SET_SIZE {
        println!("{:#?}\n", train_images.slice(s![i, .., ..]));
        println!("Label: {:#?}", train_labels[i]);
    }
}
