mod models;

use log::{error, info, trace};
use std::cmp::max;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufWriter, Write};
use std::iter::Iterator;
use std::path::Path;
use std::result::Result;
use std::time::Instant;

use eu4save::{CountryTag, Eu4Date, Eu4File, EnvTokens, query::Query, query::CountryIncomeLedger};
use eu4save::models::{Country, GameState, Eu4Save, Province};
use jomini::common::Date;
use regex::Regex;

fn round_two_digits(f: f32) -> f32 {
    return (f * 100.0).round() / 100.0;
}

fn parse_save_file<P>(file_name: P) -> Result<Eu4Save, Box<dyn Error>>
where P: AsRef<Path> {
    let data = std::fs::read(file_name)?;
    trace!("Bytes read: {:?}", data.len());

    let file = Eu4File::from_slice(&data)?;
    return Ok(file.parse_save(&EnvTokens)?);
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

fn get_income(ledger: &CountryIncomeLedger) -> f32 {
    return ledger.taxation +
        ledger.production +
        ledger.trade +
        ledger.gold +
        ledger.tariffs +
        ledger.vassals +
        ledger.harbor_fees +
        ledger.subsidies +
        ledger.war_reparations +
        ledger.interest +
        ledger.spoils_of_war +
        ledger.siphoning_income +
        ledger.condottieri +
        ledger.knowledge_sharing +
        ledger.blockading_foreign_ports +
        ledger.looting_foreign_cities +
        ledger.other;
}

fn generate_country_stats(
    save_query: &Query,
    country: &Country,
    tag: &CountryTag) -> Result<models::CondensedCountry, Box<dyn Error>> {

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

    let num_buildings = get_num_buildings(&provinces, &tag);
    let cc = models::CondensedCountry {
        total_development: round_two_digits(country.raw_development),
        real_development: round_two_digits(country.development),
        gp_score: country.great_power_score.round() as i32,
        powers_earned: [
            country.powers[0] + country.adm_spent_indexed.iter().map(|t| t.1).sum::<i32>(),
            country.powers[1] + country.dip_spent_indexed.iter().map(|t| t.1).sum::<i32>(),
            country.powers[2] + country.mil_spent_indexed.iter().map(|t| t.1).sum::<i32>(),
        ],
        technology: [
            country.technology.adm_tech as i32,
            country.technology.dip_tech as i32,
            country.technology.mil_tech as i32,
        ],
        ideas: country.active_idea_groups.clone(),
        total_ideas: country.active_idea_groups.clone().iter().map(|i| i.1).sum::<u8>(),
        current_manpower: country.manpower.round() as i32 * 1000,
        max_manpower: country.max_manpower.round() as i32 * 1000,
        average_monarch: get_avg_monarch(&country, &save_query.save().meta.date),
        income: round_two_digits(get_income(&save_query.country_income_breakdown(country))),
        income_history: save_query.save().game.income_statistics.ledger.clone().into_iter().filter(|d| d.name == *tag).map(|d| d.data).collect::<Vec<_>>()[0].clone().into(),
        number_provinces: country.num_of_cities,
        number_buildings: num_buildings,
        buildings_value: get_buildings_value(&provinces, &tag, &buildings_values),
        buildings_per_province: round_two_digits(num_buildings as f32 / country.num_of_cities as f32),
        innovativeness: round_two_digits(country.innovativeness),
        absolutism: round_two_digits(country.absolutism),
        average_development: round_two_digits(country.raw_development / country.num_of_cities as f32),
        average_development_real: round_two_digits(country.development / country.num_of_cities as f32),
    };

    Ok(cc)
}

fn get_discipline(country: &Country) -> f32 {
    let mut base: f32 = 100.0;
    // Ideas
    for (name, amt) in &country.active_idea_groups {
        if (name.contains("offensive") || name.contains("quality")) && *amt >= 7 {
            base += 5.0;
        }
    }

    // Policies
    let policies = &country.active_policies;
    for policy in policies {
        if policy.policy.contains("weapon_quality") {
            base += 5.0;
        } else if policy.policy.contains("on_our_terms") {
            base += 2.5;
        }
    }

    // Advisor
    // TBD

    // Monarch
    let events = &country.history.events;
    let monarch_events = events.into_iter().filter(|(_k, v)| v.as_monarch().is_some());
    let last_monarch = &monarch_events.last().unwrap().1;
    for (personality, _) in &last_monarch.as_monarch().unwrap().personalities {
        if personality.contains("strict") {
            base += 5.0;
        }
    }

    // Absolutism
    base += f32::min(country.absolutism, 100.0) / 20.0;

    return base;
}

fn get_army_morale(country: &Country) -> f32 {
    // If you're drilling, then lol
    let mut morale: f32 = 0.0;
    let armies = &country.armies;
    for army in armies {
        let regiments = &army.regiments;
        for r in regiments {
            morale = morale.max(r.morale);
        }
    }
    return morale;
}

fn get_force_limit(country: &Country) -> i32 {
    // This gets the total number of troops currently, not FL
    let mut troops: i32 = 0;
    let armies = &country.armies;
    for army in armies {
        let regiments = &army.regiments;
        troops += regiments.len() as i32;
    }
    return troops;
}

fn get_siege_ability(country: &Country, tag: &CountryTag, gamestate: &GameState) -> f32 {
    let mut base: f32 = 0.0;
    // Ideas
    let ideas = &country.active_idea_groups;
    for (name, amt) in ideas {
        if name.contains("offensive") && *amt >= 5 {
            base += 20.0;
        } else if name.contains("espionage") && *amt >= 3 {
            base += 10.0;
        }
    }

    // Policies
    let policies = &country.active_policies;
    for policy in policies {
        let p = &policy.policy;
        if p.contains("word_is_my_bond") || p.contains("fear_tactics") || p.contains("siege_weapons") || p.contains("military_zeal"){
            base += 10.0;
        }
    }

    // War Exhaustion
    // TBD

    // Army Tradition
    base += &country.army_tradition / 20.0;

    // Army Professionalism
    base += &country.army_professionalism / 0.05;

    // Military Hegemon
    if gamestate.military_hegemon.clone().is_some_and(|h| h.country == *tag && h.progress >= 100.0) {
        base += 20.0;
    }

    return base;
}

fn get_fort_defense(country: &Country) -> f32 {
    let mut base: f32 = 0.0;
    // Ideas
    for (name, amt) in &country.active_idea_groups {
        if name.contains("defensive") && *amt >= 5 {
            base += 25.0;
        }
    }

    // Policies
    let policies = &country.active_policies;
    for policy in policies {
        let p = &policy.policy;
        if p.contains("for the people") {
            base += 25.0;
        } else if p.contains("privy_council") || p.contains("loyal_conduct") {
            base += 15.0;
        } else if p.contains("superior_fortifications") {
            base += 10.0;
        }
    }

    // Power Projection
    base += &country.current_power_projection / 10.0;

    return base;
}

fn get_infantry_ca(country: &Country) -> f32 {
    let mut base: f32 = 0.0;
    // Ideas
    for (name, amt) in &country.active_idea_groups {
        if name.contains("mercenary") && *amt >= 6 {
            base += 10.0;
        } else if name.contains("quality") && *amt >= 1 {
            base += 10.0;
        }
    }

    // Policies
    let policies = &country.active_policies;
    for policy in policies {
        let p = &policy.policy;
        if p.contains("modern_firearm") {
            base += 15.0;
        }
    }

    return base;
}

fn get_cavalry_ca(country: &Country) -> f32 {
    let mut base: f32 = 0.0;
    // Ideas
    for (name, amt) in &country.active_idea_groups {
        if name.contains("horde") && *amt >= 7 {
            base += 25.0;
        } else if name.contains("aristocratic") && *amt >= 1 {
            base += 15.0;
        } else if name.contains("quality") && *amt >= 3 {
            base += 10.0;
        }
    }

    // Policies
    let policies = &country.active_policies;
    for policy in policies {
        let p = &policy.policy;
        if p.contains("noble_loyalty") || p.contains("psychological") {
            base += 10.0;
        }
    }
    
    return base;
}

fn get_artillery_ca(country: &Country) -> f32 {
    let mut base: f32 = 0.0;
    // Ideas
    for (name, amt) in &country.active_idea_groups {
        if name.contains("quality") && *amt >= 7 {
            base += 10.0;
        }
    }

    // Policies
    let policies = &country.active_policies;
    for policy in policies {
        let p = &policy.policy;
        if p.contains("horse_artillery") {
            base += 10.0;
        }
    }
    
    return base;
}

fn get_leader_fire(country: &Country) -> u8 {
    let mut base: u8 = 0;
    // Ideas
    for (name, amt) in &country.active_idea_groups {
        if name.contains("offensive") && *amt >= 3 {
            base += 1;
        }
    }

    // Policies
    let policies = &country.active_policies;
    for policy in policies {
        let p = &policy.policy;
        if p.contains("mining_act") {
            base += 1;
        }
    }
    
    return base;
}

fn get_leader_shock(country: &Country) -> u8 {
    let mut base: u8 = 0;
    // Ideas
    for (name, amt) in &country.active_idea_groups {
        if name.contains("offensive") && *amt >= 1 {
            base += 1;
        }
    }

    // Policies
    let policies = &country.active_policies;
    for policy in policies {
        let p = &policy.policy;
        if p.contains("inspirational_leaders") {
            base += 1;
        }
    }
    
    return base;
}

fn get_leader_maneuver(country: &Country) -> u8 {
    let mut base: u8 = 0;
    // Ideas
    for (name, amt) in &country.active_idea_groups {
        if name.contains("defensive") && *amt >= 3 {
            base += 1;
        }
    }

    // Policies
    let policies = &country.active_policies;
    for policy in policies {
        let p = &policy.policy;
        if p.contains("hired_adventurers") {
            base += 1;
        }
    }
    
    return base;
}

fn get_leader_siege(country: &Country) -> u8 {
    let mut base: u8 = 0;
    // Ideas
    for (name, amt) in &country.active_idea_groups {
        if name.contains("aristocratic") && *amt >= 7 {
            base += 1;
        }
    }

    // Policies
    let policies = &country.active_policies;
    for policy in policies {
        let p = &policy.policy;
        if p.contains("modern_siege") {
            base += 1;
        }
    }
    
    return base;
}

fn get_navy_morale(country: &Country) -> f32 {
    // If you're drilling, then lol
    let mut morale: f32 = 0.0;
    let navies = &country.navies;
    for navy in navies {
        let ships = &navy.ships;
        for s in ships {
            morale = morale.max(s.morale);
        }
    }
    return morale;
}

fn get_navy_force_limit(country: &Country) -> i32 {
    // This gets the total number of ships currently, not FL
    let mut ships: i32 = 0;
    let navies = &country.navies;
    for navy in navies {
        let s = &navy.ships;
        ships += s.len() as i32;
    }
    return ships;
}

fn get_merc_discipline(country: &Country, tag: &CountryTag, gamestate: &GameState) -> f32 {
    let mut base: f32 = 100.0;
    // Ideas
    for (name, amt) in &country.active_idea_groups {
        if name.contains("mercenary") && *amt >= 7 {
            base += 5.0;
        }
    }

    // Policies
    let policies = &country.active_policies;
    for policy in policies {
        let p = &policy.policy;
        if p.contains("mercenary_tactical") {
            base += 5.0;
        }
    }

    // Economic Hegemon
    let econ = &gamestate.economic_hegemon;
    if econ.clone().is_some_and(|h| h.country == *tag) {
        base += econ.clone().unwrap().progress / 10.0;
    }

    return base;
}

fn generate_military_stats(
    save_query: &Query,
    country: &Country,
    tag: &CountryTag) -> Result<models::CountryMilitary, Box<dyn Error>> {      
    let military = models::CountryMilitary {
        army_tradition: round_two_digits(country.army_tradition),
        army_morale: round_two_digits(get_army_morale(&country)),
        army_discipline: round_two_digits(get_discipline(&country)),
        army_force_limit: get_force_limit(&country),
        army_professionalism: round_two_digits(country.army_professionalism * 100.0),
        siege_ability: round_two_digits(get_siege_ability(&country, &tag, &save_query.save().game)),
        fort_defense: round_two_digits(get_fort_defense(&country)),
        infantry_ability: get_infantry_ca(&country),
        cavalry_ability: get_cavalry_ca(&country),
        artillery_ability: get_artillery_ca(&country),
        fire_dealt: round_two_digits(country.army_professionalism * 10.0),
        fire_received: 0.0,
        shock_dealt: round_two_digits(country.army_professionalism * 10.0),
        shock_received: 0.0,
        leader_fire: get_leader_fire(&country), //u8
        leader_shock: get_leader_shock(&country), //u8
        leader_maneuver: get_leader_maneuver(&country), //u8
        leader_siege: get_leader_siege(&country), //u8
        mercenary_discipline: get_merc_discipline(&country, &tag, &save_query.save().game),
        naval_tradition: round_two_digits(country.navy_tradition),
        naval_morale: round_two_digits(get_navy_morale(&country)),
        naval_force_limit: get_navy_force_limit(&country),    
    }; 
    Ok(military)
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

fn get_dev_ratio(mana_spent: [i32; 3]) -> String {
    let sum: i32 = mana_spent.iter().sum::<i32>();
    if sum == 0 {
        return "0/0/0".to_string();
    }
    let formatted = format!("{}/{}/{}", mana_spent[0] * 100 / sum, mana_spent[1] * 100 / sum, mana_spent[2] * 100 / sum);
    return formatted;
}

fn generate_mana(country: &Country) -> Result<models::CountryMana, Box<dyn Error>> {
    let spent_dev: [i32; 3] = [
        country.adm_spent_indexed.iter().filter(|t| t.0 == 7).map(|t| t.1).sum::<i32>(),
        country.dip_spent_indexed.iter().filter(|t| t.0 == 7).map(|t| t.1).sum::<i32>(),
        country.mil_spent_indexed.iter().filter(|t| t.0 == 7).map(|t| t.1).sum::<i32>(),
    ];
    let mana = models::CountryMana {
        mana_spent: [
            country.adm_spent_indexed.iter().map(|t| t.1).sum::<i32>(),
            country.dip_spent_indexed.iter().map(|t| t.1).sum::<i32>(),
            country.mil_spent_indexed.iter().map(|t| t.1).sum::<i32>(),
        ],
        spent_developing: spent_dev,
        developing_ratio: get_dev_ratio(spent_dev),
        spent_tech:
            country.adm_spent_indexed.iter().filter(|t| t.0 == 1).map(|t| t.1).sum::<i32>() +
            country.dip_spent_indexed.iter().filter(|t| t.0 == 1).map(|t| t.1).sum::<i32>() +
            country.mil_spent_indexed.iter().filter(|t| t.0 == 1).map(|t| t.1).sum::<i32>(),
        spent_culture:
            country.dip_spent_indexed.iter().filter(|t| [20, 33, 34, 35, 47].contains(&t.0)).map(|t| t.1).sum::<i32>(),
        spent_coring:
            country.adm_spent_indexed.iter().filter(|t| t.0 == 17).map(|t| t.1).sum::<i32>(),
        spent_inflation:
            country.adm_spent_indexed.iter().filter(|t| t.0 == 15).map(|t| t.1).sum::<i32>(),
        spent_ideas:
            country.adm_spent_indexed.iter().filter(|t| t.0 == 0).map(|t| t.1).sum::<i32>() +
            country.dip_spent_indexed.iter().filter(|t| t.0 == 0).map(|t| t.1).sum::<i32>() +
            country.mil_spent_indexed.iter().filter(|t| t.0 == 0).map(|t| t.1).sum::<i32>(),
        spent_force_march:
            country.mil_spent_indexed.iter().filter(|t| t.0 == 8).map(|t| t.1).sum::<i32>(),
        spent_generals:
            country.mil_spent_indexed.iter().filter(|t| [3, 5].contains(&t.0)).map(|t| t.1).sum::<i32>(),
        spent_unjustified:
            country.dip_spent_indexed.iter().filter(|t| t.0 == 14).map(|t| t.1).sum::<i32>(), 
    }; 
    Ok(mana)
}

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 3);

    let localisation_file = &args[1]; // "anb_countries_l_english.yml"
    let eu4_file_name = &args[2]; // "mp_Silverforge1663_02_06.eu4"
    let mut stats: models::Eu4Stats = models::Eu4Stats { 
        countries: Vec::new(),
    };

    let start = Instant::now();

    let localisation_map = parse_localisation(localisation_file);
    info!("Finished parsing localisation.");

    info!("Reading gamestate from {:?}", eu4_file_name);
    let eu4_save = parse_save_file(eu4_file_name).unwrap();
    let save_query = Query::from_save(eu4_save);
    info!("Finished parsing gamestate.");

    info!("Generating stats.");
    let players: HashMap<_, _> = save_query.players().into_iter().map(|p| (p.tag, p.name)).collect();
    info!("Players: {:?}", players);

    let countries = save_query.countries();
    for c in countries {
        let country = c.country;
        let country_tag = c.tag.to_string();
        let country_name = localisation_map.get(&country_tag).unwrap_or(&country_tag).to_string();
        if country.raw_development > 0.0 {
            trace!("{}: {:?} {:?}", stats.countries.len(), c.id, c.tag); 
            let country_stats = models::CountryStats {
                tag: country_tag,
                name: country_name,
                player: players.get(&c.tag).cloned(),
                country: generate_country_stats(&save_query, &country, &c.tag).unwrap(),
                military: generate_military_stats(&save_query, &country, &c.tag).unwrap(),
                mana: generate_mana(&country).unwrap(),
            };
            stats.countries.push(country_stats);
        }
    }
    info!("Number of countries: {}", stats.countries.len()); 
    info!("Finished generating stats.");

    let json_path = "parsed_country.json";
    let file = File::create(json_path).unwrap();
    let mut writer = BufWriter::new(file);
    let _ = serde_json::to_writer(&mut writer, &stats);
    let _ = match writer.flush() {
        Ok(w) => w,
        Err(e) => {
            error!("Error: {:?}", e);
            return;
        }
    };
    info!("Finished writing to {:?}", json_path);

    let duration = start.elapsed();
    info!("Time spent parsing: {:?}", duration);
}
