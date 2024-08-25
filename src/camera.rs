use crate::{color::*, hittable::*, ray::*, vec3::*};
use rand::{thread_rng, Rng};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples: u32,
    pub bounces: u32,
    image_height: u32,
    sample_scale: f64,
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
            samples: 100,
            bounces: 10,
            image_height: 400,
            sample_scale: 1.0,
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
          eprint!("\r {}/{} lines rendered", j + 1, self.image_height);
             for i in 0..=self.image_width - 1 {
                let mut pixel_color = Color::from(0.0);

                for _ in 0..self.samples {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(r,self.bounces, &world);
                }
                write_color( pixel_color * self.sample_scale);
            }
        }
    }

    fn initialize(&mut self) {
        //image size
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u32;

        self.sample_scale = 1.0/self.samples as f64;
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

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = sample_square();

        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x) * self.pixel_delta_u)
            + ((j as f64 + offset.y) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn ray_color(&self, r: Ray,bounces: u32, world: &HittableList) -> Color {
        if bounces == 0 {
            return Color::from(0.);
        }

        let mut rec: HitRecord = Default::default();

        if world.hit(&r, 0.001..f64::INFINITY, &mut rec) {
           let direction = rec.normal+ Vec3::random_normalized();
            return 0.5 * self.ray_color(Ray::new(rec.p, direction), bounces-1, world);       }

        let unit_direction = r.direction().normalized();
        let a = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - a) * Color::new(1., 1., 1.) + a * Color::new(0.5, 0.7, 1.0);
    }
}

fn sample_square() -> Vec3 {
    let mut rng = thread_rng();
    Vec3::new(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), 0.)
}
