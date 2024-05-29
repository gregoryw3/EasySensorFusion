pub mod fusion {
    pub mod ekf;
    pub mod kf;
}

pub mod filters {
    pub mod high_pass;
    pub mod low_pass;
}

pub mod tools {
    pub mod fusion_configurator;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = fusion::kf::KF::new(0.0, 0.0);
        assert_eq!(result.state_estimate, 0.0);
    }
}
