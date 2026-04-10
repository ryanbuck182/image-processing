mod shared;
use shared::{TRAINING_SET_SIZE, load_dataset};

fn main() {
    let (train_images, test_images) = load_dataset();

    for i in 0..TRAINING_SET_SIZE {
        println!("{:#?}\n", train_images[i].data);
        println!("Label: {:#?}", train_images[i].label);
    }
}
