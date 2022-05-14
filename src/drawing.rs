use crate::{Model, FONT_SIZE};
use nannou::color;
use nannou::prelude::*;

pub fn show_axis_labels(model: &Model) {
    model
        .draw
        .text("1")
        // Dividing by 40 means it will be in the middle of a square
        .x_y(
            0.0 + model.diameter / 40.0,
            model.radius() + model.diameter / 40.0,
        )
        .font_size(FONT_SIZE)
        .rgba(1.0, 1.0, 1.0, 0.5);

    model
        .draw
        .text("1")
        // Dividing by 40 means it will be in the middle of a square
        .x_y(
            model.radius() + model.diameter / 40.0,
            0.0 + model.diameter / 40.0,
        )
        .font_size(FONT_SIZE)
        .rgba(1.0, 1.0, 1.0, 0.5);

    model
        .draw
        .text("-1")
        // Dividing by 40 means it will be in the middle of a square
        .x_y(
            -model.radius() + model.diameter / 40.0,
            0.0 + model.diameter / 40.0,
        )
        .font_size(FONT_SIZE)
        .rgba(1.0, 1.0, 1.0, 0.5);

    model
        .draw
        .text("-1")
        // Dividing by 40 means it will be in the middle of a square
        .x_y(
            0.0 + model.diameter / 40.0,
            -model.radius() + model.diameter / 40.0,
        )
        .font_size(FONT_SIZE)
        .rgba(1.0, 1.0, 1.0, 0.5);
}

pub fn draw_cos(model: &Model) {
    model
        .draw
        .line()
        .points(Vec2::new(0.0, 0.0), Vec2::new(model.pos.x, 0.0))
        .rgb(1.0, 0.0, 0.0)
        .stroke_weight(3.0);

    model
        .draw
        .line()
        .points(
            Vec2::new(model.pos.x, model.pos.y),
            Vec2::new(model.pos.x, 0.0),
        )
        .rgba(0.0, 0.0, 0.0, 0.5)
        .stroke_weight(3.0);

    draw_cross(model, model.pos.x, 0.0);
}

pub fn draw_sin(model: &Model) {
    model
        .draw
        .line()
        .points(Vec2::new(0.0, 0.0), Vec2::new(0.0, model.pos.y))
        .rgb(0.0, 1.0, 0.0)
        .stroke_weight(3.0);

    model
        .draw
        .line()
        .points(
            Vec2::new(model.pos.x, model.pos.y),
            Vec2::new(0.0, model.pos.y),
        )
        .rgba(0.0, 0.0, 0.0, 0.5)
        .stroke_weight(3.0);

    draw_cross(model, 0.0, model.pos.y);
}

pub fn draw_tan(model: &Model) {
    if model.angle.tan() < 1000.0 {
        // Positive x means right, negative means left
        let side = model.pos.x.signum();
        let radius = model.radius();
        let tan_angle = model.angle.tan();
        let top = model.win_rect.top();
        let bottom = model.win_rect.bottom();

        model
            .draw
            .line()
            .points(
                Vec2::new(radius * side, top),
                Vec2::new(radius * side, bottom),
            )
            .rgba(0.0, 0.0, 0.0, 1.0)
            .stroke_weight(3.0);

        model
            .draw
            .line()
            .points(
                Vec2::new(radius * side, 0.0),
                Vec2::new(radius * side, tan_angle * radius * side),
            )
            .rgba(0.0, 0.0, 1.0, 1.0)
            .stroke_weight(3.0);

        draw_cross(model, radius * side, tan_angle * radius * side);

        model
            .draw
            .line()
            .points(
                Vec2::new(model.pos.x, model.pos.y),
                Vec2::new(radius * side, radius * tan_angle * side),
            )
            .rgba(0.0, 0.0, 0.0, 0.5)
            .stroke_weight(3.0);
    }
}

pub fn draw_cross(model: &Model, x: f32, y: f32) {
    let size = model.diameter / 80.0;

    model
        .draw
        .line()
        .points(Vec2::new(x - size, y - size), Vec2::new(x + size, y + size))
        .rgba(0.0, 0.0, 0.0, 0.5)
        .stroke_weight(3.0);

    model
        .draw
        .line()
        .points(Vec2::new(x + size, y - size), Vec2::new(x - size, y + size))
        .rgba(0.0, 0.0, 0.0, 0.5)
        .stroke_weight(3.0);
}

pub fn draw_arm(model: &Model) {
    model
        .draw
        .line()
        .points(Vec2::new(0.0, 0.0), Vec2::new(model.radius(), 0.0))
        .rotate(model.angle)
        .stroke_weight(4.0);

    draw_cross(model, model.pos.x, model.pos.y);
}

pub fn draw_arc(model: &Model) {
    let mut points = vec![];

    let mut i = 0.0;
    if model.angle.is_positive() {
        while i < model.angle {
            points.push(Vec2::new(
                i.cos() * model.radius() / 10.0,
                i.sin() * model.radius() / 10.0,
            ));
            i += 0.05;
        }
    } else {
        while i > model.angle {
            points.push(Vec2::new(
                i.cos() * model.radius() / 10.0,
                i.sin() * model.radius() / 10.0,
            ));
            i -= 0.05;
        }
    }

    model
        .draw
        .polyline()
        .stroke_weight(3.0)
        .rgba(0.0, 0.0, 0.0, 0.5)
        .join_round()
        .points(points);
}

pub fn draw_circle(model: &Model) {
    model
        .draw
        .ellipse()
        .no_fill()
        .radius(model.radius())
        .stroke_weight(4.0)
        .stroke(color::BLACK);
}

pub fn draw_grid(model: &Model) {
    let left = model.win_rect.left();
    let right = model.win_rect.right();
    let top = model.win_rect.top();
    let bottom = model.win_rect.bottom();

    let distance = model.diameter / 20.0;

    model
        .draw
        .line()
        .points(Vec2::new(left, 0.0), Vec2::new(right, 0.0))
        .stroke_weight(4.0);
    model
        .draw
        .line()
        .points(Vec2::new(0.0, top), Vec2::new(0.0, bottom))
        .stroke_weight(4.0);

    let mut x = 0.0;
    while x > left {
        model
            .draw
            .line()
            .points(Vec2::new(x, top), Vec2::new(x, bottom))
            .rgba(0.0, 0.0, 0.0, 0.5)
            .stroke_weight(2.0);
        x -= distance;
    }
    let mut x = 0.0;
    while x < right {
        model
            .draw
            .line()
            .points(Vec2::new(x, top), Vec2::new(x, bottom))
            .rgba(0.0, 0.0, 0.0, 0.5)
            .stroke_weight(2.0);
        x += distance;
    }
    let mut y = 0.0;
    while y > bottom {
        model
            .draw
            .line()
            .points(Vec2::new(left, y), Vec2::new(right, y))
            .rgba(0.0, 0.0, 0.0, 0.5)
            .stroke_weight(2.0);
        y -= distance;
    }
    let mut y = 0.0;
    while y < top {
        model
            .draw
            .line()
            .points(Vec2::new(left, y), Vec2::new(right, y))
            .rgba(0.0, 0.0, 0.0, 0.5)
            .stroke_weight(2.0);
        y += distance;
    }
}
