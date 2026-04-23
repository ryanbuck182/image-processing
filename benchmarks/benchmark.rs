use crate::shared::{self, load_dataset};
use crate::sequential::predict_image_categories;
use crate::parallel_1::predict_image_categories_parallel;
use crate::parallel_2::predict_image_categories_parallel_2;
use crate::shared::Image;
use std::time::Instant;
use std::time::Duration;
use num_cpus;

type PredictFn = fn(usize, &Vec<Image>, &Vec<Image>) -> Vec<u8>;
type BenchmarkResults = (Duration, usize, f64, [u32; 10], [u32; 10]);

const NUM_TEST_IMAGES: u32 = 10000;

pub fn full_benchmark(k: usize) {
    let (duration_sequential, accurate_predictions, accuracy, correct_counts, total_counts) = run_sequential_benchmark(k);
    let (duration_parallel, _, _, _, _) = run_parallel_benchmark(k);
    let (duration_parallel_2, _, _, _, _) = run_parallel_2_benchmark(k);

    // parallel speedup
    let speedup = duration_sequential.as_secs_f64() / duration_parallel.as_secs_f64();   
    let efficiency = speedup as f64 / num_cpus::get() as f64;

    // parallel_2 speedup
    let speedup_2 = duration_sequential.as_secs_f64() / duration_parallel_2.as_secs_f64();
    let efficiency_2 = speedup_2 as f64 / num_cpus::get() as f64;

    // throughput
    let sequential_throughput = NUM_TEST_IMAGES as f64 / duration_sequential.as_secs_f64();
    let parallel_throughput = NUM_TEST_IMAGES as f64 / duration_parallel.as_secs_f64();
    let parallel_2_throughput = NUM_TEST_IMAGES as f64 / duration_parallel_2.as_secs_f64();

    // print results
    
    // accuracy is the same for all methods
    println!("Accurate Predictions: {}/{}", accurate_predictions, NUM_TEST_IMAGES);
    println!("Accuracy: {}", accuracy);
    
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

    println!("\nAccuracy per label:");
    for label in 0..10 {
        let total = total_counts[label];
        let correct = correct_counts[label];
        let pct = if total > 0 { correct as f64 / total as f64 * 100.0 } else { 0.0 };
        println!("  Label {}: {}/{} ({:.1}%)", label, correct, total, pct);
    }
}

pub fn sequential_benchmark(k: usize) {
    let (duration, accurate_predictions, accuracy, correct_counts, total_counts) = run_sequential_benchmark(k);

    println!("Sequential");
    println!("Accurate Predictions: {}/{}", accurate_predictions, NUM_TEST_IMAGES);
    println!("Accuracy: {}", accuracy);
    println!("Time: {:?}", duration);
    print_accuracy_per_label(correct_counts, total_counts);
}

pub fn parallel_benchmark(k: usize) {
    let (duration, accurate_predictions, accuracy, correct_counts, total_counts) = run_parallel_benchmark(k);

    println!("Parallel 1");
    println!("Accurate Predictions: {}/{}", accurate_predictions, NUM_TEST_IMAGES);
    println!("Accuracy: {}", accuracy);
    println!("Time: {:?}", duration);
    print_accuracy_per_label(correct_counts, total_counts);
}

pub fn parallel_2_benchmark(k: usize) {
    let (duration, accurate_predictions, accuracy, correct_counts, total_counts) = run_parallel_2_benchmark(k);

    println!("Parallel 2");
    println!("Accurate Predictions: {}/{}", accurate_predictions, NUM_TEST_IMAGES);
    println!("Accuracy: {}", accuracy);
    println!("Time: {:?}", duration);
    print_accuracy_per_label(correct_counts, total_counts);
}

fn print_accuracy_per_label(correct_counts: [u32; 10], total_counts: [u32; 10]) {
    println!("\nAccuracy per label:");
    for label in 0..10 {
        let total = total_counts[label];
        let correct = correct_counts[label];
        let pct = if total > 0 { correct as f64 / total as f64 * 100.0 } else { 0.0 };
        println!("  Label {}: {}/{} ({:.1}%)", label, correct, total, pct);
    }
}

fn benchmark(k: usize, predict: PredictFn) -> BenchmarkResults {
    let (train_images, test_images) = load_dataset();

    let start_time = Instant::now();
    println!("Predicting...");
    let predicted_labels = predict(k, &test_images, &train_images);
    let duration = start_time.elapsed();

    let ( accurate_predictions, accuracy ) = shared::calculate_accuracy(&predicted_labels, &test_images);
    let ( correct_counts, total_counts ) = calculate_accuracy_per_label(&test_images, &predicted_labels);

    return (duration, accurate_predictions, accuracy, correct_counts, total_counts)
}

fn run_sequential_benchmark(k: usize) -> BenchmarkResults {
    return benchmark(k, predict_image_categories)
}

fn run_parallel_benchmark(k: usize) -> BenchmarkResults {
    return benchmark(k, predict_image_categories_parallel)
}

fn run_parallel_2_benchmark(k: usize) -> BenchmarkResults {
    return benchmark(k, predict_image_categories_parallel_2)
}

fn calculate_accuracy_per_label(test_images: &Vec<Image>, predicted_labels: &Vec<u8>) -> ([u32; 10], [u32; 10]) {    
    let actual_labels: Vec<u8> = test_images.iter().map(|img| img.label).collect();
    
    let mut correct_counts = [0u32; 10];
    let mut total_counts = [0u32; 10];
    for (actual, predicted) in actual_labels.iter().zip(predicted_labels.iter()) {
        total_counts[*actual as usize] += 1;
        if actual == predicted {
            correct_counts[*actual as usize] += 1;
        }
    }

    return (correct_counts, total_counts);
}
