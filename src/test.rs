use std;
use rand;
use image;
use {Config, SkylinePacker, Rect, Frame};

fn test_config<R : rand::Rng>(generator : &mut R, config : Config) {
    let mut packer = SkylinePacker::new(config);
    let mut frames = Vec::new();

    let rect = Rect::new(0, 0, config.width, config.height);

    loop {
        let w = generator.gen_range(1, config.width / 3);
        let h = generator.gen_range(1, config.height / 3);

        if let Some(frame) = packer.pack((w, h)) {
            assert!(config.allow_rotation || !frame.rotated);

            if frame.rotated {
                assert!(frame.frame.w == h);
                assert!(frame.frame.h == w);
            } else {
                assert!(frame.frame.w == w);
                assert!(frame.frame.h == h);
            }

            assert!(rect.contains(&frame.frame));

            for other_frame in &frames {
                let f : &Frame = other_frame;
                if f.frame.intersects(&frame.frame) {
                    println!("{:?} {:?}", f.frame, frame.frame);
                }
                assert!(!f.frame.intersects(&frame.frame));
            }

            frames.push(frame);
        } else {
            break;
        }
    }

    let mut img = image::ImageBuffer::<image::Rgb<u8>, _>::new(config.width, config.height);

    for i in 0..frames.len() {
        let f : &Frame = &frames[i];
        let color = image::Rgb { data : [((i + 1)*71 % 256) as u8, (i*59 % 256) as u8, (i*103 % 256) as u8] };
        for y in f.frame.top()..f.frame.bottom() {
            for x in f.frame.left()..f.frame.right() {
                use image::Pixel;

                let pixel = img.get_pixel(x, y).map2(&color, |a, b| a.wrapping_add(b));
                img.put_pixel(x, y, pixel);
            }
        }
    }

    std::fs::create_dir_all("target/generated-test-data").unwrap();
    img.save(format!("target/generated-test-data/test_{}x{}_{}_{}_{}.png", config.width, config.height, config.allow_rotation, config.border_padding, config.texture_padding)).unwrap();
}

fn random_config<R : rand::Rng>(generator : &mut R) -> Config {
    Config {
        width: generator.gen_range(6, 1000),
        height: generator.gen_range(6, 1000),
        allow_rotation: generator.gen(),

        border_padding: 5,
        texture_padding: 10,
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
