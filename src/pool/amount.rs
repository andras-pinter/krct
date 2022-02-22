#[derive(Debug, serde::Serialize)]
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

impl PartialOrd<f32> for Amount<f64> {
    fn partial_cmp(&self, other: &f32) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&(*other as f64))
    }
}

impl PartialEq<f32> for Amount<f64> {
    fn eq(&self, other: &f32) -> bool {
        self.0 == (*other) as f64
    }
}

#[cfg(test)]
impl<T> PartialEq<Amount<T>> for Amount<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Amount<T>) -> bool {
        self.0 == other.0
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

    #[test]
    fn test_lesser() {
        let amount = Amount(1.5);
        assert!(amount < 2.0);
    }

    #[test]
    fn test_greater() {
        let amount = Amount(1.5);
        assert!(amount > 1.0);
    }

    #[test]
    fn test_equals() {
        let amount = Amount(1.5);
        assert_eq!(amount, 1.5);
    }
}
