#[derive(Debug)]
enum TypeEndIp {
    V4,
    V6
}

fn example_enum_simple() {
    let type1 = TypeEndIp::V4;
    let type2 = TypeEndIp::V6;

    println!("\nbasic example with enum");
    println!("type1={:?}\ntype2={:?}", type1, type2);
}

fn main() {
    
    println!("*****Starting the software*****");

    example_enum_simple();  

}
