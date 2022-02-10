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