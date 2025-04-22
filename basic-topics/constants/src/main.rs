const SEGUNDOS_EM_MINUTOS: i32 = 60;
const SEGUNDOS_EM_HORAS: i32 = 60 * 60;

fn main() {

    println!("*****Início do programa******");
    let x = SEGUNDOS_EM_MINUTOS;
    println!("Segundos em 1 minutos: {x}");
    let y = SEGUNDOS_EM_HORAS;
    println!("Segundos em 1 hora: {y}");
    println!("Segundos em 5 horas: {}", y * 5);


}