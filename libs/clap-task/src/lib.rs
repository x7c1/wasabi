#[macro_use]
extern crate failure;

mod error;
pub use error::Error;
pub use error::Result as ClapTaskResult;

use crate::error::Error::SubCommandMissing;
use clap::{App, ArgMatches};
use std::iter::FromIterator;

pub trait ClapTask<T> {
    fn name(&self) -> &str;

    fn design(&self) -> App;

    fn run(&self, matches: &ArgMatches) -> T;
}

pub trait ClapTasks<T> {
    fn to_apps(&self) -> Vec<App>;

    fn sub_matches<'a>(
        &'a self,
        matches: &'a ArgMatches,
    ) -> ClapTaskResult<(&'a Box<dyn ClapTask<T>>, &'a ArgMatches<'a>)>;
}

impl<T> ClapTasks<T> for Vec<Box<dyn ClapTask<T>>> {
    fn to_apps(&self) -> Vec<App> {
        let apps = self.iter().map(|task| task.design());
        Vec::from_iter(apps)
    }

    fn sub_matches<'a>(
        &'a self,
        matches: &'a ArgMatches,
    ) -> ClapTaskResult<(&'a Box<dyn ClapTask<T>>, &'a ArgMatches<'a>)> {
        self.iter()
            .find_map(|x| matches.subcommand_matches(x.name()).map(|m| (x, m)))
            .ok_or_else(|| SubCommandMissing)
    }
}

pub trait TaskRunner<T> {
    fn run_matched_from<U: ClapTasks<T>>(&self, tasks: &U) -> ClapTaskResult<T>;
}

impl<T> TaskRunner<T> for ArgMatches<'_> {
    fn run_matched_from<U: ClapTasks<T>>(&self, tasks: &U) -> ClapTaskResult<T> {
        let (task, sub_matches) = tasks.sub_matches(self)?;
        Ok(task.run(sub_matches))
    }
}