use std::format;
pub struct Fecha {
    dia: i32,
    mes: i32,
    anio: i32,
}

impl Fecha {
    // Constructor
    pub fn new(dia: i32, mes: i32, anio: i32) -> Result<Self, String> {
        if !fecha_valida(dia, mes, anio) {
            Err(String::from("Fecha inválida"))
        } else {
            Ok(Self { dia, mes, anio })
        }
    }

    pub fn cumple_despues_de(&self, otra_fecha: &Fecha) -> bool {
        if self.mes < otra_fecha.mes{
            false
        }else{
            if self.mes > otra_fecha.mes{
                true
            }else{
                self.dia > otra_fecha.dia
            }
        }
    }
    pub fn obtener_fecha_actual() -> Self {
        let ahora = std::time::SystemTime::now();
        let segundos: u64 = ahora.duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - (6 * 3600);

        let segundos_restantes: u64 = segundos;
        let dias: u64 = segundos / 86400;

        let mut dias_restantes = dias;
        let mut a: i32 = 1970;

        while dias_restantes >= 365 {
            let dias_en_anio = if es_bisiesto(a) { 366 } else { 365 };
            if dias_restantes >= dias_en_anio {
                dias_restantes -= dias_en_anio;
                a += 1;
            } else {
                break;
            }
        }

        let mut mes_actual = 1;
        let mut dias_restantes_temp = dias_restantes;

        loop {
            let dias_en_mes: u64 = dias_en_mes(mes_actual,a) as u64;

            if dias_restantes_temp < dias_en_mes {
                break;
            }

            dias_restantes_temp -= dias_en_mes;
            mes_actual += 1;
        }

        //let segundos_en_dia = segundos_restantes % 86400;
        //let _hora_actual = segundos_en_dia / 3600;
        //let _minutos_actual = (segundos_en_dia % 3600) / 60;
        //let _segundos_actual = segundos_en_dia % 60;
        let dia_actual: i32 = (dias_restantes_temp + 1) as i32;

        Self { dia: dia_actual, mes: mes_actual, anio: a }
    }

    pub fn to_string(&self) -> String {
        format!("{:02}/{:02}/{}", self.dia, self.mes, self.anio)
    }


    pub fn from_string(string: &str) -> Result<Self, String> {
        // CORREGIDO: string ya es &str, no necesita &
        if !formato_fecha_valido(string) {
            return Err("La fecha debe tener formato DD/MM/YYYY".to_string());
        }

        let dia_str = &string[0..2];
        let mes_str = &string[3..5];
        let anio_str = &string[6..10];

        let dia = dia_str.parse::<i32>().map_err(|_| "Día inválido".to_string())?;
        let mes = mes_str.parse::<i32>().map_err(|_| "Mes inválido".to_string())?;
        let anio = anio_str.parse::<i32>().map_err(|_| "Año inválido".to_string())?;

        if !(1..=31).contains(&dia) {
            return Err("Día fuera de rango (1-31)".to_string());
        }
        if !(1..=12).contains(&mes) {
            return Err("Mes fuera de rango (1-12)".to_string());
        }

        Ok(Self { dia, mes, anio })
    }

    // Getters
    pub fn dia(&self) -> i32 {
        self.dia
    }

    pub fn mes(&self) -> i32 {
        self.mes
    }

    pub fn anio(&self) -> i32 {
        self.anio
    }
}

// Funciones fuera del bloque impl
pub fn es_bisiesto(a: i32) -> bool {
    (a % 4 == 0 && a % 100 != 0) || (a % 400 == 0)
}

pub fn formato_fecha_valido(string: &str) -> bool {
    if string.len() != 10{
        return false;
    }
    let mut indice_diagonal:i32 = 2;

    for (indice, letra ) in string.chars().enumerate(){
        if indice != (indice_diagonal.try_into().unwrap()){
            if !letra.is_numeric(){
                return false
            }
        }else{
            if letra != '/'{
                return false
            }
            if indice_diagonal == 2{
                indice_diagonal += 3;
            }
        }
    }
    true
}
fn dias_en_mes(mes: i32, anio: i32)-> i32{
    let dias_en_mes = if mes == 2 {
        if es_bisiesto(anio) { 29 } else { 28 }
    } else {
        if mes == 4 || mes == 6 || mes == 9 || mes == 11 {
            30
        } else {
            31
        }
    };
    dias_en_mes
}
fn fecha_valida(dia: i32, mes: i32, anio: i32) -> bool {
    if (1..=12).contains(&mes) {
        let dias_en_mes = dias_en_mes(mes, anio);
        (1..=dias_en_mes).contains(&dia)
    }else{
        false
    }
}