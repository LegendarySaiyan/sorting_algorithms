fn build_max_heap<T: Ord>(vec: &mut [T]) {
    
    let len = vec.len() / 2;
    
    for i in (0..len).rev() {
        heapify(vec, vec.len(), i)
    }

}


fn heapify<T: Ord>(vec: &mut [T], len: usize, i: usize) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < len && vec[left] > vec[largest] {
        largest = left;
    }

    if right < len && vec[right] > vec[largest] {
        largest = right;
    }

    if largest != i {
        vec.swap(i, largest);
        heapify(vec, len, largest);
    }
}

pub fn heapsort<T: Ord + std::fmt::Debug>(vec: &mut [T]) {
    let len = vec.len();
    
    build_max_heap(vec);
    
    for i in (1..len).rev()  {
        vec.swap(0, i);
        heapify(vec, i, 0)
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_bubble_sort() {
        use crate::heap_sort::heapsort;

        let mut vec = vec![6, 0, 2, 1, 3, 4, 6, 1, 3, 2];

        heapsort(&mut vec);

        assert_eq!(vec, [0, 1, 1, 2, 2, 3, 3, 4, 6, 6]);
    }
}
