pub fn counting_sort(vec: &mut Vec<usize>) -> Vec<usize> {
    if vec.len() <= 1 {
        return vec.clone();
    }
    
    let n = vec.len();
    let k = *vec.iter().max().unwrap();
    
    let mut c = vec![];
    
    for _ in 0..=k {
        c.push(0)
    }
    
    for i in vec {
        c[*i] += 1
    }
    
    let b: Vec<usize> = c.iter()
                    .enumerate()
                    .flat_map(|(index, &count)| vec![index; count])
                    .collect();
    
    b
}
