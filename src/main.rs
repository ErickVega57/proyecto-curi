use std::io::{self, Write};
mod contacto;
mod fecha;
mod funciones_contactos;
pub mod arbol_binario;

use contacto::{
    Contacto,
    validar_correo,
    validar_telefono,
};

use fecha::Fecha;

use funciones_contactos::{
    cargar_contactos,
    guardar_contacto,
    borrar_contacto,
    consultar_contacto,
};

use crate::funciones_contactos::{imprimir_por_fecha,imprimir_por_nombre};

fn main() {

    // Cargar contactos del archivo
    let mut contactos = cargar_contactos().unwrap_or_else(|err| {

        println!("Error al cargar contactos: {}", err);

        Vec::new()
    });

    loop {
        mostrar_menu();
        let opcion = get_input("Seleccione una opcion:");
        match opcion.as_str() {
            // AGREGAR CONTACTO
            "1" => {
                let nombre = get_input("Nombre:");
                let telefono = get_input("Telefono:");
                let fecha_str = get_input("Fecha (DD/MM/YYYY):");
                let correo = get_input("Correo:");

                // Validar teléfono
                if validar_telefono(&telefono).is_err() {
                    println!("Telefono invalido");
                    continue;
                }
                // Validar correo
                if validar_correo(&correo).is_err() {
                    println!("Correo invalido");
                    continue;
                }
                // Validar fecha
                let fecha_nacimiento = match Fecha::from_string(&fecha_str) {
                    Ok(fecha) => fecha,
                    Err(err) => {

                        println!("Error en fecha: {}", err);

                        continue;
                    }
                };
                // Crear contacto
                let contacto_nuevo = Contacto::new(
                    nombre,
                    telefono,
                    correo,
                    fecha_nacimiento,
                );
                // Guardar contacto
                match guardar_contacto(contacto_nuevo, &mut contactos) {
                    Ok(_) => {
                        println!("Contacto guardado exitosamente");
                    }
                    Err(err) => {
                        println!("Error al guardar: {}", err);
                    }
                }
            }
            // ELIMINAR CONTACTO
            "2" => {
                if contactos.is_empty() {
                    println!("No hay contactos guardados");
                } else {
                    let nombre =
                        get_input("Nombre del contacto a eliminar:");

                    match borrar_contacto(&nombre, &mut contactos) {

                        Ok(_) => {
                            println!("Contacto eliminado");
                        }

                        Err(err) => {
                            println!("Error: {}", err);
                        }
                    }
                }
            }
            // CONSULTAR CONTACTO
            "3" => {
                if contactos.is_empty() {

                    println!("No hay contactos guardados");

                } else {

                    let nombre =
                        get_input("Nombre del contacto:");

                    consultar_contacto(&nombre, &contactos);
                }
            }
            // LISTAR POR FECHA
            "4" => {
                imprimir_por_fecha(&contactos).expect("No hay contactos cargados");
            }
            // LISTAR POR ALFABETO
            "5" => {
                imprimir_por_nombre(&contactos).expect("No hay contactos cargados");
            }
            // SALIR
            "0" => {

                println!("Saliendo del programa...");
                break;
            }
            _ => {
                println!("Opcion no valida");
            }
        }
    }
}

fn mostrar_menu() {

    println!("\n===== AGENDA =====");

    println!("1. Agregar contacto");
    println!("2. Eliminar contacto");
    println!("3. Consultar contacto");
    println!("4. Listar por fecha");
    println!("5. Listar alfabeticamente");
    println!("0. Salir");
}

fn get_input(prompt: &str) -> String {

    print!("{} ", prompt);

    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .unwrap();

    input.trim().to_string()
}

