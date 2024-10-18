use std::{collections::HashMap, fmt::Display};

use rand::{seq::SliceRandom, Rng};

pub type Coordinate = (u32, u32);

pub struct Grid<const N: u32> {
    entities: HashMap<Coordinate, Entity>,
    player: Coordinate
}

impl<const N: u32> Grid<N> {
    pub fn generate(bats: u32, pits: u32) -> Self {
        let mut rng = rand::thread_rng();
        let mut entities = HashMap::new();

        for x in 0..N {
            for y in 0..N {
                entities.insert((x, y), Entity::Empty);
            }
        }

        let player = (rng.gen_range(0..N), rng.gen_range(0..N));
        entities.insert(player, Entity::Player);

        let mut valid_spots: Vec<_> = entities.keys().filter(|location| *location != &player).cloned().collect();
        valid_spots.shuffle(&mut rng);

        for _ in 0..bats {
            // Add bat slots
            let bat_pos = valid_spots.pop().unwrap();
            entities.insert(bat_pos, Entity::BigBat);
        }

        for _ in 0..pits {
            // Add pit spots
            let pit_pos = valid_spots.pop().unwrap();
            entities.insert(pit_pos, Entity::BottomlessPit);
        }

        // Enter the wumpus

        let wumpus_pos = valid_spots.pop().unwrap();
        entities.insert(wumpus_pos, Entity::Wumpus);

        Self { entities, player }
    }

    pub fn cur_pos(&self) -> &Coordinate {
        &self.player
    }

    pub fn current_room(&self) -> &Entity {
        self.entities.get(&self.player).unwrap()
    }

    pub fn look_around(&self) -> CardinalDirections {
        let (x, y) = self.player;
        let north = self.entities.get(&(x, y - 1));
        let east = self.entities.get(&(x + 1, y));
        let south = self.entities.get(&(x, y + 1));
        let west = self.entities.get(&(x - 1, y));

        CardinalDirections { north, east, south, west }
    }
}

impl<const N: u32> Display for Grid<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..N {
            for x in 0..N {
                write!(f, "{}\t", self.entities[&(x, y)])?;
            }
            write!(f, "\n\n")?;
        }
        Ok(())
    }
}

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

pub struct CardinalDirections<'grid> {
    pub north: Option<&'grid Entity>,
    pub east: Option<&'grid Entity>,
    pub south: Option<&'grid Entity>,
    pub west: Option<&'grid Entity>
}

#[cfg(test)]
mod tests {
    use super::Grid;

    #[test]
    fn test_format() {
        let grid = Grid::<5>::generate(2, 2);

        println!("{}", grid)
    }
}
