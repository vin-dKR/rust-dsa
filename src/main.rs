mod sort;

fn main() {
    let mut vec = vec![3, 2, 5, 1, 4];

    println!("{:?}", sort::selection_sort(&mut vec));
}

