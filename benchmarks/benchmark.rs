use crate::shared::{self, load_dataset};
use crate::sequential::predict_image_categories;
use crate::parallel_1::predict_image_categories_parallel;
use crate::parallel_2::predict_image_categories_parallel_2;
use crate::shared::Image;
use std::time::Instant;
use std::time::Duration;
use num_cpus;

pub fn full_benchmark(k: usize) {
    let (train_images, test_images) = load_dataset();

    let (duration_sequential, accurate_predictions_sequential, accuracy_sequential) = sequential_benchmark(k, &test_images, &train_images);
    let (duration_parallel, accurate_predictions_parallel, accuracy_parallel) = parallel_benchmark(k, &test_images, &train_images);
    let (duration_parallel_2, accurate_predictions_parallel_2, accuracy_parallel_2) = parallel_2_benchmark(k, &test_images, &train_images);

    // parallel speedup
    let speedup = duration_sequential.as_secs_f64() / duration_parallel.as_secs_f64();   
    let efficiency = speedup as f64 / num_cpus::get() as f64;

    // parallel_2 speedup
    let speedup_2 = duration_sequential.as_secs_f64() / duration_parallel_2.as_secs_f64();
    let efficiency_2 = speedup_2 as f64 / num_cpus::get() as f64;

    // throughput
    let sequential_throughput = test_images.len() as f64 / duration_sequential.as_secs_f64();
    let parallel_throughput = test_images.len() as f64 / duration_parallel.as_secs_f64();
    let parallel_2_throughput = test_images.len() as f64 / duration_parallel_2.as_secs_f64();

    // print results
    println!("Accurate Predictions Sequential: {}/{}", accurate_predictions_sequential, test_images.len());
    println!("Accurate Predictions Parallel: {}/{}", accurate_predictions_parallel, test_images.len());
    println!("Accurate Predictions Parallel_2: {}/{}", accurate_predictions_parallel_2, test_images.len());
    println!("Accuracy Sequential: {}", accuracy_sequential);
    println!("Accuracy Parallel: {}", accuracy_parallel);
    println!("Accuracy Parallel_2: {}", accuracy_parallel_2);
    println!("Time Sequential: {:?}", duration_sequential);
    println!("Time Parallel: {:?}", duration_parallel);
    println!("Time Parallel_2: {:?}", duration_parallel_2);

    println!("Speedup: {:.2}", speedup);
    println!("Efficiency: {:.2}", efficiency);
    println!("Speedup Parallel_2: {:.2}", speedup_2);
    println!("Efficiency Parallel_2: {:.2}", efficiency_2);
    println!("Sequential Throughput: {:.2} images/sec", sequential_throughput);
    println!("Parallel Throughput: {:.2} images/sec", parallel_throughput);
    println!("Parallel_2 Throughput: {:.2} images/sec", parallel_2_throughput);
}

pub fn sequential_benchmark(k: usize, test_images: &Vec<Image>, train_images: &Vec<Image>) -> (Duration, usize, f64) {
    let start_time = Instant::now();
    let predicted_labels = predict_image_categories(k, test_images, train_images);
    let duration = start_time.elapsed();

    let ( accurate_predictions, accuracy ) = shared::calculate_accuracy(&predicted_labels, &test_images);

    return (duration, accurate_predictions, accuracy)
}

pub fn parallel_benchmark(k: usize, test_images: &Vec<Image>, train_images: &Vec<Image>) -> (Duration, usize, f64) {
    let start_time = Instant::now();
    let predicted_labels = predict_image_categories_parallel(k, test_images, train_images);
    let duration = start_time.elapsed();

    let ( accurate_predictions, accuracy ) = shared::calculate_accuracy(&predicted_labels, &test_images);

    return (duration, accurate_predictions, accuracy)
}

pub fn parallel_2_benchmark(k: usize, test_images: &Vec<Image>, train_images: &Vec<Image>) -> (Duration, usize, f64) {
    let start_time = Instant::now();
    let predicted_labels = predict_image_categories_parallel_2(k, test_images, train_images);
    let duration = start_time.elapsed();

    let ( accurate_predictions, accuracy ) = shared::calculate_accuracy(&predicted_labels, &test_images);

    return (duration, accurate_predictions, accuracy)
}
