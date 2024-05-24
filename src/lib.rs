mod kf;
mod ekf;
mod filters;

pub use kf::KF;
pub use ekf::EKF;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = kf::create_kf();
        assert_eq!(result.state_estimate, 0.0);
    }
}
