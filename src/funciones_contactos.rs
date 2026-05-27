use std::cmp::Ordering;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::fs::{File, OpenOptions};
use crate::arbol_binario::arbol::Arbol;
use crate::contacto::{Contacto, validar_correo, validar_telefono};
use crate::fecha::Fecha;

pub fn borrar_contacto(nombre: &str, contactos: &mut Vec<Contacto>) -> io::Result<()> {

    // Busca la posición del contacto
    if let Some(pos) = contactos.iter().position(|c| c.nombre() == nombre) {

        // Lo elimina del vector
        contactos.remove(pos);

        // Reescribe TODO el archivo desde cero
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open("contactos.txt")?;

        // Guardar nuevamente todos los contactos
        for contacto in contactos {

            writeln!(
                file,
                "{},{},{},{}",
                contacto.nombre(),
                contacto.telefono(),
                contacto.fecha_nacimiento().to_string(),
                contacto.correo()
            )?;
        }

        Ok(())

    } else {

        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "No existe ese contacto"
        ))
    }
}

pub fn cargar_contactos() -> io::Result<Vec<Contacto>> {

    let mut contactos = Vec::new();

    let file = File::open("contactos.txt");

    let file = match file {

        Ok(file) => file,

        // Si no existe el archivo regresamos vector vacío
        Err(ref err) if err.kind() == io::ErrorKind::NotFound => {
            return Ok(contactos);
        }

        Err(err) => return Err(err),
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {

        let line = line?;

        let parts: Vec<&str> = line.split(',').collect();

        if parts.len() == 4 {

            // Convertir string -> Fecha
            if let Ok(fecha) = Fecha::from_string(parts[2].trim()) {

                let contacto = Contacto::new(
                    parts[0].trim().to_string(),
                    parts[1].trim().to_string(),
                    parts[3].trim().to_string(),
                    fecha
                );

                contactos.push(contacto);
            }
        }
    }

    Ok(contactos)
}

pub fn guardar_contacto(contacto: Contacto, contactos: &mut Vec<Contacto>) -> io::Result<()> {

    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("contactos.txt")?;

    // Validaciones
    if validar_correo(contacto.correo()).is_ok()
        && validar_telefono(contacto.telefono()).is_ok()
    {

        // Guardamos en el vector
        contactos.push(contacto.clone());

        // Guardamos en archivo
        writeln!(
            file,
            "{},{},{},{}",
            contacto.nombre(),
            contacto.telefono(),
            contacto.fecha_nacimiento().to_string(),
            contacto.correo()
        )?;

        Ok(())

    } else {

        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Datos incorrectos"
        ))
    }
}

pub fn consultar_contacto(nombre: &str, contactos: &Vec<Contacto>) {

    match contactos.iter().find(|c| c.nombre() == nombre) {

        Some(c) => {

            println!("Nombre: {}", c.nombre());
            println!("Telefono: {}", c.telefono());
            println!(
                "Fecha de nacimiento: {}",
                c.fecha_nacimiento().to_string()
            );
            println!("Correo: {}", c.correo());
        }

        None => {
            println!("Contacto no encontrado");
        }
    }
}

pub fn comparacion_por_nombre(contacto: &Contacto, otro_contacto: &Contacto) -> Ordering {
    contacto.nombre().cmp(&otro_contacto.nombre())
}

pub fn comparacion_por_fecha(contacto: &Contacto, otro_contacto: &Contacto) -> Ordering {
    let hoy = Fecha::obtener_fecha_actual();
    let diferencia_c1 = hoy.calcular_indice_diferencia(&contacto.fecha_nacimiento());
    let diferencia_c2 = hoy.calcular_indice_diferencia(&otro_contacto.fecha_nacimiento());
    diferencia_c1.cmp(&diferencia_c2)
}

pub fn imprimir_por_nombre(contactos : &Vec<Contacto>) -> io::Result<()> {
    let mut arbol_nombre = Arbol::<Contacto>::nuevo();

    // Recorrer todos los contactos
    for contacto in contactos {
        // Alimentar árbol
        arbol_nombre.insertar_ordenado(contacto.clone(), comparacion_por_nombre);
    }
    arbol_nombre.in_orden();

    Ok(())
}

pub fn imprimir_por_fecha(contactos : &Vec<Contacto>) -> io::Result<()> {
    let mut arbol_fechas = Arbol::<Contacto>::nuevo();

    // Recorrer todos los contactos
    for contacto in contactos {
        // Alimentar árbol
        arbol_fechas.insertar_ordenado(contacto.clone(), comparacion_por_fecha);
    }
    arbol_fechas.in_orden();

    Ok(())
}


