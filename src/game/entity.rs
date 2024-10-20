use std::{collections::HashSet, fmt::Display};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Entity {
    Arrow,
    Wumpus,
    BigBat,
    BottomlessPit,
    Player,
    Empty,
}

impl Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char_code = match *self {
            Entity::BigBat => "🦇",
            Entity::BottomlessPit => "🕳️",
            Entity::Arrow => ">",
            Entity::Empty => " ",
            Entity::Wumpus => "👹",
            Entity::Player => "😀",
        };

        write!(f, "{}", char_code)
    }
}
pub struct CardinalDirections<'grid>(pub [Option<&'grid Entity>; 4]);

impl<'grid> CardinalDirections<'grid> {
    pub fn nearby_rooms(&self) -> Vec<Entity> {
        let mut seen = HashSet::new();
        self.0
            .clone()
            .iter()
            .filter_map(|dir| *dir)
            .filter(|dir| seen.insert(*dir) && *dir != &Entity::Empty)
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{CardinalDirections, Entity};

    #[test]
    fn cardinal_direction_behave_properly() {
        let north = Some(&Entity::BottomlessPit);
        let south = Some(&Entity::BottomlessPit);
        let east = None;
        let west = Some(&Entity::Empty);

        let dirs = CardinalDirections([north, south, east, west]);

        assert_eq!(dirs.nearby_rooms().len(), 1)
    }
}

