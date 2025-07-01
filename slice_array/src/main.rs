/* O Tipo Slice		[4.3. The Slice Type]

Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/



fn main() {
   
	let a = [11, 22, 33, 44, 55];
	let slice = &a[1..=3];		// 3 inclusive
    let slice_2 = &a[1..3];     // não conta o 3 índice

	for elemento in slice {
		println!("primeiro array: {elemento}");
	}

    for i in slice_2 {
        println!("segundo array: {i}");
    }

}


