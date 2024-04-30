use ggez::{graphics::{self, Color, Font, Text}, nalgebra::Point2};

fn draw_text(
    text: &str,
    font: &Font,
    color: Color,
    ctx: &mut ggez::Context,
    x: f32,
    y: f32,
) -> ggez::GameResult {
    let text_obj = Text::new(text);
    let text_width = text_obj.width(ctx) as f32;
    let text_height = text_obj.height(ctx) as f32;
    let text_position = Point2::new(x - text_width / 2.0, y - text_height / 2.0);
    let text_params = graphics::DrawParam::new().dest(text_position).color(color);
    graphics::draw(ctx, &text_obj, text_params)
}

fn draw_text_left_to_right(
    text: &str,
    font: &Font,
    color: Color,
    ctx: &mut ggez::Context,
    x: f32,
    y: f32,
) -> ggez::GameResult {
    let text_obj = Text::new(text);
    let text_params = graphics::DrawParam::new().dest(Point2::new(x, y)).color(color);
    graphics::draw(ctx, &text_obj, text_params)
}
