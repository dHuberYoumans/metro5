use app::commands::HelpCommand;
use app::{app::*, commands::AppCommand, ports::MetroController};
use infra::*;
use metro5::errors::MainError;
use std::path::PathBuf;
use strip_ansi_escapes::strip;
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
                    app.state.set_filter(filter);
                    app.state.apply_filter();
                    app.scroll_state.scroll_to_top();
                }
                AppCommand::ResetFilter => app.state.reset_filter(),
                AppCommand::SetQuery(query) => {
                    app.state.set_query(query);
                    app.state.apply_query();
                    app.scroll_state.scroll_to_top();
                }
                AppCommand::ResetQuery => app.state.reset_query(),
                AppCommand::ClearState => {
                    app.state.clear_state();
                }
                AppCommand::QuitApp => break,
                AppCommand::ShowHelp => app.state.show_help(),
                AppCommand::Scroll(scroll_direction) => app.scroll(scroll_direction),
                AppCommand::HelpMenu(HelpCommand::SelectNext) => app.help_state.select_next(),
                AppCommand::HelpMenu(HelpCommand::SelectPrev) => app.help_state.select_previous(),
                AppCommand::HelpMenu(HelpCommand::ExpandSection) => app.help_state.expand_section(),
                AppCommand::HelpMenu(HelpCommand::CollapseSection) => {
                    app.help_state.collase_section()
                }
                AppCommand::WriteToFile(path) => {
                    app.state.set_path(path.clone());
                    let content: Vec<String> = app
                        .state
                        .logs
                        .iter()
                        .map(|log| {
                            let stripped = strip(log.to_string());
                            String::from_utf8_lossy(&stripped).to_string()
                        })
                        .collect();
                    infra::fs::write_to_file(PathBuf::from(path), content.join("\n"));
                }
                AppCommand::WriteAndQuit => {
                    if let Some(ref path) = app.state.path {
                        let content: Vec<String> = app
                            .state
                            .logs
                            .iter()
                            .map(|log| format!("{}", log))
                            .collect();
                        infra::fs::write_to_file(PathBuf::from(path), content.join("\n"));
                    } else {
                        todo!("implement allert: no path found");
                    }
                }
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
