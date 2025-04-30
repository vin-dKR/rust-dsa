// mod sort;
mod string;

fn main() {
    // let mut vec = vec![3, 2, 5, 1, 4];
    let mut str = String::from("Vinod");

    println!("{:?}", string::reverse_string(&mut str));
}

