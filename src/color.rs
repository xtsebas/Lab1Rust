use std::ops::{Add, Mul}; // Importamos los traits Add y Mul del módulo std::ops
use std::fmt; // Importamos el módulo std::fmt

#[derive(Debug, Copy, Clone)] // Derivamos las traits Debug, Copy y Clone para la estructura Color
pub struct Color { // Definimos la estructura Color
    pub r: u8, // Campo público r de tipo u8 (8 bits sin signo)
    pub g: u8, // Campo público g de tipo u8
    pub b: u8, // Campo público b de tipo u8
}

impl Color { // Implementamos métodos para la estructura Color
    pub fn new(r: i32, g: i32, b: i32) -> Self { // Definimos el método new que crea una nueva instancia de Color
        Color { // Inicializamos la estructura Color con los valores proporcionados
            r: r.clamp(0, 255) as u8, // Limitamos el valor de r entre 0 y 255 y lo convertimos a u8
            g: g.clamp(0, 255) as u8, // Limitamos el valor de g entre 0 y 255 y lo convertimos a u8
            b: b.clamp(0, 255) as u8, // Limitamos el valor de b entre 0 y 255 y lo convertimos a u8
        }
    }

    pub fn from_hex(hex: u32) -> Self { // Definimos el método from_hex que crea una instancia de Color a partir de un valor hexadecimal
        let r = ((hex >> 16) & 0xFF) as u8; // Extraemos el componente rojo del valor hexadecimal
        let g = ((hex >> 8) & 0xFF) as u8; // Extraemos el componente verde del valor hexadecimal
        let b = (hex & 0xFF) as u8; // Extraemos el componente azul del valor hexadecimal
        Color {r, g, b} // Devolvemos una nueva instancia de Color con los componentes extraídos
    }

    pub fn to_hex(&self) -> u32 { // Definimos el método to_hex que convierte la instancia de Color a un valor hexadecimal
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32) // Combinamos los componentes en un solo valor hexadecimal
    }
}

impl Add for Color { // Implementamos el trait Add para la estructura Color
    type Output = Self; // Definimos el tipo de salida como la misma estructura Color

    fn add(self, other: Self) -> Self::Output { // Definimos la operación de suma para Color
        Color { // Creamos una nueva instancia de Color con los componentes sumados
            r: (self.r as u32 + other.r as u32).min(255) as u8, // Sumamos los componentes rojos y los limitamos a 255
            g: (self.g as u32 + other.g as u32).min(255) as u8, // Sumamos los componentes verdes y los limitamos a 255
            b: (self.b as u32 + other.b as u32).min(255) as u8, // Sumamos los componentes azules y los limitamos a 255
        }
    }
}

impl Mul<f32> for Color { // Implementamos el trait Mul para la estructura Color con un escalar de tipo f32
    type Output = Self; // Definimos el tipo de salida como la misma estructura Color

    fn mul(self, scalar: f32) -> Self::Output { // Definimos la operación de multiplicación por escalar para Color
        Color { // Creamos una nueva instancia de Color con los componentes multiplicados por el escalar
            r: (self.r as f32 * scalar).clamp(0.0, 255.0) as u8, // Multiplicamos el componente rojo por el escalar y lo limitamos a 255
            g: (self.g as f32 * scalar).clamp(0.0, 255.0) as u8, // Multiplicamos el componente verde por el escalar y lo limitamos a 255
            b: (self.b as f32 * scalar).clamp(0.0, 255.0) as u8, // Multiplicamos el componente azul por el escalar y lo limitamos a 255
        }
    }
}

impl fmt::Display for Color { // Implementamos el trait Display para la estructura Color
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { // Definimos cómo se formatea la estructura al mostrarla
        write!(f, "Color (R: {}, G: {}, B: {})", self.r, self.g, self.b) // Escribimos el formato deseado en el formatter proporcionado
    }
}

// Módulo de pruebas
#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn test_new() {
        let color = Color::new(255, 128, 64);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
    }

    #[test]
    fn test_new_clamping() {
        let color = Color::new(300, -50, 128);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 128);
    }

    #[test]
    fn test_from_hex() {
        let color = Color::from_hex(0xFF5733);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 87);
        assert_eq!(color.b, 51);
    }

    #[test]
    fn test_to_hex() {
        let color = Color::new(255, 87, 51);
        assert_eq!(color.to_hex(), 0xFF5733);
    }

    #[test]
    fn test_add() {
        let color1 = Color::new(100, 150, 200);
        let color2 = Color::new(50, 60, 70);
        let summed_color = color1 + color2;
        assert_eq!(summed_color.r, 150);
        assert_eq!(summed_color.g, 210);
        assert_eq!(summed_color.b, 255); // Ensuring clamping
    }

    #[test]
    fn test_mul() {
        let color = Color::new(100, 150, 200);
        let scaled_color = color * 0.5;
        assert_eq!(scaled_color.r, 50);
        assert_eq!(scaled_color.g, 75);
        assert_eq!(scaled_color.b, 100);
    }

    #[test]
    fn test_display() {
        let color = Color::new(100, 150, 200);
        assert_eq!(format!("{}", color), "Color (R: 100, G: 150, B: 200)");
    }
}