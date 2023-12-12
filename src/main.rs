extern crate pixel;
use pixel::pixelation::{dominant_colors, pixelation};
fn main() {
    let img_in = image::open(r"C:\Users\Юра\workSpace\project\picture\picture\1.png").unwrap();
    let vec_colors = dominant_colors(img_in.clone());

    let img_out = pixelation(img_in, vec_colors, 10);

    img_out.save("outputnew.jpg").unwrap();
}
