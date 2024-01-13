use pixel::pixelation::*;
use clap::*;
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Img {
    // #[arg(short, long, default_value = r"C:\Project\Image\1.png")]
    img: String,
    format: String,
    coordinats: String,
}

fn main() {
   
    let string = Img::parse();
    let img_in = image::open(string.img.clone()).unwrap();
    let vec_colors = dominant_colors(img_in.clone());    
    let sqauare = generate_squares(10.0, image::open(string.img.clone()).unwrap());
    let line = line(img_in.clone(), 10);
    let img_out = paint_coordinats(sqauare.clone(), vec_colors.clone(), img_in.clone(), line.clone());
    img_out.save(r"C:\Project\Image\out_1.png").unwrap();

    let string_2 = format_color(sqauare.clone(),
     vec_colors.clone(), string.coordinats, string.format);
     
    println!("{:?}", string_2);
    
}
