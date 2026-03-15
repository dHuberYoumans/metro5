use domain::process::{ProcessEvent, RawStream, StreamKind};
use tokio::sync::mpsc::Sender;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::{ChildStderr, ChildStdout},
    task::JoinHandle,
};

pub fn ingest_logs(
    sender: Sender<ProcessEvent>,
    stdout: ChildStdout,
    stderr: ChildStderr,
) -> (JoinHandle<()>, JoinHandle<()>) {
    let handle_stdout = tokio::spawn(read_stdout(sender.clone(), stdout));
    let handle_stderr = tokio::spawn(read_stderr(sender.clone(), stderr));
    (handle_stdout, handle_stderr)
}

async fn read_stdout(sender: Sender<ProcessEvent>, stdout: ChildStdout) {
    let mut lines = BufReader::new(stdout).lines();
    while let Ok(Some(line)) = lines.next_line().await {
        let process_event = ProcessEvent::Stream(RawStream {
            stream: StreamKind::Stdout,
            line,
        });
        if sender.send(process_event).await.is_err() {
            break;
        };
    }
}

async fn read_stderr(sender: Sender<ProcessEvent>, stderr: ChildStderr) {
    let mut lines = BufReader::new(stderr).lines();
    while let Ok(Some(line)) = lines.next_line().await {
        let process_event = ProcessEvent::Stream(RawStream {
            stream: StreamKind::Stderr,
            line,
        });
        if sender.send(process_event).await.is_err() {
            break;
        };
    }
}
