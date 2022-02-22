#[derive(Debug, serde::Serialize)]
#[cfg_attr(test, derive(PartialEq))]
pub(in crate::pool) struct Amount<T>(pub T);

impl std::ops::AddAssign<f32> for Amount<f64> {
    fn add_assign(&mut self, rhs: f32) {
        self.0 += rhs as f64
    }
}

impl std::ops::AddAssign<&Amount<f32>> for Amount<f64> {
    fn add_assign(&mut self, rhs: &Amount<f32>) {
        self.0 += rhs.0 as f64
    }
}

impl std::ops::SubAssign<f32> for Amount<f64> {
    fn sub_assign(&mut self, rhs: f32) {
        if self.0 >= rhs as f64 {
            self.0 -= rhs as f64
        }
    }
}

impl std::ops::SubAssign<&Amount<f32>> for Amount<f64> {
    fn sub_assign(&mut self, rhs: &Amount<f32>) {
        self.0 -= rhs.0 as f64
    }
}

#[cfg(test)]
mod tests {
    use super::Amount;

    #[test]
    fn test_increase() {
        let mut amount = Amount(1.0);
        amount += 1.5;
        assert_eq!(amount.0, 2.5);
    }

    #[test]
    fn test_decrement() {
        let mut amount = Amount(1.5);
        amount -= 0.5;
        assert_eq!(amount.0, 1.0);
    }

    #[test]
    fn test_cannot_decrement_under_zero() {
        let mut amount = Amount(1.5);
        amount -= 2.0;
        assert_eq!(amount.0, 1.5);
    }
}
