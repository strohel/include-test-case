fn main() {
    let text_in = include!("text.in");
    let text_rs = include!("text.rs");
    println!("{} {}", text_in, text_rs);
}
