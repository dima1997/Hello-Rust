/// Suma dos numeros enteros
/// 
/// # Examples
/// 
/// ```
/// let x = 4;
/// let y = 5;
/// let suma = mi_crate::operadores::sumador::sumar(x, y);
/// 
/// assert_eq!(suma, 9)
/// ```
pub fn sumar(x: i64, y: i64) -> i64 {
    x + y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suma_cero_y_cero_es_cero() {
        assert_eq!(sumar(0,0), 0);
    }

    #[test]
    fn suma_cero_y_uno_es_uno() {
        assert_eq!(sumar(0,1), 1);
    }
}
