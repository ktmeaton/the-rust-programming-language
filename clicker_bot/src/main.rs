use enigo::*;
use rand::Rng;
use std::{thread, time};

struct MouseSettings {
    start_x: i32,
    start_y: i32,
    end_x: i32,
    end_y: i32,
    gravity: i32,
    wind: i32,
    min_wait: i32,
    max_wait: i32,
    max_step: i32,
    target_area: i32,
}

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl std::fmt::Display for MouseSettings {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(
            f,
            "start:    {}, {}\nend:      {}, {}\ngravity:  {}\nwind:     {}\nmin_wait: {}\nmax_wait: {}\nmax_step: {}",
            self.start_x, self.start_y, self.end_x, self.end_y, self.gravity, self.wind, self.min_wait, self.max_wait, self.max_step,
        )
    }
}

fn hypotenuse(a: f32, b: f32) -> f32{
    // Returns the hypothenuse of a triangle.
    //
    // a^2 + b2^ = c^2
    //
    // In this application, a and b will always be screen coordinates
    //   which are integers. Cast to float and back.

    (a * a + b * b).sqrt()

}

fn wind_mouse(mut settings: MouseSettings) {
    // WindMouse algorithm. Calls the move_mouse kwarg with each new step.
    // Author: Benjamin J. Land
    // Source: https://ben.land/post/2021/04/25/windmouse-human-mouse-movement/

    if settings.gravity < 1 {settings.gravity = 1;}

    let mut rng = rand::thread_rng();

    println!("{}", settings);
    // Get current position
    // let mut enigo = Enigo::new();
    // let (x, y) = enigo.mouse_location();

    let wait_diff: i32 = settings.max_wait - settings.min_wait;

    let old_x : i32;
    let old_y : i32;

    let new_x : i32;
    let new_y : i32;

    let step: i32;

    let mut wind_x = rng.gen_range(0.0..10.0);
    let mut wind_y = rng.gen_range(0.0..10.0);

    let mut random_dist = 0.0;

    let mut velocity_x = 0.0;
    let mut velocity_y = 0.0;
    let mut velocity: f32;

    let sqrt2 = f32::sqrt(2 as f32);
    let sqrt3 = f32::sqrt(3 as f32);
    let sqrt5 = f32::sqrt(3 as f32);

    let mut current_wait = 0;


    let mut dist = hypotenuse(
        (settings.end_x - settings.start_x) as f32,
        (settings.end_y - settings.start_y) as f32)
        as i32;

    println!("dist: {}", dist);

    while dist > 1 {
        settings.wind = std::cmp::min(settings.wind, dist);

        // If we're far from the destination, generate random wind
        if dist >= settings.target_area {
            let w = (rng.gen_range(0.0..1.0) * ((settings.wind as i32) as f32) * 2.0 + 1.0).floor() as i32;
            println!("w: {}", w);

            wind_x = (wind_x as f32) / sqrt3 + ((w - settings.wind) as f32) / sqrt5;
            wind_y = (wind_y as f32) / sqrt3 + ((w - settings.wind) as f32) / sqrt5;
        }
        // Otherwise, start reducing the wind
        else {
            wind_x /= sqrt2;
            wind_x /= sqrt2;

            // ???
            if settings.max_step < 3{
                settings.max_step = ((rng.gen_range(0.0..3.0) + 3.0) as f32).floor() as i32;
            }
            // Start reducing the step size
            else {
                settings.max_step = ((settings.max_step as f32) / sqrt5) as i32;
            }
        }

        // Adjust velocity based on the wind
        velocity_x += wind_x;
        velocity_y += wind_y;

        // Adjust velocity based on the gravity
        velocity_x = velocity_x + ((settings.gravity as f32) * ((settings.end_x - settings.start_x) as f32)) / (dist as f32);
        velocity_y = velocity_y + ((settings.gravity as f32) * ((settings.end_y - settings.start_y) as f32)) / (dist as f32);

        // Check if velocity is greater than step size
        velocity = hypotenuse(velocity_x, velocity_y);
        if velocity > (settings.max_step as f32) {
            random_dist = (settings.max_step as f32 / 2.0) + (rng.gen_range(0.0..settings.max_step as f32) / 2.0).floor();
            velocity_x = (velocity_x / velocity) * random_dist;
            velocity_y = (velocity_y / velocity) * random_dist;
        }

        old_x = settings.start_x;
        old_y = settings.start_y;
        settings.start_x += velocity_x as i32;
        settings.start_y += velocity_y as i32;

        dist = hypotenuse(
            (settings.end_x - settings.start_x) as f32,
            (settings.end_y - settings.start_y) as f32)
            as i32;

        new_x = settings.start_x;
        new_y = settings.start_y;

        step = hypotenuse(
            (settings.start_x - old_x) as f32,
            (settings.start_y - old_y) as f32)
            as i32;

        let wait = (wait_diff * (step / settings.max_step) + settings.min_wait) as i32;
        // time::Duration::from_millis(
        current_wait += wait;
        println!("{}", settings);
        println!("wind_x: {}", wind_x);
        println!("wind_y: {}", wind_y);
        println!("velocity_x: {}", velocity_x);
        println!("velocity_y: {}", velocity_y);
        println!("random_dist: {}", random_dist);
        break
    }

}

fn main() {

    let mouse_settings = MouseSettings {
        start_x: 500,
        start_y: 500,
        end_x: 1000,
        end_y: 900,
        gravity: 2,
        wind: 8,
        min_wait: 1,
        max_wait: 4,
        max_step: 3,
        target_area: 10,
    };

    wind_mouse(mouse_settings);
}
