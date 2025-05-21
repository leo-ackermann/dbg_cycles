
type letter = u8;

struct fixedLenghtLW {
    vec: Vec<letter>,
    len: usize,
}

impl fixedLenghtLW {
    pub fn get_word(self) -> &[letter] {
        &self.vec[0..self.len]
    }

    pub fn new_smallest(len: u8) {
        let mut vec = vec![0; len as usize];
        vec[len] = 1;
        fixedLenghtLW {vec, len}
    }
}

// struct fixedLenghtLW {
//     vec: Vec<letter>,
//     len: usize,
// }


#[test]
fn test_fllw_smallest() {
    let fllw = fixedLenghtLW::new_smallest(6);
    assert_eq(fllw.get_word(), [0,0,0,0,0,1]);
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
