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
