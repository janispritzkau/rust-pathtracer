use math::*;
use ray::Ray;
use rand::Rng;
use rand_xorshift::XorShiftRng;

pub struct Camera {
    camera_to_world: Matrix4<f32>, raster_to_camera: Matrix4<f32>,
    focal_distance: f32, aperture_size: f32, scale: f32
}

impl Camera {
    pub fn new(camera_to_world: Matrix4<f32>, width: usize, height: usize) -> Self {
        let aspect_ratio = width as f32 / height as f32;
        let raster_to_camera
        = Matrix4::from_nonuniform_scale(1.0, -1.0 / aspect_ratio, 1.0)
        * Matrix4::from_translation(Vector3::new(-1.0, -1.0, 0.0))
        * Matrix4::from_nonuniform_scale(2.0 / width as f32, 2.0 / height as f32, 1.0);

        Self {
            camera_to_world, raster_to_camera,
            scale: 0.75, focal_distance: 0.0, aperture_size: 0.0
        }
    }

    pub fn fov(mut self, fov_deg: f32) -> Self {
        self.scale = (fov_deg.to_radians() / 2.0).tan(); self
    }
    pub fn dof(mut self, distance: f32, size: f32) -> Self {
        self.aperture_size = size; self.focal_distance = distance; self
    }

    pub fn generate_ray(&self, raster_x: f32, raster_y: f32, rng: &mut XorShiftRng) -> Ray {
        let camera_dir = self.raster_to_camera.transform_point(
            Point3::new(raster_x, raster_y, -1.0)
        ).to_vec().mul_element_wise(Vector3::new(self.scale, self.scale, 1.0)).normalize();

        let camera_pos = Point3::new(
            (rng.gen::<f32>() - 0.5) * 2.0 * self.aperture_size,
            (rng.gen::<f32>() - 0.5) * 2.0 * self.aperture_size,
            0.0
        );
        let camera_dir = (camera_dir * self.focal_distance - camera_pos.to_vec()).normalize();

        Ray {
            origin: self.camera_to_world.transform_point(camera_pos),
            direction: self.camera_to_world.transform_vector(camera_dir)
        }
    }
}
