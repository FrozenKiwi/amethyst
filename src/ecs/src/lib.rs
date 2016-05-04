extern crate time;
extern crate specs;

mod sim;

use time::Duration;

pub use specs::{World, Component, System as Processor, RunArg, Entity, EntityBuilder, Entities,
                CreateEntities, VecStorage, HashMapStorage, AntiStorage, NullStorage, JoinIter};
pub use self::sim::{Simulation, SimBuilder};
