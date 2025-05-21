type Letter = u8;
pub type Word = Vec<Letter>;
pub type Cycle = Vec<Word>;

#[derive(Debug)]
pub struct LyndonWord {
    vec: Vec<Letter>,
    len: usize,
    max_letter: Letter,
}

impl LyndonWord {
    pub fn get_word(&self) -> Word {
        self.vec[0..self.len as usize].to_vec()
    }

    pub fn new_smallest(len: usize, max_letter: Letter) -> Self {
        let mut vec = vec![0; len];
        if len > 1 {
            vec[len - 1] = 1;
        }
        LyndonWord {
            vec,
            len,
            max_letter,
        }
    }

    pub fn iter(&mut self, fixed_length: bool) -> LyndonWordIter<'_> {
        if !fixed_length {
            self.len = 1; // if fixed length, the smallest word is aaaaab, otherwise it's a
        }
        LyndonWordIter {
            lw: self,
            fixed_length,
            stop_on_next_call: false,
        }
    }
}

pub struct LyndonWordIter<'a> {
    lw: &'a mut LyndonWord,
    fixed_length: bool,
    stop_on_next_call: bool,
}

impl<'a> Iterator for LyndonWordIter<'a> {
    type Item = Vec<Letter>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stop_on_next_call {
            return None;
        }

        let word = self.lw.get_word().to_owned();

        match self.fixed_length {
            true => {
                if let Err(_) = next_fllw(self.lw) {
                    self.stop_on_next_call = true;
                }
            }
            false => {
                if let Err(_) = next_bllw(self.lw) {
                    self.stop_on_next_call = true;
                }
            }
        }

        Some(word)
    }
}

fn next_bllw(lw: &mut LyndonWord) -> Result<(), String> {
    let n = lw.vec.len();
    let i = lw.len;

    if i == 1 && lw.vec[0] == lw.max_letter {
        return Err("This was the greatest Lyndon word of this fixed length".to_string());
    }

    for k in i..n {
        lw.vec[k] = lw.vec[k % i];
    }

    // Remove right trailing max letters, and increase the first non-max letter
    let mut i = n;
    while lw.vec[i - 1] == lw.max_letter {
        i -= 1
    }
    lw.vec[i - 1] += 1;

    lw.len = i;

    Ok(())
}

fn next_fllw(lw: &mut LyndonWord) -> Result<(), String> {
    // Source. "Generation d'une section des classes de conjugaison et arbre
    // des mots de Lyndon de longueur bornee", Jean-Pierre DUVAL (1988)
    //
    // NOTE. Just shifted the indices by one when looking to a vector cell.
    let n = lw.vec.len();
    let mut i = lw.len;

    // Add this to handle |l|=1
    if n == 1 && lw.vec[1 - 1] == lw.max_letter {
        return Err("This was the greatest Lyndon word of this fixed length".to_string());
    }

    // Remove right trailing max letters, and increase the first non-max letter
    while lw.vec[i - 1] == lw.max_letter {
        i -= 1;
    }
    lw.vec[i - 1] += 1;

    // If the first letter is maximal, then the current Lyndon word is
    // already the greatest of its size (only if n>1)
    if lw.vec[1 - 1] == lw.max_letter && n > 1 {
        return Err("This was the greatest Lyndon word of this fixed length".to_string());
    }

    // Repeat the [0..k-1] pattern integrally, as many times as possible
    // NOTE. "Boucle 1" in the source
    let mut k = i;
    while i < n - k {
        for j in 1..=i {
            lw.vec[k + j - 1] = lw.vec[j - 1]
        }
        k += i; // NOTE. There was a typo here in the source
    }

    // Repeat the [0..k-1] pattern non-integrally (stoping at index n-1)
    // Remove right trailing max letters, and update
    // Repeat the short pattern (starting at d) integrally as much as possible
    // Stop when the previous step exactly stop at index i-1
    // NOTE. "Boucle 2" in the source
    while i != n {
        for j in 1..=(n - k) {
            lw.vec[k + j - 1] = lw.vec[j - 1];
        }
        i = n;
        while lw.vec[i - 1] == lw.max_letter {
            i -= 1;
        }
        lw.vec[i - 1] += 1;
        let d = i - k;

        // NOTE. "Boucle 2.1" in the source
        while d <= n - i {
            for jj in 1..=d {
                lw.vec[i + jj - 1] = lw.vec[k + jj - 1];
            }
            i += d;
        }
        k = i;
    }
    Ok(())
}

#[test]
fn test_fllw_smallest() {
    let fllw = LyndonWord::new_smallest(6, 1);
    assert_eq!(fllw.get_word(), [0, 0, 0, 0, 0, 1]);

    let fllw = LyndonWord::new_smallest(1, 1);
    assert_eq!(fllw.get_word(), [0]);
}

#[test]
fn test_fixed_length_lw_iterator() {
    let fllw42 = [
        [0, 0, 0, 1],
        [0, 0, 0, 2],
        [0, 0, 1, 1],
        [0, 0, 1, 2],
        [0, 0, 2, 1],
        [0, 0, 2, 2],
        [0, 1, 0, 2],
        [0, 1, 1, 1],
        [0, 1, 1, 2],
        [0, 1, 2, 1],
        [0, 1, 2, 2],
        [0, 2, 1, 1],
        [0, 2, 1, 2],
        [0, 2, 2, 1],
        [0, 2, 2, 2],
        [1, 1, 1, 2],
        [1, 1, 2, 2],
        [1, 2, 2, 2],
    ];

    let mut lw = LyndonWord::new_smallest(4, 2);
    assert_eq!(lw.iter(true).collect::<Vec<_>>(), fllw42);

    let fllw12 = [[0], [1], [2]];
    let mut lw = LyndonWord::new_smallest(1, 2);
    assert_eq!(lw.iter(true).collect::<Vec<_>>(), fllw12);
}

#[test]
fn test_bounded_length_lw_iterator() {
    let bllw42: Vec<Vec<Letter>> = vec![
        vec![0],
        vec![0, 0, 0, 1],
        vec![0, 0, 0, 2],
        vec![0, 0, 1],
        vec![0, 0, 1, 1],
        vec![0, 0, 1, 2],
        vec![0, 0, 2],
        vec![0, 0, 2, 1],
        vec![0, 0, 2, 2],
        vec![0, 1],
        vec![0, 1, 0, 2],
        vec![0, 1, 1],
        vec![0, 1, 1, 1],
        vec![0, 1, 1, 2],
        vec![0, 1, 2],
        vec![0, 1, 2, 1],
        vec![0, 1, 2, 2],
        vec![0, 2],
        vec![0, 2, 1],
        vec![0, 2, 1, 1],
        vec![0, 2, 1, 2],
        vec![0, 2, 2],
        vec![0, 2, 2, 1],
        vec![0, 2, 2, 2],
        vec![1],
        vec![1, 1, 1, 2],
        vec![1, 1, 2],
        vec![1, 1, 2, 2],
        vec![1, 2],
        vec![1, 2, 2],
        vec![1, 2, 2, 2],
        vec![2],
    ];

    let mut lw = LyndonWord::new_smallest(4, 2);
    assert_eq!(lw.iter(false).collect::<Vec<_>>(), bllw42);

    let bllw12 = [[0], [1], [2]];
    let mut lw = LyndonWord::new_smallest(1, 2);
    assert_eq!(lw.iter(false).collect::<Vec<_>>(), bllw12);
}
