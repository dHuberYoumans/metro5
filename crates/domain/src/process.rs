#[derive(Debug, PartialEq)]
pub enum StreamKind {
    Stderr,
    Stdout,
}

#[derive(Debug, PartialEq)]
pub struct RawStream {
    pub stream: StreamKind,
    pub line: String,
}

#[derive(Debug, PartialEq)]
pub enum ProcessEvent {
    Key(Key),
    Stream(RawStream),
}

#[derive(Debug, PartialEq)]
pub enum Key {
    Char(char),
    Backspace,
    Enter,
    Esc,
    CtrlC,
    CtrlU,
    CtrlD,
}
