//! Define a virtual world.
//!
//!
//! # Overture
//!
//! One man cannot build a world.
//!
//! A world could consider as 'exist' when there are at lease 2 different entities.
//! The first entity is always the empty container, in this case the first entity is the struct [`World`].
//!
//! When the first entity is created, the second entity does not exist. So it cannot be called a 'World'.
//! So if the author of this framework(RuofengX) created half of the world, another half is your(Lib User) responsibility.
//!
use std::sync::{atomic::AtomicBool, Arc};

use config::{Config, File, FileFormat};

use retable::{api::PropStorage, basic::PropTag, Database};
use rustc_hash::FxHashMap;

/// A hash map of the props of the world.
///
/// Each props is created by [`retable::api::AtomStorage::create_prop`] and stored as [`std::sync::Arc`] in this hash map.
///
pub type Systems = FxHashMap<PropTag, Arc<dyn PropStorage>>;

/// A function that runs forever when the world is running.
///
/// The wheel function is run right before the world starts to tick.
/// Nothing stop the wheel from running forever, unless the world is shutting down, in which case
///
/// # Arguments
/// - `s`: Reference to the systems.
/// - `stop`: A flag to stop the wheel. Should been checked frequently.
///
/// # Return
/// - `()`: Run forever until the stop flag is set to true.
///
/// # Usage
///
/// * Run an non-stop GraphQL API along with the world.
/// * Receieve and send messages from a zmq ipc socket.
///
pub type Wheel = fn(s: &Systems, stop: &AtomicBool) -> ();

/// World container.
///
/// # Private Field
/// - `db`: The database of the world. Maintained by [`retable::db::Database`]
/// - `systems`: A hash map of the props of the world. Each props is created by [`retable::api::AtomStorage::create_prop`] and stored as [`std::sync::Arc`] in this hash map.
/// - `wheels`: The functions that run forever when the world is running, just like daemon thread. See more in [`Wheel`]
///
pub struct World {
    config: Config,
    db: Database,
    wheels: Vec<Wheel>,
}
impl World {
    pub fn new() -> Result<Self, Error> {
        let config = World::read_config().unwrap();
        Ok(World {
            config: config.clone(),
            db: Database::new(
                retable::Config::new()
                    .path(config.get_string("database.sled.path")?)
                    .temporary(config.get_bool("database.sled.temporary")?),
            )?,
            wheels: vec![],
        })
    }
    pub fn read_config() -> Result<Config, Error> {
        let builder = Config::builder()
            .set_default("database.type", "sled")?
            .set_default("database.sled.temporary", true)?
            .add_source(File::new("config.toml", FileFormat::Toml).required(false));

        builder.build().map_err(Error::from)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ConfigError(#[from] config::ConfigError),
    #[error(transparent)]
    DatabaseError(#[from] retable::error::Error),
}
