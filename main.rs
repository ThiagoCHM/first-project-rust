fn main() {
    let variavel: i32 = 1025;
    println!(
        "Variável = {},\n tamanho = {}\n",
        variavel,
        std::mem::size_of_val(&variavel)
    );

    let decimal: f32 = 2.5;
    println!(
        "decimal = {},\n tamanho = {}\n",
        decimal,
        std::mem::size_of_val(&decimal)
    );

    let mut booleana: bool = false;
    booleana = true;
    println!(
        "Booleana = {},\n Tamanho de uma booleana = {}\n",
        booleana,
        std::mem::size_of_val(&booleana)
    );

    let letra: char = 'T';
    println!(
        "Tamanho de um Caracter é = {},\n",
        std::mem::size_of_val(&letra)
    );
}
