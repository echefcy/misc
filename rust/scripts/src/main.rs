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

// use std::collections::HashMap;

// fn increase_counter(
//     n: u64,
//     hm: &mut HashMap<(u64, u64, u64, u64), bool>,
//     a1: u64,
//     b1: u64,
//     c1: u64,
//     d1: u64,
// ) -> bool {
//     let a = a1 * a1 + b1 * c1;
//     let b = a1 * b1 + b1 * d1;
//     let c = a1 * c1 + c1 * d1;
//     let d = b1 * c1 + d1 * d1;
//     if a + d >= n {
//         false
//     } else {
//         let mat = (a, b, c, d);
//         match hm.get_key_value(&mat) {
//             Some((_, true)) => false,
//             Some((_, false)) => {
//                 hm.insert(mat, true);
//                 true
//             }
//             None => {
//                 hm.insert(mat, false);
//                 false
//             }
//         }
//     }
// }

// fn e420(n: u64) -> u64 {
//     let mut counter = 0;
//     let mut hm: HashMap<(u64, u64, u64, u64), bool> = HashMap::new();
//     // let mut debug_hm : HashMap<(u64, u64, u64, u64), (bool, u64, u64, u64,u64)> = HashMap::new();

//     let mut a1 = 1;
//     let mut b1 = 1;
//     let mut c1 = 1;
//     let mut d1 = 1;
//     let sqrtn = f64::sqrt(n as f64) as u64;
//     let halfn = n / 2;

//     while a1 <= sqrtn {
//         b1 = 1;
//         c1 = 1;
//         d1 = 1;
//         while d1 <= sqrtn {
//             b1 = 1;
//             c1 = 1;
//             while b1 <= halfn {
//                 c1 = 1;
//                 while c1 <= halfn / b1 + 1 {
//                     if increase_counter(n, &mut hm, a1, b1, c1, d1) {
//                         counter += 1;
//                         // println!("{}", counter);
//                     }
//                     c1 += 1;
//                 }
//                 b1 += 1;
//             }
//             d1 += 1;
//         }
//         a1 += 1;
//     }
//     counter
// }

const N: usize = 50;
const SQRTN: usize = 8;

// determines b1, c1
fn increase_counter(sqnums: &[usize], a: usize, d: usize) -> bool {
    let mut s = 1;
    while sqnums[s] < a && s < sqnums.len() {
        let b1c1 = a - sqnums[s];
        s += 1;
    }
    true
}

fn e420() -> usize {
    let mut counter = 0;
    let mut sqnums: [usize; SQRTN] = [0; SQRTN];
    let &mut sqref = &mut sqnums;
    // initialize sqnums
    for (i, &_) in sqref.iter().enumerate() {
        sqnums[i] = i * i;
    }
    for a in 1..N {
        for d in 1..N - a {
            if increase_counter(&sqnums, a, d) {
                counter += 1;
            }
        }
    }
    counter
}

fn main() {
    // let mut n = 1000;
    // while n < 100_000{
    //     println!("n: {}, result: {}", n, e420(n));
    //     n += 1000;
    // }

    // println!("{}", e420(10_000_000));
    // println!("{}", e420(100_000));
    println!("{}", e420());
}
