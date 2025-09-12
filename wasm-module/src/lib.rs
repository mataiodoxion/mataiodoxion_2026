use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn handle_binary_protocol(data: &[u8]) -> Vec<u8> {
    if data.len() != 3 {
        return vec![0xFF]; // Error: invalid len
    }

    let opcode = data[0];
    let num1 = data[1];
    let num2 = data[2];

    match opcode {
        0x01 => {
            // Addition
            let result = num1.wrapping_add(num2);
            vec![0x01, result]
        }
        _ => vec![0xFF] // Error: unknown op
    }
}

#[wasm_bindgen]
pub fn parse_protocol_string(input: &str) -> Vec<u8> {
    input.split_whitespace()
        .filter_map(|s| {
            if s.starts_with("0x") {
                u8::from_str_radix(&s[2..], 16).ok()
            } else {
                None
            }
        })
        .collect()
}