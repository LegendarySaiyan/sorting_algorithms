pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();

    for i in 1..len {
        let mut j = i;

        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_bubble_sort() {
        use crate::insertion_sort::insertion_sort;

        let mut vec = vec![6, 0, 2, 1, 3, 4, 6, 1, 3, 2];

        insertion_sort(&mut vec);

        assert_eq!(vec, [0, 1, 1, 2, 2, 3, 3, 4, 6, 6]);
    }
}
