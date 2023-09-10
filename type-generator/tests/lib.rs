use assert_cmd::Command;
use assert_fs::TempDir;

fn setup(temp: &TempDir) -> Result<Command, Box<dyn std::error::Error>> {
    let path = temp.path();
    let mut cmd = Command::cargo_bin("lfr-typegen")?;
    cmd.current_dir(path);
    Ok(cmd)
}

static LOAD_SPEC_CMD: &str = "load-spec";
static SOURCE_FLG: &str = "--base-url";

#[cfg(test)]
mod tests {
    use crate::{setup, LOAD_SPEC_CMD, SOURCE_FLG};
    use assert_fs::TempDir;

    #[test]
    fn load_spec_from_url() -> Result<(), Box<dyn std::error::Error>> {
        let temp = TempDir::new().unwrap();
        let mut cmd = setup(&temp)?;

        cmd.arg(LOAD_SPEC_CMD)
            .arg(SOURCE_FLG)
            .arg("http://localhost:8080");
        cmd.assert().success();

        Ok(())
    }

    #[test]
    fn load_spec_from_path() -> Result<(), Box<dyn std::error::Error>> {
        let temp = TempDir::new().unwrap();
        let mut cmd = setup(&temp)?;

        cmd.arg(LOAD_SPEC_CMD)
            .arg(SOURCE_FLG)
            .arg("./assets/spec.json");
        cmd.assert().success();

        Ok(())
    }
}
