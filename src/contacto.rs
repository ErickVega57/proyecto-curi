use std::fmt;
use crate::fecha::Fecha;


#[derive(Clone)]
pub struct Contacto{
    nombre: String,
    telefono: String,
    correo: String,
    fecha_nacimiento: Fecha,
}

impl fmt::Debug for Contacto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Formato para imprimir contacto
        write!(
            f,
            "Nombre: {} \n\
            Teléfono: {} \n\
            Correo: {} \n\
            Fecha: {:?} \n",
            self.nombre, self.telefono, self.correo, self.fecha_nacimiento.to_string()
        )
    }
}

impl Contacto {
    pub fn new(nombre: String, telefono: String, correo: String, fecha_nacimiento: Fecha) -> Self {
        Self { nombre, telefono, correo, fecha_nacimiento }
    }
    pub fn nombre(&self) -> &str {
        &self.nombre
    }

    pub fn telefono(&self) -> &str {
        &self.telefono
    }

    pub fn correo(&self) -> &str {
        &self.correo
    }

    pub fn fecha_nacimiento(&self) -> &Fecha {
        &self.fecha_nacimiento
    }

    pub fn set_nombre(&mut self, nombre: String) {
        self.nombre = nombre;
    }

    pub fn set_telefono(&mut self, telefono: String) {
        self.telefono = telefono;
    }

    pub fn set_correo(&mut self, correo: String) {
        self.correo = correo;
    }

    pub fn set_fecha_nacimiento(&mut self, fecha_nacimiento: Fecha) {
        self.fecha_nacimiento = fecha_nacimiento;
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