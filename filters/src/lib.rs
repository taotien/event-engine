use chrono::TimeDelta;
use std::fmt;
pub mod filter;
pub mod good_data;
pub mod interests;
pub mod maps;

/// A type from the maps function that stores a travel_duration as TimeDelta and distance as Distance
pub struct TimeDistance {
    pub travel_duration: TimeDelta,
    pub distance: Distance,
}

/// A struct that represents a distance with units of Miles or Kilometers. Comparable despite units or Distances
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Distance {
    pub value: f64,
    pub unit: DistanceUnit,
}

impl fmt::Display for Distance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit)
    }
}

impl Distance {
    pub fn from_kilometers(kilometers: f64) -> Distance {
        Distance {
            value: kilometers,
            unit: DistanceUnit::Kilometer,
        }
    }

    pub fn from_miles(miles: f64) -> Distance {
        Distance {
            value: miles,
            unit: DistanceUnit::Mile,
        }
    }

    pub fn to_kilometers(&self) -> f64 {
        match self.unit {
            DistanceUnit::Kilometer => self.value,
            DistanceUnit::Mile => self.value * 1.60934,
        }
    }

    pub fn to_miles(&self) -> f64 {
        match self.unit {
            DistanceUnit::Kilometer => self.value / 1.60934,
            DistanceUnit::Mile => self.value,
        }
    }
}
impl Eq for Distance {}

impl Ord for Distance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_kilometers = self.to_kilometers();
        let other_kilometers = other.to_kilometers();
        self_kilometers.partial_cmp(&other_kilometers).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum DistanceUnit {
    Kilometer,
    Mile,
}
impl fmt::Display for DistanceUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DistanceUnit::Kilometer => write!(f, "Kilometer"),
            DistanceUnit::Mile => write!(f, "Mile"),
        }
    }
}
