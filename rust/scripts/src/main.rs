// fib_iter(n) is the nth fibonacci number
fn fib_iter(n: u32) -> u64 {
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

fn print_fib() {
    let mut counter = 10;
    while counter > 0 {
        println!("{}", fib_iter(10 - counter + 1));
        counter -= 1;
    }
}

use std::collections::HashMap;
const N : u64 = 50;

fn matrix_was_seen_twice(hm: &mut HashMap<(u64, u64, u64,u64), bool>, a1: u64, b1: u64, c1: u64, d1: u64) -> bool{
    let a = a1*a1 + b1*c1;
    let b = a1*b1 + b1*d1;
    let c = a1*c1 + c1*d1;
    let d = b1*c1 + d1*d1;
    if a + d >= N{
        false
    }else{
        let mat = (a,b,c,d);
        match hm.get_key_value(&(a,b,c,d)){
            Some((_, true)) => false,
            Some((_, false)) => {hm.insert(mat, true); true},
            None => {hm.insert(mat, false); false}
        }
    }
}

fn e420(n: u64) -> u64{
    let mut counter = 0;
    let mut hm : HashMap<(u64, u64, u64,u64), bool> = HashMap::new();

    let mut a1 = 0;
    let mut b1 = 0;
    let mut c1 = 0;
    let mut d1 = 0;
    let sqrtn = f64::sqrt(n as f64) as u64;
    let halfn = n/2;
    while a1 <= sqrtn{
        b1 = 0;
        c1 = 0;
        d1 = 0;
        while d1 <= sqrtn{
            if matrix_was_seen_twice(&mut hm, a1, b1, c1, d1){
                counter += 1;
            }
            b1 = 1;
            c1 = 1;
            while b1 <= halfn{
                c1 = 1;
                while c1 <= halfn/b1 + 1{
                    if matrix_was_seen_twice(&mut hm, a1, b1, c1, d1){
                        counter += 1;
                    }
                    c1 += 1;
                }
                b1 += 1;
            }
            d1 += 1;
        }
        a1 += 1;
    }
    counter
}

fn main() {
    println!("{}", e420(50));
}
