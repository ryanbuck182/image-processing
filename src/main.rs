mod shared;
mod sequential;
mod parallel_1;
mod parallel_2;

use shared::{load_dataset};
use sequential::{predict_image_categories};
use parallel_1::{predict_image_categories_parallel};
use parallel_2::{predict_image_categories_parallel_2};
use std::time::Instant;
use num_cpus;

fn main() {
    // test_sequential(3, 0);

    // let (train_images, test_images) = load_dataset();
    // let predicted_labels = predict_image_categories_parallel(3, &test_images, &train_images);

    // let ( accurate_predictions, accuracy ) = shared::calculate_accuracy(&predicted_labels, &test_images);
    // println!("Accurate Predictions: {}/{}", accurate_predictions, test_images.len());
    // println!("Accuracy: {}", accuracy);

    // for i in 0..TRAINING_SET_SIZE {
    //     println!("{:#?}\n", train_images[i].data);
    //     println!("Label: {:#?}", train_images[i].label);
    // }
    full_test(3);
}

// fn test_sequential(k: usize, test_img_index: usize) {
//     let (train_images, test_images) = load_dataset();

//     let test_image = &test_images[test_img_index];
//     let result = predict_image_category(k, test_image, &train_images);

//     println!("Predicted: {}", result);
//     println!("Actual: {}", test_image.label);
//     println!("{:#?}\n", test_image.data);
// }

fn full_test(k: usize) {
    let (train_images, test_images) = load_dataset();

    // let start_time_sequential = Instant::now();
    // let predicted_labels_sequential = predict_image_categories(k, &test_images, &train_images);
    // let duration_sequential = start_time_sequential.elapsed();

    // let start_time_parallel = Instant::now();
    // let predicted_labels_parallel = predict_image_categories_parallel(k, &test_images, &train_images);
    // let duration_parallel = start_time_parallel.elapsed();

    let predicted_labels_parallel_2 = predict_image_categories_parallel_2(k, &test_images, &train_images);

    // let speedup = duration_sequential.as_secs_f64() / duration_parallel.as_secs_f64();
    // let efficiency = speedup as f64 / num_cpus::get() as f64;
    // let sequential_throughput = test_images.len() as f64 / duration_sequential.as_secs_f64();
    // let parallel_throughput = test_images.len() as f64 / duration_parallel.as_secs_f64();

    // let ( accurate_predictions_sequential, accuracy_sequential ) = shared::calculate_accuracy(&predicted_labels_sequential, &test_images);
    // let ( accurate_predictions_parallel, accuracy_parallel ) = shared::calculate_accuracy(&predicted_labels_parallel, &test_images);
    // let ( accurate_predictions_parallel_2, accuracy_parallel_2 ) = shared::calculate_accuracy(&predicted_labels_parallel_2, &test_images);
    // println!("Accurate Predictions Sequential: {}/{}", accurate_predictions_sequential, test_images.len());
    // println!("Accurate Predictions Parallel: {}/{}", accurate_predictions_parallel, test_images.len());
    // println!("Accuracy Sequential: {}", accuracy_sequential);
    // println!("Accuracy Parallel: {}", accuracy_parallel);
    // println!("Time Sequential: {:?}", duration_sequential);
    // println!("Time Parallel: {:?}", duration_parallel);

    // println!("Speedup: {:.2}", speedup);
    // println!("Efficiency: {:.2}", efficiency);
    // println!("Sequential Throughput: {:.2} images/sec", sequential_throughput);
    // println!("Parallel Throughput: {:.2} images/sec", parallel_throughput);
}
