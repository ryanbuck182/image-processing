use image_processing::shared::{
    calculate_accuracy, calculate_distance_between_images, predict_image_category, Image,
    IMAGE_SIDE_SIZE,
};
use ndarray::Array2;

fn make_image(label: u8, fill: u8) -> Image {
    Image {
        label,
        data: Array2::from_elem((IMAGE_SIDE_SIZE, IMAGE_SIDE_SIZE), fill),
    }
}

#[test]
fn distance_between_identical_images_is_zero() {
    let a = make_image(3, 50);
    let b = make_image(7, 50);
    assert_eq!(calculate_distance_between_images(&a, &b), 0);
}

#[test]
fn distance_between_uniform_images_is_pixelwise_diff_times_area() {
    let a = make_image(0, 10);
    let b = make_image(0, 40);
    let expected = (IMAGE_SIDE_SIZE * IMAGE_SIDE_SIZE) as u32 * 30;
    assert_eq!(calculate_distance_between_images(&a, &b), expected);
}

#[test]
fn distance_is_symmetric() {
    let a = make_image(1, 12);
    let b = make_image(2, 200);
    assert_eq!(
        calculate_distance_between_images(&a, &b),
        calculate_distance_between_images(&b, &a)
    );
}

#[test]
fn distance_on_varied_pixels() {
    let mut a_data = Array2::<u8>::zeros((IMAGE_SIDE_SIZE, IMAGE_SIDE_SIZE));
    let mut b_data = Array2::<u8>::zeros((IMAGE_SIDE_SIZE, IMAGE_SIDE_SIZE));
    a_data[[0, 0]] = 100;
    b_data[[0, 0]] = 40;
    a_data[[5, 7]] = 10;
    b_data[[5, 7]] = 25;
    let a = Image { label: 0, data: a_data };
    let b = Image { label: 0, data: b_data };
    assert_eq!(calculate_distance_between_images(&a, &b), 60 + 15);
}

#[test]
fn accuracy_all_correct() {
    let imgs = vec![make_image(1, 0), make_image(2, 0), make_image(3, 0)];
    let preds = vec![1, 2, 3];
    let (correct, acc) = calculate_accuracy(&preds, &imgs);
    assert_eq!(correct, 3);
    assert!((acc - 1.0).abs() < 1e-9);
}

#[test]
fn accuracy_none_correct() {
    let imgs = vec![make_image(1, 0), make_image(2, 0)];
    let preds = vec![9, 9];
    let (correct, acc) = calculate_accuracy(&preds, &imgs);
    assert_eq!(correct, 0);
    assert!(acc.abs() < 1e-9);
}

#[test]
fn accuracy_partial() {
    let imgs = vec![
        make_image(1, 0),
        make_image(2, 0),
        make_image(3, 0),
        make_image(4, 0),
    ];
    let preds = vec![1, 2, 0, 0];
    let (correct, acc) = calculate_accuracy(&preds, &imgs);
    assert_eq!(correct, 2);
    assert!((acc - 0.5).abs() < 1e-9);
}

#[test]
fn predict_image_category_picks_majority_label() {
    fn stub(_k: usize, _img: &Image, _train: &Vec<Image>) -> Vec<u8> {
        vec![5, 5, 5, 2, 2]
    }
    let img = make_image(0, 0);
    let train = vec![make_image(0, 0)];
    assert_eq!(predict_image_category(5, &img, &train, stub), 5);
}

#[test]
fn predict_image_category_tie_prefers_lower_label() {
    // On a tie, the loop keeps the first-seen max, so label 0 wins over label 7.
    fn stub(_k: usize, _img: &Image, _train: &Vec<Image>) -> Vec<u8> {
        vec![0, 7]
    }
    let img = make_image(0, 0);
    let train = vec![make_image(0, 0)];
    assert_eq!(predict_image_category(2, &img, &train, stub), 0);
}
