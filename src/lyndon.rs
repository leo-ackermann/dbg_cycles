
type Letter = u8;

struct FixedLengthLW {
    vec: Vec<Letter>,
    len: usize,
    max_letter: Letter,
}

impl FixedLengthLW {
    pub fn get_word(&self) -> &[Letter] {
        &self.vec[0..self.len as usize]
    }

    pub fn new_smallest(len: usize, max_letter: Letter) -> Self {
        let mut vec = vec![0; len];
        vec[len-1] = 1;
        FixedLengthLW {vec, len, max_letter}
    }

    fn iter(&mut self) -> FixedLengthLWIter<'_> {
        FixedLengthLWIter { fllw:self, is_greatest:false }
    }
}

struct FixedLengthLWIter<'a> {
    fllw: &'a mut FixedLengthLW,
    is_greatest: bool,
}

impl<'a> Iterator for FixedLengthLWIter<'a> {
    type Item = Vec<Letter>;

    fn next(&mut self) -> Option<Self::Item> {

        if self.is_greatest{
            return None;
        }

        let word = self.fllw.get_word().to_owned();

        if let Err(_) = next_fllw(self.fllw) {
            self.is_greatest = true;
        }

        Some(word)
    }
}




fn next_fllw(fllw: &mut FixedLengthLW) -> Result<(), String> {
        // Source. "Generation d'une section des classes de conjugaison et arbre
        // des mots de Lyndon de longueur bornee", Jean-Pierre DUVAL (1988)
        //
        // NOTE. Just shifted the indices by one when looking to a vector cell.
        let n = fllw.len;

        // Remove right trailing max letters, and increase the first non-max letter
        let mut i = n;
        while fllw.vec[i - 1] == fllw.max_letter {
            i -= 1
        }
        fllw.vec[i - 1] += 1;

        // If the first letter is maximal, then the current Lyndon word is
        // already the greatest of its size
        if fllw.vec[1 - 1] == fllw.max_letter {
            return Err("This was the greatest Lyndon word of this size".to_string());
        }

        // Repeat the [0..k-1] pattern integrally, as many times as possible
        // NOTE. "Boucle 1" in the source
        let mut k = i;
        while i < n - k {
            for j in 1..=i {
                fllw.vec[k + j - 1] = fllw.vec[j - 1]
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
                fllw.vec[k + j - 1] = fllw.vec[j - 1];
            }
            i = n;
            while fllw.vec[i - 1] == fllw.max_letter {
                i -= 1;
            }
            fllw.vec[i - 1] += 1;
            let d = i - k;

            // NOTE. "Boucle 2.1" in the source
            while d <= n - i {
                for jj in 1..=d {
                    fllw.vec[i + jj - 1] = fllw.vec[k + jj - 1];
                }
                i += d;
            }
            k = i;
        }
        Ok(())
    }








// struct fixedLenghtLW {
//     vec: Vec<letter>,
//     len: usize,
// }


#[test]
fn test_fllw_smallest() {
    let fllw = FixedLengthLW::new_smallest(6, 1);
    assert_eq!(fllw.get_word(), [0,0,0,0,0,1]);
}

#[test]
fn test_fllw_iterator() {
    let lw42 = [
        [0, 0, 0, 1], [0, 0, 0, 2], [0, 0, 1, 1], [0, 0, 1, 2],
        [0, 0, 2, 1], [0, 0, 2, 2], [0, 1, 0, 2], [0, 1, 1, 1],
        [0, 1, 1, 2], [0, 1, 2, 1], [0, 1, 2, 2], [0, 2, 1, 1],
        [0, 2, 1, 2], [0, 2, 2, 1], [0, 2, 2, 2], [1, 1, 1, 2],
        [1, 1, 2, 2], [1, 2, 2, 2]
    ];

    let mut fllw = FixedLengthLW::new_smallest(4, 2);
    assert_eq!(fllw.iter().collect::<Vec<_>>(), lw42);
}






// fn next_lyndon_of_same_size(&mut self) -> Result<(), String> {
//     // Source. "Generation d'une section des classes de conjugaison et arbre
//     // des mots de Lyndon de longueur bornee", Jean-Pierre DUVAL (1988)
//     //
//     // NOTE. Just shifted the indices by one when looking to a vector cell.
//     let n = self.lw.len();

//     // Remove right trailing max letters, and increase the first non-max letter
//     let mut i = n;
//     while self.lw[i - 1] == self.max_letter {
//         i -= 1
//     }
//     self.lw[i - 1] += 1;

//     // If the first letter is maximal, then the current Lyndon word is
//     // already the greatest of its size
//     if self.lw[1 - 1] == self.max_letter {
//         return Err("This was the greatest Lyndon word of this size".to_string());
//     }

//     // Repeat the [0..k-1] pattern integrally, as many times as possible
//     // NOTE. "Boucle 1" in the source
//     let mut k = i;
//     while i < n - k {
//         for j in 1..=i {
//             self.lw[k + j - 1] = self.lw[j - 1]
//         }
//         k += i; // NOTE. There was a typo here in the source
//     }

//     // Repeat the [0..k-1] pattern non-integrally (stoping at index n-1)
//     // Remove right trailing max letters, and update
//     // Repeat the short pattern (starting at d) integrally as much as possible
//     // Stop when the previous step exactly stop at index i-1
//     // NOTE. "Boucle 2" in the source
//     while i != n {
//         for j in 1..=(n - k) {
//             self.lw[k + j - 1] = self.lw[j - 1];
//         }
//         i = n;
//         while self.lw[i - 1] == self.max_letter {
//             i -= 1;
//         }
//         self.lw[i - 1] += 1;
//         let d = i - k;

//         // NOTE. "Boucle 2.1" in the source
//         while d <= n - i {
//             for jj in 1..=d {
//                 self.lw[i + jj - 1] = self.lw[k + jj - 1];
//             }
//             i += d;
//         }
//         k = i;
//     }
//     Ok(())
// }
