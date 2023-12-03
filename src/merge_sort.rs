pub fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
    let len = arr.len();

    if len <= 1 {
        return;
    }

    let mid = len / 2;

    let mut left = arr[0..mid].to_vec();
    let mut right = arr[mid..].to_vec();

    merge_sort(&mut left);
    merge_sort(&mut right);

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i].clone();
            i += 1;
        } else {
            arr[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i].clone();
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_bubble_sort() {
        use crate::merge_sort::merge_sort;

        let mut vec = vec![6, 0, 2, 1, 3, 4, 6, 1, 3, 2];

        merge_sort(&mut vec);

        assert_eq!(vec, [0, 1, 1, 2, 2, 3, 3, 4, 6, 6]);
    }
}
