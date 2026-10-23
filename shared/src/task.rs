use std::process::{Command, Stdio};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Task {
    pub cmd: String,
    pub args: Option<Vec<String>>,
    pub depends: Option<Vec<String>>,
    pub flags: Option<Vec<String>>,
}

impl Task {
    pub fn run_task(&self) {
        let mut cmd = Command::new(&self.cmd);

        if let Some(ref args) = self.args {
            cmd.args(args);
        }

        for flag in &self.flags.clone().unwrap_or(vec![]) {
            if flag == "sudo" {
                cmd = Command::new("sudo");

                cmd.arg(&self.cmd);
                if let Some(ref args) = self.args {
                    cmd.args(args);
                }
            }
        }

        cmd.stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("could not spawn task: {name}");
    }
}