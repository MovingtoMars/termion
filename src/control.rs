use std::io::{self, Write};

use Style;
use color::Color;

/// Extension to the `Write` trait.
///
/// This extension to the `Write` trait is capable of producing the correct ANSI escape sequences
/// for given commands, effectively controlling the terminal.
pub trait TermWrite {

    /// Print the CSI (control sequence introducer) followed by a byte string.
    #[inline]
    fn csi(&mut self, b: &[u8]) -> io::Result<usize>;
    /// Print OSC (operating system command) followed by a byte string.
    #[inline]
    fn osc(&mut self, b: &[u8]) -> io::Result<usize>;
    /// Print DSC (device control string) followed by a byte string.
    #[inline]
    fn dsc(&mut self, b: &[u8]) -> io::Result<usize>;


    /// Clear the entire screen.
    #[inline]
    fn clear(&mut self) -> io::Result<usize> {
        self.csi(b"2J")
    }

    /// Clear everything _after_ the cursor.
    #[inline]
    fn clear_after(&mut self) -> io::Result<usize> {
        self.csi(b"J")
    }

    /// Clear everything _before_ the cursor.
    #[inline]
    fn clear_before(&mut self) -> io::Result<usize> {
        self.csi(b"1J")
    }

    /// Clear the current line.
    #[inline]
    fn clear_line(&mut self) -> io::Result<usize> {
        self.csi(b"2K")
    }

    /// Clear from the cursor until newline.
    #[inline]
    fn clear_until_newline(&mut self) -> io::Result<usize> {
        self.csi(b"K")
    }

    /// Show the cursor.
    #[inline]
    fn show_cursor(&mut self) -> io::Result<usize> {
        self.csi(b"?25h")
    }

    /// Hide the cursor.
    #[inline]
    fn hide_cursor(&mut self) -> io::Result<usize> {
        self.csi(b"?25l")
    }

    /// Move the cursor `num` spaces to the left.
    #[inline]
    fn move_cursor_left(&mut self, num: u32) -> io::Result<usize> {
        if num > 0 {
            self.csi(&[b'0' + (num / 10000) as u8,
                       b'0' + (num / 1000) as u8 % 10,
                       b'0' + (num / 100) as u8 % 10,
                       b'0' + (num / 10) as u8 % 10,
                       b'0' + num as u8 % 10,
                       b'D'])
        } else {
            Ok(0)
        }
    }
    /// Move the cursor `num` spaces to the right.
    #[inline]
    fn move_cursor_right(&mut self, num: u32) -> io::Result<usize> {
        if num > 0 {
            self.csi(&[b'0' + (num / 10000) as u8,
                       b'0' + (num / 1000) as u8 % 10,
                       b'0' + (num / 100) as u8 % 10,
                       b'0' + (num / 10) as u8 % 10,
                       b'0' + num as u8 % 10,
                       b'C'])
        } else {
            Ok(0)
        }
    }

    /// Move the cursor `num` spaces up.
    #[inline]
    fn move_cursor_up(&mut self, num: u32) -> io::Result<usize> {
        if num > 0 {
            self.csi(&[b'0' + (num / 10000) as u8,
                       b'0' + (num / 1000) as u8 % 10,
                       b'0' + (num / 100) as u8 % 10,
                       b'0' + (num / 10) as u8 % 10,
                       b'0' + num as u8 % 10,
                       b'A'])
        } else {
            Ok(0)
        }
    }

    /// Move the cursor `num` spaces down.
    #[inline]
    fn move_cursor_down(&mut self, num: u32) -> io::Result<usize> {
        if num > 0 {
            self.csi(&[b'0' + (num / 10000) as u8,
                       b'0' + (num / 1000) as u8 % 10,
                       b'0' + (num / 100) as u8 % 10,
                       b'0' + (num / 10) as u8 % 10,
                       b'0' + num as u8 % 10,
                       b'B'])
        } else {
            Ok(0)
        }
    }

    /// Scroll the window `num` spaces up.
    #[inline]
    fn scroll_up(&mut self, num: u32) -> io::Result<usize> {
        if num > 0 {
            self.csi(&[b'0' + (num / 10000) as u8,
                       b'0' + (num / 1000) as u8 % 10,
                       b'0' + (num / 100) as u8 % 10,
                       b'0' + (num / 10) as u8 % 10,
                       b'0' + num as u8 % 10,
                       b'S'])
        } else {
            Ok(0)
        }
    }

    /// Scroll the window `num` spaces down.
    #[inline]
    fn scroll_down(&mut self, num: u32) -> io::Result<usize> {
        if num > 0 {
            self.csi(&[b'0' + (num / 10000) as u8,
                       b'0' + (num / 1000) as u8 % 10,
                       b'0' + (num / 100) as u8 % 10,
                       b'0' + (num / 10) as u8 % 10,
                       b'0' + num as u8 % 10,
                       b'T'])
        } else {
            Ok(0)
        }
    }

    /// Reset the rendition mode.
    ///
    /// This will reset both the current style and color.
    #[inline]
    fn reset(&mut self) -> io::Result<usize> {
        self.csi(b"m")
    }

    /// Restore the defaults.
    ///
    /// This will reset color, position, cursor state, and so on. It is recommended that you use
    /// this before you exit your program, to avoid messing up the user's terminal.
    #[inline]
    fn restore(&mut self) -> io::Result<usize> {
        Ok(try!(self.reset()) + try!(self.clear()) + try!(self.goto(0, 0)) + try!(self.show_cursor()))
    }

    /// Go to a given position.
    ///
    /// The position is 0-based.
    #[inline]
    fn goto(&mut self, mut x: u16, mut y: u16) -> io::Result<usize> {
        x += 1;
        y += 1;

        self.csi(&[
            b'0' + (y / 10000) as u8,
            b'0' + (y / 1000) as u8 % 10,
            b'0' + (y / 100) as u8 % 10,
            b'0' + (y / 10) as u8 % 10,
            b'0' + y as u8 % 10,
            b';',
            b'0' + (x / 10000) as u8,
            b'0' + (x / 1000) as u8 % 10,
            b'0' + (x / 100) as u8 % 10,
            b'0' + (x / 10) as u8 % 10,
            b'0' + x as u8 % 10,
            b'H',
        ])
    }

    /// Set graphic rendition.
    #[inline]
    fn rendition(&mut self, r: u8) -> io::Result<usize> {
        self.csi(&[
            b'0' + r / 100,
            b'0' + r / 10 % 10,
            b'0' + r % 10,
            b'm',
        ])
    }

    /// Set foreground color.
    #[inline]
    fn color<C: Color>(&mut self, color: C) -> io::Result<usize> {
        let ansi = color.to_ansi_val();
        self.csi(&[
            b'3',
            b'8',
            b';',
            b'5',
            b';',
            b'0' + ansi / 100,
            b'0' + ansi / 10 % 10,
            b'0' + ansi % 10,
            b'm',
        ])
    }

    /// Set background color.
    #[inline]
    fn bg_color<C: Color>(&mut self, color: C) -> io::Result<usize> {
        let ansi = color.to_ansi_val();
        self.csi(&[
            b'4',
            b'8',
            b';',
            b'5',
            b';',
            b'0' + ansi / 100,
            b'0' + ansi / 10 % 10,
            b'0' + ansi % 10,
            b'm',
        ])
    }

    /// Set rendition mode (SGR).
    #[inline]
    fn style(&mut self, mode: Style) -> io::Result<usize> {
        self.rendition(mode as u8)
    }
}

impl<W: Write> TermWrite for W {
    #[inline]
    fn csi(&mut self, b: &[u8]) -> io::Result<usize> {
        Ok(try!(self.write(b"\x1B[")) + try!(self.write(b)))
    }

    #[inline]
    fn osc(&mut self, b: &[u8]) -> io::Result<usize> {
        Ok(try!(self.write(b"\x1B]")) + try!(self.write(b)))
    }

    #[inline]
    fn dsc(&mut self, b: &[u8]) -> io::Result<usize> {
        Ok(try!(self.write(b"\x1BP")) + try!(self.write(b)))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_csi() {
        let mut buf = Cursor::new(Vec::new());
        buf.csi(b"bluh").unwrap();

        assert_eq!(buf.get_ref(), b"\x1B[bluh");

        buf.csi(b"blah").unwrap();
        assert_eq!(buf.get_ref(), b"\x1B[bluh\x1B[blah");
    }

    #[test]
    fn test_csi_partial() {
        let mut buf = [0; 3];
        let mut buf = &mut buf[..];
        assert_eq!(buf.csi(b"blu").unwrap(), 3);
        assert_eq!(buf.csi(b"").unwrap(), 0);
        assert_eq!(buf.csi(b"nooooo").unwrap(), 0);
    }

    #[test]
    fn test_osc() {
        let mut buf = Cursor::new(Vec::new());
        buf.osc(b"bluh").unwrap();

        assert_eq!(buf.get_ref(), b"\x1B]bluh");

        buf.osc(b"blah").unwrap();
        assert_eq!(buf.get_ref(), b"\x1B]bluh\x1B]blah");
    }

    #[test]
    fn test_osc_partial() {
        let mut buf = [0; 3];
        let mut buf = &mut buf[..];
        assert_eq!(buf.osc(b"blu").unwrap(), 3);
        assert_eq!(buf.osc(b"").unwrap(), 0);
        assert_eq!(buf.osc(b"nooooo").unwrap(), 0);
    }

    #[test]
    fn test_dsc() {
        let mut buf = Cursor::new(Vec::new());
        buf.dsc(b"bluh").unwrap();

        assert_eq!(buf.get_ref(), b"\x1BPbluh");

        buf.dsc(b"blah").unwrap();
        assert_eq!(buf.get_ref(), b"\x1BPbluh\x1BPblah");
    }

    #[test]
    fn test_dsc_partial() {
        let mut buf = [0; 3];
        let mut buf = &mut buf[..];
        assert_eq!(buf.dsc(b"blu").unwrap(), 3);
        assert_eq!(buf.dsc(b"").unwrap(), 0);
        assert_eq!(buf.dsc(b"nooooo").unwrap(), 0);
    }

    #[test]
    fn test_clear() {
        let mut buf = Cursor::new(Vec::new());
        buf.clear().unwrap();
        assert_eq!(buf.get_ref(), b"\x1B[2J");
        buf.clear().unwrap();
        assert_eq!(buf.get_ref(), b"\x1B[2J\x1B[2J");
    }

    #[test]
    fn test_goto() {
        let mut buf = Cursor::new(Vec::new());
        buf.goto(34, 43).unwrap();
        assert_eq!(buf.get_ref(), b"\x1B[00044;00035H");
        buf.goto(24, 45).unwrap();
        assert_eq!(buf.get_ref(), b"\x1B[00044;00035H\x1B[00046;00025H");
    }

    #[test]
    fn test_style() {
        use Style;

        let mut buf = Cursor::new(Vec::new());
        buf.style(Style::Bold).unwrap();
        assert_eq!(buf.get_ref(), b"\x1B[001m");
        buf.style(Style::Italic).unwrap();
        assert_eq!(buf.get_ref(), b"\x1B[001m\x1B[003m");
    }
}
