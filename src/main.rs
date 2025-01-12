use std::error::Error;

mod actions;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    // Parameters
    let image_path = "/home/spandan/dotfiles/Wallpapers/Edward.jpeg";
    let num_centroids = 20; // Number of clusters
    let max_iterations = 20;

    let centroids = utils::extract_pallete(image_path, num_centroids, max_iterations)?;
    println!("{:#?}", centroids);

    // Generate palette image
    // Squares 100x100px
    let image_width = 100 * centroids.len() as u32;
    let image_height = 100;

    let mut image = image::RgbImage::new(image_width, image_height);

    for (i, centroid) in centroids.iter().enumerate() {
        for x in 0..100 {
            for y in 0..100 {
                let r = centroid.r as u8;
                let g = centroid.g as u8;
                let b = centroid.b as u8;
                let pixel = image::Rgb([r, g, b]);
                image.put_pixel(x + i as u32 * 100, y, pixel);
            }
        }
    }

    image.save("palette.png").unwrap();

    Ok(())
}
