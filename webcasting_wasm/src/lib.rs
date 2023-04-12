use js_sys::{Array, Uint8ClampedArray};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: String);
}

const DEGREES_IN_RADIANS: f64 = 0.0174533;
use std::f64::consts::PI;

#[derive(Clone, Copy, PartialEq, Eq)]
#[wasm_bindgen]
pub enum Wall {
    Empty,
    Wall,
}
impl TryFrom<f64> for Wall {
    type Error = ();
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        match value as usize {
            0 => Ok(Wall::Empty),
            1 => Ok(Wall::Wall),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy)]
struct Vec2 {
    x: f64,
    y: f64,
}
impl Vec2 {
    fn get_length(&self) -> f64 {
        return self.x.hypot(self.y);
    }
}
impl std::ops::Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl std::ops::Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
struct Map {
    data: Vec<Vec<Wall>>,
}
impl Map {
    fn is_wall_at_position(&self, position: Vec2) -> bool {
        if !(position.x > 0.0
            && position.x < self.data.len() as f64
            && position.y > 0.0
            && position.y < self.data[0].len() as f64)
        {
            return false;
        }
        let first_index: usize = position.x.trunc() as usize;
        let second_index: usize = position.y.trunc() as usize;
        //println!("index: {}, {}", first_index, second_index);
        if self.data[first_index][second_index] == Wall::Empty {
            return false;
        }
        println!("{} | {}", position.x, position.y);
        return true;
    }
    fn get_wall_at_position(&self, position: Vec2) -> Wall {
        if !(position.x > 0.0
            && position.x < self.data.len() as f64
            && position.y > 0.0
            && position.y < self.data[0].len() as f64)
        {
            return Wall::Empty;
        }
        let first_index: usize = position.x.trunc() as usize;
        let second_index: usize = position.y.trunc() as usize;
        return self.data[first_index][second_index];
    }
    fn try_from_js_array(array: Array, width: usize, height: usize) -> Result<Self, ()> {
        let mut data = vec![vec![Wall::Empty; height]; width];
        for n in 0..array.length() {
            let Some(value) = array.at(n as i32).as_f64() else {
                return Err(());
            };
            let Ok(value) = Wall::try_from(value) else {
                return Err(());
            };
            let i = n as usize / height;
            let j = n as usize % height;
            //log(format!("n: {} | i: {} | j: {}", n, i, j));
            data[i][j] = value;
        }
        return Ok(Map { data });
    }
}
struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}
impl Color {
    fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        return Color { r, g, b, a };
    }
}

struct Canvas {
    height: usize,
    width: usize,
    draw_color: Color,
    data: Vec<u8>,
}
impl Canvas {
    fn new(width: usize, height: usize) -> Self {
        return Canvas {
            height,
            width,
            draw_color: Color::from_rgba(0, 0, 0, 255),
            data: vec![0; height * width * 4],
        };
    }
    fn set_draw_color(&mut self, color: Color) {
        self.draw_color = color;
    }
    fn draw_pixel(&mut self, x: usize, y: usize) {
        if x < 0 || x > self.width || y < 0 || y > self.height {
            return;
        }
        self.data[4 * (x + y * self.width)] = self.draw_color.r;
        self.data[4 * (x + y * self.width) + 1] = self.draw_color.g;
        self.data[4 * (x + y * self.width) + 2] = self.draw_color.b;
        self.data[4 * (x + y * self.width) + 3] = self.draw_color.a;
    }
    fn draw_vertical_line(&mut self, x: usize, height: usize) {
        if x < 0 || x > self.width {
            return;
        }
        for i in (0.5 * ((self.height - height) as f32)).round() as usize
            ..(0.5 * ((self.height + height) as f32)).round() as usize
        {
            if i < 0 || i > self.height {
                continue;
            }
            self.data[4 * (x + i * self.width)] = self.draw_color.r;
            self.data[4 * (x + i * self.width) + 1] = self.draw_color.g;
            self.data[4 * (x + i * self.width) + 2] = self.draw_color.b;
            self.data[4 * (x + i * self.width) + 3] = self.draw_color.a;
        }
    }
    fn clear(&mut self) {
        for i in 0..self.height * self.width {
            self.data[4 * i] = self.draw_color.r;
            self.data[4 * i + 1] = self.draw_color.g;
            self.data[4 * i + 2] = self.draw_color.b;
            self.data[4 * i + 3] = self.draw_color.a;
        }
    }
}

struct RayCaster {
    x: f64,
    y: f64,
    angle: f64,
}
impl RayCaster {
    fn cast(&self, map: &Map, dof: f64) -> (f64, Wall) {
        if !(dof > 0.0) {
            return (dof, Wall::Empty);
        }
        let angle = angle_to_normal_range(self.angle);
        let sin: f64 = angle.sin();
        let cos: f64 = angle.cos();

        //println!("{} | {}, {}", angle, sin, cos);

        let step_total_delta = Vec2 {
            x: (1.0 / cos).abs(),
            y: (1.0 / sin).abs(),
        };
        let direction_coefficient = Vec2 {
            x: {
                if angle < PI * 1.5 && angle > PI * 0.5 {
                    -1.0
                } else {
                    1.0
                }
            },
            y: {
                if angle < PI && angle > 0.0 {
                    -1.0
                } else {
                    1.0
                }
            },
        };
        let mut delta_needed_for_next_tile = Vec2 {
            x: ((0.5 + 0.5 * direction_coefficient.x) - self.x.fract() * direction_coefficient.x)
                * step_total_delta.x,
            y: ((0.5 + 0.5 * direction_coefficient.y) - self.y.fract() * direction_coefficient.y)
                * step_total_delta.y,
        };
        let mut current_tile = Vec2 {
            x: (self.x + 0.0 * (direction_coefficient.x * 0.5 + 0.5)).trunc(),
            y: (self.y + 0.0 * (direction_coefficient.y * 0.5 + 0.5)).trunc(),
        };
        let mut travelled_distance = 0.0;

        loop {
            if delta_needed_for_next_tile.x < delta_needed_for_next_tile.y {
                delta_needed_for_next_tile.y -= delta_needed_for_next_tile.x;
                travelled_distance += delta_needed_for_next_tile.x.abs();
                delta_needed_for_next_tile.x = step_total_delta.x;
                current_tile.x += direction_coefficient.x;
            } else {
                delta_needed_for_next_tile.x -= delta_needed_for_next_tile.y;
                travelled_distance += delta_needed_for_next_tile.y.abs();
                delta_needed_for_next_tile.y = step_total_delta.y;
                current_tile.y += direction_coefficient.y;
            }

            if travelled_distance > dof {
                return (dof, Wall::Empty);
            }
            if map.is_wall_at_position(current_tile.clone()) {
                return (
                    travelled_distance,
                    map.get_wall_at_position(current_tile.clone()),
                );
            }
        }
    }
}

#[wasm_bindgen]
pub fn render(
    x: f64,
    y: f64,
    angle: f64,
    screen_width: usize,
    screen_height: usize,
    map_data: Array,
    map_width: usize,
    map_height: usize,
    dof: f64,
    fov: f64,
) -> Uint8ClampedArray {
    //rendering
    let mut i = 0;
    let map = Map::try_from_js_array(map_data, map_width, map_height).unwrap();
    let mut canvas = Canvas::new(screen_width, screen_height);
    canvas.set_draw_color(Color::from_rgba(0, 0, 0, 255));
    canvas.clear();

    let mut ray_caster = RayCaster { x, y, angle };
    ray_caster.angle = angle_to_normal_range(ray_caster.angle + (fov * DEGREES_IN_RADIANS) * 0.5);
    let delta_radians_per_iteration = (fov * DEGREES_IN_RADIANS) / canvas.width as f64;
    for i in 0..canvas.width {
        let (mut distance, _) = ray_caster.cast(&map, dof);
        if distance < 1.0 {
            distance = 1.0;
        }
        //distance *= (player.angle - ray_caster.angle).cos();
        let height: usize = (canvas.height as f64 / distance).round() as usize;
        let brightness = (1.0 / (distance / 2.0).powi(2) * 255.0).round() as u8;
        canvas.set_draw_color(Color::from_rgba(brightness, brightness, brightness, 255));

        canvas.draw_vertical_line(i, height);

        ray_caster.angle = angle_to_normal_range(ray_caster.angle - delta_radians_per_iteration);
    }
    return canvas.data.as_slice().try_into().unwrap();
}
fn angle_to_normal_range(input_angle: f64) -> f64 {
    let mut angle = input_angle;
    loop {
        if angle > 2.0 * PI {
            angle -= 2.0 * PI;
        } else if angle < 0.0 {
            angle += 2.0 * PI;
        } else {
            break;
        }
    }
    return angle;
}
