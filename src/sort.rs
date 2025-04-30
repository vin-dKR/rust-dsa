pub fn selection_sort(vec: &mut Vec<i32>) -> Vec<i32> {
    for i in 0..vec.len()-1 {
        let mut min_index = i;

        for j in i..vec.len() {
            if vec[min_index] > vec[j] {
                min_index = j;
            }
        }
        vec.swap(i, min_index);
    }
    return vec.to_vec();
}

pub fn bubble_sort(vec: &mut Vec<i32>) -> Vec<i32> {
    for i in 0..vec.len() {
        for j in 0..vec.len() - 1 - i {
            if vec[j] > vec[j + 1] {
                vec.swap(j, j+1);
            }
        }
    }

    return vec.to_vec();
}
