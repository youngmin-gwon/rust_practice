pub fn run() {
    // 1. function generic
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest::<i32>(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest::<char>(&char_list);
    println!("The largest char is {}", result);

    // 2. generic in struct
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 6.0, y: 12.0 };
    println!(
        "The value x is: {0}, The value y is: {1}",
        integer.x, integer.y
    );
    println!("The value x is: {0}, The value y is: {1}", float.x, float.y);

    // 3. multiple generic
    let both_integer = Ellipse { x: 5, y: 5 };
    let both_float = Ellipse { x: 10.0, y: 15.0 };
    let integer_and_float = Ellipse { x: 5, y: 4.0 };
    println!(
        "The value x is: {0}, The value y is: {1}",
        both_integer.x, both_integer.y
    );
    println!(
        "The value x is: {0}, The value y is: {1}",
        both_float.x, both_float.y
    );
    println!(
        "The value x is: {0}, The value y is: {1}",
        integer_and_float.x, integer_and_float.y
    );

    // 5. generic in method definition
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    // complicated generic
    let p1 = Ellipse { x: 5, y: 10.4 };
    let p2 = Ellipse { x: "Hello", y: "c" };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // Good news:
    //
    // - using generic types won't make your program run any slower than
    //   it would with concrete types.
    // - the compiler looks at all the places where generic code is called and
    //   generates code for the concrete types the generic code is called with.
}

// -- code below is quite annoying since it looks quite duplicated.
// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }
// --

// 1. use generic instead
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// 2. generic in struct
struct Point<T> {
    x: T,
    y: T,
}

// 3. multiple generic
struct Ellipse<T, U> {
    x: T,
    y: U,
}

// 4. generic in enum
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// 5. generic in method definitions
// - note that you have to declare T just after `impl` so you can use `T` to specify that
//   we're implementing methods on the type Point<T>
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// other instances of Point<T> where T is not of type f32 will not have this method defined
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// complicated generic
impl<T, U> Ellipse<T, U> {
    fn mixup<X, Y>(self, other: Ellipse<X, Y>) -> Ellipse<T, Y> {
        Ellipse {
            x: self.x,
            y: other.y,
        }
    }
}
