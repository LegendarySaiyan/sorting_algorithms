pub fn quicksort<T: Ord>(vec: &mut [T]) {
    if vec.len() <= 1 {
        return;
    }
    
    let q = partition(vec);
    quicksort(&mut vec[0..q]);
    quicksort(&mut vec[q + 1..]);
    
}

fn partition<T: Ord>(vec: &mut [T]) -> usize {
    
    let r = vec.len() - 1;
    
    let pivot = vec.len() / 2;
    
    vec.swap(pivot, r);
    
    let mut i = 0;
    
    for j in 0..r {
        if vec[j] <= vec[pivot] {
            vec.swap(i, j);
            i += 1;
            
        }
    }
    vec.swap(i, r);
    i 
}
