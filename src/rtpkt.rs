// rtpkt.rs

#[derive(Clone)]
pub(crate) struct Rtpkt {
    pub source_id: usize,
    pub dest_id: usize,
    pub min_cost: [i32; 4],
}

impl Rtpkt {
    pub fn new(source_id: usize, dest_id: usize, min_cost: [i32; 4]) -> Rtpkt {
        Rtpkt {
            source_id,
            dest_id,
            min_cost,
        }
    }
}
