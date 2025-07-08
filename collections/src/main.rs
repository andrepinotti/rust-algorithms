

fn main() {
    
    //NOTE -> define type of vector, recommended
    // let mut my_vec: Vec<i32> = Vec::new();
    let mut my_vec = Vec::new();
    let vec_defined= vec!["aaa", "bbb", "ccc"];

    my_vec.push(2);
    my_vec.push(45);
    my_vec.push(3);

    println!("My vec: {:?}", my_vec);
    my_vec.sort();
    println!("My vec sorted: {:?}", my_vec);
    println!("vec defined: {:?}", vec_defined);

    println!("Take some value of vector: {:?}", my_vec.get(1));

}
