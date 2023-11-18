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
