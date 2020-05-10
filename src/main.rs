fn receives_closure<F>(closure: F)
where
    F: Fn(i32) -> i32,
{
    let result = closure(1);
    println!("closure(1) => {}", result);
}

fn main() {
    {
        let y = 2;
        let add = |x| x + y;
        receives_closure(add);
    }
    {
        let y = 10;
        let add = |x| x + y;
        receives_closure(add);
    }
}
