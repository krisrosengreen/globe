mod graphics;
mod interface;

use graphics::Body;
use screen::screen::*;
use utils::utils::Pos;

#[allow(unused)]
fn setup_points(screen: &mut Screen) {
    let body_one = Body {
        pos: Pos {
            x: 5.0,
            y: 5.0,
            z: 7.0,
        },
        is_water: false
    };

    let body_two = Body {
        pos: Pos {
            x: -5.0,
            y: 5.0,
            z: 7.0,
        },
        is_water: false
    };

    let body_three = Body {
        pos: Pos {
            x: 0.0,
            y: 5.0,
            z: 15.0,
        },
        is_water: false
    };

    screen.add_body(Box::new(body_one));
    screen.add_body(Box::new(body_two));
    screen.add_body(Box::new(body_three))
}

#[allow(unused)]
fn setup_spherical_points(screen: &mut Screen) {
    let num_phi_points = 6; //240;
    let num_theta_points = 120;

    let center_x = 0.0;
    let center_y = 0.0;
    let center_z = 10.0;
    let r = 4.0;

    for phi_int in 0..num_phi_points {
        let phi = phi_int as f32 / num_phi_points as f32 * 3.1415 * 2.0;
        for theta_int in 0..num_theta_points {
            let theta = theta_int as f32 / num_theta_points as f32 * 3.1415;

            let x = r * theta.sin() * phi.cos() + center_x;
            let y = r * theta.sin() * phi.sin() + center_y;
            let z = r * theta.cos() + center_z;

            let body = Body {
                pos: Pos { x, y, z },
                is_water: false
            };

            screen.add_body(Box::new(body));
        }
    }
}

#[allow(unused)]
fn setup_ring_points(screen: &mut Screen) {
    let num_theta_points = 20;

    let center_x = 0.0;
    let center_y = 0.0;
    let center_z = 10.0;
    let r = 4.0;

    for theta_int in 0..num_theta_points {
        let theta = theta_int as f32 / num_theta_points as f32 * 3.1415 * 2.0;

        let x = center_x;
        let y = center_y + theta.sin() * r;
        let z = center_z + theta.cos() * r;

        let body = Body {
            pos: Pos { x, y, z },
            is_water: false
        };

        screen.add_body(Box::new(body));
    }
}

fn setup_world_points(screen: &mut Screen) {
    let center_x = 0.0;
    let center_y = 0.0;
    let center_z = 10.0;
    let r = 4.0;

    let sphere = map::mercator::get_world_sphere(500, 250);

    for sphere_point in sphere.points {
        let x = r * sphere_point.theta.sin() * sphere_point.phi.cos() + center_x;
        let y = r * sphere_point.theta.sin() * sphere_point.phi.sin() + center_y;
        let z = r * sphere_point.theta.cos() + center_z;

        let body = Body {
            pos: Pos { x, y, z },
            is_water: sphere_point.water
        };

        screen.add_body(Box::new(body));
    }
}

fn setup_screen() {
    let time = Time { current_time: 0.0 };

    let handler = interface::ScreenHandler::new();

    let mut screen = Screen::new(Box::new(handler));

    // setup_points(&mut screen);
    // setup_spherical_points(&mut screen);
    setup_world_points(&mut screen);
    // setup_ring_points(&mut screen);

    screen_loop(screen, time)
}

fn main() {
    setup_screen();
}
