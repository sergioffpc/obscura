use nalgebra::{Matrix4, Perspective3, UnitQuaternion, Vector3};

pub struct Camera {
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Camera {
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

#[derive(Default)]
pub struct CameraRig {
    pub position: Vector3<f32>,
    pub orientation: UnitQuaternion<f32>,
}

impl CameraRig {
    pub fn as_matrix(&self) -> Matrix4<f32> {
        let translation = Matrix4::new_translation(&-self.position);
        let rotation = self.orientation.to_rotation_matrix().to_homogeneous();

        rotation * translation
    }
}
