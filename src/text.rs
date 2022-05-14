use crate::{Model, FONT_SIZE};
use nannou::prelude::{text, Vec2};
use std::f32::consts::TAU as TWO_PI;

pub fn show_current_values(model: &Model) {
    show_list(model);
    show_moving_values(model);
}

fn show_moving_values(model: &Model) {
    let angle = if model.angle < 0.0 {
        TWO_PI + model.angle
    } else {
        model.angle
    };

    let text = format!(
        "\
Sin: {:.2}",
        angle.sin()
    );

    model
        .draw
        .text(text.as_str())
        // Division by forty to fit in a single square
        .x_y(-model.diameter / 40.0, model.pos.y)
        .wh(Vec2::ZERO)
        .no_line_wrap()
        .justify(text::Justify::Right)
        .align_text_middle_y()
        .font_size(FONT_SIZE)
        .rgba(1.0, 1.0, 1.0, 0.5);

    let text = format!(
        "\
Cos: {:.2}",
        angle.cos()
    );

    model
        .draw
        .text(text.as_str())
        // Division by forty to fit in a single square
        .x_y(
            model.pos.x - model.diameter / 20.0 * model.pos.x.signum(),
            -model.diameter / 40.0,
        )
        .wh(Vec2::ZERO)
        .no_line_wrap()
        .justify(text::Justify::Center)
        .align_text_middle_y()
        .font_size(FONT_SIZE)
        .rgba(1.0, 1.0, 1.0, 0.5);

    let text = format!(
        "\
Tan: {:.2}",
        angle.tan()
    );

    model
        .draw
        .text(text.as_str())
        // Division by forty to fit in a single square
        .x_y(
            model.radius() * model.pos.x.signum() + model.diameter / 20.0 * model.pos.x.signum(),
            -model.diameter / 40.0,
        )
        .wh(Vec2::ZERO)
        .no_line_wrap()
        .justify(text::Justify::Center)
        .align_text_middle_y()
        .font_size(FONT_SIZE)
        .rgba(1.0, 1.0, 1.0, 0.5);
}

fn show_list(model: &Model) {
    let win_rect = model.win_rect;
    let angle = if model.angle < 0.0 {
        TWO_PI + model.angle
    } else {
        model.angle
    };
    let text = format!(
        "\
Angle: {:.3} | {:.3}°
Sine: {:.3}
Cosine: {:.3}
Tangent: {:.3}",
        angle,
        angle.to_degrees(),
        angle.sin(),
        angle.cos(),
        angle.tan(),
    );
    model
        .draw
        .text(text.as_str())
        .x_y(win_rect.left() + 2.0, win_rect.top() - 2.0)
        .wh(Vec2::ZERO)
        .no_line_wrap()
        .justify(text::Justify::Left)
        .align_text_top()
        .font_size(FONT_SIZE)
        .rgba(1.0, 1.0, 1.0, 0.5);
}
