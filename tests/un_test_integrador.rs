// Este archivo es un test de integración que hace uso de l interfaz 
// que expone nuestra libreria.

use mi_crate::operadores::{multiplicador, sumador};

#[test]
fn multiplicar_dos_por_la_suma_de_tres_mas_cuatro_es_catorce() {
    let suma = sumador::sumar(3, 4);
    let producto = multiplicador::multiplicar(2, suma);
    assert_eq!(producto, 14);
}