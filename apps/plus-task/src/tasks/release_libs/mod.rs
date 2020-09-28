mod task;
use task::Params;
use task::Task;

use crate::TaskResult;
use clap::{App, Arg, ArgMatches, SubCommand};
use clap_extractor::Matcher;
use clap_task::ClapTask;

pub fn clap() -> Box<dyn ClapTask<TaskResult<()>>> {
    Box::new(Task)
}

#[async_trait]
impl ClapTask<TaskResult<()>> for Task {
    fn name(&self) -> &str {
        "release-libs"
    }

    fn design(&self) -> App {
        SubCommand::with_name(self.name())
            .about("Release libraries.")
            .long_about("Run cargo publish, git tag, git push, etc")
            .arg(
                Arg::with_name("files")
                    .long("files")
                    .required(true)
                    .takes_value(true)
                    .help("All added and modified files."),
            )
            .arg(
                Arg::with_name("dry-run")
                    .long("dry-run")
                    .help("Perform 'cargo publish --dry-run'"),
            )
    }

    async fn run<'a>(&'a self, matches: &'a ArgMatches<'a>) -> TaskResult<()> {
        use crate::core::support::release::PackageName as Name;

        let params = Params {
            files: matches.single("files").as_required()?,
            target_packages: vec![Name::EnvExtractor],
        };
        if matches.is_present("dry-run") {
            self.release_dry_run(&params)
        } else {
            self.release(&params)
        }
    }
}