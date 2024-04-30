<<<<<<< HEAD
extern crate sdl2;

use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::Window;
use sdl2::ttf::{Font, Sdl2TtfContext};

mod node;
use node::*;

mod distance_table_gui;
use distance_table_gui::*;

mod rtpkt;
use rtpkt::*;

fn get_midpoint(x1: i32, y1: i32, x2: i32, y2: i32) -> Point {
    Point::new((x1 + x2) / 2, (y1 + y2) / 2)
}

fn to_layer2(rtpkt: &Rtpkt, node0: &mut Node, node1: &mut Node, node2: &mut Node, node3: &mut Node) {
    let dest_id = rtpkt.dest_id;
<<<<<<< HEAD
    // let min_cost = rtpkt.min_cost;
<<<<<<< HEAD
    if dest_id == 0 {
        node0.rtupdate(rtpkt);
    } else if dest_id == 1 {
        node1.rtupdate(rtpkt);
    } else if dest_id == 2 {
        node2.rtupdate(rtpkt);
    } else if dest_id == 3 {
        node3.rtupdate(rtpkt);
=======

=======
>>>>>>> 51e6567 (fixed cli again, have to look into gui)
    match dest_id {
        0 => node0.rtupdate(rtpkt),
        1 => node1.rtupdate(rtpkt),
        2 => node2.rtupdate(rtpkt),
        3 => node3.rtupdate(rtpkt),
        _ => (),
>>>>>>> 730ec94 (TUI version of distance-vector done)
    }
}

fn draw_text(canvas: &mut Canvas<Window>, text: &str, font: &Font, color: Color, x: i32, y: i32) -> Result<(), String> {
    let surface = font.render(text).blended(color).map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.create_texture_from_surface(&surface)?;
    let texture_query = texture.query();
    let dst = Rect::new(x, y, texture_query.width, texture_query.height);
    canvas.copy(&texture, None, dst)?;
    Ok(())
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let width = 1200;
    let height = 700;
    let window = video_subsystem.window("Distance Vector Routing Algorithm Visualization", width, height)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build()?;
    let texture_creator = canvas.texture_creator();

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font_path = "Arial.ttf"; // Path to your font file
    let font_size = 32;
    let font = ttf_context.load_font(font_path, font_size).map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;
    
    let mut is_program_running = true;

    // Create nodes 
    let mut node0 = Node::new(0, NodeType::Node0);
    let mut node1 = Node::new(1, NodeType::Node1);
    let mut node2 = Node::new(2, NodeType::Node2);
    let mut node3 = Node::new(3, NodeType::Node3);

    // Add neighbors
    node0.add_neighbor(node1.clone());
    node0.add_neighbor(node2.clone());
    node0.add_neighbor(node3.clone());

    node1.add_neighbor(node0.clone());
    node1.add_neighbor(node2.clone());

    node2.add_neighbor(node0.clone());
    node2.add_neighbor(node1.clone());
    node2.add_neighbor(node3.clone());

    node3.add_neighbor(node0.clone());
    node3.add_neighbor(node2.clone());

    // Initialise the nodes
    node0.rtinit();
    node1.rtinit();
    node2.rtinit();
    node3.rtinit();

    let mut distance_table_node0 = DistanceTableGUIObject::new(&mut canvas, 80, 80, 0, node0.clone())?;
    let mut distance_table_node1 = DistanceTableGUIObject::new(&mut canvas, 930, 80, 1, node1.clone())?;
    let mut distance_table_node2 = DistanceTableGUIObject::new(&mut canvas, 930, 400, 2, node2.clone())?;
    let mut distance_table_node3 = DistanceTableGUIObject::new(&mut canvas, 80, 400, 3, node3.clone())?;

    distance_table_node0.set_table();
    distance_table_node1.set_table();
    distance_table_node2.set_table();
    distance_table_node3.set_table();

    let mut mx = 0;
    let mut my = 0;
    let mut click = false; // for keeping track of mouse clicks
    let mut is_algorithm_done = false;
    let mut highlighted_connection = "";
    let mut message_receiver_id = None;
    
    while is_program_running {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    is_program_running = false;
                }
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, .. } => {
                    click = true;
                }
                _ => {}
            }
        }
    
        // Update logic
        let advance_button = Rect::new(500, 25, 200, 50);
    
        if advance_button.contains_point(Point::new(mx, my)) && !is_algorithm_done {
            if click {
                distance_table_node0.reset_change_matrix();
                distance_table_node3.reset_change_matrix();
                distance_table_node1.reset_change_matrix();
                distance_table_node2.reset_change_matrix();
                let mut got_packet = false;
                let mut packet = None;
    
                if !node0.my_packets_to_send.is_empty() {
                    packet = Some(node0.my_packets_to_send.remove(0));
                    to_layer2(&packet.unwrap(), &mut node0, &mut node1, &mut node2, &mut node3);
                    got_packet = true;
                } else if !node1.my_packets_to_send.is_empty() {
                    packet = Some(node1.my_packets_to_send.remove(0));
                    to_layer2(&packet.unwrap(), &mut node0, &mut node1, &mut node2, &mut node3);
                    got_packet = true;
                } else if !node2.my_packets_to_send.is_empty() {
                    packet = Some(node2.my_packets_to_send.remove(0));
                    to_layer2(&packet.unwrap(), &mut node0, &mut node1, &mut node2, &mut node3);
                    got_packet = true;
                } else if !node3.my_packets_to_send.is_empty() {
                    packet = Some(node3.my_packets_to_send.remove(0));
                    to_layer2(&packet.unwrap(), &mut node0, &mut node1, &mut node2, &mut node3);
                    got_packet = true;
                } else {
                    is_algorithm_done = true;
                }
    
                if got_packet {
                    let src_id = packet.unwrap().source_id;
                    let dest_id = packet.unwrap().dest_id;
                    
                    if (src_id == 0 && dest_id == 1) || (src_id == 1 && dest_id == 0) {
                        highlighted_connection = "0->1";
                    } else if (src_id == 0 && dest_id == 3) || (src_id == 3 && dest_id == 0) {
                        highlighted_connection = "0->3";
                    } else if (src_id == 0 && dest_id == 2) || (src_id == 2 && dest_id == 0) {
                        highlighted_connection = "0->2";
                    } else if (src_id == 1 && dest_id == 2) || (src_id == 2 && dest_id == 1) {
                        highlighted_connection = "1->2";
                    } else if (src_id == 3 && dest_id == 2) || (src_id == 2 && dest_id == 3) {
                        highlighted_connection = "3->2";
                    }
    
                    message_receiver_id = Some(dest_id);
                }
            }
        }
    
        // Update distance table GUI objects
        distance_table_node0.update()?;
        distance_table_node3.update()?;
        distance_table_node2.update()?;
        distance_table_node1.update()?;
    
        // Drawing logic
        canvas.set_draw_color(Color::RGB(235, 224, 216));
        canvas.clear();
    
        if !is_algorithm_done {
            // Draw advance button
            canvas.set_draw_color(Color::RGB(140, 135, 135));
            canvas.draw_rect(advance_button)?;
            draw_text(&mut canvas, "Advance", &font, Color::RGB(255, 255, 255), 550, 30)?;
        } else {
            draw_text(&mut canvas, "Algorithm Complete", &font, Color::RGB(0, 0, 0), 475, 30)?;
        }
    
        // Draw the graph
        let x_offset = 200;
        let y_offset = 100;
        let node_radius = 30;
        let node_thickness = 5;

        // Draw nodes
        // Node 0
        let top_left_circle_x = (width / 2 - x_offset) as i32;
        let top_left_circle_y = (height / 2 - y_offset) as i32;
        if message_receiver_id != Some(0) {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
        } else {
            canvas.set_draw_color(Color::RGB(45, 93, 204)); // Blue color
        }
        canvas.draw_circle(Point::new(top_left_circle_x, top_left_circle_y), node_radius as i16, node_thickness as u8)?;

        // Node 3
        let bottom_left_circle_x = (width / 2 - x_offset) as i32;
        let bottom_left_circle_y = (height / 2 + y_offset) as i32;
        if message_receiver_id != Some(3) {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
        } else {
            canvas.set_draw_color(Color::RGB(45, 93, 204)); // Blue color
        }
        canvas.draw_circle(Point::new(bottom_left_circle_x, bottom_left_circle_y), node_radius as i16, node_thickness as u8)?;

        // Node 2
        let bottom_right_circle_x = (width / 2 + x_offset) as i32;
        let bottom_right_circle_y = (height / 2 + y_offset) as i32;
        if message_receiver_id != Some(2) {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
        } else {
            canvas.set_draw_color(Color::RGB(45, 93, 204)); // Blue color
        }
        canvas.draw_circle(Point::new(bottom_right_circle_x, bottom_right_circle_y), node_radius as i16, node_thickness as u8)?;

        // Node 1
        let top_right_circle_x = (width / 2 + x_offset) as i32;
        let top_right_circle_y = (height / 2 - y_offset) as i32;
        if message_receiver_id != Some(1) {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
        } else {
            canvas.set_draw_color(Color::RGB(45, 93, 204)); // Blue color
        }
        canvas.draw_circle(Point::new(top_right_circle_x, top_right_circle_y), node_radius as i16, node_thickness as u8)?;

        // Draw node connections
        let connection_thickness = 5;
        let connection_offset = node_radius;

        // Node 0 -> 1
        let connection_color = if highlighted_connection == "0->1" { Color::RGB(45, 93, 204) } else { Color::RGB(0, 0, 0) };
        canvas.set_draw_color(connection_color);
        canvas.draw_line(Point::new(top_left_circle_x + connection_offset, top_left_circle_y), Point::new(top_right_circle_x - connection_offset, top_right_circle_y))?;

        // Node 0 -> 3
        let connection_color = if highlighted_connection == "0->3" { Color::RGB(45, 93, 204) } else { Color::RGB(0, 0, 0) };
        canvas.set_draw_color(connection_color);
        canvas.draw_line(Point::new(top_left_circle_x, top_left_circle_y + connection_offset), Point::new(bottom_left_circle_x, bottom_left_circle_y - connection_offset))?;

        // Node 0 -> 2
        let connection_color = if highlighted_connection == "0->2" { Color::RGB(45, 93, 204) } else { Color::RGB(0, 0, 0) };
        canvas.set_draw_color(connection_color);
        canvas.draw_line(Point::new(top_left_circle_x + connection_offset / 2 + 5, top_left_circle_y + connection_offset / 2), Point::new(bottom_right_circle_x - connection_offset / 2 - 5, bottom_right_circle_y - connection_offset / 2))?;

        // Node 1 -> 2
        let connection_color = if highlighted_connection == "1->2" { Color::RGB(45, 93, 204) } else { Color::RGB(0, 0, 0) };
        canvas.set_draw_color(connection_color);
        canvas.draw_line(Point::new(top_right_circle_x, top_right_circle_y + connection_offset), Point::new(bottom_right_circle_x, bottom_right_circle_y - connection_offset))?;

        // Node 3 -> 2
        let connection_color = if highlighted_connection == "3->2" { Color::RGB(45, 93, 204) } else { Color::RGB(0, 0, 0) };
        canvas.set_draw_color(connection_color);
        canvas.draw_line(Point::new(bottom_left_circle_x + connection_offset, bottom_left_circle_y), Point::new(bottom_right_circle_x - connection_offset, bottom_right_circle_y))?;


        // Your drawing logic here
        // Draw node labels
        draw_text(&mut canvas, "0", &font, Color::RGB(0, 0, 0), top_left_circle_x, top_left_circle_y)?;
        draw_text(&mut canvas, "1", &font, Color::RGB(0, 0, 0), top_right_circle_x, top_right_circle_y)?;
        draw_text(&mut canvas, "2", &font, Color::RGB(0, 0, 0), bottom_right_circle_x, bottom_right_circle_y)?;
        draw_text(&mut canvas, "3", &font, Color::RGB(0, 0, 0), bottom_left_circle_x, bottom_left_circle_y)?;

        // Draw distance table GUI objects
        distance_table_node0.draw(&mut canvas)?;
        distance_table_node1.draw(&mut canvas)?;
        distance_table_node2.draw(&mut canvas)?;
        distance_table_node3.draw(&mut canvas)?;

        canvas.present();
    }

    Ok(())
}

fn main(){
    println!("Hello, World!");
}

