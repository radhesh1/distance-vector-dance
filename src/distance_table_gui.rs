mod node;
mod draw_text;

use ggez::{graphics, Context, GameResult};
use ggez::graphics::{DrawParam, Text, Color};
use node::Node;

struct DistanceTableGUIObject {
    x: f32,
    y: f32,
    id: i32,
    my_node: Node,
    distance_table: Vec<Vec<i32>>,
    change_matrix: Vec<Vec<i32>>,
    font_normal: graphics::Font,
    color_black: Color,
    color_blue: Color,
    padding: f32,
    width: f32,
    height: f32,
}

impl DistanceTableGUIObject {
    fn new(ctx: &mut Context, x: f32, y: f32, id: i32, node: Node, font_normal: graphics::Font) -> Self {
        DistanceTableGUIObject {
            x,
            y,
            id,
            my_node: node,
            distance_table: vec![
                vec![1, 2, 9999, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12],
                vec![13, 14, 15, 16],
            ],
            change_matrix: vec![
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
            ],
            font_normal,
            color_black: Color::new(0.0, 0.0, 0.0, 1.0),
            color_blue: Color::new(0.176, 0.365, 0.8, 1.0),
            padding: 50.0,
            width: 190.0,
            height: 190.0,
        }
    }

    fn reset_change_matrix(&mut self) {
        self.change_matrix = vec![
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];
    }

    fn set_table(&mut self) {
        for row in 0..4 {
            for col in 0..4 {
                self.distance_table[row][col] = self.my_node.distance_table[row][col];
            }
        }
    }

    fn update(&mut self) {
        for row in 0..4 {
            for col in 0..4 {
                if self.distance_table[row][col] != self.my_node.distance_table[row][col] {
                    self.change_matrix[row][col] = 1;
                }
                self.distance_table[row][col] = self.my_node.distance_table[row][col];
            }
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Drawing logic here
        self.draw_text_left_to_right(ctx, &self.id.to_string(), self.x, self.y, self.color_black)?;

        // Draw headers
        let headers = ["0", "1", "2", "3"];
        for (i, header) in headers.iter().enumerate() {
            self.draw_text_left_to_right(ctx, header, self.x + self.padding * (i as f32 + 1.0), self.y, self.color_black)?;
            self.draw_text_left_to_right(ctx, header, self.x, self.y + self.padding * (i as f32 + 1.0), self.color_black)?;
        }

        // Draw chart lines
        graphics::set_color(ctx, self.color_black)?;
        graphics::line(ctx, &[
            [self.x + self.padding - 16.0, self.y],
            [self.x + self.padding - 16.0, self.y + self.padding + self.height]
        ])?;
        graphics::line(ctx, &[
            [self.x, self.y + self.padding - 8.0],
            [self.x + self.padding + self.width, self.y + self.padding - 8.0]
        ])?;

        // Draw "from" and "to" labels
        self.draw_text_left_to_right(ctx, "from", self.x - 70.0, self.y + self.height / 2.0 + 20.0, self.color_black)?;
        self.draw_text_left_to_right(ctx, "to", self.x + self.width / 2.0 + 30.0, self.y - 40.0, self.color_black)?;

        // Draw distance table values
        for row in 0..4 {
            for col in 0..4 {
                let val = self.distance_table[col][row];
                let str_val = if val == 9999 { "inf." } else { &val.to_string() };
                self.draw_text_left_to_right(ctx, str_val, self.x + self.padding + self.padding * row as f32, self.y + self.padding + self.padding * col as f32, self.color_black)?;
            }
        }

        // Draw distance table changed values
        for row in 0..4 {
            for col in 0..4 {
                if self.change_matrix[col][row] == 1 {
                    let val = self.distance_table[col][row];
                    let str_val = if val == 9999 { "inf." } else { &val.to_string() };
                    self.draw_text_left_to_right(ctx, str_val, self.x + self.padding + self.padding * row as f32, self.y + self.padding + self.padding * col as f32, self.color_blue)?;
                }
            }
        }

        Ok(())
    }
}
