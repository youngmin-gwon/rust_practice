/*
Primitive Types--

1. Scalar
Integer types: u8, i8, u16, i16, u32, i32(default), u64, i64, u128, i128 (number of bits they take in memory)
               [arch] isize, usize (32 or 64 bits depending on architecture)
    Integer Literals:
                [Decimal       ] : 98_222
                [Hex           ] : 0xff
                [Octal         ] : 0o77
                [Binary        ] : 0b1111_0000
                [Byte(u8 only) ] : b'A'

Floating Point types: f32, f64(default: almost same speed as f32)
Boolean type: bool
Character type: char(4 bytes, Unicode Scalar Value => ASCII+Accented letters+emoji)

2. Compound
Tuple type: (): fixed, immutable, and can have multiple type values

Arrays (fixed length) <-> Vector (growable length)
*/

// Rust is a statically typed language, which means that it must know the types of all
// variable at compile time, the compiler can usually infer what type we want to use
// based on the value and how we use it.
pub fn run() {
    // Default is "i32"
    let x: i32 = 1;

    // Default is "f64"
    let y: f64 = 2.5;

    // Add explicit type
    let z: i64 = 454545454545;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active: bool = true;

    // Get boolean from expression
    let is_greater: bool = 10 < 4;

    // Character
    let a1: char = 'a';
    // Even emoji
    let face: char = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}
