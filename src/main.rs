// Este es el archivo de entrada de nuestro programa

use mi_crate::operadores::{multiplicador, sumador};

fn main() {
    let suma = sumador::sumar(10, 15);
    let producto = multiplicador::multiplicar(2, suma);
    println!("2 x (10 + 15) = {}", producto);
}
