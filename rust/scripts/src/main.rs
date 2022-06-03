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

// Primitive implementation: correct but memory overflows
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

// fn debug_increase_counter(
//     n: usize,
//     hm: &mut HashMap<(usize, usize, usize, usize), (bool, usize, usize, usize, usize)>,
//     a1: usize,
//     b1: usize,
//     c1: usize,
//     d1: usize,
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
//             Some((_, (true, _, _, _, _))) => false,
//             Some((_, (false, _, _, _, _))) => {
//                 let old_mat = match hm.insert(mat, (true, a1, b1, c1, d1)) {
//                     Some((_, a, b, c, d)) => (a, b, c, d),
//                     _ => (0, 0, 0, 0),
//                 };
//                 println!("{} {} {} {}", mat.0, mat.1, mat.2, mat.3);
//                 println!("{} {} {} {}", old_mat.0, old_mat.1, old_mat.2, old_mat.3);
//                 println!("{} {} {} {}\n", a1, b1, c1, d1);
//                 true
//             }
//             None => {
//                 hm.insert(mat, (false, a1, b1, c1, d1));
//                 false
//             }
//         }
//     }
// }

// fn e420(n: usize) -> usize {
//     let mut counter = 0;
//     let mut hm: HashMap<(usize, usize, usize, usize), bool> = HashMap::new();
//     let mut debug_hm: HashMap<(usize, usize, usize, usize), (bool, usize, usize, usize, usize)> =
//         HashMap::new();

//     let mut a1 = 1;
//     let mut b1 = 1;
//     let mut c1 = 1;
//     let mut d1 = 1;
//     let sqrtn = f64::sqrt(n as f64) as usize;
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
//                     if debug_increase_counter(n, &mut debug_hm, a1, b1, c1, d1) {
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
use std::collections::HashMap;

fn factors(b1c1: usize) -> Vec<usize> {
    // factors of b1c1 stored in an array
    let sqrtb1c1 = f64::sqrt(b1c1 as f64) as usize;
    let mut ret = Vec::new();
    // TODO: optimize this
    for i in 1..sqrtb1c1{
        if b1c1%i == 0{
            ret.push(i);
        }
    }
    ret
}

fn count_a_d_not_equal(s: usize, sqnums: &[usize], a: usize, d: usize) -> usize{
    let mut counter = 0;
    let b1c1 = a - sqnums[s];
    let a1 = f64::sqrt((a - b1c1) as f64) as usize;
    let d1 = f64::sqrt((d - b1c1) as f64) as usize;
    assert_eq!(a1*a1 + b1c1, a);
    assert_eq!(b1c1 + d1*d1, d);
    let fs = factors(b1c1);
    let mut memo : HashMap<(usize, usize), bool> = HashMap::new();
    // TODO: optimize choosing b1, c1 and calculating b, c
    for factor in fs{
        let b1 = factor;
        let c1 = b1c1 / b1;
        assert_eq!(b1*c1, b1c1);
        let b = a1*b1 + b1*d1;
        let c = a1*c1 + c1*d1;
        match memo.get_key_value(&(b, c)){
            None => {memo.insert((b, c), false); ()},
            Some((_, false)) => {memo.insert((b, c), true); counter += 1}
            Some((_, true)) => ()
        }
    }
    counter
}

// fn square_sum_pair(sqnums: &[usize], target: usize) -> (usize, usize){
//     // finds (s, s') such that sqnums[s] + sqnums[s'] = target
// }

fn count_a_d_equal(sqnums: &[usize], a: usize) -> usize {
    // basically counts the number of distinct pairs of square numbers (order doesn't matter, but the numbers are distinct)
    // that add up to a, and then multiply that number by 2 to account for b1 and c1 being exchangeable
    let mut counter = 0;
    let mut s = 1;
    while s < sqnums.len(){
        let mut s_ = s + 1;
        while s_ < sqnums.len(){
            if sqnums[s] + sqnums[s_] == a{
                counter += 1;
            }
            s_ += 1;
        }
        s += 1;
    }
    // TODO: optimize this function

    counter
}

fn square_diff_pair(sqnums: &[usize], a: usize, d: usize) -> (usize, usize) {
    // finds (s, s') such that sqnums[s] - sqnums[s'] = a - d
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
fn count_a_d(sqnums: &[usize], a: usize, d: usize) -> usize {
    let mut counter = 0; // counts how many matrices, given a, d, satisfies the problem conditions

    // a - b1*c1 = a1^2, d - b1*c1 = d1^2 ==> a - d = sqnums[s] - sqnums[s'] for some s, s'
    let (s, _) = square_diff_pair(sqnums, a, d);
    if a == d {
        counter += count_a_d_equal(sqnums, a)
    }else{
        counter += count_a_d_not_equal(s, sqnums, a, d)
    }

    counter
}

fn e420(n : usize) -> usize {
    // Definition of matrix multiplication yields the conditions
    // a - b1*c1 = a1^2, d - b1*c1 = d1^2.
    // This implies a - b1*c1 and d  b1*c1 must be square
    // numbers and a1 = sqrt(a-b1*c1) and d1 = sqrt(d - b1*c1).
    // This in turn determines the multiplicand matrix and therefore
    // the product matrix. So a, d, b1, c1 determines the entire equation.

    let mut counter = 0; // counts how many matrices satisfy the problem condition

    // sqnums is such that sqnums[s] = s^2
    let sqrtn = f64::sqrt(n as f64) as usize;
    let mut sqnums = Vec::with_capacity(sqrtn);
    for i in 0..sqnums.len(){
        sqnums[i] = i*i;
    }

    // fixes a, d
    for a in 1..n {
        for d in 1..n - a {
            counter += count_a_d(&sqnums, a, d);
        }
    }
    counter
}

fn main() {
    // println!("{}", e420(10_000_000));
    // println!("{}", e420(100_000));
    // println!("{}", e420(1000));

    println!("{}", e420(1000));
}

