use nalgebra_glm::Vec3;

#[derive(Clone, Copy, PartialEq)]
pub enum WarpState {
    Idle,
    Warping,
    Arriving,
}

pub struct WarpEffect {
    pub state: WarpState,
    pub progress: f32,
    pub duration: f32,
    pub start_position: Vec3,
    pub target_position: Vec3,
    pub current_time: f32,
}

impl WarpEffect {
    pub fn new() -> Self {
        WarpEffect {
            state: WarpState::Idle,
            progress: 0.0,
            duration: 1.0, 
            start_position: Vec3::zeros(),
            target_position: Vec3::zeros(),
            current_time: 0.0,
        }
    }

    pub fn start_warp(&mut self, from: Vec3, to: Vec3) {
        self.state = WarpState::Warping;
        self.progress = 0.0;
        self.current_time = 0.0;
        self.start_position = from;
        self.target_position = to;
    }

    pub fn update(&mut self, delta_time: f32) -> Option<Vec3> {
        match self.state {
            WarpState::Idle => None,
            
            WarpState::Warping => {
                self.current_time += delta_time;
                self.progress = (self.current_time / self.duration).min(1.0);

                if self.progress >= 1.0 {
                    self.state = WarpState::Arriving;
                    self.progress = 0.0;
                    self.current_time = 0.0;
                }

                let eased_progress = self.ease_in_out_cubic(self.progress);
                Some(self.interpolate_position(eased_progress))
            }

            WarpState::Arriving => {
                self.current_time += delta_time;
                self.progress = (self.current_time / 0.3).min(1.0);

                if self.progress >= 1.0 {
                    self.state = WarpState::Idle;
                    return Some(self.target_position);
                }

                Some(self.target_position)
            }
        }
    }

    fn ease_in_out_cubic(&self, t: f32) -> f32 {
        if t < 0.5 {
            4.0 * t * t * t
        } else {
            1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
        }
    }

    fn interpolate_position(&self, t: f32) -> Vec3 {
        self.start_position * (1.0 - t) + self.target_position * t
    }

    pub fn get_distortion_factor(&self) -> f32 {
        match self.state {
            WarpState::Idle => 0.0,
            WarpState::Warping => {
                let peak = 0.5;
                let distance_from_peak = (self.progress - peak).abs();
                1.0 - (distance_from_peak * 2.0)
            }
            WarpState::Arriving => {
                1.0 - self.progress
            }
        }
    }

    pub fn is_active(&self) -> bool {
        self.state != WarpState::Idle
    }
}

impl Default for WarpEffect {
    fn default() -> Self {
        Self::new()
    }
}