use crate::shared::{Image, calculate_distance_between_images, predict_image_category};
use priority_queue::PriorityQueue;

pub fn predict_image_categories(
    k: usize,
    images: &Vec<Image>,
    train_images: &Vec<Image>,
) -> Vec<u8> {
    let mut predicted_labels = Vec::with_capacity(images.len());
    for image in images {
        let predicted_label = predict_image_category(k, &image, &train_images, find_closest_images);
        predicted_labels.push(predicted_label);
        // println!("Image {} - Predicted: {}, Actual: {}", i, predicted_label, image.label);
    }
    predicted_labels
}

fn find_closest_images(k: usize, image: &Image, train_images: &Vec<Image>) -> Vec<u8> {
    let mut pq = PriorityQueue::with_capacity(k as usize);

    for (i, train_image) in train_images.iter().enumerate() {
        let distance = calculate_distance_between_images(image, &train_image);
        if pq.len() < k {
            pq.push(i, distance);
        } else if *pq.peek().unwrap().1 > distance {
            pq.pop();
            pq.push(i, distance);
        }
    }

    let mut closest_labels = Vec::new();
    for (index, _) in pq.into_iter() {
        closest_labels.push(train_images[index].label);
    }
    closest_labels
}
