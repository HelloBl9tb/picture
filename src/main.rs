use pixel::pixelation::*;
use clap::*;
use egui::*;
#[derive(Parser,Debug)]
#[command(author, version, about, long_about = None)]
struct Img {
    #[arg(short = 'p', long = "path")]
    path: String,
    #[arg(short = 'f', long = "format")]
    format: String,
    #[arg(short = 'c', long = "coords")]
    coordinats: String,
}

fn main() {
   
    let string = Img::parse();
    let img_in = image::open(string.path.clone()).unwrap();
    let vec_colors = dominant_colors(img_in.clone());    
    let sqauare = generate_squares(10.0, image::open(string.path.clone()).unwrap());
    let line = line(img_in.clone(), 10);
    let img_out = paint_coordinats(sqauare.clone(), vec_colors.clone(), img_in.clone(), line.clone());
    img_out.save(r"C:\Project\Image\out_1.png").unwrap();

    let string_2 = format_color(sqauare.clone(),
     vec_colors.clone(), string.coordinats, string.format);
     
    println!("{:?}", string_2);
    
}
