//! This crate contains the `Camera` struct, and associated functions.
//! The functions are here to provide a simple way to render a scene, but feel free to implement your own methods on the `Camera` struct to render to different outputs.
//!
//! # Example
//! To render a scene, you can simply use one of the premade renderers, like `render_to_bytes` or `render_to_string`, these have a callback function that is called every line, which can be used to show progress.
//! ```
//! use rtwlib::{camera::Camera, hittable::*, material::*, vec3::*};
//!
//! camera.render_to_bytes(world, |progress| println!("Progress: {}%", progress));
//! ```
//!
//! If you want more control over your renders, the `get_ray` and `ray_color` functions are the backbone of the rendering process, and can be used to create your own rendering functions.
//! When rendering manually ( without a premade function ) the generall process is as follows:
//!     1. Initalize the camera for rendering using `initialize`.
//!     2. Create a `Ray` using `get_ray` with the pixel coordinates.
//!     3. Trace the ray using `ray_color` to get the color of the pixel.
//! From here, you can do whatever you want with the color, save it to a buffer, write it to a file, or even display it immediately on screen.
//!
//! Cameras can have
use crate::{color::*, hittable::*, ray::*, vec3::*};
use rand::{thread_rng, Rng};

/// #Camera
///
/// Represents the Camera rendering the scene. It contains a position, target, up vector and all other rendering settings.
/// Implements the `Default` trait, so you can create a new camera with `Camera::new()`.
///
/// The public feilds can be modified directly, but you should call `initialize` after changing any settings to ensure the camera is properly set up. the pre-made render functions will call `initialize` automatically.
///
/// # Fields
/// Public:
/// * `aspect_ratio` - The aspect ratio of the camera ( width / height ), this does not need to be set manually, and is calculated from the image width and height. If you do change it, your render will have black bars.
/// * `image_width` - The width of the image in pixels
/// * `image_height` - The height of the image in pixels.
/// * `samples` - The number of rays to be traced per pixel, higher values will result in a cleaner image, but will take longer to render.
/// * `bounces` - The maximum number of times a ray can bounce before being terminated, higher values will result in more complex lighting, but will take longer to render.
/// * `vfov` - The field of view of the camera.
/// * `lookfrom` - The location of the camera.
/// * `lookat` - The point the camera is looking at, used to calculate rotation.
/// * `vup` - The up vector of the camera, generally `(0, 1, 0)` for a camera with Y as up.
/// * `defocus_angle` - The angle of the defocus disk, used to set blur strength. 0.0 disables distance blur effect.
/// * `focus_dist` - The distance from the camera to the focus plane.
/// * `sky` - The sky object, used to render the background of the scene.
///
/// Private: (used for internal rendering calculations)
/// * `sample_scale` - The scale of the samples, calculated as `1.0 / samples as f64`.
/// * `center` - The center of the camera.
/// * `pixel00_loc` - The location of the top left pixel.
/// * `pixel_delta_u` - The vector between pixels on the U axis.
/// * `pixel_delta_v` - The vector between pixels on the V axis.
/// * `u` - U axis.
/// * `v` - V axis.
/// * `w` - W axis.
/// * `defocus_disc_u` - The U axis of the defocus disk.
/// * `defocus_disc_v` - The V axis of the defocus disk.
pub struct Camera {
    /// The aspect ratio of the camera ( width / height )
    pub aspect_ratio: f64,
    /// The width of the image in pixels
    pub image_width: u32,
    /// The height of the image in pixels
    pub image_height: u32,
    /// The number of rays to be traced per pixel
    pub samples: u32,
    /// The maximum number of times a ray can bounce before being terminated
    pub bounces: u32,
    /// The feild of view of the camera
    pub vfov: f64,
    /// The location of the camera
    pub lookfrom: Point3,
    /// The point the camera is looking at, used to calculate rotation.
    pub lookat: Point3,
    /// The up vector of the camera, generally `(0, 1, 0)` for a camera with Y as up
    pub vup: Vec3,
    /// The angle of the defocus disk, 0.0 disables distance blur effect.
    pub defocus_angle: f64,
    /// The distance from the camera to the focus plane
    pub focus_dist: f64,

    sample_scale: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disc_u: Vec3,
    defocus_disc_v: Vec3,
    sky: Box<dyn Sky>,
}

impl Camera {
    /// Creates and initalizes a new default camera
    pub fn new() -> Camera {
        let mut cam = Camera::default();
        cam.initialize();
        cam
    }
    /// Renders the scene to a string, runs progress callback every line
    /// * `world` - The HittableList representing the scene
    /// * `progress` - A callback function with u32 parameter.
    ///
    /// Returns a `String` containing the rgb values for each pixel, each value seperated by a space, and each pixel seperated by a newline.
    ///
    /// `R G B\n
    /// R G B\n
    /// ...`

    pub fn render_to_string<F>(&mut self, world: HittableList, mut progress: F) -> String
    where
        F: FnMut(u32),
    {
        self.initialize();
        let mut buffer =
            String::with_capacity((self.image_width * self.image_height * 12) as usize);

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_color = Color::from(0.0);

                for _ in 0..self.samples {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(r, self.bounces, &world);
                }
                let rgb = (pixel_color * self.sample_scale).to_rgb_bytes();
                buffer.push_str(&format!("{} {} {}\n", rgb[0], rgb[1], rgb[2]));
            }
            progress(j);
        }
        buffer
    }

    /// Renders the scene to a buffer of bytes, runs progress callback every line
    /// * `world` - The HittableList representing the scene
    /// * `progress` - A callback function with u32 parameter.
    ///
    /// Returns a `Vec<u8>`, where every three bytes represent the RGB values of a pixel.
    /// `[R, G, B, R, G, B, ...]`
    pub fn render_to_bytes<F>(&mut self, world: HittableList, mut progress: F) -> Vec<u8>
    where
        F: FnMut(u32),
    {
        self.initialize();
        let mut buffer = Vec::new();

        for j in 0..=self.image_height - 1 {
            for i in 0..=self.image_width - 1 {
                let mut pixel_color = Color::from(0.0);

                for _ in 0..self.samples {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(r, self.bounces, &world);
                }
                buffer.extend_from_slice(&(pixel_color * self.sample_scale).to_rgb_bytes());
            }
            progress(j);
        }
        buffer
    }
    /// Initalizes camera settings based on current properties.
    /// This should be run any time the resolution, location, lookat, sample count, focus amount or focus distance is changed.
    pub fn initialize(&mut self) {
        //image size
        self.aspect_ratio = self.image_width as f64 / self.image_height as f64;
        self.sample_scale = 1.0 / self.samples as f64;
        self.center = self.lookfrom;

        //camera and viewport
        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        self.w = (self.lookfrom - self.lookat).normalized();
        self.u = cross(&self.vup, &self.w).normalized();
        self.v = cross(&self.w, &self.u);
        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;
        //Creates the vectors across the edge of the viewport
        //calculates the delta vectors, for movement between pixels.
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left =
            self.center - (self.focus_dist * self.w) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
        //scales defocus disk properly
        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.0).to_radians().tan();
        self.defocus_disc_u = self.u * defocus_radius;
        self.defocus_disc_v = self.v * defocus_radius;
    }

    /// Creates an initial ray targeting pixel (i, j)
    /// The rays are slightly jittered, to acheive proper multi-sample averages.
    /// * `i`, `j` - The target pixel
    pub fn get_ray(&self, i: u32, j: u32) -> Ray {
        //creates rays from defocus disk pointing at a random point in pixel i, j
        let offset = sample_square();

        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x) * self.pixel_delta_u)
            + ((j as f64 + offset.y) * self.pixel_delta_v);

        let ray_origin = defocus_disk_sample(&self);
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }
    /// Returns the distance from the camera to a target point.
    pub fn get_distance(&self, target: Point3) -> f64 {
        (target - self.lookfrom).length()
    }
    /// Return's the canera's sample scale, Should generally be used as opposed to acessing the value directly, as it should not be modified mid-trace.
    pub fn get_sample_scale(&self) -> f64 {
        self.sample_scale
    }

    /// Traces a ray through the scene, and outputs a final color.
    /// * `r` - The `Ray` to be traced.
    /// * `bounces` - The maximum depth of the trace.
    /// * `world` - A HittableList of objects, representing the scene.
    pub fn ray_color(&self, r: Ray, bounces: u32, world: &HittableList) -> Color {
        //actually traces the
        //ray
        if bounces == 0 {
            return Color::from(0.);
        }

        let mut rec: HitRecord = Default::default();

        if world.hit(&r, 0.001..f64::INFINITY, &mut rec) {
            let mut scattered = Ray::new(Vec3::from(0.), Vec3::from(0.));
            let mut attenuation = Color::from(1.);

            if rec.mat.scatter(&r, &rec, &mut attenuation, &mut scattered) {
                //does bounce/scattter for materials of hit object
                return attenuation * self.ray_color(scattered, bounces - 1, world);
            }

            return Color::new(0., 0., 0.); // Show up around the edge of metals
        }

        // if the ray hits nothing, calculates a sky color

        return self.sky.color(r);
    }
    /// Returns the height of the camera's image
    pub fn get_height(&self) -> u32 {
        self.image_height
    }
}
impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 600,
            samples: 100,
            bounces: 10,
            image_height: 600,
            sample_scale: 1.0,
            vfov: 90.0,
            lookfrom: Point3::from(0.),
            lookat: Point3::new(0., 0., -1.),
            vup: Vec3::new(0., 1., 0.),
            defocus_angle: 0.,
            focus_dist: 10.,
            center: Vec3::from(0.0),
            pixel00_loc: Vec3::from(0.0),
            pixel_delta_u: Vec3::from(0.0),
            pixel_delta_v: Vec3::from(0.0),
            u: Vec3::from(0.0),
            v: Vec3::from(0.0),
            w: Vec3::from(0.0),
            defocus_disc_u: Vec3::from(0.0),
            defocus_disc_v: Vec3::from(0.0),
            sky: Box::new(GradientSky {
                start: Color::new(0.5, 0.7, 1.0),
                end: Color::new(1.0, 1.0, 1.0),
            }),
        }
    }
}

/// Creates a vector with the X and Y origins set as a random point between 0 and 1
fn sample_square() -> Vec3 {
    let mut rng = thread_rng();
    Vec3::new(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), 0.)
}
/// Creates a point somehwere in the area of the camera's virtual "lens", used to simulate DOF
fn defocus_disk_sample(cam: &Camera) -> Point3 {
    let p = Vec3::random_in_unit_disk();
    cam.center + (p.x * cam.defocus_disc_u) + (p.y * cam.defocus_disc_v)
}

/// Any object that implements the `Sky` trait can be used as a sky for the camera.
/// For a simple sky, you can use the [`Color`] struct which renders a solid color, or the [`GradientSky`] struct which renders a gradient between two colors.
/// You can also implement your own sky by implementing the `Sky` trait for your struct.
pub trait Sky {
    /// Returns the color of the sky for a given ray
    fn color(&self, ray: Ray) -> Vec3;
}
/// A simple gradient sky, that fades from one color to another based on the Y value of the ray direction
pub struct GradientSky {
    /// The color at the top of the sky
    pub start: Color,
    /// The color at the bottom of the sky
    pub end: Color,
}

impl Sky for Color {
    fn color(&self, _direction: Ray) -> Vec3 {
        *self
    }
}
impl Sky for GradientSky {
    fn color(&self, ray: Ray) -> Vec3 {
        let t = 0.5 * (ray.direction.normalized().y + 1.0);
        self.start * (1.0 - t) + self.end * t
    }
}
