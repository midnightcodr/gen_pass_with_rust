use rand::Rng;

pub fn gen_pass (chars:String, len: u8) -> String {
    const upper_chars: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const lower_chars: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    const numeric_chars: &[u8] = b"0123456789";
    const pun_chars: &[u8] = b")(*&^%$#@!~";
    let mut bag: Vec<u8> = Vec::new();
    
    for char in chars.split("").filter(|&x| !x.is_empty()) {
        match char {
            "u" => {
                bag.extend_from_slice(upper_chars);
            },
            "l" => {
                bag.extend_from_slice(lower_chars);
            },
            "n" => {
                bag.extend_from_slice(numeric_chars);
            },
            "p" => {
                bag.extend_from_slice(pun_chars);
            },
            _ => {
                panic!("Incorrect chars");
                // bag.extend_from_slice(upper_chars);
                // bag.extend_from_slice(lower_chars);
                // bag.extend_from_slice(numeric_chars);
                // bag.extend_from_slice(pun_chars);
            }
        }
    }
    // src2.iter().map(|b| *b as char).collect::<Vec<_>>()
    // bag.iter().map(|b| *b as char).collect::<String>()
    // const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
    //                         abcdefghijklmnopqrstuvwxyz\
    //                         0123456789)(*&^%$#@!~";
    let mut rng = rand::thread_rng();

    let password: String = (0..len)
        .map(|_| {
            let idx = rng.gen_range(0..bag.len());
            bag[idx] as char
        })
        .collect();

    password
}
