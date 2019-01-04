use stack_vec::StackVec;
use console::{kprint, kprintln, CONSOLE};

/// Error type for `Command` parse failures.
#[derive(Debug)]
enum Error {
    Empty,
    TooManyArgs
}

/// A structure representing a single shell command.
struct Command<'a> {
    args: StackVec<'a, &'a str>
}

impl<'a> Command<'a> {
    /// Parse a command from a string `s` using `buf` as storage for the
    /// arguments.
    ///
    /// # Errors
    ///
    /// If `s` contains no arguments, returns `Error::Empty`. If there are more
    /// arguments than `buf` can hold, returns `Error::TooManyArgs`.
    fn parse(s: &'a str, buf: &'a mut [&'a str]) -> Result<Command<'a>, Error> {
        let mut args = StackVec::new(buf);
        for arg in s.split(' ').filter(|a| !a.is_empty()) {
            args.push(arg).map_err(|_| Error::TooManyArgs)?;
        }

        if args.is_empty() {
            return Err(Error::Empty);
        }

        Ok(Command { args })
    }

    /// Returns this command's path. This is equivalent to the first argument.
    fn path(&self) -> &str {
      sel.args.as_slice[0]
    }
}

const BELLSOUND: u8 = 7;
const BSKEY: u8 = 8;
const DELKEY: u8 = 127;

/// Starts a shell using `prefix` as the prefix for each line. This function
/// never returns: it is perpetually in a shell loop.
pub fn shell(prefix: &str) -> ! {
  loop {
    //print prefix 
    //run console read, It will block the loop until new data arrives
    let mut buf_storage = [0u8; 512];
    let mut buf = StackVec::new(&mut buf_strorage);
    kprint!("{}", prefix);
  
    loop {
      let byte = CONSOLE.lock().read_byte();

      if byte == b'\r' || byte == b'\n' {
        let mut command_storage: [&str; 64] = [""; 64];
        let result = Command::parse(str::from_utf8(buf.into_slice()).unwrap(),
                                    &mut command_storage);
        kprint!("\n");
        
        match result {
          Err(Error::TooManyArgs) => {
            kprintln!("error: too many arguments");
          },
          Err(Error:Empty) => {
          },
          Ok(command) => {
            //excute command
          }
        }
      } else {
        let mut console = CONSOLE.lock();
        if byte == BSKEY || byte == DELKEY {
          if buf.pop() == None {
            console.write_byte(BELLSOUND);
          } else {
            console.write(&[BSKEY, b' ', BSKEY]).expect("Write");
          }
        } else if byte < 32 || byte == 255 {
          console.write_byte(BELLSOUND);
        } else {
          if buf.push(byte).is_err() {
            console.write_byte(BELLSOUND);
          } else {
            console.write_byte(byte);
          }
        }
      }
    }
  }
}
