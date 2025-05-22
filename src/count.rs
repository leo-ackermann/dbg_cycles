/**
*
* #### count.rs ####
*
* Depending on the regime of parameters (order of the graph, length of the path,
* size of the alphabet), couting simple cycles can either be done by (1)
* applying explicit formulas that are, when they exist, either proved or
* conjectured (2) counting the number of perfect lyndon words, relying on the
* iterators of lyndon.rs
*
**/
use crate::lyndon::LyndonWord;
use crate::math::{factorial, mobius, psi};
use crate::perfect::is_perfect;
use crate::words::Word;
use reikna::totient::totient as phi;

use divisors::get_divisors;
use num::integer::binomial;

// The count enum wrap the counting results to keep an eye on the way they where
// generated.
#[derive(Debug, PartialEq)]
pub enum Count {
    FromProvedFormula(u32),
    FromConjecturedFormula(u32),
    FromEnum(u32),
    NoFormula,
}

// This to_option method allows to use .unwrap() afterward. This enable to
// retrieve the value, removing a lot of matching pain "en passant".
impl Count {
    pub fn to_option(&self) -> Option<u32> {
        match *self {
            Count::FromProvedFormula(x) => Some(x),
            Count::FromConjecturedFormula(x) => Some(x),
            Count::FromEnum(x) => Some(x),
            Count::NoFormula => None,
        }
    }
}

// The formulas that morally correspond to the fourth section of the paper are
// compiled here.

// Number of de Bruijn sequence
fn nb_dbs(order: usize, sigma: u8) -> u32 {
    let sigma_to_kminusone = u32::pow(sigma as u32, (order - 1) as u32);
    let a = u32::pow(factorial((sigma - 1) as u32), sigma_to_kminusone);
    let b = u32::pow(sigma as u32, sigma_to_kminusone - order as u32);
    a * b
}

// Number of Lyndon words
fn nb_lw(l: usize, sigma: u8) -> u32 {
    let mut divisors_of_l = get_divisors(l);

    // FIXME (low priority). Get rid of the "divisors" crate to get rid of this
    // dirty fix, needed because the definition of divisor is not consistent
    // between 1, 2, and the rest of values.
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
        let quotient = (l / d) as u32;
        let power = i64::pow(sigma as i64, quotient);
        sum += mobius_coeff * power;
    }
    (sum / l as i64) as u32
}

// Number of NON-perfect LW, when the length of the LW is "+2" compared to the
// perfectness criterion
// STATUS: proved
fn nb_nplw_plustwo(k: usize, sigma: u8) -> u32 {
    phi(k as u64 + 2) as u32 * binomial(sigma as u32, 2) as u32
}

// Number of NON-perfect LW, when the length of the LW is "+2" compared to the
// perfectness criterion
// STATUS: conjectured
fn nb_nplw_plusthree(k: usize, sigma: u8) -> u32 {
    let s = sigma as u32;
    psi(k as u64 + 3) as u32 * (s - 1) * s * s / 2 - s * (s - 1)
}

// Counting functions

// A counting function that only relies on enumeration of perfect Lyndon words,
// leveraging Theorem [cite once stabilized] of the paper. First, Lyndon words
// are collected. Then, they are filtered to only keep the perfect ones. The
// size of the vector is finally returned.
pub fn count_cycles_only_enum(length: usize, order: usize, sigma: u8) -> Count {
    let mut lws = LyndonWord::new_smallest(length as usize, sigma - 1)
        .iter(true) // fixed_length = true
        .collect::<Vec<Word>>();
    // if length <= order, all LW are perfect -> no need to check :)
    if length > order {
        lws.retain(|w| is_perfect(w, order))
    }
    Count::FromEnum(lws.len() as u32)
}

// A counting function primarily relying on formulas, giving hand to
// enumeration-based counting when no such formula exist (unless the flag
// only_formula is on).
pub fn count_cycles_with_formula(
    length: usize,
    order: usize,
    sigma: u8,
    only_formula: bool,
) -> Count {
    if length <= order + 1 {
        Count::FromProvedFormula(nb_lw(length, sigma))
    } else if length == order + 2 {
        Count::FromProvedFormula(nb_lw(length, sigma) - nb_nplw_plustwo(order, sigma))
    } else if length == order + 3 {
        Count::FromConjecturedFormula(nb_lw(length, sigma) - nb_nplw_plusthree(order, sigma))
    } else if length == usize::pow(sigma as usize, order as u32) {
        Count::FromProvedFormula(nb_dbs(order, sigma))
    } else {
        if only_formula {
            Count::NoFormula
        } else {
            count_cycles_only_enum(length, order, sigma)
        }
    }
}

//
//
//
// ,--. ,--.        ,--.  ,--.        ,--.                 ,--.
// |  | |  |,--,--, `--',-'  '-.    ,-'  '-. ,---.  ,---.,-'  '-. ,---.
// |  | |  ||      \,--.'-.  .-'    '-.  .-'| .-. :(  .-''-.  .-'(  .-'
// '  '-'  '|  ||  ||  |  |  |        |  |  \   --..-'  `) |  |  .-'  `)
//  `-----' `--''--'`--'  `--'        `--'   `----'`----'  `--'  `----'
//
//
//

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

#[test]
fn test_nb_dbs() {
    // Right hand side derived from OEIS's A016031
    assert_eq!(nb_dbs(2, 2), 1);
    assert_eq!(nb_dbs(3, 2), 2);
    assert_eq!(nb_dbs(4, 2), 16);
    assert_eq!(nb_dbs(5, 2), 2048);
    assert_eq!(nb_dbs(6, 2), 67108864);
}

#[test]
fn test_nb_lw() {
    let a001037 = [
        1, 2, 1, 2, 3, 6, 9, 18, 30, 56, 99, 186, 335, 630, 1161, 2182, 4080, 7710, 14532, 27594,
        52377, 99858, 190557, 364722, 698870, 1342176,
    ];
    for i in 1..a001037.len() {
        assert_eq!(nb_lw(i, 2), a001037[i]);
    }
}

#[test]
fn test_count_cycles_with_formula() {
    // Not only formula (calling count_cycles_with_enum to the rescue)
    assert_eq!(
        count_cycles_with_formula(1, 3, 2, false)
            .to_option()
            .unwrap(),
        2
    );
    assert_eq!(
        count_cycles_with_formula(2, 3, 2, false)
            .to_option()
            .unwrap(),
        1
    );
    assert_eq!(
        count_cycles_with_formula(3, 3, 2, false)
            .to_option()
            .unwrap(),
        2
    );
    assert_eq!(
        count_cycles_with_formula(4, 3, 2, false)
            .to_option()
            .unwrap(),
        3
    );
    assert_eq!(
        count_cycles_with_formula(5, 3, 2, false)
            .to_option()
            .unwrap(),
        2
    );
    assert_eq!(
        count_cycles_with_formula(6, 3, 2, false)
            .to_option()
            .unwrap(),
        3
    );
    assert_eq!(
        count_cycles_with_formula(7, 3, 2, false)
            .to_option()
            .unwrap(),
        4
    );
    assert_eq!(
        count_cycles_with_formula(8, 3, 2, false)
            .to_option()
            .unwrap(),
        2
    );
    assert_eq!(
        count_cycles_with_formula(9, 3, 2, false)
            .to_option()
            .unwrap(),
        0
    );

    // Only formula (stating Count::NoFormula when stucked)
    assert_eq!(
        count_cycles_with_formula(1, 3, 2, true).to_option(),
        Some(2)
    );
    assert_eq!(
        count_cycles_with_formula(2, 3, 2, true).to_option(),
        Some(1)
    );
    assert_eq!(
        count_cycles_with_formula(3, 3, 2, true).to_option(),
        Some(2)
    );
    assert_eq!(
        count_cycles_with_formula(4, 3, 2, true).to_option(),
        Some(3)
    );
    assert_eq!(
        count_cycles_with_formula(5, 3, 2, true).to_option(),
        Some(2)
    );
    assert_eq!(
        count_cycles_with_formula(6, 3, 2, true).to_option(),
        Some(3)
    );
    assert_eq!(count_cycles_with_formula(7, 3, 2, true).to_option(), None);
    assert_eq!(
        count_cycles_with_formula(8, 3, 2, true).to_option(),
        Some(2)
    );
    assert_eq!(count_cycles_with_formula(9, 3, 2, true).to_option(), None);
}
