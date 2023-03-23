# 🌍 rscountries

rscountries is a clone of the popular [RESTCountries API](https://restcountries.com) implemented in Rust via Actix Web 🦀. 

It provides information about countries, such as their population, area, flag, and more! 
This project aims to be fast, reliable, and easy to use.

*Note: only v3.1 is currently (being) implemented!*

## 🚀 Getting Started

These instructions will help you set up the project and run it on your local machine for development and testing purposes.

## 🛠️ Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [Git](https://git-scm.com/downloads)

## 📥 How to Run Locally

1. Clone the repository:

```bash
git clone https://github.com/Zi0P4tch0/rscountries.git
```

2. Change directory to the repository

```bash
cd rscountries 
```

3. Run the project
```bash
cargo run
```

Default port is **8080**.

## Features

- Built on top of Actix Web
- Good-looking terminal logging via `slog` (implemented as an Actix Web middleware).
- Comes with Dockerfile for easy testing/deployment (final image is around 100MB in size).

## 📚 Usage

Once the server is running, you can make requests to the API using any HTTP client or web browser.

## 🌐 Endpoints (WIP)

- `GET /v3.1/all`: Retrieve a list of all countries
    - Example: `http://localhost:8080/v3.1/all`
- `GET /v3.1/name/{name}`: Retrieve a list of countries whose name contains `{name}` (case-insensitive).
    - Example: `http://localhost:8080/v3.1/name/eesti`
    - Example: `http://localhost:8080/v3.1/name/deutschland`
- `GET /v3.1/name/{name}?fullText=true`: Retrieve a list of countries whose name is `{name}` (case-insensitive).
    - Example: `http://localhost:8080/v3.1/name/aruba?fullText=true`


## 📋 API Data Fields (v3.1)

| Field                    | Info |
|--------------------------|------|
| cca2                     | ISO 3166-1 alpha-2 two-letter country codes |
| cca3                     | ISO 3166-1 alpha-3 three-letter country codes |
| altSpellings             | Alternate spellings of the country name |
| area                     | Geographical size |
| borders                  | Border countries |
| idd                      | International dialing codes |
| capital                  | Capital cities |
| car > signs              | Car distinguished (oval) signs |
| car > side               | Car driving side |
| cioc                     | Code of the International Olympic Committee |
| coatOfArms               | [MainFacts.com](https://mainfacts.com/coat-of-arms-countries-world) links to SVG and PNG images |
| continents               | List of continents the country is on |
| currencies               | List of all currencies |
| demonyms (m/f)           | Genderized inhabitants of the country |
| independent              | ISO 3166-1 independence status (the country is considered a sovereign state) |
| fifa                     | FIFA code |
| flag                     | v3: flag emoji |
| flags                    | [Flagpedia](https://flagpedia.net/) links
| gini                     | Worldbank [Gini](https://data.worldbank.org/indicator/SI.POV.GINI) index |
| landlocked               | Landlocked country |
| languages                | List of official languages |
| latlng                   | Latitude and longitude |
| maps                     | Link to Google maps and Open Street maps |
| name                     | Country name |
| name > official/common   | Official and common country name |
| nativeName > official/common | Official and common native country name |
| population               | Country population |
| region                   | UN [demographic regions](https://unstats.un.org/unsd/methodology/m49/) |
| status                   | ISO 3166-1 assignment status |
| subregion                | UN [demographic subregions](https://unstats.un.org/unsd/methodology/m49/) |
| tld                      | Internet top level domains |
| translations             | List of country name translations |
| unMember                 | UN Member status |

### 📜 License

This project is licensed under the MIT License. For more details, see the [LICENSE](LICENSE) file.
