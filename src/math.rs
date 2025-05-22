/**
*
* #### math.rs ####
*
* A few maths functions needed elsewhere, and that do not appear in crate (or
* were litteraly 30s to recode)
*
**/
use reikna::totient::totient as phi;
use slow_primes::Primes; // not that slow in fact :)

// A naive factorial
pub fn factorial(n: u32) -> u32 {
    match n {
        0 => 1,
        _ => n * factorial(n - 1),
    }
}

// The mobius function evaluates to: (A) 1 when call on 1 (B) 0 when call on n
// such that at least one prime appears twice in its prime decomposition (C)
// (-1)^k otherwise, where k is the number of prime factors in the prime
// decomposition of n.
pub fn mobius(n: usize) -> i8 {
    if n == 1 {
        return 1;
    }
    let sieve = Primes::sieve(10000);
    let decomposition = sieve.factor(n).unwrap();
    if decomposition.iter().any(|&(_, pow)| pow > 1) {
        return 0;
    } else {
        if decomposition.len() % 2 == 0 {
            1
        } else {
            -1
        }
    }
}

// A non-standard function, used in our conjectured formula
pub fn psi(n: u64) -> u64 {
    match n % 4 {
        0 => 3 * phi(n) / 2,
        2 => 2 * phi(n),
        _ => phi(n),
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
fn test_mobius() {
    let a008683 = [
        0, 1, -1, -1, 0, -1, 1, -1, 0, 0, 1, -1, 0, -1, 1, 1, 0, -1, 0, -1, 0, 1, 1, -1, 0, 0, 1,
        0, 0, -1, -1, -1, 0, 1,
    ];
    for i in 1..a008683.len() {
        assert_eq!(mobius(i), a008683[i]);
    }
}
