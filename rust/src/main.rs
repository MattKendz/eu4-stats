use log::{error, info, trace};
use std::cmp::max;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufWriter, Write};
use std::iter::Iterator;
use std::path::Path;
use std::time::Instant;

use eu4save::{CountryTag, Eu4Date, Eu4File, EnvTokens, query::Query};
use eu4save::models::{Country, Province};
use jomini::JominiDeserialize;
use jomini::common::Date;
use regex::Regex;
use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Debug, Clone, JominiDeserialize, Default)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct CondensedCountry {
    pub tag: String,
    pub name: String,
    pub total_development: f32,
    pub real_development: f32,
    pub gp_score: i32,
    pub powers_earned: [i32; 3],
    pub technology: [i32; 3],
    pub ideas: Vec<(String, u8)>,
    pub total_ideas: u8,
    pub current_manpower: i32,
    pub max_manpower: i32,
    pub average_monarch: [f32; 3],
    pub income: f32,
    pub number_provinces: i32,
    pub number_buildings: i32,
    pub buildings_value: i32,
    pub buildings_per_province: f32,
    pub innovativeness: f32,
    pub absolutism: f32,
    pub average_development: f32,
    pub average_development_real: f32,
    pub player: Option<String>,
    pub army_professionalism: f32,
}

impl Serialize for CondensedCountry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer, {
        let mut s = serializer.serialize_struct("CondensedCountry", 23)?;
        s.serialize_field("tag", &self.tag)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("total_development", &self.total_development)?;
        s.serialize_field("real_development", &self.real_development)?;
        s.serialize_field("gp_score", &self.gp_score)?;
        s.serialize_field("powers_earned", &self.powers_earned)?;
        s.serialize_field("technology", &self.technology)?;
        s.serialize_field("ideas", &self.ideas)?;
        s.serialize_field("total_ideas", &self.total_ideas)?;
        s.serialize_field("current_manpower", &self.current_manpower)?;
        s.serialize_field("max_manpower", &self.max_manpower)?;
        s.serialize_field("average_monarch", &self.average_monarch)?;
        s.serialize_field("income", &self.income)?;
        s.serialize_field("number_provinces", &self.number_provinces)?;
        s.serialize_field("number_buildings", &self.number_buildings)?;
        s.serialize_field("buildings_value", &self.buildings_value)?;
        s.serialize_field("buildings_per_province", &self.buildings_per_province)?;
        s.serialize_field("innovativeness", &self.innovativeness)?;
        s.serialize_field("absolutism", &self.absolutism)?;
        s.serialize_field("average_development", &self.average_development)?;
        s.serialize_field("average_development_real", &self.average_development_real)?;
        s.serialize_field("player", &self.player)?;
        s.serialize_field("army_professionalism", &self.army_professionalism)?;
        s.end()
    }
}

#[derive(Debug, Clone, JominiDeserialize, Default)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct Eu4Stats {
    pub countries: Vec<CondensedCountry>
}

impl Serialize for Eu4Stats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer, {
        let mut s = serializer.serialize_struct("Eu4Stats", 1)?;
        s.serialize_field("countries", &self.countries)?;
        s.end()
    }
}

fn round_two_digits(f: f32) -> f32 {
    return (f * 100.0).round() / 100.0;
}

fn get_avg_monarch(country: &Country, current_date: &Eu4Date) -> [f32; 3] {
    let start_date = Date::parse("1444.11.11").unwrap();
    let mut last_date = Date::parse("1444.11.11").unwrap();
    let mut last_ruler = [0, 0, 0];
    let mut monarch_power_generated = [0.0, 0.0, 0.0];
    let events = &country.history.events;
    let monarch_events = events.into_iter().filter(|(_k, v)| v.as_monarch().is_some() /*&& start_date.days_until(&k) >= 0*/);
    for (date, e) in monarch_events {
        let monarch = e.as_monarch().unwrap();
        trace!("{:?}: {} [{}, {}, {}]", date, monarch.name, monarch.adm, monarch.dip, monarch.mil);
        if start_date.days_until(date) <= 0 {
            last_date = *date;
            last_ruler[0] = monarch.adm;
            last_ruler[1] = monarch.dip;
            last_ruler[2] = monarch.mil;
            continue;
        }
        let days_passed = max(last_date, start_date).days_until(date);
        trace!("Adding {} days as {:?}", days_passed, last_ruler);
        for i in 0..3 {
            monarch_power_generated[i] += (days_passed * last_ruler[i] as i32) as f32;
        }

        last_date = *date;
        last_ruler[0] = monarch.adm;
        last_ruler[1] = monarch.dip;
        last_ruler[2] = monarch.mil;
    }

    let days_passed = last_date.days_until(current_date);
    trace!("Adding {} days as {:?}", days_passed, last_ruler);
    for i in 0..3 {
        monarch_power_generated[i] += (days_passed * last_ruler[i] as i32) as f32;
        monarch_power_generated[i] /= start_date.days_until(current_date) as f32;
        monarch_power_generated[i] = round_two_digits(monarch_power_generated[i]);
        trace!("{}: {}", i, monarch_power_generated[i]);
        assert!(monarch_power_generated[i] >= 0.0 && monarch_power_generated[i] <= 6.0);
    }
    
    return monarch_power_generated;
}

fn get_num_buildings(provinces: &Vec<Province>, tag: &CountryTag) -> i32 {
    let country_provinces = provinces
                            .into_iter()
                            .filter(|p| p.owner.is_some_and(|o| o == *tag));
    let mut num_buildings = 0;
    for province in country_provinces {
        num_buildings += province.buildings.values().filter(|v| **v).count();
    }
    return num_buildings as i32;
}

fn get_buildings_value(provinces: &Vec<Province>, tag: &CountryTag, values: &HashMap<&str, i32>) -> i32 {
    let country_provinces = provinces
                            .into_iter()
                            .filter(|p| p.owner.is_some_and(|o| o == *tag));
    let mut buildings_value: i32 = 0;
    for province in country_provinces {
        let buildings: Vec<String> = province.buildings.clone().into_iter().filter(|(_k, v)| *v).map(|(k, _v)| k).collect();
        for b in buildings {
            buildings_value += values.get(&*b).unwrap();
        }
    }
    return buildings_value as i32;
}

fn generate_stats<P>(file_name: P, localisation_map: HashMap<String, String>) -> Result<Eu4Stats, Box<dyn Error>>
where P: AsRef<Path>, {
    trace!("Map: {:?}", localisation_map);
    let data = std::fs::read(file_name)?;
    trace!("Bytes read: {:?}", data.len());

    let file = Eu4File::from_slice(&data)?;
    let save = file.parse_save(&EnvTokens)?;

    let save_query = Query::from_save(save);
    let players: HashMap<_, _> = save_query.players().into_iter().map(|p| (p.tag, p.name)).collect();
    info!("Players: {:?}", players);

    let provinces: Vec<Province> = save_query.save()
                    .game
                    .provinces
                    .clone()
                    .into_values()
                    .collect::<Vec<Province>>();

    let buildings_values = HashMap::from([
        ("courthouse", 100),
        ("town_hall", 200),
        ("university", 300),
        ("workshop", 100),
        ("counting_house", 400),
        ("temple", 100),
        ("cathedral", 300),
        ("shipyard", 100),
        ("grand_shipyard", 300),
        ("dock", 100),
        ("drydock", 300),
        ("marketplace", 100),
        ("trade_depot", 300),
        ("stock_exchange", 400),
        ("coastal_defense", 100),
        ("naval_battery", 200),
        ("barracks", 100),
        ("training_fields", 300),
        ("regimental_camp", 200),
        ("conscription_center", 400),
        ("fort_15th", 200),
        ("fort_16th", 400),
        ("fort_17th", 600),
        ("fort_18th", 800),
        ("farm_estate", 500),
        ("ramparts", 500),
        ("impressment_offices", 500),
        ("wharf", 500),
        ("textile", 500),
        ("weapons", 500),
        ("state_house", 500),
        ("plantations", 500),
        ("tradecompany", 500),
        ("soldier_households", 500),
        ("mills", 500),
        ("furnace", 500),
        ("mage_tower", 500),
        ("fort_magic", 500),
        ("native_earthwork", 100),
        ("native_fortified_house", 200),
        ("native_storehouse", 100),
        ("native_longhouse", 100),
        ("native_great_trail", 100),
        ("native_three_sisters_field", 100),
    ]);

    let mut stats: Eu4Stats = Eu4Stats { countries: Vec::new() };
    let countries = save_query.countries();
    for country in countries {
        if country.country.raw_development > 0.0 {
            trace!("{}: {:?} {:?}", stats.countries.len(), country.id, country.tag);      
            let country_tag = country.tag.to_string();
            let country_name = localisation_map.get(&country_tag).unwrap_or(&country_tag).to_string();
            let num_buildings = get_num_buildings(&provinces, &country.tag);
            let cc = CondensedCountry {
                tag: country_tag,
                name: country_name,
                total_development: round_two_digits(country.country.raw_development),
                real_development: round_two_digits(country.country.development),
                gp_score: country.country.great_power_score.round() as i32,
                powers_earned: [
                    country.country.powers[0] + country.country.adm_spent_indexed.iter().map(|t| t.1).sum::<i32>(),
                    country.country.powers[1] + country.country.dip_spent_indexed.iter().map(|t| t.1).sum::<i32>(),
                    country.country.powers[2] + country.country.mil_spent_indexed.iter().map(|t| t.1).sum::<i32>(),
                ],
                technology: [
                    country.country.technology.adm_tech as i32,
                    country.country.technology.dip_tech as i32,
                    country.country.technology.mil_tech as i32,
                ],
                ideas: country.country.active_idea_groups.clone(),
                total_ideas: country.country.active_idea_groups.clone().iter().map(|i| i.1).sum::<u8>(),
                current_manpower: country.country.manpower.round() as i32 * 1000,
                max_manpower: country.country.max_manpower.round() as i32 * 1000,
                average_monarch: get_avg_monarch(&country.country, &save_query.save().meta.date),
                income: round_two_digits(country.country.ledger.income.iter().sum::<f32>()),
                number_provinces: country.country.num_of_cities,
                number_buildings: num_buildings,
                buildings_value: get_buildings_value(&provinces, &country.tag, &buildings_values),
                buildings_per_province: round_two_digits(num_buildings as f32 / country.country.num_of_cities as f32),
                innovativeness: round_two_digits(country.country.innovativeness),
                absolutism: round_two_digits(country.country.absolutism),
                average_development: round_two_digits(country.country.raw_development / country.country.num_of_cities as f32),
                average_development_real: round_two_digits(country.country.development / country.country.num_of_cities as f32),
                player: players.get(&country.tag).cloned(),
                army_professionalism: round_two_digits(country.country.army_professionalism * 100.0),
            };           
            stats.countries.push(cc);
        }
    }
    info!("Number of countries: {}", stats.countries.len());
    Ok(stats)
}

fn parse_localisation<P>(file_name: P) -> HashMap<String, String>
where P: AsRef<Path>, {
    let mut localisation_map: HashMap<String, String> = HashMap::new();
    let re = Regex::new(r#"(\w\d{2}):0 "(.*)""#).unwrap();
    if let Ok(lines) = read_lines(file_name) {
        for line in lines.flatten() {
            let Some(groups) = re.captures(&line) else { continue };
            localisation_map.insert((&groups[1]).to_string(), (&groups[2]).to_string());
        }
    }
    return localisation_map;
}

fn read_lines<P>(file_name: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(file_name)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 3);

    let localisation_file = &args[1]; // "anb_countries_l_english.yml"
    let eu4_file_name = &args[2]; // "mp_Silverforge1663_02_06.eu4"
    let start = Instant::now();

    let localisation_map = parse_localisation(localisation_file);
    info!("Finished parsing localisation.");

    info!("Reading gamestate from {:?}", eu4_file_name);
    let eu4stats = match generate_stats(eu4_file_name, localisation_map) {
        Ok(stats) => stats,
        Err(e) => {
            error!("Error: {:?}", e);
            return;
        }
    };
    info!("Finished parsing gamestate.");

    let json_path = Path::new(&eu4_file_name).with_extension("json");
    let file = File::create(json_path.clone()).unwrap();
    let mut writer = BufWriter::new(file);
    let _ = serde_json::to_writer(&mut writer, &eu4stats);
    let _ = match writer.flush() {
        Ok(w) => w,
        Err(e) => {
            error!("Error: {:?}", e);
            return;
        }
    };
    info!("Finished writing to {:?}", json_path.clone());

    let duration = start.elapsed();
    info!("Time spent parsing: {:?}", duration);
}
