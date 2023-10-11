use nalgebra::{Matrix4, Perspective3, Point3, Rotation3, Translation3, Vector3};

pub struct Projection {
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Projection {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            aspect: width as f32 / height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        }
    }

    pub fn as_matrix(&self) -> Matrix4<f32> {
        Perspective3::new(self.aspect, self.fovy, self.znear, self.zfar)
            .as_matrix()
            .to_owned()
    }
}

#[derive(Debug, Default)]
pub struct View {
    pub position: Point3<f32>,
    pub yaw: f32,
    pub pitch: f32,
}

impl View {
    pub fn as_matrix(&self) -> Matrix4<f32> {
        let translation = Translation3::from(-self.position);
        let rotation = Rotation3::from_axis_angle(&Vector3::x_axis(), self.pitch)
            * Rotation3::from_axis_angle(&Vector3::y_axis(), self.yaw);

        rotation.to_homogeneous() * translation.to_homogeneous()
    }
}
