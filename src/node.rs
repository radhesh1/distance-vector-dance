use crate::rtpkt::Rtpkt;

#[derive(Clone)]
pub enum NodeType {
    Node0,
    Node1,
    Node2,
    Node3,
}

#[derive(Clone)]
pub struct Node {
    pub id: usize,
    pub node_type: NodeType,
    pub(crate) neighbors: Vec<Node>,
    pub(crate) distance_table: [[i32; 4]; 4],
    pub my_packets_to_send: Vec<Rtpkt>,
}

impl Node {
    pub fn new(id: usize, node_type: NodeType) -> Self {
        Node {
            id,
            node_type,
            neighbors: Vec::new(),
            distance_table: [
                [9999, 9999, 9999, 9999],
                [9999, 9999, 9999, 9999],
                [9999, 9999, 9999, 9999],
                [9999, 9999, 9999, 9999],
            ],
            my_packets_to_send: Vec::new(),
        }
    }

    pub fn add_neighbor(&mut self, neighbors: Node) {
        self.neighbors.push(neighbors);
    }

    pub fn rtinit(&mut self) {
        match self.node_type {
            NodeType::Node0 => {
                self.distance_table = [
                    [0, 1, 3, 7],
                    [9999, 9999, 9999, 9999],
                    [9999, 9999, 9999, 9999],
                    [9999, 9999, 9999, 9999],
                ];
            }
            NodeType::Node1 => {
                self.distance_table = [
                    [9999, 9999, 9999, 9999],
                    [1, 0, 1, 9999],
                    [9999, 9999, 9999, 9999],
                    [9999, 9999, 9999, 9999],
                ];
            }
            NodeType::Node2 => {
                self.distance_table = [
                    [9999, 9999, 9999, 9999],
                    [9999, 9999, 9999, 9999],
                    [3, 1, 0, 2],
                    [9999, 9999, 9999, 9999],
                ];
            }
            NodeType::Node3 => {
                self.distance_table = [
                    [9999, 9999, 9999, 9999],
                    [9999, 9999, 9999, 9999],
                    [9999, 9999, 9999, 9999],
                    [7, 9999, 2, 9999],
                ];
            }
        }
        for n in &self.neighbors {
            let current_id = n.id;
            let min_cost = [
                self.distance_table[self.id][0],
                self.distance_table[self.id][1],
                self.distance_table[self.id][2],
                self.distance_table[self.id][3],
            ];
            self.my_packets_to_send
                .push(Rtpkt::new(self.id, current_id, min_cost));
        }
    }

    pub fn rtupdate(&mut self, rtpacket: &Rtpkt) {
        let source_id = rtpacket.source_id;
        let dest_id = rtpacket.dest_id;
        let min_cost = rtpacket.min_cost;

        let row = source_id;
        for col in 0..4 {
            self.distance_table[row][col] = min_cost[col];
        }

        let mut other_rows = Vec::new();
        if dest_id != 0 {
            other_rows.push(0);
        }
        if dest_id != 1 {
            other_rows.push(1);
        }
        if dest_id != 2 {
            other_rows.push(2);
        }
        if dest_id != 3 {
            other_rows.push(3);
        }

        for col in 0..4 {
            for &other_r in &other_rows {
                if self.distance_table[dest_id][other_r] + self.distance_table[other_r][col]
                    < self.distance_table[dest_id][col]
                {
                    self.distance_table[dest_id][col] =
                        self.distance_table[dest_id][other_r] + self.distance_table[other_r][col];

                    for n in &self.neighbors {
                        let current_id = n.id;
                        let min_cost = [
                            self.distance_table[self.id][0],
                            self.distance_table[self.id][1],
                            self.distance_table[self.id][2],
                            self.distance_table[self.id][3],
                        ];
                        self.my_packets_to_send
                            .push(Rtpkt::new(self.id, current_id, min_cost));
                    }
                }
            }
        }
    }

    pub fn print_distance_table(&self) {
        println!("\n***Node {}***", self.id);
        for row in &self.distance_table {
            println!("{:?}", row);
        }
    }
}
