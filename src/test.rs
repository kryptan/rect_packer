use std;
use rand;
use image;
use {Config, Packer, Rect};


fn generate_in_range<R : rand::Rng>(generator : &mut R, min : i32, max : i32) -> i32 {
    // to ensure that edge cases are tested generate them with higher probability
    if generator.gen_weighted_bool(10) {
        0
    } else if generator.gen_weighted_bool(10) {
        max - 1
    } else if min >= max {
        min
    } else if generator.gen_weighted_bool(4) {
        generator.gen_range(min, max)
    } else {
        let new_max = std::cmp::max(min + 1, (max - min)/7);
        generator.gen_range(min, new_max)
    }
}

fn test_config<R : rand::Rng>(generator : &mut R, config : Config, generate_images : bool) {
    let mut packer = Packer::new(config);
    let mut frames = Vec::new();

    let rect = Rect::new(config.border_padding, config.border_padding, config.width - 2*config.border_padding, config.height - 2*config.border_padding);

    let mut num_failed_attempts = 0;
    while num_failed_attempts < 20 {
        let w = generate_in_range(generator, 0, config.width + 1);
        let h = generate_in_range(generator, 0, config.height + 1);

        if let Some(frame) = packer.pack(w, h) {
            let rotated = frame.w != w;

            assert!(config.allow_rotation || !rotated);

            if rotated {
                assert!((frame.w, frame.h) == (h, w));
            } else {
                assert!((frame.w, frame.h) == (w, h));
            }

            if !rect.contains(&frame) {
                println!("{:?} {:?}", rect, frame);
            }

            assert!(rect.contains(&frame));

            for other_frame in &frames {
                let f : &Rect = other_frame;
                assert!(!f.intersects(&frame));
            }

            frames.push(frame);
        } else {
            num_failed_attempts += 1;
        }
    }

    if generate_images {
        let mut img = image::ImageBuffer::<image::Rgb<u8>, _>::new(config.width as u32, config.height as u32);

        for i in 0..frames.len() {
            let f: &Rect = &frames[i];
            let color = image::Rgb { data: [((i + 1) * 71 % 256) as u8, (i * 59 % 256) as u8, (i * 103 % 256) as u8] };
            for y in f.top()..f.bottom() {
                for x in f.left()..f.right() {
                    use image::Pixel;

                    let pixel = img.get_pixel(x as u32, y as u32).map2(&color, |a, b| a.wrapping_add(b));
                    img.put_pixel(x as u32, y as u32, pixel);
                }
            }
        }

        std::fs::create_dir_all("target/generated-test-data").unwrap();
        img.save(format!("target/generated-test-data/test_{}x{}_{}_{}_{}.png", config.width, config.height, config.allow_rotation, config.border_padding, config.rectangle_padding)).unwrap();
    }
}

fn random_config<R : rand::Rng>(generator : &mut R) -> Config {
    let width = generator.gen_range(0, 1000);
    let height = generator.gen_range(0, 1000);

    let min = std::cmp::min(width, height);

    Config {
        width: width,
        height: height,
        allow_rotation: generator.gen(),

        border_padding: generate_in_range(generator, 0, min),
        rectangle_padding: generate_in_range(generator, 0, min),
    }
}

fn test(n : u32, generate_images : bool) {
    let mut generator = rand::thread_rng();

    for _ in 0..n {
        let config = random_config(&mut generator);
        test_config(&mut generator, config, generate_images);
    }
}

#[test]
fn generate_images() {
    test(100, true);
}

#[test]
fn packing() {
    let handles : Vec<_> = (0..8).map(|_|
        std::thread::spawn(move || test(10000, false))
    ).collect();

    for handle in handles {
        handle.join().unwrap();
    }
}
