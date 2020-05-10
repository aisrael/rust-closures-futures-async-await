fn receives_closure<F>(closure: F)
where
    F: Fn(i32) -> i32,
{
    let result = closure(1);
    println!("closure(1) => {}", result);
}

fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 4
}

fn curry<F>(f: F, x: i32) -> impl Fn(i32) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    move |y| f(x, y)
}

fn generic_curry<F, X, Y, Z>(f: F, x: X) -> impl Fn(Y) -> Z
where
    F: Fn(X, Y) -> Z,
    X: Copy,
{
    move |y| f(x, y)
}

fn main() {
    {
        let y = 2;
        receives_closure(|x| x + y);
    }
    {
        let y = 3;
        receives_closure(|x| x + y);
    }
    {
        let closure = returns_closure();
        println!("closure(1) => {}", closure(1));
    }
    {
        let closure = returns_closure();
        receives_closure(closure);
    }
    {
        let add = |x, y| x + y;
        let closure = curry(add, 5);
        println!("closure(1) => {}", closure(1));
    }
    {
        let two = 2;
        let add = |x, y| x + y + two;
        let closure = generic_curry(add, 4);
        receives_closure(closure);
    }
}
