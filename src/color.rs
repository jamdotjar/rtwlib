//! This module contans all functions and structs related to colors and color manipulation.
//! This includes the `Color` struct, an alias for `Vec3`, and functions to convert colors to different formats, as well as color manipulation functions such as gamma correction.
use crate::vec3::Vec3;

///Converts a linear color value to a gamma corrected value.
pub fn linear_to_gamma(linear: f64) -> f64 {
    if linear > 0. {
        return linear.sqrt();
    }
    return 0.;
}

///Converts a gamma corrected color value to a linear value.
/// This is the inverse of `linear_to_gamma`.
pub fn gamma_to_linear(gamma: f64) -> f64 {
    return gamma * gamma;
}
///Converts a linear color to a gamma corrected color.
pub fn linear_color_to_gamma(color: Vec3) -> Vec3 {
    Vec3 {
        x: linear_to_gamma(color.x),
        y: linear_to_gamma(color.y),
        z: linear_to_gamma(color.z),
    }
}
///Converts a gamma corrected color to a linear color.
pub fn gamma_color_to_linear(color: Vec3) -> Vec3 {
    Vec3 {
        x: gamma_to_linear(color.x),
        y: gamma_to_linear(color.y),
        z: gamma_to_linear(color.z),
    }
}

impl Color {
    ///Converts a color to a byte array containing the RGB values of the color.
    pub fn to_rgb_bytes(mut self) -> [u8; 3] {
        self = gamma_color_to_linear(self);

        let intensity = (0.0, 0.999);
        //convert colors from 0-1 f64 range to 8 bit integer (0-255)
        let rbyte = (self.x.clamp(intensity.0, intensity.1) * 255.0) as u8;
        let gbyte = (self.y.clamp(intensity.0, intensity.1) * 255.) as u8;
        let bbyte = (self.z.clamp(intensity.0, intensity.1) * 255.) as u8;

        [rbyte, gbyte, bbyte]
    }
    ///Converts a color to a hexadecimal string, starting with a `#`.
    pub fn to_hex(&self) -> String {
        let bytes = self.to_rgb_bytes();
        format!("#{:02x}{:02x}{:02x}", bytes[0], bytes[1], bytes[2])
    }
}

pub type Color = Vec3;
