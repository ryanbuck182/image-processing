use priority_queue::PriorityQueue;
use crate::shared::{Image, calculate_distance_between_images, predict_image_category};
use threadpool::ThreadPool;
use num_cpus;
use std::sync::mpsc::channel;

pub fn predict_image_categories_parallel_2(k: usize, images: &Vec<Image>, train_images: &Vec<Image>) -> Vec<u8> {
    let mut predicted_labels = Vec::with_capacity(images.len());
    for (i, image) in images.iter().enumerate() {
        let predicted_label = predict_image_category(k, &image, &train_images, find_closest_images_parallel_2);
        predicted_labels.push(predicted_label);
        println!("Image {} - Predicted: {}, Actual: {}", i, predicted_label, image.label);
    }
    predicted_labels
}

fn find_closest_images_parallel_2(k: usize, image: &Image, train_images: &Vec<Image>) -> Vec<u8> {
    let mut pq = PriorityQueue::with_capacity(k as usize);

    let n_workers = num_cpus::get();
    let n_jobs = train_images.len();
    let pool = ThreadPool::new(n_workers);

    let (tx, rx) = channel();
    for i in 0..n_jobs {
        let tx = tx.clone();
        let j = i.clone();
        let image_clone = image.clone();
        let training_image = train_images[i].clone();
        pool.execute(move || {
            let distance = calculate_distance_between_images(&image_clone, &training_image);
            tx.send((j, distance)).expect("channel will be there waiting for the pool");
        });
    }

    let results: Vec<(usize, u32)> = rx.iter().take(n_jobs).collect();

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
