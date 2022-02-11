use std::fmt::Debug;

pub fn permutations<T: Copy + Debug>(set: &Vec<T>) -> Vec<Vec<T>> {
    // let permutations_count: usize = (1..(set.len() + 1)).into_iter().product();
    if set.len() == 1 {
        vec![set.to_owned()]
    } else {
        let mut perms: Vec<Vec<T>> = vec![];
        for (i, e) in set.into_iter().enumerate() {
            let mut set_clone = set.clone();
            set_clone.remove(i);
            for p in permutations(&set_clone) {
                let mut perm: Vec<T> = vec![*e];
                for sub_e in p {
                    perm.push(sub_e);
                }
                perms.push(perm);
            }
        }
        perms
    }
}

pub fn parity(permutation: &Vec<i32>) -> i8 {
    let mut inversions_count: u32 = 0;
    for (i, e) in permutation.into_iter().enumerate() {
        for (j, f) in permutation.into_iter().enumerate() {
            if i < j && *e > *f {
                inversions_count = inversions_count + 1;
            }
        }
    }
    if inversions_count % 2 == 0 {
        1
    } else {
        -1
    }
}