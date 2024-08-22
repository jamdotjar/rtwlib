use crate::{color::*, hittable::*, ray::*, sphere::*, vec3::*};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    image_height: u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            aspect_ratio: 1.0,
            image_width: 400,
            image_height: 400,
            center: Vec3::from(0.0),
            pixel00_loc: Vec3::from(0.0),
            pixel_delta_u: Vec3::from(0.0),
            pixel_delta_v: Vec3::from(0.0),
        }
    }
    pub fn render(&mut self, world: HittableList) {
        self.initialize();

        println!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for j in 0..=self.image_height - 1 {
            for i in 0..=self.image_width - 1 {
                let pixel_center = self.pixel00_loc
                    + (i as f64 * self.pixel_delta_u)
                    + (j as f64 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let r: Ray = Ray {
                    origin: self.center,
                    direction: ray_direction,
                };

                let pixel_color: Color = self.ray_color(r, &world);
                write_color(pixel_color);
            }
            eprint!("\r {}/{} lines rendered", j + 1, self.image_height)
        }
    }

    fn initialize(&mut self) {
        //image size
               self.image_height = (self.image_width as f64 / self.aspect_ratio) as u32;

        //camera and viewport
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);
        let camera_center = Point3::new(0.0, 0.0, 0.0);

        //Creates the vectors across the edge of the viewport
        let viewport_u = vec3!(viewport_width, 0, 0);
        let viewport_v = vec3!(0, -viewport_height, 0);

        //calculates the delta vectors, for movement between pixels.
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left =
            camera_center - vec3!(0, 0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn ray_color(&self, r: Ray, world: &HittableList) -> Color {
        let mut rec: HitRecord = Default::default();

        if world.hit(&r, 0.0..f64::INFINITY, &mut rec) {
            return 0.5 * (rec.normal + Color::new(1., 1., 1.));
        }

        let unit_direction = r.direction().normalized();
        let a = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - a) * Color::new(1., 1., 1.) + a * Color::new(0.5, 0.7, 1.0);
    }
}
