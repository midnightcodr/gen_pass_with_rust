use rand::RngExt;

pub fn gen_pass(chars: String, len: u16) -> Result<String, &'static str> {
    const UPPER_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const LOWER_CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    const NUMERIC_CHARS: &[u8] = b"0123456789";
    const PUN_CHARS: &[u8] = b")(*&^%$#@!~";
    let mut bag: Vec<u8> = Vec::new();

    for char in chars.split("").filter(|&x| !x.is_empty()) {
        match char {
            "u" => {
                bag.extend_from_slice(UPPER_CHARS);
            }
            "l" => {
                bag.extend_from_slice(LOWER_CHARS);
            }
            "n" => {
                bag.extend_from_slice(NUMERIC_CHARS);
            }
            "p" => {
                bag.extend_from_slice(PUN_CHARS);
            }
            _ => {
                return Err("Incorrect values for param --chars");
            }
        }
    }
    let mut rng = rand::rng();

    let bag_len = bag.len();
    let password: String = (0..len)
        .map(|_| {
            let idx = rng.random_range(0..bag_len);
            bag[idx] as char
        })
        .collect();

    Ok(password)
}
