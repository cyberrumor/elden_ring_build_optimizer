use soup::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

extern crate reqwest;
extern crate soup;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Slot {
    Helm,
    Chest,
    Gauntlets,
    Legs,
    Empty,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArmorPiece {
    pub name: String,
    pub slot: Slot,
    pub physical: u16,
    pub slash: u16,
    pub strike: u16,
    pub pierce: u16,
    pub magic: u16,
    pub fire: u16,
    pub lightning: u16,
    pub holy: u16,
    pub immunity: u16,
    pub robustness: u16,
    pub focus: u16,
    pub vitality: u16,
    pub poise: u16,
    pub weight: u16,
    pub maximize_stat: u16,
}

impl ArmorPiece {
    #[must_use]
    pub const fn new(slot: Slot) -> Self {
        Self {
            name: String::new(),
            slot,
            physical: 0,
            slash: 0,
            strike: 0,
            pierce: 0,
            magic: 0,
            fire: 0,
            lightning: 0,
            holy: 0,
            immunity: 0,
            robustness: 0,
            focus: 0,
            vitality: 0,
            poise: 0,
            weight: 0,
            maximize_stat: 0,
        }
    }
}

impl std::fmt::Display for Slot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArmorSet {
    pub helm: String,
    pub chest: String,
    pub gauntlets: String,
    pub legs: String,
    pub physical: u16,
    pub slash: u16,
    pub strike: u16,
    pub pierce: u16,
    pub magic: u16,
    pub fire: u16,
    pub lightning: u16,
    pub holy: u16,
    pub immunity: u16,
    pub robustness: u16,
    pub focus: u16,
    pub vitality: u16,
    pub poise: u16,
    pub weight: u16,
    pub maximize_stat: u16,
}

impl ArmorSet {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            helm: String::new(),
            chest: String::new(),
            gauntlets: String::new(),
            legs: String::new(),
            physical: 0,
            slash: 0,
            strike: 0,
            pierce: 0,
            magic: 0,
            fire: 0,
            lightning: 0,
            holy: 0,
            immunity: 0,
            robustness: 0,
            focus: 0,
            vitality: 0,
            poise: 0,
            weight: 0,
            maximize_stat: 0,
        }
    }
    #[must_use]
    pub fn from(
        helm: ArmorPiece,
        chest: ArmorPiece,
        gauntlet: ArmorPiece,
        leg: ArmorPiece,
    ) -> Self {
        Self {
            helm: helm.name,
            chest: chest.name,
            gauntlets: gauntlet.name,
            legs: leg.name,

            physical: helm.physical + chest.physical + gauntlet.physical + leg.physical,

            slash: helm.slash + chest.slash + gauntlet.slash + leg.slash,

            strike: helm.strike + chest.strike + gauntlet.strike + leg.strike,

            pierce: helm.pierce + chest.pierce + gauntlet.pierce + leg.pierce,

            magic: helm.magic + chest.magic + gauntlet.magic + leg.magic,

            fire: helm.fire + chest.fire + gauntlet.fire + leg.fire,

            lightning: helm.lightning + chest.lightning + gauntlet.lightning + leg.lightning,

            holy: helm.holy + chest.holy + gauntlet.holy + leg.holy,

            immunity: helm.immunity + chest.immunity + gauntlet.immunity + leg.immunity,

            robustness: helm.robustness + chest.robustness + gauntlet.robustness + leg.robustness,

            focus: helm.focus + chest.focus + gauntlet.focus + leg.focus,

            vitality: helm.vitality + chest.vitality + gauntlet.vitality + leg.vitality,

            poise: helm.poise + chest.poise + gauntlet.poise + leg.poise,

            weight: helm.weight + chest.weight + gauntlet.weight + leg.weight,

            maximize_stat: helm.maximize_stat
                + chest.maximize_stat
                + gauntlet.maximize_stat
                + leg.maximize_stat,
        }
    }
}

impl std::fmt::Display for ArmorSet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "\
            chest:      {}\n\
            helm:       {}\n\
            gauntlets:  {}\n\
            legs:       {}\n\
            weight:     {}\n\
            physical:   {}\n\
            slash:      {}\n\
            strike:     {}\n\
            pierce:     {}\n\
            magic:      {}\n\
            fire:       {}\n\
            lightning:  {}\n\
            holy:       {}\n\
            immunity:   {}\n\
            robustness: {}\n\
            focus:      {}\n\
            vitality:   {}\n\
            poise:      {}\n",
            self.chest,
            self.helm,
            self.gauntlets,
            self.legs,
            f32::from(self.weight) / 10.0,
            f32::from(self.physical) / 10.0,
            f32::from(self.slash) / 10.0,
            f32::from(self.strike) / 10.0,
            f32::from(self.pierce) / 10.0,
            f32::from(self.magic) / 10.0,
            f32::from(self.fire) / 10.0,
            f32::from(self.lightning) / 10.0,
            f32::from(self.holy) / 10.0,
            f32::from(self.immunity) / 10.0,
            f32::from(self.robustness) / 10.0,
            f32::from(self.focus) / 10.0,
            f32::from(self.vitality) / 10.0,
            f32::from(self.poise) / 10.0,
        )
    }
}

#[must_use]
pub fn get_set(weight_restriction: u16, pieces: Vec<ArmorPiece>) -> ArmorSet {
    let (helms, rest): (Vec<ArmorPiece>, Vec<ArmorPiece>) =
        pieces.into_iter().partition(|x| x.slot == Slot::Helm);
    let (chests, rest): (Vec<ArmorPiece>, Vec<ArmorPiece>) =
        rest.into_iter().partition(|x| x.slot == Slot::Chest);
    let (gauntlets, legs): (Vec<ArmorPiece>, Vec<ArmorPiece>) =
        rest.into_iter().partition(|x| x.slot == Slot::Gauntlets);

    let mut result = ArmorSet::new();
    let mut potential_set: ArmorSet;
    println!("Finding the best set...");
    for helm in helms {
        for chest in &chests {
            for gauntlet in &gauntlets {
                for leg in &legs {
                    // Don't allocate an ArmorSet if this gear is too heavy.
                    if helm.weight + chest.weight + gauntlet.weight + leg.weight
                        > weight_restriction
                    {
                        continue;
                    }
                    // Don't allocate an ArmorSet if maximize stat is not greater than or equal.
                    if helm.maximize_stat
                        + chest.maximize_stat
                        + gauntlet.maximize_stat
                        + leg.maximize_stat
                        < result.maximize_stat
                    {
                        continue;
                    }
                    potential_set =
                        ArmorSet::from(helm.clone(), chest.clone(), gauntlet.clone(), leg.clone());
                    if potential_set.maximize_stat == result.maximize_stat {
                        // Choose the one with the lower weight
                        if potential_set.weight < result.weight {
                            result = potential_set.clone();
                        }
                    } else if potential_set.maximize_stat > result.maximize_stat {
                        // Strictly better, ignore weight.
                        result = potential_set.clone();
                    }
                }
            }
        }
    }
    result
}

#[must_use]
pub fn get_pieces_from_text(slot: &Slot, text: &str, maximize_stat: usize) -> Vec<ArmorPiece> {
    let mut pieces: Vec<ArmorPiece> = Vec::new();
    let soup = Soup::new(text);
    let table = soup.tag("table").find().expect("no table");
    let tbody = table.tag("tbody").find().expect("no tbody");
    for row in tbody.tag("tr").find_all() {
        let mut piece = ArmorPiece::new(Slot::Empty);
        piece.slot = slot.clone();
        for (i, td) in row.tag("td").find_all().enumerate() {
            if i > 14 {
                break;
            }

            let text = td.text().trim().to_string();
            let mut num: u16 = 0;
            if i != 0 {
                let mut parse_result: f32;
                parse_result = if let Ok(value) = text.parse::<f32>() {
                    value
                } else {
                    // println!("Couldn't convert {} to f32, skipping {}", text, piece.name);
                    continue;
                };

                if parse_result < 0.0 {
                    // Ignore scarabs or other pieces with negative values.
                    // println!("parse_result was {parse_result}, expected non-zero positive! {}", piece.name);
                    continue;
                }
                parse_result *= 10.0;
                // There is only ever have one significant digit,
                // so truncating is fine here.
                num = parse_result as u16;
            }

            match i {
                0 => {
                    piece.name = td.text().trim().to_lowercase().to_string();
                }
                1 => {
                    piece.physical = num;
                }
                2 => {
                    piece.strike = num;
                }
                3 => {
                    piece.slash = num;
                }
                4 => {
                    piece.pierce = num;
                }
                5 => {
                    piece.magic = num;
                }
                6 => {
                    piece.fire = num;
                }
                7 => {
                    piece.lightning = num;
                }
                8 => {
                    piece.holy = num;
                }
                9 => {
                    piece.immunity = num;
                }
                10 => {
                    piece.robustness = num;
                }
                11 => {
                    piece.focus = num;
                }
                12 => {
                    piece.vitality = num;
                }
                13 => {
                    piece.poise = num;
                }
                14 => {
                    piece.weight = num;
                }
                _ => {}
            }
        }
        match maximize_stat {
            0 => {
                piece.maximize_stat = piece.physical;
            }
            1 => {
                piece.maximize_stat = piece.strike;
            }
            2 => {
                piece.maximize_stat = piece.slash;
            }
            3 => {
                piece.maximize_stat = piece.pierce;
            }
            4 => {
                piece.maximize_stat = piece.magic;
            }
            5 => {
                piece.maximize_stat = piece.fire;
            }
            6 => {
                piece.maximize_stat = piece.lightning;
            }
            7 => {
                piece.maximize_stat = piece.holy;
            }
            8 => {
                piece.maximize_stat = piece.immunity;
            }
            9 => {
                piece.maximize_stat = piece.robustness;
            }
            10 => {
                piece.maximize_stat = piece.focus;
            }
            11 => {
                piece.maximize_stat = piece.vitality;
            }
            12 => {
                piece.maximize_stat = piece.poise;
            }
            _ => {}
        }
        pieces.push(piece);
    }
    pieces
}

#[must_use]
pub fn load_from_file(slot: &Slot, xdg_dirs: &xdg::BaseDirectories) -> Option<String> {
    let file = xdg_dirs.get_cache_file(format!("{slot}.html"));
    if Path::exists(&file) {
        match std::fs::read_to_string(file) {
            Ok(something) => {
                return Some(something);
            }
            _ => {
                return None;
            }
        }
    }
    None
}

#[must_use]
pub fn save_to_file(slot: &Slot, xdg_dirs: &xdg::BaseDirectories, text: &String) -> bool {
    let Ok(path) = xdg_dirs.place_cache_file(format!("{slot}.html")) else {
        return false
    };
    let Ok(mut file) = File::create(path) else {
        return false
    };
    matches!(write!(file, "{text}"), Ok(_))
}

#[must_use]
pub fn load_from_web(
    slot: &Slot,
    urls: &HashMap<String, String>,
    client: &reqwest::blocking::Client,
) -> Option<String> {
    let slot_string = slot.to_string();
    let url = urls.get(&slot_string).expect("couldn't find url");

    let request = client.get(url).send();

    let Ok(resp) = request else { return None };
    let Ok(body) = resp.text() else { return None};
    Some(body)
}

#[must_use]
pub fn get_text_or_die(
    slot: &Slot,
    xdg_dirs: &xdg::BaseDirectories,
    urls: &HashMap<String, String>,
    client: &reqwest::blocking::Client,
) -> String {
    let text = load_from_file(slot, xdg_dirs).map_or_else(
        || {
            println!("Fetching {slot} data from web");
            load_from_web(slot, urls, client).map_or_else(
                || {
                    println!("could not read {slot} data from web");
                    std::process::exit(1);
                },
                |file| file,
            )
        },
        |file| file,
    );
    if xdg_dirs.get_cache_file(format!("{slot}.html")).exists() {
        return text;
    }
    if save_to_file(slot, xdg_dirs, &text) {
        println!("successfully cached {slot}.html");
    } else {
        println!("could not save {slot}.html");
        std::process::exit(1);
    }
    text
}

#[must_use]
pub fn get_pieces(maximize_stat: usize) -> Vec<ArmorPiece> {
    let Ok(xdg_dirs) = xdg::BaseDirectories::with_prefix("elden_boc") else {
        println!("could not get xdg directories!");
        std::process::exit(1);
    };
    if xdg_dirs.create_cache_directory("").is_err() {
        println!("unable to create cache directory");
        std::process::exit(1);
    };
    let mut urls: HashMap<String, String> = HashMap::new();
    urls.insert(
        "Helm".to_string(),
        "https://eldenring.wiki.fextralife.com/Helms".to_string(),
    );
    urls.insert(
        "Chest".to_string(),
        "https://eldenring.wiki.fextralife.com/Chest+Armor".to_string(),
    );
    urls.insert(
        "Gauntlets".to_string(),
        "https://eldenring.wiki.fextralife.com/Gauntlets".to_string(),
    );
    urls.insert(
        "Legs".to_string(),
        "https://eldenring.wiki.fextralife.com/Leg+Armor".to_string(),
    );
    let mut pieces: Vec<ArmorPiece> = Vec::new();
    let slots: Vec<Slot> = vec![Slot::Helm, Slot::Chest, Slot::Gauntlets, Slot::Legs];
    let client = reqwest::blocking::Client::new();
    for slot in slots {
        let text: String = get_text_or_die(&slot, &xdg_dirs, &urls, &client);
        let slot_pieces: Vec<ArmorPiece> = get_pieces_from_text(&slot, &text, maximize_stat);
        pieces.extend(slot_pieces);
        let empty_piece = ArmorPiece::new(slot);
        pieces.push(empty_piece);
    }
    pieces
}
