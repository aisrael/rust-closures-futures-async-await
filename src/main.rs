fn main() {
    let hello = |who| format!("Hello, {}!", &who);
    let message = hello("world");
    println!("{}", message);
}
