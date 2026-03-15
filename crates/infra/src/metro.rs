use async_trait::async_trait;
use std::process::Stdio;
use tokio::{
    io::AsyncWriteExt,
    process::{Child, ChildStderr, ChildStdin, ChildStdout, Command},
};

use crate::errors::InfraError;
use app::{errors::ApplicationError, ports::MetroController};

pub struct MetroGuard {
    pub pid: u32,
}

impl MetroGuard {
    pub fn new(pid: u32) -> Self {
        Self { pid }
    }
}

impl Drop for MetroGuard {
    fn drop(&mut self) {
        let _ = std::process::Command::new("kill")
            .args(["-TERM", &format!("-{}", self.pid)])
            .status();
    }
}

pub struct MetroProcess {
    pub child: Child,
    pub stdin: Option<ChildStdin>,
    pub stdout: Option<ChildStdout>,
    pub stderr: Option<ChildStderr>,
}

impl MetroProcess {
    pub fn spawn() -> Result<Self, InfraError> {
        let mut child = Command::new("npx")
            .args(["react-native", "start", "--client-logs"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .process_group(0)
            .spawn()
            .expect("failed to start metro");
        let stdin = child.stdin.take().unwrap();
        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();
        Ok(Self {
            child,
            stdin: Some(stdin),
            stdout: Some(stdout),
            stderr: Some(stderr),
        })
    }
}

#[async_trait]
impl MetroController for MetroProcess {
    async fn send_command(&mut self, cmd: &str) -> Result<(), ApplicationError> {
        if let Some(stdin) = &mut self.stdin {
            stdin.write_all(cmd.as_bytes()).await?;
            stdin.flush().await?;
        }
        Ok(())
    }

    async fn kill(&mut self) -> Result<(), ApplicationError> {
        if let Some(pid) = self.child.id() {
            std::process::Command::new("kill")
                .args(["-TERM", &format!("-{}", pid)])
                .status()?;
        }
        self.child.wait().await?;
        Ok(())
    }
}
