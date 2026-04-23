use std::collections::HashMap;

use priority_queue::PriorityQueue;
use crate::shared::{Image, calculate_distance_between_images, predict_image_category};
use rayon::prelude::*;


pub fn predict_image_categories_parallel(k: usize, images: &Vec<Image>, train_images: &Vec<Image>) -> Vec<u8> {
    let mut predicted_labels = Vec::with_capacity(images.len());
    let mut accuracy_per_image: Vec<(u8, u8)> = Vec::new();
    
    for (i, image) in images.iter().enumerate() {
        let predicted_label = predict_image_category(k, &image, &train_images, find_closest_images_parallel);
        predicted_labels.push(predicted_label);
        println!("Image {} - Predicted: {}, Actual: {}", i, predicted_label, image.label);
        accuracy_per_image.push((image.label, predicted_label));
    }

    let mut correct_counts = [0u32; 10];
    let mut total_counts = [0u32; 10];
    for (actual, predicted) in &accuracy_per_image {
        total_counts[*actual as usize] += 1;
        if actual == predicted {
            correct_counts[*actual as usize] += 1;
        }
    }
    println!("\nAccuracy per label:");
    for label in 0..10 {
        let total = total_counts[label];
        let correct = correct_counts[label];
        let pct = if total > 0 { correct as f64 / total as f64 * 100.0 } else { 0.0 };
        println!("  Label {}: {}/{} ({:.1}%)", label, correct, total, pct);
    }
    
    predicted_labels
}

fn find_closest_images_parallel(k: usize, image: &Image, train_images: &Vec<Image>) -> Vec<u8> {
    let mut pq = PriorityQueue::with_capacity(k as usize);
    let total = train_images.len();

    let results: Vec<(usize, u32)> = (0..total)
        .into_par_iter()
        .map(|i| {
            let distance = calculate_distance_between_images(image, &train_images[i]);
            (i, distance)
        })
        .collect();

    for (i, distance) in results {
        pq.push(i, distance);
        if pq.len() > k {
            pq.pop();
        }
    }

    let mut closest_labels = Vec::new();
    for (index, _) in pq.into_iter() {
        closest_labels.push(train_images[index].label);
    }
    closest_labels
}
