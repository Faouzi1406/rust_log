pub enum Log {
    Error(&'static str),
    Warning(&'static str),
    Print(&'static str),
    Info(&'static str)
}
