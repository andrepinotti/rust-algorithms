#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32
}

fn main() {
    
    let scale = 2;
    
    let rec = Rectangle{
        width: dbg!(30 * scale),
        height: 30,
    };
    
    println!("{:?}", rec);
    println!("{:?}", area(rec));
}

fn area(rec: Rectangle) -> i32{
    dbg!(rec.width * rec.height)    
}