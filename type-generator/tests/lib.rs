use assert_cmd::Command;

fn setup() -> Result<Command, Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new().unwrap();
    let path = temp.path();
    let mut cmd = Command::cargo_bin("lfr-typegen")?;
    cmd.current_dir(path);
    Ok(cmd)
}

#[cfg(test)]
mod tests {

    use crate::setup;

    #[test]
    fn load_spec_from_url() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = setup()?;

        cmd.arg("name").arg("http://localhost:8080");
        cmd.assert().success();

        Ok(())
    }

    #[test]
    fn load_spec_from_path() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = setup()?;

        cmd.arg("name").arg("./assets/spec.json");
        cmd.assert().success();

        Ok(())
    }
}
