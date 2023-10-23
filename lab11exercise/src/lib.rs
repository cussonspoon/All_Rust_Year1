use hex;

const FOX: &str = "The quick brown fox jumps over the lazy dog.";
const ENCODED_DATA: &[u8] = &[
0x76, 0x5C, 0x57, 0x30, 0x41, 0x47, 0x5D, 0x52,
0x5E, 0x30, 0x53, 0x43, 0x58, 0x44, 0x59, 0x30,
0x55, 0x58, 0x4C, 0x30, 0x5F, 0x47, 0x5B, 0x40,
0x42, 0x30, 0x58, 0x45, 0x57, 0x43, 0x30, 0x46,
0x5C, 0x57, 0x30, 0x5A, 0x51, 0x4F, 0x4D, 0x30,
0x56, 0x58, 0x54, 0x39
];

fn encode(b: u8) -> u8 {
    match b {
        0x0 => 0x0,
        0x1 => 0x1,
        0x2 => 0x3,
        0x3 => 0x2,
        0x4 => 0x6,
        0x5 => 0x7,
        0x6 => 0x5,
        0x7 => 0x4,
        0x8 => 0xC,
        0x9 => 0xD,
        0xA => 0xF,
        0xB => 0xE,
        0xC => 0xA,
        0xD => 0xB,
        0xE => 0x9,
        0xF => 0x8,
        _ => panic!("Invalid input"),
    }
}

fn encode_hex(input: u8) -> u8{
    let mut result: Vec<char> = Vec::new(); 
    let upper = encode(input >> 4);
    let lower = encode(input & 0xF);

    let up = match std::char::from_digit(upper as u32, 16) {
        Some(c) => c.to_ascii_uppercase(),
        None => panic!("Invalid decimal number"),
    };
    let low = match std::char::from_digit(lower as u32, 16) {
        Some(c) => c.to_ascii_uppercase(),
        None => panic!("Invalid decimal number"),
    };
    result.push(up);
    result.push(low);
    
    let ans = format!("{}{}", result[0], result[1]);
    let result2 = u64::from_str_radix(&ans, 16);
    result2.unwrap() as u8
}

fn encode_hex_data(input: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    for i in input {
        result.push(encode_hex(*i));
    }
    result
}


fn decode(b: u8) -> u8 {
    match b {
        0x0 => 0x0,
        0x1 => 0x1,
        0x2 => 0x3,
        0x3 => 0x2,
        0x4 => 0x7,
        0x5 => 0x6,
        0x6 => 0x4,
        0x7 => 0x5,
        0x8 => 0xF,
        0x9 => 0xE,
        0xA => 0xC,
        0xB => 0xD,
        0xC => 0x8,
        0xD => 0x9,
        0xE => 0xB,
        0xF => 0xA,
        _ => panic!("Invalid input"),
    }
}

fn decode_hex(input: u8) -> u8 {
    let mut result: Vec<char> = Vec::new(); 
    let upper = decode(input >> 4);
    let lower = decode(input & 0xF);

    let up = match std::char::from_digit(upper as u32, 16) {
        Some(c) => c.to_ascii_uppercase(),
        None => panic!("Invalid decimal number"),
    };
    let low = match std::char::from_digit(lower as u32, 16) {
        Some(c) => c.to_ascii_uppercase(),
        None => panic!("Invalid decimal number"),
    };
    result.push(up);
    result.push(low);
    
    let ans = format!("{}{}", result[0], result[1]);
    let result2 = u64::from_str_radix(&ans, 16);
    result2.unwrap() as u8
}

fn decode_hex_data(input: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    for i in input {
        result.push(decode_hex(*i));
    }
    result
}


#[test]
fn test_encode_hex() {
    assert_eq!(
    (0..16).map(encode_hex).collect::<Vec<_>>(),[ 0x0, 0x1, 0x3, 0x2, 0x6, 0x7, 0x5, 0x4, 0xC, 0xD, 0xF, 0xE, 0xA, 0xB, 0x9, 0x8 ]);
    assert_eq!(encode_hex(0x54), 0x76);
    assert_eq!(encode_hex(0x68), 0x5C);
    let original_data = FOX.as_bytes();
    let encoded_data = ENCODED_DATA;
    assert_eq!(encode_hex_data(original_data), encoded_data);
}

#[test]
fn test_decode_hex() {
    assert_eq!(
    (0..16).map(decode_hex).collect::<Vec<_>>(),[ 0x0, 0x1, 0x3, 0x2, 0x7, 0x6, 0x4, 0x5, 0xF, 0xE, 0xC, 0xD, 0x8, 0x9, 0xB, 0xA ]);
    assert_eq!(decode_hex(0x76), 0x54);
    assert_eq!(decode_hex(0x5C), 0x68);
    let original_data = FOX.as_bytes();
    let encoded_data = ENCODED_DATA;
    assert_eq!(decode_hex_data(encoded_data), original_data);
}

struct XPM2Image {
    colors: Vec<(String, String)>,
    pixels: Vec<String>,
}

fn make_xpm2(ctable: &[(String, String)], pixels: &[String]) -> XPM2Image {
    XPM2Image {
        colors: ctable.to_vec(),
        pixels: pixels.to_vec(),
    }
}

#[test]
fn test_make_xpm2() {
let ctable = &[
("#".into(), "#000000".into()),
("-".into(), "#FFFFFF".into())
];
let rows = ["##--", "##--", "--##", "--##"];
let pixels: Vec<_> = rows.iter().map(|r| r.to_string()).collect();
let img = make_xpm2(ctable, &pixels);
assert_eq!(
img.colors,
[ ("#".into(), "#000000".into()),
("-".into(), "#FFFFFF".into()) ]);
assert_eq!(img.pixels.len(), 4);
assert_eq!(img.pixels.iter().map(|r| r.len()).max(), Some(4));
assert_eq!(img.colors.len(), 2);
}

