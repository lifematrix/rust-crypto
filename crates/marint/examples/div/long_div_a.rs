use marint::MarInt;

fn main() {
    // Little-endian limbs: [lo, hi, ...]
    //    let a: Vec<u64> = vec![0xFFFF_FFFF_FFFF_FFFF, 0x0000_0000_0000_0002]; // example
    //    let b: Vec<u64> = vec![3]; // divide by 3
    let a: Vec<u64> = vec![2, 3, 4, 5];
    let b: Vec<u64> = vec![7, 8];

    let (q, r) = MarInt::longdiv_limbs(&a, &b);

    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("q = {:?}", q);
    println!("r = {:?}", r);
}
