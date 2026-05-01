struct WeatherStation {
    temperature_c: f64,
    humidity: f64,
    wind_speed: f64
}

impl WeatherStation {
    fn comfort_index(&self) -> f64 {
        (100.0 - self.humidity) - self.temperature_c - 22.0
    }

    fn is_stormy(&self) -> bool {
        self.wind_speed > 80.0
    }
}

fn main() {
    let station = WeatherStation{
        wind_speed: 2.1231,
        temperature_c: 2.4523,
        humidity: 12.453
    };

    if station.is_stormy() {
        println!("Stormy");
    }
    
    let index = station.comfort_index();
    
    dbg!(index);
}

// --------------------------------------------------------------------------------


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn is_stormy_should_return_false() {
        let mock = WeatherStation {
            wind_speed: 10.0,
            temperature_c: 0.0,
            humidity: 0.0
        };

        assert!(!mock.is_stormy())
    }

    #[test]
    fn is_stormy_should_return_true() {
        let mock = WeatherStation {
            wind_speed: 90.0,
            temperature_c: 0.0,
            humidity: 0.0
        };

        assert!(mock.is_stormy())
    }

    #[test]
    fn comfort_index_should_return_correct_index() {
        let mock = WeatherStation{
            wind_speed: 10.0,
            temperature_c: 20.0,
            humidity: 10.0
        };

        assert_eq!(mock.comfort_index(), 48.0);
    }
}