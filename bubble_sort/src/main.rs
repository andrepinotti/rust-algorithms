fn main() {
    
    let mut x = [78, 14, 45, 765, 344, 54];
    println!("Array não ordenado {:?}",x);
    let mut aux: i32;
    let mut cont = 0;
    for i in 0..x.len()-1 {
        cont = cont + 1;
        for j in 0..x.len()-i-1{
            if x[j] > x[j+1] {
                aux = x[j+1];
                x[j+1] = x[j];
                x[j] = aux;
    
            }
        }
    }

    println!("Comparações: {}", cont);
    println!("Meu array ordenado: {:?}", x);

}
