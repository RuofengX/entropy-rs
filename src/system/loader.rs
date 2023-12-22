use std::sync::OnceLock;

use builder_macro::object_struct;

use crate::system::{Ignite, MergeFn, Rolling, TickFn, _00_nothing, _01_clock};

object_struct!(pub SystemBuilder -> SystemMeta{
        pub name: &'static str,
        pub ignite: &'static (dyn Ignite + Sync + Send),
        pub rolling: &'static (dyn Rolling + Sync + Send),
        pub merge: &'static (dyn MergeFn + Sync + Send),
        pub tick: &'static (dyn TickFn + Sync + Send),
    }
);

#[macro_export]
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
            )*
            loader
        }

    };
}

pub fn system_load() -> super::Loaders {
    let rtn = OnceLock::new();
    rtn.get_or_init(|| {
        load_system![_00_nothing, _01_clock] // TODO: add system here.
    });
    rtn
}
