
fn format_f32_exp(exp: u16) -> String {
    if exp > 127 {
        format!("{:08b}", exp - 127)
    } else {
        format!("-{:08b}", 127 - exp)
    }
}

fn parse_f32(d: f32) {
    let bits = d.to_bits();
    let sign = ((bits >> 31) != 0) as u8;
    let exponent = ((bits >> 23) & 0xFF) as u16;
    let fraction = (bits & 0x7FFFFF) as u32;

    println!("the number : {}, bits: {:032b}", d, bits);
    println!("sign: {:b}", sign);
    println!("exponent: {:08b}, {} without bias", exponent, format_f32_exp(exponent));
    println!("fraction: {:023b}", fraction);

}


fn format_f64_exp(exp: u16) -> String {
    if exp > 1023 {
        format!("{:011b}", exp - 1023)
    } else {
        format!("-{:011b}", 1023 - exp)
    }
}

fn parse_f64(d: f64) {
    let bits = d.to_bits();
    let sign = ((bits >> 63) != 0) as u8;
    let exponent = ((bits >> 52) & 0x7FF) as u16; // 11 bits
    let fraction = (bits & 0xF_FFFF_FFFF_FFFF) as u64;

    println!("the number : {}, bits: {:064b}", d, bits);
    println!("sign: {:b}", sign);
    println!("exponent: {:011b}, {} without bias", exponent, format_f64_exp(exponent));
    println!("fraction: {:052b}", fraction);

}

// fn main() {
//     let d32: f32 = 0.15625;

//     let bs = d32.to_ne_bytes();  // the default endianness is little endian.

//     println!("bs: {}", bs.iter().map(|b| format!("{:08b}", b)).collect::<Vec<String>>().join(" "));

//     let bits = d32.to_bits();

//     println!("bits: {}", d32.to_bits());
// }

// fn main() {
//     let d32: f32 = 0.15625;
//     parse_f32(d32);

//     println!("{}", "-".repeat(80));

//     let d64: f64 = 0.15625;
//     parse_f64(d64);
// }
fn main() {
    let x = u64::MAX;
    let xd = x as f64;
    println!("MAX of u64: {:016X}", x);
    parse_f64(xd);
    println!("{}", "-".repeat(80));

    let y = u32::MAX;
    let yd = y as f32;
    println!("MAX of u32: {:08X}", y);
    parse_f32(yd);

    {
        println!("{:=<80}", "");
        let s = 1u64 << 53;
        let s1 = 1.0 / (s as f64);
        let s2 = (1.0 / s1) as u64;
        let eq = s == s2;
        println!("s={s} s1={s1} s2={s2} {eq}");

        let x = u64::MAX;
        // ffffffffffffffff
        println!("max of u64 = {x:x}");
        for i in 0..64 {
            println!("{}", "~".repeat(80));
            let x1 = x >> i; 
            let y1 = x1 as f64;
            let y2 = y1 as u64;
            let eq = x1 == y2;
            println!("i={i} x1={x1:x} y1={y1:.12} y2={y2:x} {eq}");
            parse_f64(y1);
            println!("{x1:064b}, {y2:064b}");
        }
    }

    println!("{}", "-".repeat(80));
    let z = 2.0f64.powi(64);
    parse_f64(z);
    let zu = z as u64;
    println!("zu = {:016X}", zu);

    // let zuu = z.try_into::<u64>().ok_or("out of range");
    // let zuu = TryInto::<u64>::try_into(z).ok_or("out of range");
    // println!("zu = {:016X}", zuu);
}