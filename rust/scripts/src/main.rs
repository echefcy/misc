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
//                 println!("{} {}\n{} {}\n", mat.0, mat.1, mat.2, mat.3);
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

// --------------------------------------------------------------------------------
const N: usize = 50;
const SQRTN: usize = 8;

fn square_pair(sqnums: &[usize], a: usize, d: usize) -> (usize, usize) {
    // finds (s, s') such that sqnums[s] - sqnums[s'] = a - d
    // precondition: a != d
    let sqdiff = a - d;
    for s in 1..sqnums.len() {
        for s_ in 1..sqnums.len() {
            if sqdiff == sqnums[s] - sqnums[s_] {
                return (s, s_);
            }
        }
    }
    (0, 0)
    // TODO: optimize this function
}

// determines b1, c1
fn increment_counter(sqnums: &[usize], a: usize, d: usize) -> usize {
    let mut counter = 0; // counts how many matrices, given a, d, satisfies the problem conditions

    //  a - b1*c1 = a1^2, d - b1*c1 = d1^2 ==> a - d = sqnums[s] - sqnums[s'] for some s, s'
    let (s, s_) = square_pair(sqnums, a, d);

    42
}

fn e420() -> usize {
    // Definition of matrix multiplication yields the conditions
    // a - b1*c1 = a1^2, d - b1*c1 = d1^2.
    // This implies a - b1*c1 and d  b1*c1 must be square
    // numbers and a1 = sqrt(a-b1*c1) and d1 = sqrt(d - b1*c1).
    // This in turn determines the multiplicand matrix and therefore
    // the product matrix. So a, d, b1, c1 determines the entire equation.

    let mut counter = 0; // counts how many matrices satisfy the problem condition

    // sqnums is such that sqnums[s] = s^2
    let mut sqnums: [usize; SQRTN] = [0; SQRTN];
    let &mut sqref = &mut sqnums;
    for (i, &_) in sqref.iter().enumerate() {
        sqnums[i] = i * i;
    }

    // fixes a, d
    for a in 1..N {
        for d in 1..N - a {
            counter += increment_counter(&sqnums, a, d);
        }
    }
    counter
}

fn main() {
    // println!("{}", e420(10_000_000));
    // println!("{}", e420(100_000));
    // println!("{}", e420(1000));

    println!("{}", e420());
}
