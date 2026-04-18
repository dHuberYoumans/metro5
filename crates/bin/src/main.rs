use app::{app::*, commands::AppCommand, ports::MetroController};
use infra::*;
use metro5::errors::MainError;
use tokio::process::{ChildStderr, ChildStdout};
use tokio::sync::mpsc;
use tui::Tui;

#[tokio::main]
async fn main() -> Result<(), MainError> {
    // term cleanup on panic
    tui::panic_hook_init();
    let mut metro = MetroProcess::spawn()?;
    // process cleanup on panic
    let _guard = MetroGuard::new(metro.child.id().expect("no pid found"));
    let stdout = metro.stdout.take().unwrap();
    let stderr = metro.stderr.take().unwrap();
    let mut controller: Box<dyn MetroController> = Box::new(metro);
    run(stdout, stderr, &mut controller).await?;
    Ok(())
}

async fn run(
    stdout: ChildStdout,
    stderr: ChildStderr,
    controller: &mut Box<dyn MetroController>,
) -> Result<(), MainError> {
    let mut app = App::new();
    let mut tui = Tui::init()?;
    let (tx, mut rx) = mpsc::channel(1000);
    let (logs_stdout_handle, logs_stderr_handle) = ingest_logs(tx.clone(), stdout, stderr);
    let keys_handle = ingest_keys(tx.clone());
    while let Some(event) = rx.recv().await {
        if let Some(cmd) = app.handle_process_event(event)? {
            match cmd {
                AppCommand::SendToMetro(cmd) => controller.send_command(&cmd).await?,
                AppCommand::SetFilter(filter) => {
                    app.set_filter(filter);
                    app.apply_filter();
                    app.scroll_state.scroll_to_top();
                }
                AppCommand::ResetFilter => app.reset_filter(),
                AppCommand::SetQuery(query) => {
                    app.set_query(query);
                    app.apply_query();
                    app.scroll_state.scroll_to_top();
                }
                AppCommand::ResetQuery => app.reset_query(),
                AppCommand::ClearState => {
                    app.clear_state();
                }
                AppCommand::QuitApp => break,
                AppCommand::ShowHelp => app.show_help(),
                AppCommand::ShowManual => app.show_man(),
                AppCommand::Scroll(scroll_direction) => app.scroll(scroll_direction),
            }
        };
        tui.draw(&mut app)?;
    }
    tui.restore()?;
    logs_stdout_handle.abort();
    logs_stderr_handle.abort();
    keys_handle.abort();
    controller.kill().await?;
    Ok(())
}
