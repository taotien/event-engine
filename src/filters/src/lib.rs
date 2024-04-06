use chrono::TimeDelta;
use google_maps::prelude::*;
use std::time::Duration;
use url::Url;
pub mod maps;

pub struct Event {
    pub name: String,
    pub start_time: DateTime<Local>,
    pub end_time: DateTime<Local>,
    pub location: String,
    pub desc: String,
    pub price: u64,
    pub tags: Vec<String>,
    pub source: Url,
}

pub struct EventFilter {
    pub home_location: String,
    pub transit_method: TravelMode,
    pub max_radius_distance: Distance,
    pub max_radius_time: Duration,
    pub interests: Vec<String>,
}

pub struct TimeDistance {
    travel_duration: TimeDelta,
    distance: Distance,
}

#[derive(Debug)]
pub struct Distance {
    pub value: f64,
    pub unit: DistanceUnit,
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

#[derive(Debug)]
pub enum DistanceUnit {
    Kilometer,
    Mile,
}
