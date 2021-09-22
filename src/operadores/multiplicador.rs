use crate::operadores::sumador::{sumar};

/// Multiplica dos numeros enteros
/// 
/// # Examples
/// 
/// ```
/// let x = 4;
/// let y = 5;
/// let producto = mi_crate::operadores::multiplicador::multiplicar(x, y);
/// 
/// assert_eq!(producto, 20);
/// ```
pub fn multiplicar(x: i64, y:i64) -> i64 {
    let es_negativo = ((x < 0) && (y > 0)) || ((x > 0) && (y < 0));
    let x_abs = x.abs();
    let y_abs = y.abs();

    let mut producto = 0;
    for _ in 0..(y_abs) {
        producto = sumar(producto, x_abs)
    }
    if es_negativo {
        producto *= -1
    }
    producto
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiplicar_dos_y_dos_es_cuatro() {
        assert_eq!(multiplicar(2,2), 4);
    }

    #[test]
    fn multiplicar_dos_y_tres_es_seis() {
        assert_eq!(multiplicar(2,3), 6);
    }

    #[test]
    fn multiplicar_dos_y_menos_tres_es_menos_seis() {
        assert_eq!(multiplicar(2,-3), -6);
    }
}
