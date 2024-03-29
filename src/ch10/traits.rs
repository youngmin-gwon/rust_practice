use core::fmt::Display;
use std::fmt::Debug;

// similar to a feature called interfaces in other languages
pub fn run() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle2 {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    let tweet3 = Tweet3 {
        username: String::from("horse_ebooks"),
    };

    println!("1 new tweet: {}", tweet3.summarize());
}

// 1. defining a trait
pub trait Summary {
    fn summarize(&self) -> String;
}

// 2. implementing a trait on a type
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// One restriction to note is that we can implement a trait
// on a type only if at least one of the trait or the type is local to our crate.
//
// his restriction is part of a property called `coherence`,
// and more specifically `the orphan rule`, so named because the parent type is not present.

// 3. default implementations
pub trait Summary2 {
    // default implementation
    fn summarize(&self) -> String {
        String::from("(Read more)")
    }
}

pub struct NewsArticle2 {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary2 for NewsArticle2 {}

// 1) default create can call other methods in the same trait
pub trait Summary3 {
    fn summarize(&self) -> String {
        format!("(Read more from {})", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}

struct Tweet3 {
    pub username: String,
}

impl Summary3 for Tweet3 {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// 4. traits as parameters
// 1) syntax sugar form(short, convenient)
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// 2) original form(verbose, expressive)
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// 3) difference?
pub fn notify3(item1: &impl Summary, item2: &impl Summary) {}
// if both parameters has equal trait
pub fn notify4<T: Summary>(item1: &T, item2: &T) {}

// 4) more than one trait
pub trait TextToSpeech {}

pub fn notify5(item: &(impl Summary + TextToSpeech)) {}

pub fn notify6<T: Summary + TextToSpeech>(item: &T) {}

// 5) clearer trait bounds with where clauses
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {}

fn some_function2<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
}

// 5. Returning types that implement traits
// - returning only one type is allowed due to restrictions
//   around how the `impl Trait` syntax is implemented in the compiler.
//   "Using trait objects that allow for values of different types" will be covered in Ch.17.
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// 6. Using trait bounds to conditionally implement methods
// - trait and trait bounds let us write code that uses generic type parameters to reduce duplication
//   but also specify to the compiler that we want the generic type to have particular behavior.
// - it is called `blanket implementations` and is extensively used
//    in the Rust Standard library.
// - if you want a reference on blanket implementations, look into "Implementors" section.
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x)
        } else {
            println!("The largest member is y = {}", self.y)
        }
    }
}
// Rust moves these errors to compile time so we’re forced to fix the problems before
// our code is even able to run. Additionally, we don’t have to write code that checks
// for behavior at runtime because we’ve already checked at compile time.
// Doing so improves performance without having to give up the flexibility of generics.
