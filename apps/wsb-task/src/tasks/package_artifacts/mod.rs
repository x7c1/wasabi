mod task;
use task::Task;

use crate::tasks::shared;
use crate::tasks::shared::build_target;
use crate::{TaskOutput, TaskResult};
use clap::{App, ArgMatches, SubCommand};
use clap_task::ClapTask;

pub fn define() -> Box<dyn ClapTask<TaskResult<TaskOutput>>> {
    Box::new(Task)
}

#[async_trait]
impl ClapTask<TaskResult<TaskOutput>> for Task {
    fn name(&self) -> &str {
        "package-artifacts"
    }

    fn design(&self) -> App {
        SubCommand::with_name(self.name())
            .about("Compress and archive artifacts.")
            .arg(build_target::arg())
    }

    async fn run<'a>(&'a self, matches: &'a ArgMatches<'a>) -> TaskResult<TaskOutput> {
        let params = shared::Params::from_matches(matches)?;
        self.start(&params)?;
        Ok(TaskOutput::empty())
    }
}
