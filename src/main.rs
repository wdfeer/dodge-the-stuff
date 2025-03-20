use std::fmt::Debug;
use nannou::{App, Frame};
use nannou::color::BLACK;
use nannou::event::Update;
use nannou::prelude::DARKGREEN;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    last_time: f32,
    dead: bool,
    enemies: [f32; 64]
}

fn model(_app: &App) -> Model {
    Model {
        last_time: 0f32,
        dead: false,
        enemies: [0.0; 64],
    }
}

fn update(app: &App, model: &mut Model, update: Update) {

}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);

    let draw = app.draw();
    let win = app.window_rect();

    draw.ellipse()
        .x_y(app.mouse.x, app.mouse.y)
        .radius(win.w() / 100f32)
        .color(DARKGREEN);

    draw.to_frame(app, &frame).unwrap();
}