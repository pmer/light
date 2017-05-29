extern crate sdl2;

mod bresenham;
mod geometry;

use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::PixelFormatEnum::RGB332;

use bresenham::*;
use geometry::*;

struct Settings {
    window: Dimensions,
}

#[derive(Clone, Eq, PartialEq)]
struct Light {
    on: bool,
    point: Point,
    size: i32,
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    started: bool,
    grid: Dimensions,
    light: Light,
    obstacles: Vec<Rect>,
    new_obstacle_start: Option<Point>,
    new_obstacle: Option<Rect>,
}

#[derive(Clone, Eq, PartialEq)]
struct Bitmap {
    pixels: Vec<u8>,
}

const BLACK: u8 = 0u8;
const RED: u8 = 224u8;
const GREEN: u8 = 28u8;
const BLUE: u8 = 3u8;
const WHITE: u8 = 255u8;

const GREENS: [u8; 7] = [4, 8, 12, 16, 20, 24, 28];

fn handle_input(mut state: State,
                settings: &Settings,
                event_pump: &mut EventPump)
                -> Result<State, ()> {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => return Err(()),
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return Err(()),
            Event::MouseMotion { x, y, .. } => handle_mouse_move(&mut state, settings, x, y),
            Event::MouseButtonDown { x, y, mouse_btn, .. } => {
                handle_mouse_down(&mut state, settings, x, y, mouse_btn)
            }
            Event::MouseButtonUp { x, y, .. } => handle_mouse_up(&mut state, settings, x, y),
            _ => {}
        }
    }

    Ok(state)
}

fn handle_mouse_move(state: &mut State, settings: &Settings, x: i32, y: i32) {
    state.started = true;
    state.light.point = logical_point(&state, settings, x, y);

    if let Some(new_obstacle_start) = state.new_obstacle_start {
        let obstacle_end = logical_point(&state, settings, x, y);
        let new_obstacle = Rect::from_points(new_obstacle_start, obstacle_end);
        state.new_obstacle = Some(new_obstacle);
    }
}

fn handle_mouse_down(state: &mut State, settings: &Settings, x: i32, y: i32, btn: MouseButton) {
    let mouse_location = logical_point(&state, settings, x, y);

    match btn {
        // Place an obstacle.
        MouseButton::Left => {
            state.light.on = false;
            state.new_obstacle_start = Some(mouse_location);
        }
        // Remove obstacles.
        MouseButton::Right => {
            state.obstacles.retain(|obs| !obs.contains(mouse_location));
        }
        _ => {}
    }
}

fn handle_mouse_up(state: &mut State, settings: &Settings, x: i32, y: i32) {
    // If we were placing an obstacle, its size is now determined. Add it to the state.
    if let Some(new_obstacle_start) = state.new_obstacle_start {
        let obstacle_end = logical_point(&state, settings, x, y);
        let new_obstacle = Rect::from_points(new_obstacle_start, obstacle_end);
        if new_obstacle.area() > 0 {
            state.obstacles.push(new_obstacle);
        }
    }

    state.light.on = true;
    state.new_obstacle_start = None;
    state.new_obstacle = None;
}

// Given a mouse position inside the game window, determine where that is on the game bitmap.
fn logical_point(state: &State, settings: &Settings, x: i32, y: i32) -> Point {
    assert!(0 <= x && x < settings.window.width && 0 <= y && y < settings.window.height);
    Point {
        x: x * state.grid.width / settings.window.width,
        y: y * state.grid.height / settings.window.height,
    }
}

fn update() {}

// Create a Bitmap showing the current scene.
fn render(state: &State) -> Bitmap {
    let started = state.started;
    let grid = &state.grid;
    let light = &state.light;
    let obstacles = &state.obstacles;
    let new_obstacle = state.new_obstacle;

    let npixels = grid.width as usize * grid.height as usize;
    let mut pixels = vec![BLACK; npixels];

    if started && light.on {
        render_light(&mut pixels, &grid, &obstacles, &light);
    }

    for obstacle in obstacles {
        render_obstacle(&mut pixels, grid.width, obstacle);
    }

    if let Some(obstacle) = new_obstacle {
        render_obstacle(&mut pixels, grid.width, &obstacle);
    }

    Bitmap { pixels }
}

fn render_light(pixels: &mut Vec<u8>, grid: &Dimensions, obstacles: &Vec<Rect>, light: &Light) {
    for yoff in -light.size..light.size + 1 {
        for xoff in -light.size..light.size + 1 {
            let x = light.point.x + xoff;
            let y = light.point.y + yoff;
            let illuminated_point = Point { x, y };
            let distance = Point { x: xoff, y: yoff }.magnitude();
            if (distance as i32) < light.size {
                if 0 <= x && x < grid.width && 0 <= y && y < grid.height {
                    if has_line_of_sight(light.point, illuminated_point, obstacles) {
                        let offset = (y as usize) * (grid.width as usize) + (x as usize);
                        let intensity = 1.0 - distance / (light.size as f32);
                        pixels[offset] = green(intensity);
                    }
                }
            }
        }
    }
}

fn has_line_of_sight(a: Point, b: Point, obstacles: &Vec<Rect>) -> bool {
    let line: Vec<Point> = bresenham(a, b);
    for obs in obstacles {
        for pixel in &line {
            if obs.contains(*pixel) {
                return false;
            }
        }
    }
    true
}

fn green(percent: f32) -> u8 {
    GREENS[(percent * ((GREENS.len() - 1) as f32)) as usize]
}

fn render_obstacle(pixels: &mut Vec<u8>, grid_width: i32, obstacle: &Rect) {
    for y in 0..obstacle.h {
        for x in 0..obstacle.w {
            let offset = ((obstacle.y + y) as usize) * (grid_width as usize) +
                         ((obstacle.x + x) as usize);
            pixels[offset] = RED;
        }
    }
}

pub fn main() {
    // Initialize game parameters with default values.
    let settings = Settings {
        window: Dimensions {
            width: 640,
            height: 480,
        },
    };

    let mut state = State {
        started: false,
        grid: Dimensions {
            width: 160,
            height: 120,
        },
        light: Light {
            on: true,
            point: Point { x: 0, y: 0 },
            size: 15,
        },
        obstacles: vec![],
        new_obstacle_start: None,
        new_obstacle: None,
    };

    let mut bitmap = Bitmap { pixels: vec![] };

    // Create window.
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut timer_subsystem = sdl_context.timer().unwrap();

    let window = video_subsystem
        .window("Light",
                settings.window.width as u32,
                settings.window.height as u32)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().present_vsync().build().unwrap();

    let mut texture = renderer
        .create_texture_streaming(RGB332, state.grid.width as u32, state.grid.height as u32)
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    // Main loop.
    loop {
        let old_state = state.clone();

        // Handle player input since last frame.
        state = match handle_input(state, &settings, &mut event_pump) {
            Err(_) => return,
            Ok(state) => state,
        };

        // Resize the video texture if user input resulted in a change of grid size.
        if old_state.grid != state.grid {
            texture = renderer
                .create_texture_streaming(RGB332,
                                          state.grid.width as u32,
                                          state.grid.height as u32)
                .unwrap();
        }

        // Simulate the game world for a frame.
        update();

        // Render a frame if our state has changed.
        if old_state == state {
            timer_subsystem.delay(16); // Wait 16 milliseconds.
        } else {
            let old_bitmap = bitmap.clone();

            // Create a bitmap. (CPU memory)
            bitmap = render(&state);

            // Update the screen if our bitmap has changed.
            if old_bitmap == bitmap {
                timer_subsystem.delay(16); // Wait 16 milliseconds.
            } else {
                // Upload the bitmap to the streaming texture. (GPU memory)
                texture
                    .update(None, &bitmap.pixels, state.grid.width as usize)
                    .unwrap();

                // Copy the texture to the back buffer. (GPU memory)
                renderer.copy(&texture, None, None).unwrap();

                // Flip the front and back buffers and pause for vsync. (Display)
                renderer.present();
            }
        }
    }
}
