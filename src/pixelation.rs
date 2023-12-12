use image::{DynamicImage, GenericImageView, Rgba, RgbaImage};

pub fn dominant_colors(img: DynamicImage) -> Vec<Rgba<u8>> {
    let (width, height) = img.dimensions();
    let mut vec_colors = Vec::new();
    let cell_size = 10;

    for y in (0..height).step_by(cell_size) {
        for x in (0..width).step_by(cell_size) {
            let mut color_counts = std::collections::HashMap::new();

            for y2 in y..y + cell_size as u32 {
                for x2 in x..x + cell_size as u32 {
                    let pixel = img.get_pixel(x2, y2);
                    *color_counts.entry(pixel).or_insert(0) += 1;
                }
            }

            if let Some((color, _)) = color_counts.iter().max_by_key(|(_, count)| *count) {
                vec_colors.push(*color);
            }
        }
    }

    vec_colors
}

pub fn pixelation(img: DynamicImage, vec_colors: Vec<Rgba<u8>>, cell_size: u32) -> DynamicImage {
    let (width, height) = img.dimensions();

    let mut img_out = RgbaImage::new(width, height);

    img_out.fill(0u8);

    let cells_x = width / cell_size;
    let cells_y = height / cell_size;

    for y in 0..cells_y {
        for x in 0..cells_x {
            let mut color_index = 0;

            for y2 in 0..cell_size {
                for x2 in 0..cell_size {
                    let x_cell = x * cell_size + x2;
                    let y_cell = y * cell_size + y2;

                    if x_cell < width && y_cell < height {
                        let color = vec_colors[color_index];
                        let brightness = (color[0] as u32 + color[1] as u32 + color[2] as u32) / 3;
                        let pixel =
                            Rgba([brightness as u8, brightness as u8, brightness as u8, 255]);

                        img_out.put_pixel(x_cell, y_cell, pixel);
                    }

                    color_index = (color_index + 1) % vec_colors.len();
                }
            }
        }
    }

    DynamicImage::ImageRgba8(img_out)
}
