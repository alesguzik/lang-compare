use std::os;

// fib returns a function that returns
// successive Fibonacci numbers.
fn fib(n: u64) -> u64 {
	if n < 2 {
		return n;
	}
	return fib(n-1) + fib(n-2);
}

fn main() {
    let args = os::args();
    let args = args.as_slice();
	let n :u64 = from_str::<u64>(args[1].as_slice()).unwrap();

    let result = fib(n);
	println!("LANGUAGE Rust: {0}", result);
}