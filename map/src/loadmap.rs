use std::path::Path;

use image::DynamicImage;

pub fn load_image() -> DynamicImage{
    let im = image::open(Path::new("images/world.png")).unwrap();

    im
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_load() {
        load_image();
        assert!(true);
    }
}