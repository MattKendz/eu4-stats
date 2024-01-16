use jomini::JominiDeserialize;
use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Debug, Clone, JominiDeserialize, Default)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct CondensedCountry {
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
}

impl Serialize for CondensedCountry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer, {
        let mut s = serializer.serialize_struct("CondensedCountry", 19)?;
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
        s.end()
    }
}

#[derive(Debug, Clone, JominiDeserialize, Default)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct CountryMilitary {
    pub army_tradition: f32,
    pub army_morale: f32,
    pub army_discipline: f32,
    pub army_force_limit: i32,
    pub army_professionalism: f32,
    pub siege_ability: f32,
    pub fort_defense: f32,
    pub infantry_ability: f32,
    pub cavalry_ability: f32,
    pub artillery_ability: f32,
    pub fire_dealt: f32,
    pub fire_received: f32,
    pub shock_dealt: f32,
    pub shock_received: f32,
    pub leader_fire: u8,
    pub leader_shock: u8,
    pub leader_maneuver: u8,
    pub leader_siege: u8,
    pub mercenary_discipline: f32,
    pub naval_tradition: f32,
    pub naval_morale: f32,
    pub naval_force_limit: i32,
}

impl Serialize for CountryMilitary {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer, {
        let mut s = serializer.serialize_struct("CountryMilitary", 22)?;
        s.serialize_field("army_tradition", &self.army_tradition)?;
        s.serialize_field("army_morale", &self.army_morale)?;
        s.serialize_field("army_discipline", &self.army_discipline)?;
        s.serialize_field("army_force_limit", &self.army_force_limit)?;
        s.serialize_field("army_professionalism", &self.army_professionalism)?;
        s.serialize_field("siege_ability", &self.siege_ability)?;
        s.serialize_field("fort_defense", &self.fort_defense)?;
        s.serialize_field("infantry_ability", &self.infantry_ability)?;
        s.serialize_field("cavalry_ability", &self.cavalry_ability)?;
        s.serialize_field("artillery_ability", &self.artillery_ability)?;
        s.serialize_field("fire_dealt", &self.fire_dealt)?;
        s.serialize_field("fire_received", &self.fire_received)?;
        s.serialize_field("shock_dealt", &self.shock_dealt)?;
        s.serialize_field("shock_received", &self.shock_received)?;
        s.serialize_field("leader_fire", &self.leader_fire)?;
        s.serialize_field("leader_shock", &self.leader_shock)?;
        s.serialize_field("leader_maneuver", &self.leader_maneuver)?;
        s.serialize_field("leader_siege", &self.leader_siege)?;
        s.serialize_field("mercenary_discipline", &self.mercenary_discipline)?;
        s.serialize_field("naval_tradition", &self.naval_tradition)?;
        s.serialize_field("naval_morale", &self.naval_morale)?;
        s.serialize_field("naval_force_limit", &self.naval_force_limit)?;
        s.end()
    }
}

#[derive(Debug, Clone, JominiDeserialize, Default)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct CountryMana {
    pub mana_spent: [i32; 3],
    pub spent_developing: [i32; 3],
    pub developing_ratio: String,
    pub spent_tech: i32,
    pub spent_culture: i32,
    pub spent_coring: i32,
    pub spent_inflation: i32,
    pub spent_ideas: i32,
    pub spent_force_march: i32,
    pub spent_generals: i32,
    pub spent_unjustified: i32,
}

impl Serialize for CountryMana {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer, {
        let mut s = serializer.serialize_struct("CountryMana", 11)?;
        s.serialize_field("mana_spent", &self.mana_spent)?;
        s.serialize_field("spent_developing", &self.spent_developing)?;
        s.serialize_field("developing_ratio", &self.developing_ratio)?;
        s.serialize_field("spent_tech", &self.spent_tech)?;
        s.serialize_field("spent_culture", &self.spent_culture)?;
        s.serialize_field("spent_coring", &self.spent_coring)?;
        s.serialize_field("spent_inflation", &self.spent_inflation)?;
        s.serialize_field("spent_ideas", &self.spent_ideas)?;
        s.serialize_field("spent_force_march", &self.spent_force_march)?;
        s.serialize_field("spent_generals", &self.spent_generals)?;
        s.serialize_field("spent_unjustified", &self.spent_unjustified)?;
        s.end()
    }
}

#[derive(Debug, Clone, JominiDeserialize, Default)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct CountryStats {
    pub tag: String,
    pub name: String,
    pub player: Option<String>,
    pub country: CondensedCountry,
    pub military: CountryMilitary,
    pub mana: CountryMana,
}

impl Serialize for CountryStats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer, {
        let mut s = serializer.serialize_struct("CountryStats", 6)?;
        s.serialize_field("tag", &self.tag)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("player", &self.player)?;
        s.serialize_field("country", &self.country)?;
        s.serialize_field("military", &self.military)?;
        s.serialize_field("mana", &self.mana)?;
        s.end()
    }
}

#[derive(Debug, Clone, JominiDeserialize, Default)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct Eu4Stats {
    pub countries: Vec<CountryStats>,
}

impl Serialize for Eu4Stats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer, {
        let mut s = serializer.serialize_struct("Eu4Stats", 1)?;
        s.serialize_field("countries", &self.countries)?;
        s.end()
    }
}