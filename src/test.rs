use std;
use rand;
use image;
use {Config, Packer, Rect};

fn test_config<R : rand::Rng>(generator : &mut R, config : Config) {
    let mut packer = Packer::new(config);
    let mut frames = Vec::new();

    let rect = Rect::new(0, 0, config.width, config.height);

    loop {
        let w = generator.gen_range(1, config.width / 3);
        let h = generator.gen_range(1, config.height / 3);

        if let Some(frame) = packer.pack((w, h)) {
            let rotated = frame.w != w;

            assert!(config.allow_rotation || !rotated);

            if rotated {
                assert!((frame.w, frame.h) == (h, w));
            } else {
                assert!((frame.w, frame.h) == (w, h));
            }

            assert!(rect.contains(&frame));

            for other_frame in &frames {
                let f : &Rect = other_frame;
                assert!(!f.intersects(&frame));
            }

            frames.push(frame);
        } else {
            break;
        }
    }

    let mut img = image::ImageBuffer::<image::Rgb<u8>, _>::new(config.width, config.height);

    for i in 0..frames.len() {
        let f : &Rect = &frames[i];
        let color = image::Rgb { data : [((i + 1)*71 % 256) as u8, (i*59 % 256) as u8, (i*103 % 256) as u8] };
        for y in f.top()..f.bottom() {
            for x in f.left()..f.right() {
                use image::Pixel;

                let pixel = img.get_pixel(x, y).map2(&color, |a, b| a.wrapping_add(b));
                img.put_pixel(x, y, pixel);
            }
        }
    }

    std::fs::create_dir_all("target/generated-test-data").unwrap();
    img.save(format!("target/generated-test-data/test_{}x{}_{}_{}_{}.png", config.width, config.height, config.allow_rotation, config.border_padding, config.rectangle_padding)).unwrap();
}

fn random_config<R : rand::Rng>(generator : &mut R) -> Config {
    Config {
        width: generator.gen_range(6, 1000),
        height: generator.gen_range(6, 1000),
        allow_rotation: generator.gen(),

        border_padding: 5,
        rectangle_padding: 10,
    }
}

#[test]
fn packing() {
    let handles : Vec<_> = (0..8).map(|_|
        std::thread::spawn(move || {
            let mut generator = rand::thread_rng();

            for _ in 0..100 {
                let config = random_config(&mut generator);
                test_config(&mut generator, config);
            }
        })
    ).collect();

    for handle in handles {
        handle.join().unwrap();
    }
}
