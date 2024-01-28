extern crate piston_window;

use std::net::{TcpListener, TcpStream};
use std::io::Read;

use piston_window::*;
use std::sync::{Arc, Mutex};
use std::thread;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

lazy_static::lazy_static!(
    static ref CIRCLE_LIST: Mutex<Vec<Circle>> = Mutex::new(Vec::new());
);

struct Circle {
    color: [f32; 4],
    radius: f64,
    position: [f64; 2],
}

impl Circle {
    fn draw_circle(&self, c: Context, g: &mut G2d) {
        let circle = ellipse::circle(self.position[0] * WIDTH as f64, HEIGHT as f64 - self.position[1] * HEIGHT as f64, self.radius);
        ellipse(self.color, circle, c.transform, g);
    }

    fn update_position(&mut self, x: f64, y: f64) {
        self.position = [x, y];
    }
}

fn handle_client(mut stream: TcpStream) {
    println!("Connection established!");
    let mut circle_list = CIRCLE_LIST.lock().unwrap();
    let circle = Circle {
        color: [1.0, 0.0, 0.0, 1.0],
        radius: 10.0,
        position: [0.0, 0.0],
    };
    circle_list.push(circle);
    let circle_index = circle_list.len() - 1;
    std::mem::drop(circle_list);
    loop {
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();
        // should receive a string in the form (f64, f64)
        let msg = String::from_utf8_lossy(&buffer[..]);
        let msg = msg.trim_matches(char::from(0));
        let msg = msg.trim();
        let msg = msg.split(" ").collect::<Vec<&str>>();
        let x = msg[0].parse::<f64>().unwrap();
        let y = msg[1].parse::<f64>().unwrap();
        println!("x: {}, y: {}", x, y);
        let mut circle_list = CIRCLE_LIST.lock().unwrap();
        let circle = circle_list.get_mut(circle_index).unwrap();
        circle.update_position(x, y);
    }
}

fn main() {
    std::thread::spawn(|| {
        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
        println!("Listening on port 8080");
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            std::thread::spawn(move || {
                handle_client(stream);
            });
        }
    });
    
    let mut window: PistonWindow = WindowSettings::new("Circle", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            let circle_list = CIRCLE_LIST.lock().unwrap();
            for circle in circle_list.iter() {
                circle.draw_circle(c, g);
            }
        });
    }
}
