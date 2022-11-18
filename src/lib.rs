use rand::Rng;

pub fn gen_pass (chars:String, len: u16) -> String {
    const UPPER_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const LOWER_CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    const NUMERIC_CHARS: &[u8] = b"0123456789";
    const PUN_CHARS: &[u8] = b")(*&^%$#@!~";
    let mut bag: Vec<u8> = Vec::new();
    
    for char in chars.split("").filter(|&x| !x.is_empty()) {
        match char {
            "u" => {
                bag.extend_from_slice(UPPER_CHARS);
            },
            "l" => {
                bag.extend_from_slice(LOWER_CHARS);
            },
            "n" => {
                bag.extend_from_slice(NUMERIC_CHARS);
            },
            "p" => {
                bag.extend_from_slice(PUN_CHARS);
            },
            _ => {
                panic!("Incorrect chars");
            }
        }
    }
    let mut rng = rand::thread_rng();

    let password: String = (0..len)
        .map(|_| {
            let idx = rng.gen_range(0..bag.len());
            bag[idx] as char
        })
        .collect();

    password
}
