use std::path::Path;

use image::DynamicImage;

pub fn load_image() -> DynamicImage {
    let im = image::open(Path::new("images/world.png")).unwrap();

    im
}
