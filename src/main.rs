use futures::future;
use futures::future::FutureExt;
use log::trace;
use simplelog::{Config, LevelFilter, SimpleLogger};
use std::convert::Infallible;
use std::error::Error;
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;
use tokio::time::delay_for;

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

fn returns_impl_future_i32() -> impl Future<Output = i32> {
    future::ready(42)
}

fn returns_dyn_future_i32() -> Pin<Box<dyn Future<Output = i32>>> {
    if rand::random() {
        Box::pin(future::ready(42))
    } else {
        Box::pin(future::lazy(|_| 1337))
    }
}

fn returns_future_result() -> impl Future<Output = Result<i32, impl Error>> {
    future::ok::<_, Infallible>(42) // the _ is inferred from the parameter type
}

fn returns_delayed_future() -> impl Future<Output = i32> {
    delay_for(Duration::from_millis(500)).then(|_| futures::future::ready(42))
}

fn returns_future_chain() -> impl Future<Output = ()> {
    future::lazy(|_| trace!("in returns_future_chain()"))
        .then(|_| {
            trace!("in first then");
            future::ready("Hello from rt.block_on()")
        })
        .inspect(|result| trace!("future::ready() -> {}", result))
        .then(|_| returns_impl_future_i32())
        .inspect(|result| trace!("returns_impl_future_i32() -> {}", result))
        .then(|_| returns_dyn_future_i32())
        .inspect(|result| trace!("returns_dyn_future_i32() -> {}", result))
        .then(|_| returns_future_result())
        .map(|result| result.unwrap())
        .inspect(|result| trace!("returns_future_result().unwrap() -> {}", result))
        .then(|_| returns_delayed_future())
        .inspect(|result| trace!("returns_delayed_future() -> {}", result))
        .then(|_| future::ready(()))
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
    {
        let result = rt.block_on(returns_impl_future_i32());
        trace!("{}", result);
    }
    {
        let result = rt.block_on(returns_dyn_future_i32());
        trace!("{}", result);
    }
    {
        let result = rt.block_on(returns_future_result());
        trace!("{}", result.unwrap());
    }
    rt.block_on(returns_future_chain());
}
