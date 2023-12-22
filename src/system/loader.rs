use builder_macro::object_struct;

use crate::system::{Ignite, MergeFn, Rolling, TickFn};

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
            let mut loader: FxHashMap<&'static str, crate::system::loader::SystemMeta> = FxHashMap::default();

            $(
                loader.insert(
                    $p::NAME,
                    crate::system::loader::SystemBuilder::new(
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
