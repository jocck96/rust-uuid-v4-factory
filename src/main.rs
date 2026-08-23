use std::time::SystemTime;

fn generate_uuid_v4() -> String {
    // Basic pseudo-random UUID generation for lightweight demo
    let mut bytes = [0u8; 16];
    let seed = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
    for i in 0..16 {
        bytes[i] = ((seed >> (i * 8)) & 0xff) as u8;
    }
    
    // Set version to 4 (random)
    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    // Set variant to 10xx
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    
    format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        bytes[0], bytes[1], bytes[2], bytes[3],
        bytes[4], bytes[5],
        bytes[6], bytes[7],
        bytes[8], bytes[9],
        bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]
    )
}

fn main() {
    println!("{}", generate_uuid_v4());
}
