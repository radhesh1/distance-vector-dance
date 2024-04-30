mod node;
mod rtpkt;

use node::Node;
use rtpkt::Rtpkt;

use crate::node::NodeType;

fn to_layer2(
    rtpkt: &Rtpkt,
    node0: &mut Node,
    node1: &mut Node,
    node2: &mut Node,
    node3: &mut Node,
) {
    // Unpacking
    // let source_id = rtpkt.source_id;
    let dest_id = rtpkt.dest_id;
    // let min_cost = rtpkt.min_cost;
    if dest_id == 0 {
        node0.rtupdate(rtpkt);
    } else if dest_id == 1 {
        node1.rtupdate(rtpkt);
    } else if dest_id == 2 {
        node2.rtupdate(rtpkt);
    } else if dest_id == 3 {
        node3.rtupdate(rtpkt);
    }
    
}

fn print_nodes(node0: &Node, node1: &Node, node2: &Node, node3: &Node) {
    println!("****************************************************");
    node0.print_distance_table();
    node1.print_distance_table();
    node2.print_distance_table();
    node3.print_distance_table();
    println!("****************************************************\n");
}

fn main() {
    let mut node0 = Node::new(0, NodeType::Node0);
    let mut node1 = Node::new(1, NodeType::Node1);
    let mut node2 = Node::new(2, NodeType::Node2);
    let mut node3 = Node::new(3, NodeType::Node3);

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

    node0.rtinit();
    node1.rtinit();
    node2.rtinit();
    node3.rtinit();

    let is_program_running = true;

    while is_program_running {
        if !node0.my_packets_to_send.is_empty() {
            while let Some(packet) = node0.my_packets_to_send.pop() {
                to_layer2(&packet, &mut node0, &mut node1, &mut node2, &mut node3);
                print_nodes(&node0, &node1, &node2, &node3);
            }
        } else if !node1.my_packets_to_send.is_empty() {
            while let Some(packet) = node1.my_packets_to_send.pop() {
                to_layer2(&packet, &mut node0, &mut node1, &mut node2, &mut node3);
                print_nodes(&node0, &node1, &node2, &node3);
            }
        } else if !node2.my_packets_to_send.is_empty() {
            while let Some(packet) = node2.my_packets_to_send.pop() {
                to_layer2(&packet, &mut node0, &mut node1, &mut node2, &mut node3);
                print_nodes(&node0, &node1, &node2, &node3);
            }
        } else if !node3.my_packets_to_send.is_empty() {
            while let Some(packet) = node3.my_packets_to_send.pop() {
                to_layer2(&packet, &mut node0, &mut node1, &mut node2, &mut node3);
                print_nodes(&node0, &node1, &node2, &node3);
            }
        } else {
            break;
        }
    }

    println!("-------------------Program done------------------");
}
