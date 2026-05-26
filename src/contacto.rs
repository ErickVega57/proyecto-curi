use crate::fecha::Fecha;

pub struct Contacto{
    nombre : String,
    telefono : String,
    correo : String,
    fecha_cumple: Fecha,
}

impl Contacto {
    pub fn new(nombre: &str, telefono: &str, correo: &str, fecha_cumple: Fecha) -> Self {
        let nombre = nombre.to_string();
        let telefono = telefono.to_string();
        let correo = correo.to_string();

        Self{nombre, telefono, correo, fecha_cumple}
    }

    // Getters
    pub fn nombre(&self) -> &str {
        &self.nombre
    }

    pub fn telefono(&self) -> &str {
        &self.telefono
    }

    pub fn correo(&self) -> &str {
        &self.correo
    }

    pub fn fecha_cumple(&self) -> &Fecha {
        &self.fecha_cumple
    }
}
// Métodos fuera del bloque impl
pub fn validar_correo(correo: &str) -> Result<(), String> {
    if correo.contains("@") && correo.contains("."){
        Ok(())
    } else {
        Err(String::from("Correo no válido"))
    }
}

pub fn validar_telefono(telefono: &str) -> Result<(), String> {
    if telefono.chars().all(|c| c.is_digit(10)) && telefono.len() == 10 {
        Ok(())
    } else{
        Err(String::from("Teléfono no válido"))
    }
}