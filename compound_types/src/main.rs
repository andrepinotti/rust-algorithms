fn main() {

    let t1 = (500, 6.4, true); 
    let t2 = (500, 7.8, false);

    println!("Minha tupla 1 tem: {:?} \nMinha tupla 2 tem: {:?}", t1, t2);

    //Quebrando a tupla em partes
    let (x1, y1, z1) = t2;
    println!("\nAgora minha tupla tem: {x1} {y1} {z1}");

    //Índices da tupla, pegando valor por valor
    println!("\nÍndices da tupla: {:?} {:?} {:?}", t1.0, t1.1, t1.2);

    meses();

}

fn meses(){

    let meses = ["Janeiro", "Fevereiro", "Março", "Abril", "Maio", "Junho", "Julho", "Agosto",
    "Setembro", "Outubro", "Novembro", "Dezembro"];

    let cc = [3; 5]; // Quando colocamos um ponto e vírgula é porque finalizamos o primeiro valor do array 
    //+ o numero de vezes que ele será repetido 
    let dd = [2, 5];

    println!("Array CC: {:?}", cc);
    println!("Array DD: {:?}", dd);
    println!("Pegando um valor do nosso array meses: {}", meses[4]);

}

