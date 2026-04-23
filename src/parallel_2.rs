use priority_queue::PriorityQueue;
use crate::shared::{Image, calculate_distance_between_images, predict_image_category};
use threadpool::ThreadPool;
use num_cpus;
use std::sync::{mpsc::channel, Arc};

pub fn predict_image_categories_parallel_2(k: usize, images: &Vec<Image>, train_images: &Vec<Image>) -> Vec<u8> {
    let mut predicted_labels = Vec::with_capacity(images.len());
    let n_workers = num_cpus::get();
    let pool = ThreadPool::new(n_workers);
    for (i, image) in images.iter().enumerate() {
        let predicted_label = predict_image_category_parallel_2(k, &image, &train_images, &pool);
        predicted_labels.push(predicted_label);
        println!("Image {} - Predicted: {}, Actual: {}", i, predicted_label, image.label);
    }
    predicted_labels
}

fn predict_image_category_parallel_2(k: usize, image: &Image, train_images: &Vec<Image>, pool: &ThreadPool) -> u8 {
    let closest_labels = find_closest_images_parallel_2(k, image, &train_images, pool);

    let mut label_counts = [0; 10];
    for label in closest_labels {
        label_counts[label as usize] += 1;
    }

    let mut predicted_label = 0;
    for (label, count) in label_counts.iter().enumerate() {
        if *count > label_counts[predicted_label as usize] {
            predicted_label = label as u8;
        }
    }
    predicted_label
}

fn find_closest_images_parallel_2(k: usize, image: &Image, train_images: &Vec<Image>, pool: &ThreadPool) -> Vec<u8> {
    let mut pq = PriorityQueue::with_capacity(k as usize);

    let n_workers = num_cpus::get();
    let chunk_size = (train_images.len() + n_workers - 1) / n_workers;

    
    let image = Arc::new(image.clone());

    let (tx, rx) = channel();

    for (chunk_idx, chunk) in train_images.chunks(chunk_size).enumerate() {
        let tx = tx.clone();
        let image_ref = Arc::clone(&image);
        let start_idx = chunk_idx * chunk_size;
        let chunk = chunk.to_vec();
        
        pool.execute(move || {
            let mut loacl_results = Vec::with_capacity(chunk.len());

            for (i,training_image) in chunk.iter().enumerate() {
                let distance = calculate_distance_between_images(&image_ref, training_image);
                loacl_results.push((start_idx + i, distance));
            }
            tx.send(loacl_results).unwrap();
        });
    }
    
    drop(tx);

    let mut all_results = Vec::with_capacity(train_images.len());
    for chunk in rx {
        all_results.extend(chunk);
    }

    for (i, distance) in all_results {
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
