#[derive(Debug)]
enum TypeEndIp {
    V4,
    V6
}

#[derive(Debug)]
struct EndIp{
    type_ip: TypeEndIp,
    adress: String
}

fn example_enum_simple() {
    let type1 = TypeEndIp::V4;
    let type2 = TypeEndIp::V6;

    println!("\nbasic example with enum");
    println!("type1={:?}\ntype2={:?}", type1, type2);
}

fn example_with_struct(){
    let home = EndIp{
        type_ip: TypeEndIp::V4,
        adress: String::from("127.0.0.1")
    };

    let loopback = EndIp {
        type_ip: TypeEndIp::V6,
        adress: String::from("::1")
    };

    println!("\nExample with Struct\n");
    println!("home: {:?} \nloop: {:?}", home, loopback);
}

#[derive(Debug)]
enum IpAddr{
    V4(String),
    V6(String)
}   

fn example_with_enum(){

    println!("\nExample With Enum\n");
    
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    
    println!("home: {:?} \nloop: {:?}", home, loopback);

}


fn main() {
    
    println!("*****Starting the software*****");

    example_enum_simple();  
    example_with_struct();
    example_with_enum();

}
