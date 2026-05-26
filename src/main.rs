mod fecha;
mod contacto;

use crate::fecha::Fecha;

fn main() {
    // Usar constructor (sin dia:, mes:, anio:)
    let fecha1 = Fecha::new(31, 3, 2024).unwrap();
    let fecha3 = Fecha::new(31, 16, 2024);
    match fecha3 {
        Ok(f) => println!("Fecha válida: {}", f.to_string()),
        Err(e) => println!("Error: {}", e),
    }

    // Usar from_string
    match Fecha::from_string("20/01/2024") {
        Ok(fecha2) => {
            // Usar método de instancia
            if fecha1.cumple_despues_de(&fecha2) {
                println!("Fecha1 cumple despues de Fecha2");
            } else {
                println!("Fecha1 NO cumple despues de Fecha2");
            }

            // Mostrar fechas
            println!("{}",fecha1.to_string());
            println!("{}",fecha2.to_string());

            // Usar getters (necesitas implementarlos)
            println!("Día de fecha1: {}", fecha1.dia());
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    // Obtener fecha actual
    let fecha_actual = Fecha::obtener_fecha_actual();

    println!("{}",fecha_actual.to_string())
}