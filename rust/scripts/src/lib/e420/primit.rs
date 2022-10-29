// Primitive implementation: correct but memory overflows
use std::collections::HashMap;

fn increase_counter(
    n: u64,
    hm: &mut HashMap<(u64, u64, u64, u64), bool>,
    a1: u64,
    b1: u64,
    c1: u64,
    d1: u64,
) -> bool {
    let a = a1 * a1 + b1 * c1;
    let b = a1 * b1 + b1 * d1;
    let c = a1 * c1 + c1 * d1;
    let d = b1 * c1 + d1 * d1;
    if a + d >= n {
        false
    } else {
        let mat = (a, b, c, d);
        match hm.get_key_value(&mat) {
            Some((_, true)) => false,
            Some((_, false)) => {
                hm.insert(mat, true);
                true
            }
            None => {
                hm.insert(mat, false);
                false
            }
        }
    }
}

fn debug_increase_counter(
    n: usize,
    hm: &mut HashMap<(usize, usize, usize, usize), (bool, usize, usize, usize, usize)>,
    a1: usize,
    b1: usize,
    c1: usize,
    d1: usize,
) -> bool {
    let a = a1 * a1 + b1 * c1;
    let b = a1 * b1 + b1 * d1;
    let c = a1 * c1 + c1 * d1;
    let d = b1 * c1 + d1 * d1;
    if a + d >= n {
        false
    } else {
        let mat = (a, b, c, d);
        match hm.get_key_value(&mat) {
            Some((_, (true, _, _, _, _))) => false,
            Some((_, (false, _, _, _, _))) => {
                let old_mat = match hm.insert(mat, (true, a1, b1, c1, d1)) {
                    Some((_, a, b, c, d)) => (a, b, c, d),
                    _ => (0, 0, 0, 0),
                };
                println!("{} {} {} {}", mat.0, mat.1, mat.2, mat.3);
                println!("{} {} {} {}", old_mat.0, old_mat.1, old_mat.2, old_mat.3);
                println!("{} {} {} {}\n", a1, b1, c1, d1);
                true
            }
            None => {
                hm.insert(mat, (false, a1, b1, c1, d1));
                false
            }
        }
    }
}

pub fn e420(n: usize) -> usize {
    let mut counter = 0;
    let mut hm: HashMap<(usize, usize, usize, usize), bool> = HashMap::new();
    let mut debug_hm: HashMap<(usize, usize, usize, usize), (bool, usize, usize, usize, usize)> =
        HashMap::new();

    let mut a1 = 1;
    let mut b1 = 1;
    let mut c1 = 1;
    let mut d1 = 1;
    let sqrtn = f64::sqrt(n as f64) as usize;
    let halfn = n / 2;

    while a1 <= sqrtn {
        b1 = 1;
        c1 = 1;
        d1 = 1;
        while d1 <= sqrtn {
            b1 = 1;
            c1 = 1;
            while b1 <= halfn {
                c1 = 1;
                while c1 <= halfn / b1 + 1 {
                    if debug_increase_counter(n, &mut debug_hm, a1, b1, c1, d1) {
                        counter += 1;
                        // println!("{}", counter);
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