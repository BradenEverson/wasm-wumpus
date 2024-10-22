use entity::Entity;
use grid::Grid;
use rand::{thread_rng, Rng};
use wasm_bindgen::prelude::wasm_bindgen;

pub mod entity;
pub mod grid;

#[wasm_bindgen]
pub struct GameSession {
    grid: Grid<5>,
    moved_to: Option<Entity>,
    state: Option<bool>,
    carried: bool,
}

#[wasm_bindgen]
impl GameSession {
    pub fn new(bats: u16, pits: u16, arrows: u8) -> Option<GameSession> {
        Some(Self {
            grid: Grid::generate(bats, pits, arrows)?,
            moved_to: None,
            state: None,
            carried: false,
        })
    }

    pub fn perform_action(&mut self, action: Action, direction: Direction) {
        let desired_direction = self.grid.get_coords_to(direction);

        if let Some(action_direction) = desired_direction {
            match action {
                Action::Move => {
                    self.moved_to = self.grid.move_to(action_direction);
                    self.respond_to_movement()
                }
                Action::Shoot => {
                    if self.grid.shoot_at(action_direction) {
                        self.state = Some(true)
                    }
                }
            }
        }
    }

    pub fn get_state(&self) -> Option<bool> {
        self.state
    }

    pub fn render(&self) -> String {
        self.grid.to_string()
    }

    pub fn get_curr_room(&self) -> Option<Entity> {
        self.moved_to
    }

    pub fn arrows_left(&self) -> u8 {
        self.grid.arrows_left()
    }

    pub fn was_carried(&mut self) -> bool {
        let carried = self.carried;
        self.carried = false;
        carried
    }

    pub fn get_status_messages(&self) -> Vec<String> {
        let mut res = vec![];
        let nearby = self.grid.look_around().nearby_rooms();

        for room in nearby {
            let room_msg = match room {
                Entity::BigBat => Some("You hear whooshing nearby...".to_string()),
                Entity::BottomlessPit => Some("You feel a faint breeze.".to_string()),
                Entity::Wumpus => Some("You smell a Wumpus a lurkin'".to_string()),
                _ => None,
            };

            if let Some(msg) = room_msg {
                res.push(msg)
            }
        }

        res
    }
}

impl GameSession {
    fn respond_to_movement(&mut self) {
        if let Some(moved_to) = self.moved_to {
            match moved_to {
                Entity::Wumpus | Entity::BottomlessPit => self.state = Some(false),
                Entity::BigBat => {
                    let mut rng = thread_rng();
                    let x = rng.gen_range(0..5);
                    let y = rng.gen_range(0..5);
                    self.moved_to = self.grid.move_to((x, y));
                    self.carried = true;
                },
                _ => {}
            }
        }
    }
}

#[wasm_bindgen]
pub enum Action {
    Move,
    Shoot,
}

#[wasm_bindgen]
pub enum Direction {
    North,
    South,
    East,
    West,
}
