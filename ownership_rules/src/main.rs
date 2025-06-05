/*		[4.1. What is Ownership?]

Quais são as regras do Ownership ?

Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/


Regras do Ownership
	- Cada valor em Rust tem um proprietário (owner)
	- A cada momento pode existir apenas um proprietário para cada valor
	- Quando o proprietário chega ao final do seu escopo o valor é destruído (dropped)


Como a memória de um valor na Heap é liberada ?
	- Malloc/Free explícitos (linguagem C)
	- Coletor de lixo/Garbage Colector (linguagem Java)
Rust:
	- Conceito de Ownership
	- Quando termina o escopo da variável owner/proprietária do valor
*/



fn main() {
	
	let mut s3 = String::from("alo");

	// String literal "alo" é criado na área de código

	// Valor tipo String é criado na área de Heap
	// "alo" é copiado da área de código para o String na Heap
	// Valor 'Tipo String' pode mudar de tamanho durante a execução

	// Cria a variável mutável s3
	// s3 fica no Stack pois tem tamanho fixo e conhecido
	// s3 aponta para o valor 'Tipo String' que está na Heap

	println!("Valor de s3 é {}", s3);
	
	// Podemos mudar o conteúdo da variável 'Tipo String'
	// push_str() acrescenta um 'String literal' ao final do conteúdo da variável
	s3.push_str(", mundo");
	// s3 está na Stack, s3 aponta o String que está no Heap
	// O string literal ", mundo" é copiado da área de Código para o String na área de Heap 
	// Mais memória foi alocada automaticamente para a variável 'Tipo String'
	// s3 não muda de tamanho pois ela apenas aponta para a variável 'Tipo String'
	println!("Valor de s3 agora é {}", s3);

	// s3 é dona (owns) da variável 'Tipo String'
	// Como faço para liberar a memória usada pela variável 'Tipo String' ?
	// A memória é liberada automaticamente quando termina o escopo do seu dono

	{
		let s4 = String::from("alo alo alo alo");
		println!("Valor de s4 é {}", s4);
    }	// s4 é dona do String na Heap	
		// Tanto a memória de 's4' na Stack como a memória do String na Heap são liberadas
    	// Função 'drop' que faz isto é chamada automaticamente pelo compilador

	// Somente pode haver um dono para cada valor na Heap
	let s5 = s3;
	//println!("Valor de s3 é {s3}");	// s3 foi invalidada, perdeu a propriedade do String
	println!("Valor de s5 é {}", s5);

	let x = 10;			// 'x' e 'y' estão na Stack
	let y = x;				// Conteúdo de 'x' pode ser copiado

	println!("x={}, y={}", x , y);

}

