#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {

    fn square(size:i32) -> Self {
        Self {
            width: size,
            height: size
        }
    }

    fn area(&self) -> i32{
        &self.height * &self.width
    }

}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    
    let scale = 2;
    
    let rec = Rectangle{
        width: dbg!(30 * scale),
        height: 30,
    };

    let rec_1=  Rectangle{
        width: dbg!(5 * scale),
        height: 7
    };

    let rec_2 = Rectangle {
        width: dbg!(7 * scale),
        height: 9
    };

    println!("RECTANGLES AND YOUR AREAS\n");

    println!("{:?}", rec);
    println!("Area of rec is: {:?}", rec.area());

    println!("{:?}", rec_1);
    println!("{:?}", rec_1.area());
    
    println!("{:?}", rec_2);
    println!("{:?}", rec_2.area());

    println!("\nCAN REC CONTAIN REC 1");
    let x = rec.can_hold(&rec_1);
    if x { println!("YES") } else { println!("NOT"); }

    println!("\nCAN REC 1 CONTAIN REC 2");
    let y = rec_1.can_hold(&rec_2);
    if y { println!("YES"); } else { println!("NOT"); }

    println!("\nCAN REC 2 CONTAIN REC 1");
    let z = rec_2.can_hold(&rec_1);
    if z { println!("YES"); } else { println!("NOT"); }

    let new_rec = Rectangle::square(3);

    println!("ASSOCIATED FUNCTION RETURNS");
    println!("{:?}", new_rec);
    println!("Area of new rectangle: {:?}", new_rec.area());

}
