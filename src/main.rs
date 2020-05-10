use futures::future;
use log::trace;
use simplelog::{Config, LevelFilter, SimpleLogger};

fn receives_closure<F>(closure: F)
where
    F: Fn(i32) -> i32,
{
    let result = closure(1);
    trace!("closure(1) => {}", result);
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
    // Initialize simplelog logging
    let _ = SimpleLogger::init(LevelFilter::Trace, Config::default());

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
        receives_closure(closure);
    }
    {
        let add = |x, y| x + y;
        let closure = curry(add, 5);
        receives_closure(closure);
    }
    {
        let two = 2;
        let add = |x, y| x + y + two;
        let closure = generic_curry(add, 4);
        receives_closure(closure);
    }
    {
        let concat = |s, t: &str| format!("{}{}", s, t);
        let closure = generic_curry(concat, "Hello, ");
        let result = closure("world!");
        trace!("{}", result);
    }

    // Tokio runtime
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.enter(|| {
        trace!("in rt.enter()");
        tokio::spawn(future::lazy(|_| trace!("in tokio::spawn()")));
    });
    rt.spawn(future::lazy(|_| trace!("in rt.spawn()")));
    rt.block_on(future::lazy(|_| trace!("in rt.block_on()")));
    {
        let result = rt.block_on(future::ready("Hello from rt.block_on()"));
        trace!("{}", result);
    }
}
