// License key generator for testing
// Run with: cargo run --bin keygen

use rand::Rng;

fn main() {
    let prefixes = ["SCPR", "STRM", "CLIP", "PRO1"];
    
    println!("=== Stream Clipper License Key Generator ===\n");
    
    for prefix in prefixes {
        let key = generate_valid_key(prefix);
        println!("{}: {}", prefix, key);
    }
    
    println!("\nUse any of these keys to activate Pro features.");
}

fn generate_valid_key(prefix: &str) -> String {
    let mut rng = rand::rng();
    let chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().collect();
    let middle: String = (0..12).map(|_| chars[rng.random_range(0..chars.len())]).collect();
    let data = format!("{}{}", prefix, middle);
    let checksum = calc_checksum(&data);
    let full = format!("{}{}", data, checksum);
    format!("{}-{}-{}-{}", &full[0..5], &full[5..10], &full[10..15], &full[15..20])
}

fn calc_checksum(d: &str) -> String {
    let mut s: u32 = 0;
    for (i, c) in d.chars().enumerate() {
        let v = c as u32;
        let w = ((i + 1) * 7) as u32;
        s = s.wrapping_add(v.wrapping_mul(w));
    }
    let r = s % 0xFFFF;
    format!("{:04X}", r)
}
