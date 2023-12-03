pub fn quicksort<T: Ord + std::fmt::Debug>(vec: &mut [T]) {
    if vec.len() <= 1 {
        return;
    }
    
    let q = partition(vec);
    
    quicksort(&mut vec[0..q]);
    quicksort(&mut vec[q + 1..])
    
}

fn partition<T: Ord + std::fmt::Debug>(vec: &mut [T]) -> usize {

    let pivot_index = vec.len() / 2;
    
    let p: usize = vec.len() - 1;
    
    vec.swap(pivot_index, p);

    let mut i = 0;
    for j in 0..p {
        if vec[j] <= vec[p] {
            vec.swap(i, j);
            i += 1;
        }
    }

    vec.swap(i, p);
    i
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_bubble_sort() {
        use crate::quicksort::quicksort;

        let mut vec = vec![6, 0, 2, 1, 3, 4, 6, 1, 3, 2];

        quicksort(&mut vec);

        assert_eq!(vec, [0, 1, 1, 2, 2, 3, 3, 4, 6, 6]);
    }
}
