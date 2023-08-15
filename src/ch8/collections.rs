use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

pub fn run() {
    array();
    vector();
    store_enum_in_vector();
    string();
    hashmap();
}

fn array() {
    let a = [1, 2, 3];
}

fn vector() {
    let mut v1: Vec<i32> = Vec::new();

    v1.push(1);
    v1.push(2);
    v1.push(3);

    // vector initialization macro
    let v2 = vec![1, 2, 3];

    // access elements
    let v3 = vec![1, 2, 3, 4, 5];
    let third = &v3[2];
    // error: immutable reference
    // v3.push(6);
    println!("The third element is {third}");
    // invalid index
    // let invalid = &v3[20];
    // println!("The third element is {}", invalid);
    //
    // runtime error
    // - contrast to array
    // - vector is stored in heap
    //
    // safer method: get
    match v3.get(20) {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    // iterating over vector
    let mut v = vec![1, 2, 3, 4, 5];

    for i in &mut v {
        // dereference operator
        *i += 50;
    }

    for i in &v {
        println!("{i}");
    }
}

fn store_enum_in_vector() {
    // vector can store only one type of data
    //
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    match &row[1] {
        SpreadsheetCell::Int(i) => println!("{i}"),
        _ => println!("Not an integer"),
    };
}

fn string() {
    // Strings are stored as a collection of UTF-8 encoded bytes.
    let s1 = String::new();
    let s2 = "initial contents";
    let s3 = s2.to_string();
    let mut s4 = String::from("initial contents");
    s4.push_str("bar");
    s4.push('!');

    // this saves a little bit of memory
    let s5 = s1 + &s2;
    println!("{s5}{s2}");

    // indexing is not possible because of unicode.
    // Because indicating 1 byte to variable-indexed String(2 bytes for first char)
    // is obsolete.
    let value = "Здравствуйте";
    let hello = String::from(value);
    // let c: char = hello[0];
    //
    // determine which value you want to access
    // 1. bytes
    for b in hello.bytes() {
        println!("{b}");
    }
    // 2. scalars
    for b in hello.chars() {
        println!("{b}");
    }
    // 3. grapheme clusters
    // need to import crate "unicode-segmentation"
    for b in hello.graphemes(true) {
        println!("{b}");
    }
}

fn hashmap() {
    // 1. how to declare hashmap
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();

    // note that passing in value not by reference
    // this overrides values
    scores.insert(blue, 300);
    scores.insert(yellow, 400);

    // 2. how to put values if there is no such key
    //
    // if you don't want to override value, then use this syntax
    scores.entry(String::from("Yellow")).or_insert(30);
    scores.entry(String::from("Blue")).or_insert(40);

    let team_name = String::from("Red");
    let score = scores.get(&team_name);
    match score {
        Some(value) => println!("{team_name}: {value}"),
        None => println!("Not found"),
    };

    // 3. how to iterate hashmap
    for (k, v) in &scores {
        println!("{k}: {v}")
    }

    // 4. how to make word counter using hashmap
    // update hashmap based on old value
    let text = "hello world woderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        // or_insert returns reference
        *count += 1;
    }

    println!("{:?}", map);
}
