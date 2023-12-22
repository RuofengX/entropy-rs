use bincode_sled::Tree;
use builder_macro::object_struct;
use paste::paste;
use sled::Db;

use crate::{
    basic::{Value, EID},
    system::{self, Ignite, MergeFn, Rolling, TickFn},
};

object_struct!(SystemBuilder -> SystemMeta{
        name: &'static str,
        ignite: &'static (dyn Ignite + Sync + Send),
        rolling: &'static (dyn Rolling + Sync + Send),
        merge: &'static (dyn MergeFn + Sync + Send),
        tick: &'static (dyn TickFn + Sync + Send),
    }
);

pub(crate) type Loaders = Vec<SystemMeta>;
pub(crate) static LOADERS: Loaders = Loaders::new();

macro_rules! load_system{
    () => {Vec::<SystemBuilder>::new()};
    ( $( $p:ident),* ) => {
        {
            let mut loader: Vec<SystemMeta> = Vec::new();

            $(
                loader.push(
                    SystemBuilder::new(
                        $p::NAME,
                        $p::IGNITE,
                        $p::ROLLING,
                        $p::MERGE,
                        $p::TICK,
                    ).build()
                );
            )*;
            loader
        }

    };
}

use system::_00_nothing;
static LOADER: Vec<SystemMeta> = load_system![_00_nothing];
