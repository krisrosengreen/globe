use image::GenericImageView;

use crate::loadmap::load_image;

pub struct Sphere {
    pub points: Vec<SphereCoords>
}

pub struct SphereCoords {
    pub phi: f32,
    pub theta: f32,
    pub water: bool
}

pub fn get_world_sphere(phi_points: u32, theta_points: u32) -> Sphere {
    let mut coords: Vec<SphereCoords> = Vec::new();

    let im = load_image();

    //width, height
    let dimensions = im.dimensions();

    let x_jump = dimensions.0/phi_points;
    let y_jump = dimensions.1/theta_points;

    for x_loop in 0..phi_points {
        let x_index = x_loop * x_jump;
        for y_loop in 0..theta_points {
            let y_index = y_loop*y_jump;

            let merc_phi = x_index as f32 / dimensions.0 as f32 * 3.1415 * 2.0;
            let merc_theta = y_index as f32 / dimensions.1 as f32 * 3.1415;

            let color = im.get_pixel(x_index, y_index);
            //println!("{:?}", color);
            let blueness = color.0[2];

            let is_blue = blueness == 244;

            let coord = SphereCoords {
                phi: merc_phi,
                theta: merc_theta,
                water: is_blue
            };

            coords.push(coord);
        }
    }

    Sphere { points: coords }
}