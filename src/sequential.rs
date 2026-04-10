use priority_queue::PriorityQueue;
use shared::{Image, calculate_distance_between_images};

pub fn find_closest_images(k: u32, image: &Image, train_images: Vec<Image>) -> Vec<u8> {
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

pub fn predict_image_category(k: u32, image: &Image, train_images: Vec<Image>) -> u8 {

}
