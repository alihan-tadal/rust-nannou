// use nannou::mesh::*;
use nannou::prelude::*;

struct Model {}
fn main() {
    nannou::app(model)
        .event(event)
        .simple_window(view)
        .run();
}

fn model(_: &App) -> Model {
    Model {}
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}
fn view(_app: &App, _model: &Model, _frame: Frame) {}
