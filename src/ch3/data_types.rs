pub fn run() {
    // `Scalar`
    // - integer
    //   - rust default is i32
    //   - signed: i8, i16, i32, i64, i128, isize(arch)
    //   - unsigned: u8, u16, u32, u64, u128, usize(arch)
    //   - isize, usize arch means type depending on architecture(64bits or 32bits)
    //     - The primary situation in which you’d use isize or usize is when
    //       indexing some sort of collection.
    //   - number literals example:
    //   `57u8`,
    //   `1_000`(decimal, visual separator for 1000),
    //   `0xff`(hex),
    //   `0o77`(octal),
    //   `0b1111_0000`(binary),
    //   `b'A'`(Byte only)
    // - floating-point
    //  - f32(single precision), f64(double precision)
    //  - default is f64(on modern cpu, roughly same speed as f32)
    // - boolean
    //  - bool, 1 byte
    // - character
    //  - char
    //  - single quotes
    //      - string literals use double quotes
    //  - 4 bytes
    //  - char represents Unicode Scalar Value, more than ASCII(Emoji)
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{z}, {c}, {heart_eyed_cat}");

    // `Compound`
    // - tuple
    //  - a general way of grouping together
    //    a number of values with a variety of types
    //    into one compound type
    //  - cannot grow or shrink in size
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // tuple matching
    let (x, y, z) = tup;
    // tuple indexing
    let x1 = tup.0;
    let x2 = tup.1;
    let x3 = tup.2;
    println!("{x}, {y}, {z}");
    println!("{x1}, {x2}, {x3}");

    // - array
    //  - every element must have the same type
    //  - fixed length
    //      - vector: growable collection type
    //  - useful when you know the number of elements will not change
    //    ex) months
    // let a = [1, 2, 3, 4, 5];
    // let b: [i32; 5] = [1, 2, 3, 4, 5]; // [element type;length]
    let d = [2_000; 5_500]; // contains 5 elements of value 3
    let first = d[0];
    let second = d[1_000];
    // let bs = d[10000000]; // this error happens at runtime
    println!("{first}, {second}");
}
