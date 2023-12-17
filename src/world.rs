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
use std::{
    cell::OnceCell,
    sync::{atomic::AtomicBool, Arc},
};

use config::{Config, File, FileFormat};

use retable::{basic::PropTag, Database, Prop};
use rustc_hash::FxHashMap;

use crate::wheel::Wheels;

/// A hash map of the props of the world.
///
/// Each props is created by [`retable::db::Database::create_prop`] and stored as [`std::sync::Arc`] in this hash map.
///
pub type Systems = FxHashMap<PropTag, Arc<Prop>>;

/// The flat that the world is end.
pub type AbortFlag = Arc<AtomicBool>;

/// The static meta info of one world.
///
/// Meta is protected by a OnceCell right after world created.
///
/// # Properties
/// - `config`: The World config items, using crate `config::Config`.
/// - `db`: The database of the world.
/// - `wheels`: The functions that run forever when the world is running, just like daemon thread. See more in [`crate::wheel::Wheels`].
struct WorldMeta {
    pub config: Config,
    pub db: Database,
    pub wheels: Wheels,
}

/// World container.
///
/// # Private Field
/// - `systems`: A hash map of the props of the world. Each props is created by [`retable::api::AtomStorage::create_prop`] and stored as [`std::sync::Arc`] in this hash map.
///
pub struct World {
    meta: OnceCell<WorldMeta>,
    systems: Systems,
}

impl World {
    pub fn new() -> Result<Self, Error> {
        // read config
        let config = World::read_config()?;

        // build meta
        let meta = WorldMeta {
            config: config.clone(),
            db: Database::new(
                retable::Config::new()
                    .path(config.get_string("database.sled.path")?)
                    .temporary(config.get_bool("database.sled.temporary")?),
            )?,
            wheels: Wheels::default(),
        };
        // TODO: Get systems from the database.

        // start the wheels.

        Ok(World {
            meta: OnceCell::from(meta), // freeze the meta.
            systems: Systems::default(),
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
