use marnd::{MOSEntropy, MOSRng};

fn main() {
    for i in 0..10 {
        println!("#{} Read {:X}", i, MOSRng.next_u64().unwrap());
    }
}