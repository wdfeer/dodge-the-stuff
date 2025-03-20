use std::fmt::{Debug, Pointer};
use nannou::{App, Frame};
use nannou::color::{BLACK, WHITE};
use nannou::color::chromatic_adaptation::AdaptInto;
use nannou::event::Update;
use nannou::prelude::DARKGREEN;
use nannou::prelude::real::Real;
use nannou::rand::random;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    dead: bool,
    enemies: [(f32, f32); 64]
}

const EMPTY_ENEMY: (f32, f32) = (f32::MIN, f32::MIN);
fn model(_app: &App) -> Model {
    Model {
        dead: false,
        enemies: [EMPTY_ENEMY; 64],
    }
}

fn get_random_coord(max: f32) -> f32 {
    (random::<f32>() - 0.5) * max
}

fn update(app: &App, model: &mut Model, update: Update) {
    if !model.dead  {
        let rect = app.window_rect();
        for e in model.enemies.iter_mut() {
            if e.clone() == EMPTY_ENEMY && random::<f32>() > update.since_last.as_secs_f32() {
                e.0 = get_random_coord(rect.w());
                e.1 = rect.h()
            }

            if e.clone() != EMPTY_ENEMY {
                // e.1 -= 10.0 * update.since_last.as_secs_f32();
                if e.1 < rect.h() / 2.0 {
                    e.0 = EMPTY_ENEMY.0;
                    e.1 = EMPTY_ENEMY.1;
                }
            }

            if (app.mouse.x - e.0).hypot(app.mouse.y - e.1) < 20f32 {
                model.dead = true;
                println!("Died!");

                break;
            }
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);

    let draw = app.draw();
    let win = app.window_rect();

    model.enemies.iter()
        .filter(|pos| pos.clone().clone() != EMPTY_ENEMY)
        .for_each(|pos| {
            draw.ellipse()
                .x_y(pos.0, pos.1)
                .radius(4f32)
                .color(WHITE);
        });

    if model.dead {
        draw.text("Dead!")
            .font_size(64)
            .x_y(0.0, 0.0);
    } else {
        draw.ellipse()
            .x_y(app.mouse.x, app.mouse.y)
            .radius(16f32)
            .color(DARKGREEN);
    }


    draw.to_frame(app, &frame).unwrap();
}