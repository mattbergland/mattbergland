use rayon::prelude::*;
use std::{env, fs::File, io::BufWriter, time::Instant};
use tracer::{Color, Renderer};
fn main() {
    let mut w = 1600u32;
    let mut h = 900u32;
    let mut samples = 64u32;
    let mut scene = 0u32;
    let mut output = String::from("../images/rust-tracer-render.png");
    let a: Vec<String> = env::args().collect();
    let mut i = 1;
    while i < a.len() {
        match a[i].as_str() {
            "--width" => {
                i += 1;
                w = a[i].parse().unwrap()
            }
            "--height" => {
                i += 1;
                h = a[i].parse().unwrap()
            }
            "--samples" => {
                i += 1;
                samples = a[i].parse().unwrap()
            }
            "--scene" => {
                i += 1;
                scene = a[i].parse().unwrap()
            }
            "--output" => {
                i += 1;
                output = a[i].clone()
            }
            "--help" => {
                println!("tracer-cli [--width N] [--height N] [--samples N] [--scene 0|1] [--output PATH]");
                return;
            }
            _ => {}
        }
        i += 1
    }
    let start = Instant::now();
    let mut r = Renderer::new(w, h, scene);
    let mut total_rays = 0u64;
    for pass in 0..samples {
        let rendered: Vec<(Color, u32)> = (0..(w * h) as usize)
            .into_par_iter()
            .map(|idx| r.render_pixel(idx as u32 % w, idx as u32 / w, pass, 0x1234_5678))
            .collect();
        let mut pass_rays = 0u64;
        for (dst, (src, rays)) in r.pixels.iter_mut().zip(rendered) {
            *dst += src;
            pass_rays += rays as u64;
        }
        r.last_pass_rays = pass_rays;
        total_rays += pass_rays;
        r.samples = pass + 1;
        if pass == 0 || pass + 1 == samples || pass % 8 == 7 {
            println!(
                "pass {}/{} ({:.1}%)",
                pass + 1,
                samples,
                (pass + 1) as f32 / samples as f32 * 100.0
            )
        }
    }
    let rgba = r.rgba();
    let f = File::create(&output).expect("output");
    let mut enc = png::Encoder::new(BufWriter::new(f), w, h);
    enc.set_color(png::ColorType::Rgba);
    enc.set_depth(png::BitDepth::Eight);
    enc.write_header().unwrap().write_image_data(&rgba).unwrap();
    let secs = start.elapsed().as_secs_f64();
    println!(
        "wrote {} ({}x{}, {} samples) in {:.2}s ({:.2} Mrays/s)",
        output,
        w,
        h,
        samples,
        secs,
        total_rays as f64 / secs / 1e6
    );
}
