use priority_queue::PriorityQueue;
use crate::shared::{Image, calculate_distance_between_images};

pub fn predict_image_category(k: u32, image: &Image, train_images: Vec<Image>) -> u8 {
    let closest_labels = find_closest_images(k, image, train_images);

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

fn find_closest_images(k: u32, image: &Image, train_images: Vec<Image>) -> Vec<u8> {
    let pq = PriorityQueue::with_capacity(k as usize);

    for train_image in train_images {
        let distance = calculate_distance_between_images(image, &train_image);
        if pq.len() < k as usize || *pq.peek().unwrap().1 > distance {
            pq.pop();
            pq.push(train_image.label, distance);
        }
    }

    let closest_labels = Vec::new();
    for (label, _) in pq.into_iter() {
        closest_labels.push(label);
    }
    closest_labels
}
