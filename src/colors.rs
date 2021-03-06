// Let's not care if the TERM variable is 'dumb'
// since we depend on escape sequences
pub const ALTERNATE_ON: &str = "\x1b[?1049h";
pub const ALTERNATE_OFF: &str = "\x1b[?1049l";
pub const CURSOR_SHOW: &str = "\x1b[?25h";
pub const CURSOR_HIDE: &str = "\x1b[?25l";
pub const CURSOR_TOP_LEFT: &str = "\x1b[;H";
pub const COLOR_RESET: &str = "\x1b[0;0m";

pub const COLOR_RED: &str = "\x1b[0;31m";
pub const COLOR_GREEN: &str = "\x1b[0;32m";
