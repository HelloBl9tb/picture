use pixel::pixelation::*;
use clap::*;
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Img {
    // #[arg(short, long, default_value = r"C:\Project\Image\1.png")]
    img: String,
}

fn main() {
   
    let string = Img::parse();
    let img_in = image::open(string.img).unwrap();
    let vec_colors = dominant_colors(img_in.clone());
    let img_out = pixelation(img_in.clone(), vec_colors, 10);
    img_out.save(r"C:\Project\Image\out_1.png").unwrap();
}
