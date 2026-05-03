enum DeliveryOption {
    Letter,
    Package { weight: f64 },
    Palette { weight: f64, count: usize },
}

impl DeliveryOption {
    fn cost(&self) -> f64 {
        match self {
            DeliveryOption::Letter => 2.0,
            DeliveryOption::Package { weight } => (0.5 * weight) + 2.0,
            DeliveryOption::Palette { weight, count } => {
                10.0 + weight * 0.2 + (*count as f64 * 1.5)
            }
        }
    }
}

fn main() {}

// --------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delivery_cost_letter_should_return_2() {
        assert_eq!(DeliveryOption::Letter.cost(), 2.0);
    }

    #[test]
    fn delivery_cost_package_should_return_3() {
        assert_eq!(DeliveryOption::Package {weight: 2.0}.cost(), 3.0);
    }

    #[test]
    fn delivery_cost_palette_should_return_160_4() {
        assert_eq!(
            DeliveryOption::Palette {
                weight: 2.0,
                count: 100
            }.cost(),
            160.4
        );
    }
}
