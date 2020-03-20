mod cargo;

pub use cargo::build_apps;
pub use cargo::build_pilot;
use clap::ArgMatches;
use std::marker::PhantomData;

pub mod support;

pub struct Action<TARGET, PARAMS>(PhantomData<(TARGET, PARAMS)>);

impl<T, P> Action<T, P> {
    pub fn from<F>(target: T, matches: &ArgMatches, to_params: F) -> (Action<T, P>, P)
    where
        F: Fn(T, &ArgMatches) -> P,
    {
        let params = to_params(target, matches);
        (Action(PhantomData), params)
    }
}
