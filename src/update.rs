use crate::drawing::*;
use crate::model::Model;
use crate::text::*;
use nannou::prelude::*;

// Update the state of our application every frame
pub fn update(app: &App, model: &mut Model, _update: Update) {
    if app.mouse.buttons.left().is_down() {
        model.angle = app.mouse.y.atan2(app.mouse.x);
    }
    model.pos = Vec2::new(
        model.angle.cos() * model.radius(),
        model.angle.sin() * model.radius(),
    );
}

// Draw our stuff to the screen every frame
pub fn view(app: &App, model: &Model, frame: Frame) {
    // Draw background
    model.draw.background().rgb(0.1569, 0.1569, 0.1569);

    draw_grid(model);
    draw_circle(model);
    draw_arc(model);
    draw_cos(model);
    draw_sin(model);
    draw_tan(model);
    draw_arm(model);

    show_axis_labels(model);

    show_current_values(model);

    // Push stuff to screen
    model
        .draw
        .to_frame(app, &frame)
        .expect("Unable to draw to the frame");
}
