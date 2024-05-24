use std::ops::{Add, Mul, Sub};

/// A simple low-pass filter
pub struct LowPassFilter<T> {
    /// The smoothing factor
    alpha: T,
    /// The last output value
    last_output: Option<T>,
}

impl<T> LowPassFilter<T> 
    where
    T: Copy + Mul<Output = T> + Sub<Output = T> + Add<Output = T> + From<f64>, 
{
    /// Create a new low-pass filter with the given smoothing factor
    /// 
    /// # Arguments
    /// alpha - The smoothing factor
    pub fn new(alpha: T) -> Self {
        Self {
            alpha,
            last_output: None,
        }
    }
    
    pub fn filter(&mut self, input: T) -> T {
        match self.last_output {
            Some(last_output) => {
                // y[n] = α * x[n] + (1 - α) * y[n-1]
                // y[n] is the output at time n
                // x[n] is the input at time n
                // y[n-1] is the output at time n-1
                // α is the smoothing factor
                let output = self.alpha * input + (T::from(1.0) - self.alpha) * last_output;
                self.last_output = Some(output);
                output
            },
            None => {
                self.last_output = Some(input);
                input
            },
        }
    }
}