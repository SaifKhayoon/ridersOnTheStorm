use nalgebra::Vector3;

pub struct CameraControl {
  position: Vector3<f32>,
  target: Vector3<f32>,
  up: Vector3<f32>,
  pitch: f32,
  yaw: f32,
  roll: f32,
  fov: f32,
}

impl CameraControl {
  pub fn new() -> Self {
    Self {
      position: Vector3::new(0.0, 0.0, 0.0),
      target: Vector3::new(0.0, 0.0, 0.0),
      up: Vector3::new(0.0, 1.0, 0.0),
      pitch: 0.0,
      yaw: 0.0,
      roll: 0.0,
      fov: 90.0,
    }
  }

  pub fn set_position(&mut self, position: Vector3<f32>) {
    self.position = position;
  }

  pub fn set_target(&mut self, target: Vector3<f32>) {
    self.target = target;
  }

  pub fn set_up(&mut self, up: Vector3<f32>) {
    self.up = up;
  }

  pub fn set_pitch(&mut self, pitch: f32) {
    self.pitch = pitch;
  }

  pub fn set_yaw(&mut self, yaw: f32) {
    self.yaw = yaw;
  }

  pub fn set_roll(&mut self, roll: f32) {
    self.roll = roll;
  }

  pub fn set_fov(&mut self, fov: f32) {
    self.fov = fov;
  }

  
    pub fn look_at(&mut self) {
      // Set the camera's position, target, and up vectors based on the pitch, yaw, and roll angles
      self.position.x = self.target.x + self.pitch.cos() * self.yaw.sin();
      self.position.y = self.target.y + self.pitch.sin();
      self.position.z = self.target.z + self.pitch.cos() * self.yaw.cos();
      self.up = Vector3::new(self.pitch.cos() * self.yaw.cos(), self.pitch.sin(), self.pitch.cos() * self.yaw.sin()).cross(&Vector3::new(-self.yaw.sin(), 0.0, self.yaw.cos()));
    }
  
    pub fn projection_matrix(&self) -> Matrix4<f32> {
      // Return the projection matrix for the camera
      let aspect_ratio = 800.0 / 600.0;
      let fov = self.fov.to_radians();
      let z_near = 0.1;
      let z_far = 100.0;
      let f = 1.0 / (fov / 2.0).tan();
      Matrix4::new(
        f / aspect_ratio, 0.0, 0.0, 0.0,
        0.0, f, 0.0, 0.0,
        0.0, 0.0, (z_far + z_near) / (z_near - z_far), (2.0 * z_far * z_near) / (z_near - z_far),
        0.0, 0.0, -1.0, 0.0
      )
    }
  
    pub fn view_matrix(&self) -> Matrix4<f32> {
      // Return the view matrix for the camera
      Matrix4::look_at_rh(&self.position, &self.target, &self.up)
    }
  }
  

