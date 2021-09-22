# Ejemplo de proyecto en Rust

## Esquema de directorios

### src/
Aqui va el codigo de nuestro programa y/o libreria.

#### Test unitarios
Los test unitarios se definen dentro del mismo archivo en donde se encuentra la función o clase que se desea testear:

```
#[cfg(test)]
mod tests {
    super::*;

    #[test]
    fn un_test() {
        assert_eq!(2 + 2, 4);
    }
}
```

La macro `#[cfg(test)]` hace que el modulo **tests** solo compile cuando se ejecuta el comando: `cargo test`.

La expresión `super::*;` permite utilizar las funciones definidas en el mismo archivo pero por fuera del modulo **tests**.

La macro `#[test]` le indica al compilador que la siguiente función es un test.

#### Subdirectorios
Para utilizar subdirectorios se debe crear un archivo **mod.rs** en donde deben declararse los modulos y submodulos que existen en el subdirectorio:
```
mod <nombre del modulo o archivo>
pub mod <nombre del modulo o archivo> // modulo publico
```

#### Imports
Para importar un archivo se debe especificar la ruta del mismo desde el archivo `crate` de referencia que se corresponde al archivo `lib.rs`. Por ejemplo:
```
use crate::operadores::sumador::sumar
use crate::operadores::sumador{suma}
use crate::operadores::{sumador} // => Se utiliza como sumador::suma(...)
```

#### Documentación
Doble barra `//`: Comentario comunes.
Triple barra `///`: Documentación que se propaga a la documentación generada por el comando `cargo doc` (utilizar el flag `--open` para abrir la documentación tras generla). En este tipo de documentación se pueden definir ejemplos, que se ejecutan como tests y se visualizan en el html que cargo genera.
Doble barra con signo de exclamación `//!`: Documentación de uso exclusivo del archivo **lib.rs**. Se utiliza para introducir nuestra libreria.

### target/
Aqui se almacenan los archivos compilados, los ejecutables, y la documentación generada.
Este directorios es muy grande, por lo que no debe ser *pusheado* al repositorio.

### tests/
En esta carpeta se pueden colocar varios archivos que corresponden a distintos tests integradores que solo pueden hacer uso de la API que **lib.rs** y los **mod.rs** subsecuentes definen.