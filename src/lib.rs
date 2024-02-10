pub mod core;

#[cfg(test)]
pub mod tests {
    use super::core;
    use lazy_static::lazy_static;

    const MAXIMIZE_STAT: usize = 0;
    const WEIGHT_RESTRICTION: u16 = 60;
    lazy_static! {
        static ref PIECES: Vec<core::ArmorPiece> = core::get_pieces(MAXIMIZE_STAT);
    }

    fn str_to_bytes(value: &str) -> [u8; core::MAX_NAME_LENGTH] {
        let mut result: [u8; core::MAX_NAME_LENGTH] = [0; core::MAX_NAME_LENGTH];
        let bytes = value.as_bytes();
        let length = bytes.len().min(core::MAX_NAME_LENGTH);
        result[..length].copy_from_slice(&bytes[..length]);
        result
    }

    #[test]
    fn test_attribtue_numbers_helm() {
        let mut cleanrot_helm: core::ArmorPiece = core::ArmorPiece::new(core::Slot::Empty);
        for piece in &*PIECES {
            if piece.name == str_to_bytes("cleanrot helm") {
                cleanrot_helm = piece.clone();
                break;
            }
        }
        let expected = core::ArmorPiece {
            // Cleanrot Helm has a unique number for each stat.
            name: str_to_bytes("cleanrot helm"),
            name_length: "cleanrot helm".len(),
            slot: core::Slot::Helm,
            physical: 52,
            slash: 58,
            strike: 48,
            pierce: 63,
            magic: 45,
            fire: 46,
            lightning: 40,
            holy: 48,
            immunity: 270,
            robustness: 290,
            focus: 120,
            vitality: 140,
            poise: 90,
            weight: 64,
            maximize_stat: 52,
        };
        assert_eq!(cleanrot_helm, expected);
    }

    #[test]
    fn test_attribute_numbers_chest() {
        let mut cleanrot_armor: core::ArmorPiece = core::ArmorPiece::new(core::Slot::Empty);
        for piece in &*PIECES {
            if piece.name == str_to_bytes("cleanrot armor") {
                cleanrot_armor = piece.clone();
                break;
            }
        }
        let expected = core::ArmorPiece {
            name: str_to_bytes("cleanrot armor"),
            name_length: "cleanrot armor".len(),
            slot: core::Slot::Chest,
            physical: 146,
            slash: 160,
            strike: 134,
            pierce: 175,
            magic: 126,
            fire: 128,
            lightning: 114,
            holy: 135,
            immunity: 630,
            robustness: 670,
            focus: 280,
            vitality: 320,
            poise: 270,
            weight: 150,
            maximize_stat: 146,
        };
        assert_eq!(cleanrot_armor, expected);
    }

    #[test]
    fn test_attribute_numbers_gauntlet() {
        let mut cleanrot_gauntlets: core::ArmorPiece = core::ArmorPiece::new(core::Slot::Empty);
        for piece in &*PIECES {
            if piece.name == str_to_bytes("cleanrot gauntlets") {
                cleanrot_gauntlets = piece.clone();
                break;
            }
        }
        let expected = core::ArmorPiece {
            name: str_to_bytes("cleanrot gauntlets"),
            name_length: "cleanrot gauntlets".len(),
            slot: core::Slot::Gauntlets,
            physical: 36,
            slash: 40,
            strike: 33,
            pierce: 44,
            magic: 31,
            fire: 32,
            lightning: 28,
            holy: 33,
            immunity: 210,
            robustness: 220,
            focus: 90,
            vitality: 110,
            poise: 60,
            weight: 50,
            maximize_stat: 36,
        };
        assert_eq!(cleanrot_gauntlets, expected);
    }

    #[test]
    fn test_attribute_numbers_greaves() {
        let mut cleanrot_greaves: core::ArmorPiece = core::ArmorPiece::new(core::Slot::Empty);
        for piece in &*PIECES {
            if piece.name == str_to_bytes("cleanrot greaves") {
                cleanrot_greaves = piece.clone();
                break;
            }
        }
        let expected = core::ArmorPiece {
            name: str_to_bytes("cleanrot greaves"),
            name_length: "cleanrot greaves".len(),
            slot: core::Slot::Legs,
            physical: 84,
            slash: 92,
            strike: 76,
            pierce: 101,
            magic: 72,
            fire: 73,
            lightning: 65,
            holy: 77,
            immunity: 390,
            robustness: 410,
            focus: 170,
            vitality: 200,
            poise: 160,
            weight: 93,
            maximize_stat: 84,
        };
        assert_eq!(cleanrot_greaves, expected);
    }

    #[test]
    fn test_attribute_numbers_set() {
        let mut cleanrot_helm: core::ArmorPiece = core::ArmorPiece::new(core::Slot::Empty);
        let mut cleanrot_armor: core::ArmorPiece = core::ArmorPiece::new(core::Slot::Empty);
        let mut cleanrot_gauntlets: core::ArmorPiece = core::ArmorPiece::new(core::Slot::Empty);
        let mut cleanrot_greaves: core::ArmorPiece = core::ArmorPiece::new(core::Slot::Empty);
        for piece in &*PIECES {
            if piece.name == str_to_bytes("cleanrot helm") {
                cleanrot_helm = piece.clone();
            } else if piece.name == str_to_bytes("cleanrot armor") {
                cleanrot_armor = piece.clone();
            } else if piece.name == str_to_bytes("cleanrot gauntlets") {
                cleanrot_gauntlets = piece.clone();
            } else if piece.name == str_to_bytes("cleanrot greaves") {
                cleanrot_greaves = piece.clone();
            } else {
                continue;
            }
        }
        let cleanrot_set: core::ArmorSet = core::ArmorSet::from(
            cleanrot_helm,
            cleanrot_armor,
            cleanrot_gauntlets,
            cleanrot_greaves,
        );

        let expected: core::ArmorSet = core::ArmorSet {
            helm: str_to_bytes("cleanrot helm"),
            helm_length: "cleanrot helm".len(),
            chest: str_to_bytes("cleanrot armor"),
            chest_length: "cleanrot armor".len(),
            gauntlets: str_to_bytes("cleanrot gauntlets"),
            gauntlets_length: "cleanrot gauntlets".len(),
            legs: str_to_bytes("cleanrot greaves"),
            legs_length: "cleanrot greaves".len(),
            physical: 318,
            slash: 350,
            strike: 291,
            pierce: 383,
            magic: 274,
            fire: 279,
            lightning: 247,
            holy: 293,
            immunity: 1500,
            robustness: 1590,
            focus: 660,
            vitality: 770,
            poise: 580,
            weight: 357,
            maximize_stat: 318,
        };

        assert_eq!(cleanrot_set, expected);
    }

    #[test]
    fn test_best_set() {
        let expected: core::ArmorSet = core::ArmorSet {
            helm: str_to_bytes(""),
            helm_length: 0,
            chest: str_to_bytes("cloth garb"),
            chest_length: "cloth garb".len(),
            gauntlets: str_to_bytes("gold bracelets"),
            gauntlets_length: "gold bracelets".len(),
            legs: str_to_bytes("old aristocrat shoes"),
            legs_length: "old aristocrat shoes".len(),
            physical: 97,
            slash: 63,
            strike: 82,
            pierce: 77,
            magic: 202,
            fire: 202,
            lightning: 193,
            holy: 190,
            immunity: 770,
            robustness: 420,
            focus: 970,
            vitality: 1000,
            poise: 80,
            weight: 60,
            maximize_stat: 97,
        };
        let result = core::get_set(WEIGHT_RESTRICTION, (&*PIECES.clone()).to_vec());
        println!("{}", result);
        assert_eq!(result, expected);
    }
}
