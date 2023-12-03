pub fn counting_sort(vec: &mut Vec<usize>) {

    if vec.len() <= 1 {
        return;
    }
    
    let k = *vec.iter().max().unwrap();
    
    let mut c = vec![];
    
    for _ in 0..=k {
        c.push(0)
    }
    
    for i in &mut *vec {
        c[*i] += 1
    }

    let mut index = 0;
    for (num, &freq) in c.iter().enumerate() {
        for _ in 0..freq {
            vec[index] = num;
            index += 1;
        }
    }
    
    
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_bubble_sort() {
        use crate::counting_sort::counting_sort;

        let mut vec = vec![6, 0, 2, 1, 3, 4, 6, 1, 3, 2];

        counting_sort(&mut vec);

        assert_eq!(vec, [0, 1, 1, 2, 2, 3, 3, 4, 6, 6]);
    }
}
