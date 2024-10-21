use entity::Entity;
use grid::Grid;
use wasm_bindgen::prelude::wasm_bindgen;

pub mod entity;
pub mod grid;

#[wasm_bindgen]
pub struct GameSession {
    grid: Grid<5>,
    moved_to: Option<Entity>,
    state: Option<bool>
}

#[wasm_bindgen]
impl GameSession {
    pub fn new(bats: u32, pits: u32) -> GameSession {
        Self { grid: Grid::generate(bats, pits), moved_to: None, state: None }
    }

    pub fn perform_action(&mut self, action: Action, direction: Direction) {

        let desired_direction = self.grid.get_coords_to(direction);

        if let Some(action_direction) = desired_direction {
            match action {
                Action::Move => {
                    self.moved_to = self.grid.move_to(action_direction);
                    self.respond_to_movement()
                },
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
}

impl GameSession {
    fn respond_to_movement(&mut self) {
        if let Some(moved_to) = self.moved_to {
            match moved_to {
                Entity::Wumpus | Entity::BottomlessPit => self.state = Some(false),
                Entity::BigBat => todo!(),
                _ => {}
            }
        }

        self.moved_to = None
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
    West
}
