use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountryName {
    pub common: String,
    pub official: String,
    native: Option<HashMap<String, CountryName>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CountryCurrency {
    name: String,
    symbol: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CountryInternationalDirectDialing {
    root: Option<String>,
    suffixes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CountryTranslation {
    official: String,
    common: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CountryDemonym {
    #[serde(rename = "m")]
    male: String,
    #[serde(rename = "f")]
    female: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CountryMaps {
    #[serde(rename = "googleMaps")]
    google_maps: String,
    #[serde(rename = "openStreetMaps")]
    openstreetmaps: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CountryCar {
    signs: Option<Vec<String>>,
    side: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CountryImages {
    png: Option<String>,
    svg: Option<String>,
    alt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]

struct CountryCapitalInfo  {
    #[serde(rename = "latlng")]
    latitude_and_longitude: Option<Vec<f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CountryPostalCode {
    format: String,
    regex: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Country {
    pub name: CountryName,
    #[serde(rename = "tld")]
    top_level_domains: Option<Vec<String>>,
    cca2: String,
    ccn3: Option<String>,
    cca3: String,
    cioc: Option<String>,
    independent: Option<bool>,
    status: String,
    #[serde(rename = "unMember")]
    un_member: Option<bool>,
    currencies: Option<HashMap<String, CountryCurrency>>,
    #[serde(rename = "idd")]
    international_direct_dialing: Option<CountryInternationalDirectDialing>,
    capital: Option<Vec<String>>,
    alt_spellings: Option<Vec<String>>,
    region: String,
    subregion: Option<String>,
    languages: Option<HashMap<String, String>>,
    translations: HashMap<String, CountryTranslation>,
    #[serde(rename = "latlng")]
    latitude_and_longitude: Vec<f64>,
    landlocked: bool,
    borders: Option<Vec<String>>,
    area: f64,
    demonym: Option<HashMap<String, CountryDemonym>>,
    flag: String,
    maps: CountryMaps,
    population: u64,
    gini: Option<HashMap<String, f64>>,
    fifa: Option<String>,
    car: CountryCar,
    timezones: Vec<String>,
    continents: Vec<String>,
    flags: CountryImages,
    #[serde(rename = "coatOfArms")]
    coat_of_arms: CountryImages,
    #[serde(rename = "startOfWeek")]
    start_of_week: String,
    #[serde(rename = "capitalInfo")]
    capital_info: CountryCapitalInfo,
    #[serde(rename = "postalCode")]
    postal_code: Option<CountryPostalCode>
}
