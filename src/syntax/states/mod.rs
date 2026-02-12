pub mod normal;

pub use super::*;

impl PDA {
    pub fn build(&mut self) {
        self.add_normal_states();
    }
}