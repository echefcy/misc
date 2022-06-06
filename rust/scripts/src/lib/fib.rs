
// fib_iter(n) is the nth fibonacci number
pub fn fib_iter(n: u32) -> u64 {
    let mut counter = 1;
    let mut last = 0;
    let mut new = 1;
    while counter < n {
        let temp = new;
        new = last + new;
        last = temp;
        counter += 1;
    }
    new
}

pub fn print_fib() {
    let mut counter = 10;
    while counter > 0 {
        println!("{}", fib_iter(10 - counter + 1));
        counter -= 1;
    }
}