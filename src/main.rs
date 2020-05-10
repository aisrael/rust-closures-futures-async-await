fn receives_closure<F>(closure: F)
where
    F: Fn(i32, i32) -> i32,
{
    let result = closure(1, 2);
    println!("closure(1, 2) => {}", result);
}

fn main() {
    let add = |x, y| x + y;
    receives_closure(add);
}
