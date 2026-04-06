use mnist::{Mnist, MnistBuilder};
use ndarray::prelude::{Array3, s};

fn main() {
    let Mnist {
        trn_img,
        // trn_lbl,
        // tst_img,
        // tst_lbl,
        ..
    } = MnistBuilder::new()
        .training_images_filename("archive/train-images.idx3-ubyte")
        .training_labels_filename("archive/train-labels.idx1-ubyte")
        .test_images_filename("archive/t10k-images.idx3-ubyte")
        .test_labels_filename("archive/t10k-labels.idx1-ubyte")
        .label_format_digit()
        .training_set_length(50_000)
        .test_set_length(10_000)
        .finalize();

   let train_data = Array3::from_shape_vec((50_000, 28, 28), trn_img)
       .expect("Error converting images to Array3 struct");
   println!("{:#?}\n", train_data.slice(s![0, .., ..]));
}
