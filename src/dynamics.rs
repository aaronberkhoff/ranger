use crate::state::State;

pub mod kepler;

pub trait Dynamic {
    fn propagate(&self, state0: State, dt: f64) -> State;
}
