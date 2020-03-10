#[macro_use]
extern crate async_trait;

#[macro_use]
extern crate clap;

#[macro_use]
extern crate failure;

pub mod commands;

mod error;
pub use error::Error;
pub use error::Result as TaskResult;

mod output;
mod tasks;
pub use output::TaskOutput;

#[tokio::main]
async fn main() {
    match run().await {
        Ok(output) => {
            println!("{}", output.as_str());
        }
        Err(e) => {
            eprintln!("wsb-task failed: {:#?}", e);
            exit(1);
        }
    }
}

async fn run() -> TaskResult<TaskOutput> {
    let tasks = tasks::define_all();
    init()
        .subcommands(tasks.to_apps())
        .get_matches()
        .run_matched_from(&tasks)
        .await?
}

fn init<'a, 'b>() -> App<'a, 'b> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
}
