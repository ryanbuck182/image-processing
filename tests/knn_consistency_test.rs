use image_processing::parallel_1::predict_image_categories_parallel;
use image_processing::parallel_2::predict_image_categories_parallel_2;
use image_processing::sequential::predict_image_categories;
use image_processing::shared::{Image, IMAGE_SIDE_SIZE};
use ndarray::Array2;

fn uniform_image(label: u8, fill: u8) -> Image {
    Image {
        label,
        data: Array2::from_elem((IMAGE_SIDE_SIZE, IMAGE_SIDE_SIZE), fill),
    }
}

fn synthetic_dataset() -> (Vec<Image>, Vec<Image>) {
    // For each label 0..10, create 3 training exemplars at pixel value `label * 20`.
    // Test images match the same pattern — nearest neighbors should share the label.
    let mut train = Vec::new();
    for label in 0u8..10 {
        for _ in 0..3 {
            train.push(uniform_image(label, label.saturating_mul(20)));
        }
    }
    let mut test = Vec::new();
    for label in 0u8..10 {
        test.push(uniform_image(label, label.saturating_mul(20)));
    }
    (train, test)
}

#[test]
fn sequential_predicts_correctly_on_synthetic_data() {
    let (train, test) = synthetic_dataset();
    let preds = predict_image_categories(3, &test, &train);
    assert_eq!(preds.len(), test.len());
    for (i, pred) in preds.iter().enumerate() {
        assert_eq!(*pred, i as u8, "test image {} misclassified", i);
    }
}

#[test]
fn parallel_1_matches_sequential() {
    let (train, test) = synthetic_dataset();
    let seq = predict_image_categories(3, &test, &train);
    let par = predict_image_categories_parallel(3, &test, &train);
    assert_eq!(seq, par);
}

#[test]
fn parallel_2_matches_sequential() {
    let (train, test) = synthetic_dataset();
    let seq = predict_image_categories(3, &test, &train);
    let par = predict_image_categories_parallel_2(3, &test, &train);
    assert_eq!(seq, par);
}

#[test]
fn k_equals_one_returns_nearest_label() {
    // Train set has a unique nearest neighbor per test image.
    let train = vec![
        uniform_image(0, 0),
        uniform_image(1, 80),
        uniform_image(2, 160),
        uniform_image(3, 240),
    ];
    let test = vec![
        uniform_image(0, 2),   // closest to label 0
        uniform_image(1, 78),  // closest to label 1
        uniform_image(2, 158), // closest to label 2
        uniform_image(3, 238), // closest to label 3
    ];
    let preds = predict_image_categories(1, &test, &train);
    assert_eq!(preds, vec![0, 1, 2, 3]);

    let par1 = predict_image_categories_parallel(1, &test, &train);
    let par2 = predict_image_categories_parallel_2(1, &test, &train);
    assert_eq!(par1, preds);
    assert_eq!(par2, preds);
}
