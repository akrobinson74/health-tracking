use chrono::{NaiveDate, NaiveTime};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FoodItemInput {
    pub date: Option<String>,
    pub time: Option<String>,
    pub name: String,
    pub weight: f64,
    pub calories: f64,
    pub notes: Option<String>,
    pub url: Option<String>,
}

impl FoodItemInput {
    pub fn date_as_native_date(&self) -> NaiveDate {
        let date_parse_result = NaiveDate::parse_from_str(self.date.as_ref().unwrap(), "%Y-%m-%d");
        if let Ok(_) = date_parse_result {
            date_parse_result.unwrap()
        } else {
            panic!("date parse error")
        }
    }

    pub fn time_as_native_time(&self) -> NaiveTime {
        let time_parse_result = NaiveTime::parse_from_str(self.time.as_ref().unwrap(), "%H:%M");

        if let Ok(_) = time_parse_result {
            time_parse_result.unwrap()
        } else {
            panic!("time parse error")
        }
    }
}