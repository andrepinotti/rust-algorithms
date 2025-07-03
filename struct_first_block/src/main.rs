struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn main() {

    let mut user1 = User {
        active: true,
        username: "André".to_string(),
        email: "amostradinho@gmail.com".to_string(),
        sign_in_count: 1 
    };

    println!("{:?}", user1.username);

}
