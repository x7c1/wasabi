use crate::s3api::{TEST_BUCKET, TEST_WORKSPACE_DIR};
use std::io;
use std::path::{Path, PathBuf};
use wsb_pilot::cmd::CommandRunner;
use wsb_pilot::PilotResult;

#[test]
fn return_zero_on_succeeded() -> PilotResult<()> {
    let sample = get_sample1();
    let expected = {
        upload(&sample)?;
        read_to_string(&sample.upload_src)?
    };
    let actual = {
        download(&sample)?;
        read_to_string(&sample.download_dst)?
    };
    assert_eq!(actual, expected, "correctly uploaded.");
    Ok({})
}

fn upload(target: &Sample) -> PilotResult<()> {
    wsb_s3api()
        .arg("put-object")
        .args(&["--bucket", &TEST_BUCKET])
        .args(&["--key", &target.object_key])
        .args(&["--body", &target.upload_src.to_string_lossy()])
        .output()?;

    Ok({})
}

fn download(target: &Sample) -> PilotResult<()> {
    aws_s3api()
        .arg("get-object")
        .args(&["--bucket", &TEST_BUCKET])
        .args(&["--key", &target.object_key])
        .arg(&target.download_dst)
        .output()?;

    Ok({})
}

fn read_to_string(path: &Path) -> io::Result<String> {
    let path_str: &str = &path.to_string_lossy();
    let output = cat().arg(path_str).output()?;
    Ok(output.stdout_to_string())
}

#[test]
fn output_e_tag_is_correct() -> PilotResult<()> {
    let sample = get_sample1();
    let run = |runner: CommandRunner| {
        runner
            .arg("put-object")
            .args(&["--bucket", &TEST_BUCKET])
            .args(&["--key", &sample.object_key])
            .args(&["--body", &sample.upload_src.to_string_lossy()])
            .output()
    };
    let aws_json = {
        let output = run(aws_s3api())?;
        output.stdout_to_json()?
    };
    let wsb_json = {
        let output = run(wsb_s3api())?;
        output.stdout_to_json()?
    };
    assert_eq!(wsb_json["ETag"], aws_json["ETag"]);

    Ok({})
}

#[test]
fn return_non_zero_on_failed() -> PilotResult<()> {
    let output = wsb_s3api().arg("unknown-subcommand").execute()?;
    assert_eq!(1, output.status_code(), "return zero if it succeeded.");
    Ok({})
}

lazy_static! {
    static ref WORKSPACE: PathBuf = PathBuf::new()
        .join(&*TEST_WORKSPACE_DIR)
        .join("s3api")
        .join("put-object");
}

fn get_sample1() -> Sample {
    Sample {
        object_key: "s3api/put-object/foo/bar/sample.txt".to_string(),
        upload_src: "./sample.txt".into(),
        download_dst: "./downloaded.tmp".into(),
    }
}

fn aws_s3api() -> CommandRunner {
    super::aws_s3api().current_dir(&*WORKSPACE)
}

fn wsb_s3api() -> CommandRunner {
    super::wsb_s3api().current_dir(&*WORKSPACE)
}

fn cat() -> CommandRunner {
    super::cat().current_dir(&*WORKSPACE)
}

struct Sample {
    object_key: String,
    upload_src: PathBuf,
    download_dst: PathBuf,
}
