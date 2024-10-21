use std::{collections::HashMap, fmt::Display};

use rand::{seq::SliceRandom, thread_rng, Rng};

use super::{
    entity::{CardinalDirections, Entity},
    Direction,
};

pub type Coordinate = (u8, u8);

pub struct Grid<const N: u8> {
    entities: HashMap<Coordinate, Entity>,
    player: Coordinate,
    wumpus: Coordinate,
    arrows: u8,
}

impl<const N: u8> Grid<N> {
    pub fn generate(bats: u8, pits: u8) -> Self {
        let mut rng = rand::thread_rng();
        let mut entities = HashMap::new();

        for x in 0..N {
            for y in 0..N {
                entities.insert((x, y), Entity::Empty);
            }
        }

        let player = (rng.gen_range(0..N), rng.gen_range(0..N));
        entities.insert(player, Entity::Player);

        let mut valid_spots: Vec<_> = entities
            .keys()
            .filter(|location| *location != &player)
            .cloned()
            .collect();
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

        Self {
            entities,
            player,
            arrows: 5,
            wumpus: wumpus_pos,
        }
    }

    pub fn cur_pos(&self) -> &Coordinate {
        &self.player
    }

    pub fn get_coords_to(&self, direction: Direction) -> Option<Coordinate> {
        let (x, y) = self.player;

        match direction {
            Direction::North => {
                if y == 0 {
                    None
                } else {
                    Some((x, y - 1))
                }
            }
            Direction::South => {
                if y == N - 1 {
                    None
                } else {
                    Some((x, y + 1))
                }
            }
            Direction::East => {
                if x == N - 1 {
                    None
                } else {
                    Some((x + 1, y))
                }
            }
            Direction::West => {
                if x == 0 {
                    None
                } else {
                    Some((x - 1, y))
                }
            }
        }
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

        CardinalDirections([north, east, south, west])
    }

    pub fn move_to(&mut self, new_pos: Coordinate) -> Option<Entity> {
        self.entities.insert(self.player, Entity::Empty);

        self.entities.insert(new_pos, Entity::Player)
    }

    pub fn shoot_at(&mut self, shooting: Coordinate) -> bool {
        self.dec();
        if self.arrows == 0 {
            return false;
        }

        if let Entity::Wumpus = self.entities[&shooting] {
            self.entities.insert(shooting, Entity::Empty);
            true
        } else {
            self.entities.insert(self.wumpus, Entity::Empty);

            let valid_spots: Vec<_> = self
                .entities
                .iter()
                .filter(|(_, location)| *location == &Entity::Empty)
                .map(|(pos, _)| pos)
                .cloned()
                .collect();

            let mut rng = thread_rng();

            let new_wumpus = valid_spots[rng.gen_range(0..valid_spots.len())];
            self.wumpus = new_wumpus;

            self.entities.insert(self.wumpus, Entity::Wumpus);

            false
        }
    }

    fn dec(&mut self) {
        if self.arrows > 0 {
            self.arrows -= 1
        }
    }
}

impl<const N: u8> Display for Grid<N> {
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

#[cfg(test)]
mod tests {
    use crate::game::entity::Entity;

    use super::Grid;

    #[test]
    fn test_format() {
        let grid = Grid::<5>::generate(2, 2);

        println!("{}", grid)
    }

    #[test]
    fn arrows_run_out_properly() {
        let mut grid: Grid<5> = Grid::generate(0, 0);
        // Clear the wumpus so we always miss
        grid.entities.insert(grid.wumpus, Entity::Empty);
        let mut arrows_shot = 0;
        while grid.arrows > 0 {
            grid.shoot_at((0, 0));
            arrows_shot += 1
        }

        assert_eq!(arrows_shot, 5)
    }

    #[test]
    fn shooting_a_wumpus_wins() {
        let mut grid: Grid<5> = Grid::generate(0, 0);
        assert!(grid.shoot_at(grid.wumpus))
    }
}
