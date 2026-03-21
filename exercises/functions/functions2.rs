// functions2.rs
//
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a
// hint.
const PI: f64 = 3.14159265358979;

fn main() {
    call_me(3);
    println!("{}", call_me_f(PI, 114514));
}

fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn call_me_f(num: f64, n: i32) -> f64 {
    num * n as f64
}