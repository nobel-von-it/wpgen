use image::{ImageBuffer, Rgb};
use rand::Rng;

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

fn get_random_color() -> u8 {
    rand::thread_rng().gen_range(0..=255)
}

fn get_random_rgb() -> Rgb<u8> {
    Rgb([get_random_color(), get_random_color(), get_random_color()])
}

fn calc_color(start_c: &Rgb<u8>, end_c: &Rgb<u8>, t: f32, i: usize) -> u8 {
    (start_c[i] as f32 * (1.0 - t) + end_c[i] as f32 * t) as u8
}

fn calc_full_rgb(start_c: &Rgb<u8>, end_c: &Rgb<u8>, t: f32) -> Rgb<u8> {
    let r = calc_color(start_c, end_c, t, 0);
    let g = calc_color(start_c, end_c, t, 1);
    let b = calc_color(start_c, end_c, t, 2);

    Rgb([r, g, b])
}

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    if args.len() != 1 {
        println!("Usage: wpgen <path>");
        return;
    }
    let path = &args[0];

    let mut img = ImageBuffer::new(WIDTH, HEIGHT);
    let mut rng = rand::thread_rng();

    let start_c = get_random_rgb();
    let end_c = get_random_rgb();

    let direction = rng.gen_range(0..3);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let t = match direction {
                0 => y as f32 / HEIGHT as f32,
                1 => x as f32 / WIDTH as f32,
                _ => ((x + y) as f32) / (WIDTH + HEIGHT) as f32,
            };
            let pixel_c = calc_full_rgb(&start_c, &end_c, t);
            img.put_pixel(x, y, pixel_c);
        }
    }

    img.save(path).unwrap();
}
