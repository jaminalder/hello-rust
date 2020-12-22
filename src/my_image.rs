#![deny(clippy::all)]
#![forbid(unsafe_code)]

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use rand::Rng;
use std::ops::Range;
use std::iter::Map;
use std::f64::consts::PI;

const WIDTH: u32 = 1600;
const HEIGHT: u32 = 1200;
const BOX_SIZE: i16 = 64;
const WHITE: [u8;4] = [255, 255, 255, 3];
const BLACK: [u8;4] = [0, 0, 0, 255];

pub fn draw_window() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            //draw(pixels.get_frame());
            clear(pixels.get_frame());
            // draw_spirograph(pixels.get_frame());
            draw_rect(pixels.get_frame());
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input.key_pressed(VirtualKeyCode::Space) {
                window.request_redraw();
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize(size.width, size.height);
                window.request_redraw();
            }
        }
    });
}

fn draw(frame: &mut [u8]) {
    let color_a = rand_color();
    let color_b = rand_color();
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = (i % WIDTH as usize) as i16;
        let y = (i / WIDTH as usize) as i16;

        let col = stripes(x, y, color_a, color_b);

        pixel.copy_from_slice(&col);
    }
}

fn clear(frame: &mut [u8]) {
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        pixel.copy_from_slice(&BLACK);
    }
}

fn draw_spirograph(frame: &mut [u8]) {
    // println!("pixel frame size: {}", frame.len())


    let points = scale_points(spirograph_points(rand::thread_rng().gen::<f64>(), rand::thread_rng().gen::<f64>(), 1000), 600.0);
    // println!("points: {:?}", points);

    for p in points {
        let pixel = get_mut_pixel(p, frame);
        pixel.copy_from_slice(&WHITE);
    }
}

fn draw_rect(frame: &mut [u8]) {
    let points = scale_points(rect_points(0.2, 0.2, 0.4, 0.4), 600.0);
    //println!("rect points: {:?}", points);
    points.into_iter().for_each(|p| {
        let pixel = get_mut_pixel(p, frame);
        pixel.copy_from_slice(&WHITE);
    })
}

fn get_mut_pixel(p: (i16, i16), frame: &mut [u8]) -> &mut [u8] {
    let idx = point_to_pixel_idx(p);
    frame.get_mut(idx..idx+4).expect("point out of range")
}

fn point_to_pixel_idx(p: (i16, i16)) -> usize {
    // println!("p to idx: {:?}", p);
    let i = (p.1 as usize * WIDTH as usize + p.0 as usize) * 4;
    // println!("idx: {:?}", i);

    i as usize
}

fn scale_points(ps:Vec<(f64,f64)>, factor:f64) -> Vec<(i16, i16)> {
    ps.into_iter().map(|p| ((p.0 * factor + 800.0) as i16, (p.1 * factor + 600.0) as i16)).collect()
}

fn spirograph_points(k:f64, l:f64, multiplier:i32) -> Vec<(f64, f64)> {
    float_range(0.0, PI * 128.0, multiplier).into_iter()
        .map(|t| spirograph_point(t, k, l))
        .collect()
}

fn spirograph_point(t:f64, k:f64, l:f64) -> (f64, f64) {
    let x = (1.0 - k) * t.cos() + l * k * (((1.0 - k) / k) * t).cos();
    let y = (1.0 - k) * t.sin() + l * k * (((1.0 - k) / k) * t).sin();
    (x, y)
}

fn rect_points(x:f64, y:f64, w:f64, h:f64) -> Vec<(f64, f64)> {
    let mut result:Vec<(f64, f64)> = Vec::new();
    for xi in float_range(x, x+w, 1000).iter() {
        for yi in float_range(y, y + h, 1000).iter() {
            result.push((*xi, *yi));
        }
    }
    result
}


fn float_range(start: f64, end: f64, multiplier: i32) -> Vec<f64> {
    let factor = multiplier as f64;
    let int_range = ((start * factor) as i32..(end * factor) as i32);
    int_range.map(|n| n as f64 / factor).collect()
}


fn stripes(x:i16, y:i16, color_a:[u8;4], color_b:[u8;4]) -> [u8; 4] {
    if (x % (WIDTH as i16 / 20)) as i16 > (WIDTH as i16 / 40) {
        if y < (HEIGHT as i16 / 2) {
            color_a
        } else {
            color_b
        }
    } else {
        if y < (HEIGHT as i16 / 2) {
            color_b
        } else {
            color_a
        }
    }
}

fn rand_color() -> [u8; 4] {
    let r = rand::thread_rng().gen_range(0, 255) as u8;
    let g = rand::thread_rng().gen_range(0, 255) as u8;
    let b = rand::thread_rng().gen_range(0, 255) as u8;
    [r, g, b, 255]
}

