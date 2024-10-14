extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::Duration;
use std::collections::VecDeque;
use std::iter::zip;

use rand::prelude::*;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .vulkan()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut rng = thread_rng();
    let mut i = 0;

    // Set the initial background color before drawing
    // canvas.set_draw_color(Color::RGB(0, 128, 128));
    // canvas.clear();
    // canvas.present();
    let mut buffer: VecDeque::<Rect> = VecDeque::new();
    let mut color_buffer: VecDeque::<Color> = VecDeque::new();

    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Set the color for the new rectangle
        color_buffer.push_back(Color::RGB(
            i,
            64u8.wrapping_add(2u8.wrapping_mul(i)),
            255 - i,
        ));
        if color_buffer.len() > 60 {
            color_buffer.pop_front();
        }

        // Draw a rectangle with random position and dimensions
        let width = rng.gen_range(50..=200);
        let height = rng.gen_range(50..=200);
        buffer.push_back(Rect::new(rng.gen_range(0..=600), rng.gen_range(0..=400), width, height));
        if buffer.len() > 60 {
            buffer.pop_front();
        }
        for (r, c) in zip(&buffer, &color_buffer) {
            canvas.set_draw_color(*c);
            canvas.fill_rect(*r).unwrap();
        }

        // Handle events like quitting the application
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                _ => {}
            }
        }

        // Present the canvas with the updated frame
        canvas.present();

        // Delay to keep the loop running at ~60 FPS
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}