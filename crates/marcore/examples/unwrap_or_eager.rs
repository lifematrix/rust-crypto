fn expensive_fallback() -> u64 {
    println!("⚠️  expensive_fallback() CALLED!");
    42
}

fn main() {
    println!("--- Case 1: unwrap_or (eager) with Some ---");
    let opt = Some(10u64);
    let v = opt.unwrap_or(expensive_fallback());
    println!("Result = {}", v);

    println!("\n--- Case 2: unwrap_or_else (lazy) with Some ---");
    let opt = Some(20u64);
    let v = opt.unwrap_or_else(|| expensive_fallback());
    println!("Result = {}", v);

    println!("\n--- Case 3: unwrap_or (eager) with None ---");
    let opt: Option<u64> = None;
    let v = opt.unwrap_or(expensive_fallback());
    println!("Result = {}", v);

    println!("\n--- Case 4: unwrap_or_else (lazy) with None ---");
    let opt: Option<u64> = None;
    let v = opt.unwrap_or_else(|| expensive_fallback());
    println!("Result = {}", v);

    println!("\n--- Case 5: unwrap_or_else (lazy) with Some ---");
    let opt = Some(50u64);
    let v = opt.unwrap_or_else(|| 55);
    println!("Result = {}", v);
}