// All coloring functionality is implemented here.
use ansi_term::Color;
use std::{
    io::IsTerminal,
    sync::atomic::{AtomicBool, Ordering},
};

pub fn color_setup_win10() {
    // Fix color issue for windows 10
    #[cfg(windows)]
    ansi_term::enable_ansi_support().ok();
}

pub struct ColorOn(Color);
impl ColorOn {
    pub fn cyan() -> ColorOn {
        ColorOn(Color::Cyan)
    }
    pub fn yellow() -> ColorOn {
        ColorOn(Color::Yellow)
    }
}
impl std::fmt::Display for ColorOn {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        if is_stdout_and_color_enabled() {
            write!(fmt, "{}", self.0.prefix())?;
        }
        Ok(())
    }
}

pub struct ColorOff;
impl std::fmt::Display for ColorOff {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        if is_stdout_and_color_enabled() {
            write!(fmt, "{}", Color::White.suffix())?;
        }
        Ok(())
    }
}

pub struct StdOutColored<S>(Color, S);
impl<S: std::fmt::Display> StdOutColored<S> {
    pub fn cyan(s: S) -> StdOutColored<S> {
        StdOutColored(Color::Cyan, s)
    }
    pub fn green(s: S) -> StdOutColored<S> {
        StdOutColored(Color::Green, s)
    }
    pub fn red(s: S) -> StdOutColored<S> {
        StdOutColored(Color::Red, s)
    }
    pub fn yellow(s: S) -> StdOutColored<S> {
        StdOutColored(Color::Yellow, s)
    }
}
impl<S: std::fmt::Display> std::fmt::Display for StdOutColored<S> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        if is_stdout_and_color_enabled() {
            write!(fmt, "{}", self.0.prefix())?;
            write!(fmt, "{}", self.1)?;
            write!(fmt, "{}", self.0.suffix())?;
        } else {
            write!(fmt, "{}", self.1)?;
        }
        Ok(())
    }
}
pub struct StdOutColoredDebug<S>(Color, S);
impl<S: std::fmt::Debug> StdOutColoredDebug<S> {
    pub fn red(s: S) -> StdOutColoredDebug<S> {
        StdOutColoredDebug(Color::Red, s)
    }
}
impl<S: std::fmt::Debug> std::fmt::Debug for StdOutColoredDebug<S> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        if is_stdout_and_color_enabled() {
            write!(fmt, "{}", self.0.prefix())?;
            write!(fmt, "{:?}", self.1)?;
            write!(fmt, "{}", self.0.suffix())?;
        } else {
            write!(fmt, "{:?}", self.1)?;
        }
        Ok(())
    }
}

pub struct StdErrColored<S>(Color, S);
impl<S: std::fmt::Display> StdErrColored<S> {
    pub fn red(s: S) -> StdErrColored<S> {
        StdErrColored(Color::Red, s)
    }
    pub fn yellow(s: S) -> StdErrColored<S> {
        StdErrColored(Color::Yellow, s)
    }
}
impl<S: std::fmt::Display> std::fmt::Display for StdErrColored<S> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        if is_stderr_and_color_enabled() {
            write!(fmt, "{}", self.0.prefix())?;
            write!(fmt, "{}", self.1)?;
            write!(fmt, "{}", self.0.suffix())?;
        } else {
            write!(fmt, "{}", self.1)?;
        }
        Ok(())
    }
}

static OUT_COLORED: AtomicBool = AtomicBool::new(true);
static ERR_COLORED: AtomicBool = AtomicBool::new(true);

fn is_stdout_and_color_enabled() -> bool {
    OUT_COLORED.load(Ordering::Relaxed)
}
fn is_stderr_and_color_enabled() -> bool {
    ERR_COLORED.load(Ordering::Relaxed)
}
pub fn set_colors_active(b: bool) {
    OUT_COLORED.store(b && std::io::stdout().is_terminal(), Ordering::Relaxed);
    ERR_COLORED.store(b && std::io::stderr().is_terminal(), Ordering::Relaxed);
}
