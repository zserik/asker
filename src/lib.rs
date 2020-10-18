use bitflags::bitflags;
use std::io::{self, Write};

bitflags! {
    pub struct Flags: u8 {
        /// retry if the user pressed return immediately
        const RETRY_EMPTY_LINE = 0b00000001;

        /// assume that we have a TTY
        /// (normally not necessary, as it is autodetected)
        const HAVE_TTY = 0b00000010;
    }
}

pub struct Asker {
    xin: io::Stdin,
    xout: io::Stdout,
    buf: String,
    flags: Flags,
}

impl Asker {
    pub fn new(mut flags: Flags) -> Self {
        use atty::Stream as TtyS;
        if !flags.contains(Flags::HAVE_TTY) && atty::is(TtyS::Stdin) && atty::is(TtyS::Stdout) {
            flags |= Flags::HAVE_TTY;
        }
        Self {
            xin: io::stdin(),
            xout: io::stdout(),
            buf: String::new(),
            flags,
        }
    }

    pub fn ask(&mut self, prompt: &str) -> io::Result<&mut String> {
        let retry_empty_line = self.flags.contains(Flags::RETRY_EMPTY_LINE);
        let have_tty = self.flags.contains(Flags::HAVE_TTY);

        loop {
            if have_tty {
                write!(&mut self.xout, "{}", prompt)?;
                self.xout.flush()?;
                self.buf.clear();
                self.xin.read_line(&mut self.buf)?;
            }
            self.xin.read_line(&mut self.buf)?;
            if !retry_empty_line || !self.buf.is_empty() {
                break Ok(&mut self.buf);
            }
        }
    }
}
