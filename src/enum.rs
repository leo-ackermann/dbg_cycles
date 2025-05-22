/**
*
* #### enum.rs ####
*
* The enumeration of simple cycles primarily relies on the enumerations of
* Lyndon words, through Duvals' algorithms. They are then filtered, to keep only
* the perfect ones, and mapped to cycles.
*
**/
use crate::lyndon::LyndonWord;
use crate::perfect::{is_perfect, is_perfect_lyndon};
use crate::words::{Cycle, Word};

// In the paper, we presented to distinct bijective map, depending on the regime
// of parameter. While not surprising (we distinguish the two regimes to ease
// the presentation), these maps readily coincide on their implementation. The
// following function thus accounts for f and g.
fn map_word_to_cycle(w: Word, k: usize) -> Vec<Word> {
    let mut cycle = <Vec<Word>>::new();
    for i in 0..=w.len() {
        // First letter of the rotation of lw to consider
        // Build word = node in dbg
        let mut node = vec![0; k];
        for j in 0..k {
            node[j] = w[(i + j) % w.len()];
        }
        cycle.push(node);
    }

    cycle
}

// The two distinct iterators on Lyndon words are here extended with perfect
// filtering and the previously defined map. Note that the filtering strategy
// slightly vary between the cases (the "length>order" switch is either present
// at this level, or hidden in "is_perfect_lyndon").

pub fn enum_cycles_fixed_length(length: usize, order: usize, sigma: u8) -> Vec<Cycle> {
    let mut collection = <Vec<Cycle>>::new();

    // Recover LyndonWords
    let mut lw = LyndonWord::new_smallest(length as usize, sigma - 1)
        .iter(true)
        .collect::<Vec<Word>>();
    // Filter perfect only
    if length > order {
        lw.retain(|w| is_perfect(w, order))
    }
    // Map to cycles
    for plw in lw {
        collection.push(map_word_to_cycle(plw, order));
    }

    collection
}

pub fn enum_cycles_bounded_length(length: usize, order: usize, sigma: u8) -> Vec<Cycle> {
    let mut collection = <Vec<Cycle>>::new();

    // Recover LyndonWords
    let mut lw = LyndonWord::new_smallest(length as usize, sigma - 1)
        .iter(false)
        .collect::<Vec<Word>>();
    // Filter perfect only
    lw.retain(|w| is_perfect_lyndon(w, order));
    // Map to cycles
    for plw in lw {
        collection.push(map_word_to_cycle(plw, order));
    }

    collection
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
fn test_map_word_to_cycle() {
    // l <= k regime
    assert_eq!(
        map_word_to_cycle(vec![0, 1], 3),
        [[0, 1, 0], [1, 0, 1], [0, 1, 0]]
    );
    assert_eq!(
        map_word_to_cycle(vec![1, 1, 2], 5),
        [
            [1, 1, 2, 1, 1],
            [1, 2, 1, 1, 2],
            [2, 1, 1, 2, 1],
            [1, 1, 2, 1, 1]
        ]
    );

    // l > k regime
    assert_eq!(
        map_word_to_cycle(vec![0, 0, 0, 1, 1, 1], 3),
        [
            [0, 0, 0],
            [0, 0, 1],
            [0, 1, 1],
            [1, 1, 1],
            [1, 1, 0],
            [1, 0, 0],
            [0, 0, 0]
        ]
    );
    assert_eq!(
        map_word_to_cycle(vec![0, 0, 1, 0, 1, 1], 3),
        [
            [0, 0, 1],
            [0, 1, 0],
            [1, 0, 1],
            [0, 1, 1],
            [1, 1, 0],
            [1, 0, 0],
            [0, 0, 1]
        ]
    );
    assert_eq!(
        map_word_to_cycle(vec![0, 0, 1, 1, 0, 1], 3),
        [
            [0, 0, 1],
            [0, 1, 1],
            [1, 1, 0],
            [1, 0, 1],
            [0, 1, 0],
            [1, 0, 0],
            [0, 0, 1]
        ]
    );
}

#[test]
fn test_enum_cycles_fixed_length() {
    // l <= k regime
    let cycles_2_3_2 = [[[0, 1, 0], [1, 0, 1], [0, 1, 0]]];
    assert_eq!(enum_cycles_fixed_length(2, 3, 2), cycles_2_3_2);
    let cycles_1_3_2 = [[[0, 0, 0], [0, 0, 0]], [[1, 1, 1], [1, 1, 1]]];
    assert_eq!(enum_cycles_fixed_length(1, 3, 2), cycles_1_3_2);

    // l > k regime
    let cycles_6_3_2 = [
        [
            [0, 0, 0],
            [0, 0, 1],
            [0, 1, 1],
            [1, 1, 1],
            [1, 1, 0],
            [1, 0, 0],
            [0, 0, 0],
        ],
        [
            [0, 0, 1],
            [0, 1, 0],
            [1, 0, 1],
            [0, 1, 1],
            [1, 1, 0],
            [1, 0, 0],
            [0, 0, 1],
        ],
        [
            [0, 0, 1],
            [0, 1, 1],
            [1, 1, 0],
            [1, 0, 1],
            [0, 1, 0],
            [1, 0, 0],
            [0, 0, 1],
        ],
    ];
    assert_eq!(enum_cycles_fixed_length(6, 3, 2), cycles_6_3_2);
}

#[test]
fn test_enum_cycles_bounded_length() {
    let cycles_3_2 = vec![
        // 1-long
        vec![vec![0, 0, 0], vec![0, 0, 0]],
        vec![vec![1, 1, 1], vec![1, 1, 1]],
        // 2-long
        vec![vec![0, 1, 0], vec![1, 0, 1], vec![0, 1, 0]],
        // 3-long
        vec![vec![0, 0, 1], vec![0, 1, 0], vec![1, 0, 0], vec![0, 0, 1]],
        vec![vec![0, 1, 1], vec![1, 1, 0], vec![1, 0, 1], vec![0, 1, 1]],
        // 4-long
        vec![
            vec![0, 0, 0],
            vec![0, 0, 1],
            vec![0, 1, 0],
            vec![1, 0, 0],
            vec![0, 0, 0],
        ],
        vec![
            vec![0, 0, 1],
            vec![0, 1, 1],
            vec![1, 1, 0],
            vec![1, 0, 0],
            vec![0, 0, 1],
        ],
        vec![
            vec![0, 1, 1],
            vec![1, 1, 1],
            vec![1, 1, 0],
            vec![1, 0, 1],
            vec![0, 1, 1],
        ],
        // 5-long
        vec![
            vec![0, 0, 0],
            vec![0, 0, 1],
            vec![0, 1, 1],
            vec![1, 1, 0],
            vec![1, 0, 0],
            vec![0, 0, 0],
        ],
        vec![
            vec![0, 0, 1],
            vec![0, 1, 1],
            vec![1, 1, 1],
            vec![1, 1, 0],
            vec![1, 0, 0],
            vec![0, 0, 1],
        ],
        // 6-long
        vec![
            vec![0, 0, 0],
            vec![0, 0, 1],
            vec![0, 1, 1],
            vec![1, 1, 1],
            vec![1, 1, 0],
            vec![1, 0, 0],
            vec![0, 0, 0],
        ],
        vec![
            vec![0, 0, 1],
            vec![0, 1, 0],
            vec![1, 0, 1],
            vec![0, 1, 1],
            vec![1, 1, 0],
            vec![1, 0, 0],
            vec![0, 0, 1],
        ],
        vec![
            vec![0, 0, 1],
            vec![0, 1, 1],
            vec![1, 1, 0],
            vec![1, 0, 1],
            vec![0, 1, 0],
            vec![1, 0, 0],
            vec![0, 0, 1],
        ],
        // 7-long
        vec![
            vec![0, 0, 0],
            vec![0, 0, 1],
            vec![0, 1, 0],
            vec![1, 0, 1],
            vec![0, 1, 1],
            vec![1, 1, 0],
            vec![1, 0, 0],
            vec![0, 0, 0],
        ],
        vec![
            vec![0, 0, 0],
            vec![0, 0, 1],
            vec![0, 1, 1],
            vec![1, 1, 0],
            vec![1, 0, 1],
            vec![0, 1, 0],
            vec![1, 0, 0],
            vec![0, 0, 0],
        ],
        vec![
            vec![0, 0, 1],
            vec![0, 1, 0],
            vec![1, 0, 1],
            vec![0, 1, 1],
            vec![1, 1, 1],
            vec![1, 1, 0],
            vec![1, 0, 0],
            vec![0, 0, 1],
        ],
        vec![
            vec![0, 0, 1],
            vec![0, 1, 1],
            vec![1, 1, 1],
            vec![1, 1, 0],
            vec![1, 0, 1],
            vec![0, 1, 0],
            vec![1, 0, 0],
            vec![0, 0, 1],
        ],
        // 8-long
        vec![
            vec![0, 0, 0],
            vec![0, 0, 1],
            vec![0, 1, 0],
            vec![1, 0, 1],
            vec![0, 1, 1],
            vec![1, 1, 1],
            vec![1, 1, 0],
            vec![1, 0, 0],
            vec![0, 0, 0],
        ],
        vec![
            vec![0, 0, 0],
            vec![0, 0, 1],
            vec![0, 1, 1],
            vec![1, 1, 1],
            vec![1, 1, 0],
            vec![1, 0, 1],
            vec![0, 1, 0],
            vec![1, 0, 0],
            vec![0, 0, 0],
        ],
    ];

    let mut computed_cycles = enum_cycles_bounded_length(9, 3, 2);
    computed_cycles.sort_by_key(|x| (x.len(), x.clone()));
    assert_eq!(cycles_3_2, computed_cycles);
}

#[test]
fn test_enum_all_cycles() {
    fn enum_all_cycles(order: usize, sigma: u8) -> Vec<Cycle> {
        enum_cycles_bounded_length(usize::pow(sigma as usize, order as u32), order, sigma)
    }

    let cycles_3_2 = vec![
        // 1-long
        vec![vec![0, 0, 0], vec![0, 0, 0]],
        vec![vec![1, 1, 1], vec![1, 1, 1]],
        // 2-long
        vec![vec![0, 1, 0], vec![1, 0, 1], vec![0, 1, 0]],
        // 3-long
        vec![vec![0, 0, 1], vec![0, 1, 0], vec![1, 0, 0], vec![0, 0, 1]],
        vec![vec![0, 1, 1], vec![1, 1, 0], vec![1, 0, 1], vec![0, 1, 1]],
        // 4-long
        vec![
            vec![0, 0, 0],
            vec![0, 0, 1],
            vec![0, 1, 0],
            vec![1, 0, 0],
            vec![0, 0, 0],
        ],
        vec![
            vec![0, 0, 1],
            vec![0, 1, 1],
            vec![1, 1, 0],
            vec![1, 0, 0],
            vec![0, 0, 1],
        ],
        vec![
            vec![0, 1, 1],
            vec![1, 1, 1],
            vec![1, 1, 0],
            vec![1, 0, 1],
            vec![0, 1, 1],
        ],
        // 5-long
        vec![
            vec![0, 0, 0],
            vec![0, 0, 1],
            vec![0, 1, 1],
            vec![1, 1, 0],
            vec![1, 0, 0],
            vec![0, 0, 0],
        ],
        vec![
            vec![0, 0, 1],
            vec![0, 1, 1],
            vec![1, 1, 1],
            vec![1, 1, 0],
            vec![1, 0, 0],
            vec![0, 0, 1],
        ],
        // 6-long
        vec![
            vec![0, 0, 0],
            vec![0, 0, 1],
            vec![0, 1, 1],
            vec![1, 1, 1],
            vec![1, 1, 0],
            vec![1, 0, 0],
            vec![0, 0, 0],
        ],
        vec![
            vec![0, 0, 1],
            vec![0, 1, 0],
            vec![1, 0, 1],
            vec![0, 1, 1],
            vec![1, 1, 0],
            vec![1, 0, 0],
            vec![0, 0, 1],
        ],
        vec![
            vec![0, 0, 1],
            vec![0, 1, 1],
            vec![1, 1, 0],
            vec![1, 0, 1],
            vec![0, 1, 0],
            vec![1, 0, 0],
            vec![0, 0, 1],
        ],
        // 7-long
        vec![
            vec![0, 0, 0],
            vec![0, 0, 1],
            vec![0, 1, 0],
            vec![1, 0, 1],
            vec![0, 1, 1],
            vec![1, 1, 0],
            vec![1, 0, 0],
            vec![0, 0, 0],
        ],
        vec![
            vec![0, 0, 0],
            vec![0, 0, 1],
            vec![0, 1, 1],
            vec![1, 1, 0],
            vec![1, 0, 1],
            vec![0, 1, 0],
            vec![1, 0, 0],
            vec![0, 0, 0],
        ],
        vec![
            vec![0, 0, 1],
            vec![0, 1, 0],
            vec![1, 0, 1],
            vec![0, 1, 1],
            vec![1, 1, 1],
            vec![1, 1, 0],
            vec![1, 0, 0],
            vec![0, 0, 1],
        ],
        vec![
            vec![0, 0, 1],
            vec![0, 1, 1],
            vec![1, 1, 1],
            vec![1, 1, 0],
            vec![1, 0, 1],
            vec![0, 1, 0],
            vec![1, 0, 0],
            vec![0, 0, 1],
        ],
        // 8-long
        vec![
            vec![0, 0, 0],
            vec![0, 0, 1],
            vec![0, 1, 0],
            vec![1, 0, 1],
            vec![0, 1, 1],
            vec![1, 1, 1],
            vec![1, 1, 0],
            vec![1, 0, 0],
            vec![0, 0, 0],
        ],
        vec![
            vec![0, 0, 0],
            vec![0, 0, 1],
            vec![0, 1, 1],
            vec![1, 1, 1],
            vec![1, 1, 0],
            vec![1, 0, 1],
            vec![0, 1, 0],
            vec![1, 0, 0],
            vec![0, 0, 0],
        ],
    ];

    let mut computed_cycles = enum_all_cycles(3, 2);
    computed_cycles.sort_by_key(|x| (x.len(), x.clone()));
    assert_eq!(cycles_3_2, computed_cycles);
}
