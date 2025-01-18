use crate::vec3::Vec3;

pub fn linear_to_gamma(linear: f64) -> f64 {
    if linear > 0. {
        return linear.sqrt();
    }
    return 0.;
}
impl Vec3 {
    pub fn as_rgb_bytes(mut self) -> [u8; 3] {
        self.x = linear_to_gamma(self.x);
        self.y = linear_to_gamma(self.y);
        self.z = linear_to_gamma(self.z);

        let intensity = (0.0, 0.999);
        //convert colors from 0-1 f64 range to 8 bit integer (0-255)
        let rbyte = (self.x.clamp(intensity.0, intensity.1) * 255.0) as u8;
        let gbyte = (self.y.clamp(intensity.0, intensity.1) * 255.) as u8;
        let bbyte = (self.z.clamp(intensity.0, intensity.1) * 255.) as u8;

        [rbyte, gbyte, bbyte]
    }
}

pub type Color = Vec3;
