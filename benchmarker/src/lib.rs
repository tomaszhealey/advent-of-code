use std::fmt::Display;

///Takes a FnOnce that returns a T: Display, it then returns the function's return value and how
///long it took to execute as a String
pub fn benchmark<T: Display>(target: impl FnOnce() -> T) -> String {
    use std::time::Instant;
    let now = Instant::now();

    let output = target();

    let elapsed = now.elapsed();
    format!("{output}, Elapsed: {:.2?}", elapsed)
}

pub fn benchmark_return<T>(target: impl FnOnce() -> T) -> (T, String) {
    use std::time::Instant;
    let now = Instant::now();

    let output = target();

    let elapsed = now.elapsed();
    (output, format!("Elapsed: {:.2?}", elapsed))
}

pub fn elapsed<T>(target: impl FnOnce() -> T, msg: &str) -> T {
    use std::time::Instant;
    let now = Instant::now();

    let output = target();

    let elapsed = now.elapsed();
    println!("{msg}, Elapsed: {:.2?}", elapsed);

    output
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use super::*;

    #[test]
    fn it_works() {
        let input = "Hello World";
        benchmark(|| do_something(&input));
        benchmark(|| do_something(&input));
    }

    fn do_something(_input: &str) -> usize {
        thread::sleep(Duration::from_secs(2));
        10
    }
}
