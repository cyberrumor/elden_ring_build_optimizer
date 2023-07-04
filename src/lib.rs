pub mod core;
use lazy_static::lazy_static;

#[cfg(test)]
pub mod tests {
    use super::lazy_static;
    use crate::core;

    const MAXIMIZE_STAT: usize = 0;
    const WEIGHT_RESTRICTION: u16 = 60;
    lazy_static! {
        static ref PIECES: Vec<core::ArmorPiece> = core::get_pieces(MAXIMIZE_STAT);
    }

    #[test]
    fn test_attribtue_numbers_helm() {
        let mut cleanrot_helm: core::ArmorPiece = core::ArmorPiece::new(core::Slot::Empty);
        for piece in &*PIECES {
            if piece.name == String::from("cleanrot helm") {
                cleanrot_helm = piece.clone();
                break;
            }
        }
        let expected = core::ArmorPiece {
            // Cleanrot Helm has a unique number for each stat.
            name: String::from("cleanrot helm"),
            slot: core::Slot::Helm,
            physical: 52,
            strike: 48,
            slash: 58,
            pierce: 63,
            magic: 45,
            fire: 46,
            lightning: 40,
            holy: 48,
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
            if piece.name == String::from("cleanrot armor") {
                cleanrot_armor = piece.clone();
                break;
            }
        }
        let expected = core::ArmorPiece {
            name: String::from("cleanrot armor"),
            slot: core::Slot::Chest,
            physical: 146,
            strike: 134,
            slash: 160,
            pierce: 175,
            magic: 126,
            fire: 128,
            lightning: 114,
            holy: 135,
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
            if piece.name == String::from("cleanrot gauntlets") {
                cleanrot_gauntlets = piece.clone();
                break;
            }
        }
        let expected = core::ArmorPiece {
            name: String::from("cleanrot gauntlets"),
            slot: core::Slot::Gauntlets,
            physical: 36,
            strike: 33,
            slash: 40,
            pierce: 44,
            magic: 31,
            fire: 32,
            lightning: 28,
            holy: 33,
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
            if piece.name == String::from("cleanrot greaves") {
                cleanrot_greaves = piece.clone();
                break;
            }
        }
        let expected = core::ArmorPiece {
            name: String::from("cleanrot greaves"),
            slot: core::Slot::Legs,
            physical: 84,
            strike: 76,
            slash: 92,
            pierce: 101,
            magic: 72,
            fire: 73,
            lightning: 65,
            holy: 77,
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
            if piece.name == String::from("cleanrot helm") {
                cleanrot_helm = piece.clone();
            } else if piece.name == String::from("cleanrot armor") {
                cleanrot_armor = piece.clone();
            } else if piece.name == String::from("cleanrot gauntlets") {
                cleanrot_gauntlets = piece.clone();
            } else if piece.name == String::from("cleanrot greaves") {
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
            helm: String::from("cleanrot helm"),
            chest: String::from("cleanrot armor"),
            gauntlets: String::from("cleanrot gauntlets"),
            legs: String::from("cleanrot greaves"),

            physical: 318,
            strike: 291,
            slash: 350,
            pierce: 383,
            magic: 274,
            fire: 279,
            lightning: 247,
            holy: 293,
            poise: 580,
            weight: 357,
            maximize_stat: 318,
        };

        assert_eq!(cleanrot_set, expected);
    }

    #[test]
    fn test_best_set() {
        let expected: core::ArmorSet = core::ArmorSet {
            helm: String::from(""),
            chest: String::from("cloth garb"),
            gauntlets: String::from("gold bracelets"),
            legs: String::from("old aristocrat shoes"),
            physical: 97,
            slash: 63,
            strike: 82,
            pierce: 77,
            magic: 202,
            fire: 202,
            lightning: 193,
            holy: 190,
            poise: 80,
            weight: 60,
            maximize_stat: 97,
        };
        let result = core::get_set(WEIGHT_RESTRICTION, (&*PIECES.clone()).to_vec());
        println!("{}", result);
        assert_eq!(result, expected);
    }
}
