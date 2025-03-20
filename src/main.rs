use nannou::{App, Frame};
use nannou::color::BLACK;
use nannou::prelude::DARKGREEN;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    frame.clear(BLACK);

    let draw = app.draw();
    let win = app.window_rect();

    let t = app.time;
    draw.ellipse()
        .x_y(app.mouse.x, app.mouse.y)
        .radius(win.w() / 100f32)
        .color(DARKGREEN);

    draw.to_frame(app, &frame).unwrap();
}
