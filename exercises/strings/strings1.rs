// strings1.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings1` for hints ;)

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    // I was wondering what the idiomatic way to do this is and found an
    // interesting discussion at [1].
    //
    // [1] https://users.rust-lang.org/t/what-is-the-idiomatic-way-to-convert-str-to-string/12160/2
    String::from("blue")
}
