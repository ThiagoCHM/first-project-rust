const PI: f32 = 3.14;
static mut GLOBAL: u8 = 1;

fn soma(a: i32, b: i32) -> i32 {
    a + b
}

fn sombra() {
    let a = 1287;

    {
        let b = 70 * 7;
        println!("b = {}", b);
    }

    println!("a de fora = {}", a);
}

fn escopo() {
    println!("PI = {}", PI);

    unsafe {
        println!("variavel_global = {}", GLOBAL);
    }

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

fn main() {
    escopo();
    sombra();

    println!("Soma = {}", soma(2, 3));
}
