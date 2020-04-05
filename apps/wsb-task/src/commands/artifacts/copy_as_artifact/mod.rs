mod params;
pub use params::{Params, ParamsBuilder};

use crate::commands::copy_as_artifact;
use crate::commands::support::Definable;
use crate::TaskResult;
use shellwork::core::command;
use shellwork::core::command::{Runner, Unprepared};

impl Definable for copy_as_artifact::Params<'_> {
    fn define(&self) -> TaskResult<Runner<Unprepared>> {
        let recursive = if self.src.is_dir() {
            Some("--recursive")
        } else {
            None
        };
        let runner = command::program("cp")
            .arg("--no-target-directory")
            .push_arg(recursive)
            .args(&[&self.src, &self.dst]);

        Ok(runner)
    }
}