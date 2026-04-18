use crossterm::event::EventStream;
use crossterm::event::KeyModifiers;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use domain::process::{Key, ProcessEvent};
use futures::StreamExt;
use tokio::{sync::mpsc::Sender, task::JoinHandle};

pub fn ingest_keys(sender: Sender<ProcessEvent>) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut stream = EventStream::new();
        while let Some(Ok(Event::Key(key))) = stream.next().await {
            let event = resolve_key(key);
            if let Some(event) = event
                && sender.send(event).await.is_err()
            {
                break;
            }
        }
    })
}

fn resolve_key(key: KeyEvent) -> Option<ProcessEvent> {
    if key.kind != KeyEventKind::Press {
        return None;
    };
    let key = match key.code {
        KeyCode::Char(c) if key.modifiers.contains(KeyModifiers::CONTROL) => match c {
            'c' => Key::CtrlC,
            'd' => Key::CtrlD,
            'u' => Key::CtrlU,
            _ => Key::Char(c),
        },
        KeyCode::Char(c) => Key::Char(c),
        KeyCode::Backspace => Key::Backspace,
        KeyCode::Enter => Key::Enter,
        KeyCode::Esc => Key::Esc,
        _ => return None,
    };
    Some(ProcessEvent::Key(key))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::KeyModifiers;

    #[test]
    fn key_event_must_be_press() {
        let key = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE);
        let result = resolve_key(KeyEvent {
            kind: KeyEventKind::Release,
            ..key
        });
        assert_eq!(result, None);
        let result = resolve_key(KeyEvent {
            kind: KeyEventKind::Repeat,
            ..key
        });
        assert_eq!(result, None);
    }

    #[test]
    fn detects_enter_pressed() {
        let key = KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE);
        let result = resolve_key(KeyEvent {
            kind: KeyEventKind::Press,
            ..key
        });
        assert_eq!(result, Some(ProcessEvent::Key(Key::Enter)));
    }

    #[test]
    fn detects_esc_pressed() {
        let key = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);
        let result = resolve_key(KeyEvent {
            kind: KeyEventKind::Press,
            ..key
        });
        assert_eq!(result, Some(ProcessEvent::Key(Key::Esc)));
    }

    #[test]
    fn detects_backspace_pressed() {
        let key = KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE);
        let result = resolve_key(KeyEvent {
            kind: KeyEventKind::Press,
            ..key
        });
        assert_eq!(result, Some(ProcessEvent::Key(Key::Backspace)));
    }

    #[test]
    fn detects_char_pressed() {
        let char = 'c';
        let key = KeyEvent::new(KeyCode::Char(char), KeyModifiers::NONE);
        let result = resolve_key(KeyEvent {
            kind: KeyEventKind::Press,
            ..key
        });
        assert_eq!(result, Some(ProcessEvent::Key(Key::Char(char))));
    }
}
