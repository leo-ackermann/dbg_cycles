use crate::lyndon::{LyndonWord, Word, Cycle};
use crate::perfect::{is_perfect};

#[derive(Debug, PartialEq)]
enum Count {
    FromFormula(FromFormula),
    FromEnum(usize),
}
#[derive(Debug, PartialEq)]
enum FromFormula {
    NoFormula,
    FromConjecturedFormula(usize),
    FromProvedFormula(usize),
}

fn count_cycles_only_enum(length: usize, order: usize, sigma: u8) -> Count {
    // Recover LyndonWords
    let mut lws = LyndonWord::new_smallest(length as usize, sigma-1).iter(true).collect::<Vec<Word>>();
    // Filter perfect only
    if length > order {
        lws.retain(|w| is_perfect(w, order))
    }
    // Return the size
    Count::FromEnum(lws.len())
}

#[test]
fn test_count_cycles_only_enum() {
    assert_eq!(count_cycles_only_enum(1, 3, 2), Count::FromEnum(2));
    assert_eq!(count_cycles_only_enum(2, 3, 2), Count::FromEnum(1));
    assert_eq!(count_cycles_only_enum(3, 3, 2), Count::FromEnum(2));
    assert_eq!(count_cycles_only_enum(4, 3, 2), Count::FromEnum(3));
    assert_eq!(count_cycles_only_enum(5, 3, 2), Count::FromEnum(2));
    assert_eq!(count_cycles_only_enum(6, 3, 2), Count::FromEnum(3));
    assert_eq!(count_cycles_only_enum(7, 3, 2), Count::FromEnum(4));
    assert_eq!(count_cycles_only_enum(8, 3, 2), Count::FromEnum(2));
    assert_eq!(count_cycles_only_enum(9, 3, 2), Count::FromEnum(0));
}

use slow_primes::Primes;

fn mobius(n: usize) -> i8 {
    if n == 1 {
        return 1;
    }
    let sieve = Primes::sieve(10000);
    let decomposition = sieve.factor(n).unwrap();
    // if square case
    if decomposition.iter().any(|&(_, pow)| pow > 1) {
        return 0
    } else {
        if decomposition.len() % 2 == 0 { 1 } else { -1 }
    }
}

#[test]
fn test_mobius() {
    let a008683 = [0, 1, -1, -1, 0, -1, 1, -1, 0, 0, 1, -1, 0, -1, 1, 1, 0, -1, 0, -1, 0, 1, 1, -1, 0, 0, 1, 0, 0, -1, -1, -1, 0, 1];
    for i in 1..a008683.len() {
        assert_eq!(mobius(i), a008683[i]);
    }
}


use divisors::get_divisors;

fn nb_lw(l: usize, sigma: u8) -> u32 {
    let mut divisors_of_l = get_divisors(l);

    // Fix implementation of get_divisors...
    if l == 1 {
        divisors_of_l = [1].to_vec();
    } else if l == 2 {
        divisors_of_l = [1, 2].to_vec();
    } else {
        divisors_of_l.push(1);
        divisors_of_l.push(l);
    }

    let mut sum = 0;
    for d in divisors_of_l {
        let mobius_coeff = mobius(d) as i64;
        let quotient = (l/d) as u32;
        let power = i64::pow(sigma as i64, quotient);
        sum += mobius_coeff * power;
    }
    (sum / l as i64) as u32
}

#[test]
fn test_nb_lw() {
    let a001037 = [1, 2, 1, 2, 3, 6, 9, 18, 30, 56, 99, 186, 335, 630, 1161, 2182, 4080, 7710, 14532, 27594, 52377, 99858, 190557, 364722, 698870, 1342176];
    for i in 1..a001037.len() {
        assert_eq!(nb_lw(i, 2), a001037[i]);
    }
}

use crate::math::factorial;

fn nb_dbs(order: usize, sigma: u8) -> u32 {
    let sigma_to_kminusone = u32::pow(sigma as u32, (order-1) as u32);
    let a = u32::pow(factorial((sigma -1) as u32), sigma_to_kminusone);
    let b = u32::pow(sigma as u32, sigma_to_kminusone - order as u32);
    a * b
}

#[test]
fn test_nb_dbs() {
    // A016031
    assert_eq!(nb_dbs(2, 2), 1);
    assert_eq!(nb_dbs(3, 2), 2);
    assert_eq!(nb_dbs(4, 2), 16);
    assert_eq!(nb_dbs(5, 2), 2048);
    assert_eq!(nb_dbs(6, 2), 67108864);
}

use num::integer::binomial;
use reikna::totient::totient as phi;

fn nb_nplw_plustwo(k: usize, sigma: u8) -> u32 {
    phi(k as u64+2) as u32 * binomial(sigma as u32, 2) as u32
}

fn psi(n: u64) -> u64 {
    match n % 4 {
        0 => 3 * phi(n) / 2,
        2 => 2 * phi(n),
        _ => phi(n),
    }
}

fn nb_nplw_plusthree(k: usize, sigma: u8) -> u32 {
    let s = sigma as u32;
    psi(k as u64 + 3) as u32 * (s - 1) * s * s / 2 - s * (s - 1)
}





// fn count_cycles_with_formula(length: usize, order:usize, sigma:u8, only_formula: bool) -> Count {
//     if length <= order+1 {
//         Count(FromFormula(Proved(nb_lw(length, sigma))))
//     } else if length == order+2 {
//         FromFormula::Proved(nb_lw(length, sigma) - nb_nplw_plustwo(k, sigma))
//     } else if length == order+3 {
//         FromFormula::Proved(nb_lw(length, sigma) - nb_nplw_plusthree(k, sigma))
//     } else {
//         if only_formula{
//             FromFormula::NoFormula
//         } else {
//             count_cycles_only_enum()
//         }
//     }
// }
