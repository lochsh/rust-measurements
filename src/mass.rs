// Constants, metric
const KILOGRAM_GRAM_FACTOR: f64 = 1000.0;
const KILOGRAM_MILLIGRAM_FACTOR: f64 = 1000000.0;
const KILOGRAM_MICROGRAM_FACTOR: f64 = 1000000000.0;
const KILOGRAM_NANOGRAM_FACTOR: f64 = 1000000000000.0;

/// The `Mass` struct can be used to deal with mass in a common way.
/// Common metric units are supported
///
/// # Example
///
/// ```
/// use measurements::Mass;
///
/// let flour = Mass::from_kilograms(100.0);
/// println!("There are {} kg of flour in this recipe.", flour);
/// ```

#[derive(Copy, Clone, Debug)]
pub struct Mass {
    kilograms: f64,
}

impl Mass {
    // Inputs, metric
    pub fn from_kilograms(kilograms: f64) -> Self {
        Mass { kilograms: kilograms }
    }

    pub fn from_grams(grams: f64) -> Self {
        Self::from_kilograms(grams / KILOGRAM_GRAM_FACTOR)
    }

    // Outputs, metric
    pub fn as_kilograms(&self) -> f64 {
        self.kilograms
    }

    pub fn as_grams(&self) -> f64 {
        self.kilograms * KILOGRAM_GRAM_FACTOR
    }
}

impl ::std::fmt::Display for Mass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:.1} kg", self.as_kilograms())
    }
}
