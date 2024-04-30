use crate::rtpkt::Rtpkt;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::f64;

const COLOR_PACKET: Color = Color::RGB(255, 0, 0);
const COLOR_RED: Color = Color::RGB(255, 0, 0);
const COLOR_GREEN: Color = Color::RGB(0, 255, 0);
const COLOR_BLUE: Color = Color::RGB(0, 0, 255);
const COLOR_YELLOW: Color = Color::RGB(255, 255, 0);

struct VisualPacket<'a> {
    screen: &'a mut sdl2::render::WindowCanvas,
    x: f64,
    y: f64,
    x_speed: f64,
    y_speed: f64,
    x_dest: f64,
    y_dest: f64,
    my_packet: Rtpkt,
    has_reached_dest: bool,
    color: Color,
}

impl<'a> VisualPacket<'a> {
    fn new(
        screen: &'a mut sdl2::render::WindowCanvas,
        x: f64,
        y: f64,
        x_speed: f64,
        y_speed: f64,
        x_dest: f64,
        y_dest: f64,
        packet: Rtpkt,
        color: Color,
    ) -> VisualPacket<'a> {
        VisualPacket {
            screen,
            x,
            y,
            x_speed,
            y_speed,
            x_dest,
            y_dest,
            my_packet: packet,
            has_reached_dest: false,
            color,
        }
    }

    fn update(&mut self) {
        self.x += self.x_speed;
        self.y += self.y_speed;

        if ((self.x - self.x_dest).powi(2) + (self.y - self.y_dest).powi(2)).sqrt() < 10.0 {
            self.has_reached_dest = true;
        }
    }

    fn draw(&self) {
        self.screen.set_draw_color(self.color);
        let center = Point::new(self.x as i32, self.y as i32);
        self.screen
            .draw_circle(center, 10, 3)
            .expect("Circle drawing failed");
    }
}
