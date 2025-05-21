use crate::Word;

pub fn is_perfect(word: &Word, k: usize) -> bool {
    let mut wword = word.clone();
    wword.extend(word.clone());

    let mut kmers = <Vec<Word>>::new();
    for i in 0..word.len() {
        kmers.push(wword[i..i + k].to_vec());
    }

    kmers.sort();
    kmers.dedup();

    kmers.len() == word.len()
}

pub fn is_perfect_lyndon(word: &Word, k: usize) -> bool {
    if word.len() <= k {
        return true;
    }

    let mut wword = word.clone();
    wword.extend(word.clone());

    let mut kmers = <Vec<Word>>::new();
    for i in 0..word.len() {
        kmers.push(wword[i..i + k].to_vec());
    }

    kmers.sort();
    kmers.dedup();

    kmers.len() == word.len()
}

#[test]
fn test_is_perfect() {
    assert_eq!(is_perfect(&vec![0, 1, 0, 1, 1], 2), false);
    assert_eq!(is_perfect(&vec![0, 1, 0, 1, 1], 3), false);
    assert_eq!(is_perfect(&vec![0, 1, 0, 1, 1], 4), true);
    assert_eq!(is_perfect(&vec![0, 1, 0, 1, 1], 5), true);
    assert_eq!(is_perfect(&vec![0, 1, 0, 1, 1], 6), true);
}
