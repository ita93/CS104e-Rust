use stack_vec::StackVec;
use console::{kprint, kprintln, CONSOLE};

use std::str;
use std::io::Write;

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
      self.args[0]
    }
}

const BELLSOUND: u8 = 7;
const BSKEY: u8 = 8;
const DELKEY: u8 = 127;

const WELCOME: &str = r#"ONI OS"#;

/// Starts a shell using `prefix` as the prefix for each line. This function
/// never returns: it is perpetually in a shell loop.
pub fn shell(prefix: &str) -> ! {  
  kprintln!("{}", WELCOME);
  loop {
    //print prefix 
    //run console read, It will block the loop until new data arrives
    let mut buf_storage = [0u8; 512];
    let mut buf = StackVec::new(&mut buf_storage);
    kprint!("{}", prefix);

    //Put in aloop
    
    /* Read input byte to byte and push read byte to stack.
    * If input byte is DEL, we pop one from our stack.
    * If input byte is new line character, we parse command and excute it.
    */
    loop {
      let byte = CONSOLE.lock().read_byte();

      if byte == b'\r' || byte == b'\n' {
        let mut command_storage: [&str; 64] = [""; 64];
        let result = Command::parse(str::from_utf8(buf.into_slice()).unwrap(),
                                    &mut command_storage);
        CONSOLE.lock().write_byte(b'\n');
        CONSOLE.lock().write_byte(b'\r');
        
        match result {
          Err(Error::TooManyArgs) => {
            CONSOLE.lock().write(b"error: too many arguments\n");
          },
          Err(Error::Empty) => {
          }
          Ok(command) => {
            excute(&command);
          }
        }
        break
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

fn excute(cmd: &Command) {
  match cmd.args[0] {
    "echo" => {
      kprint!("This is an echo command\n");
    }
    _ => {
      kprint!("error: command not found\n");
    }
  }
}