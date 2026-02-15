use marnd::MOSEntropy;

fn main() {
    for i in 0..10 {
        println!("#{} Read {:X}", i, MOSEntropy::next_u64().unwrap());
    }
}
