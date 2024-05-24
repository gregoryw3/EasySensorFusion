pub struct KF {
    pub state_estimate: f64,
    pub estimate_cov: f64,
}

impl KF {
    pub fn new(x: f64, p: f64) -> KF {
        KF { state_estimate: x, estimate_cov: p }
    }
    
    pub fn update(&mut self, measurement_value: f64, measurement_noise_cov: f64) {
        let previous_state_estimate = self.state_estimate;
        let previous_estimate_cov = self.estimate_cov + measurement_noise_cov;
        let kalman_gain = previous_estimate_cov / (previous_estimate_cov + measurement_noise_cov);
        self.state_estimate = previous_state_estimate + kalman_gain * (measurement_value - previous_state_estimate);
        self.estimate_cov = (1.0 - kalman_gain) * previous_estimate_cov;
    }
}

impl Default for KF {
    fn default() -> Self {
        KF::new(0.0, 1.0)
    }
}