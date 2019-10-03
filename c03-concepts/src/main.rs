fn main() {
    // const MAX_POINTS: u32 = 100_000 // exemplo de constante. Constantes podem ser declarados em
    // qualquer escopo, incluindo o global.
    // let x = 5; // variável imutável. Se assemelha a constante, e seu valor não pode ser alterado
    let mut x = 5; // variável mutável.
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // quando este comportamento acontece, diz-se que a variável foi "sombreada".
    // A diferença do sombreamento de variáveis para variáveis mutáveis é que variáveis sombreadas
    // são novas variáveis de fato, só que com o mesmo nome e com os valores das variáveis antigas,
    // e com a vantagem de ainda ser imutável. 
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);

    // Com o sombreamento de variáveis, fica mais fácil também mudar o tipo da variável. Com
    // variáveis mutáveis isso não é permitido
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);

    // rust suporta também tipos compostos, como tuplas:
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of second element of tuple is: {}", tup.1);
    
    // uma variável precedida por um underscore é marcada como "unused" pelo 
    //compilador e desabilita o warning
    let (a,_b,_c) = tup;     
    println!("The value of a is: {}", a);
    println!("The value of b is: {}", _b);

    // a diferença de tuplas para arrays é que os elementos de um array obrigatoriamente têm que
    // ser do mesmo tipo e o tamabnho é fixo.
    let _arr = [1, 2, 3, 4];
    // para escrever arrays com o mesmo valor em todos eles:
    let _empty_arr = [3;5]; // [3, 3, 3, 3, 3];
    // ou com elementos de tipo declarado:
    let _declared_arr: [i32;5] = [1, 2, 3, 4, 5];
    
}
