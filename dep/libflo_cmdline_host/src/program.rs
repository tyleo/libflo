use error::*;
use libflo_std::{ Libflo, LIBFLO };
use std::io::Stdin;
use std::sync::Arc;

pub struct Program {
    stdin: Stdin,
    libflo: Libflo
}

impl Program {
    pub fn new(stdin: Stdin, libflo: Libflo) -> Self {
        Program { stdin: stdin, libflo: libflo }
    }

    fn destructure(self) -> (Stdin, Libflo) {
        (self.stdin, self.libflo)
    }

    pub unsafe fn run(self) -> Result<()> {
        let (stdin, libflo) = self.destructure();
        let libflo = Arc::new(libflo);
        LIBFLO.set(libflo.clone())?;
        libflo.construct(&libflo)?;

        while libflo.is_running()? {
            let mut line = String::new();
            stdin.read_line(&mut line)
                .map_err(|err| Error::from(ErrorKind::IoError(err)))
                .chain_err(|| ErrorKind::ErrorReadingInputStream)?;

            let result = libflo.receive(&line);
            if let Err(err) = result {
                let mut msg = "Error:\n".to_string();
                for err in err.iter() {
                    msg = format!("{}Next Error: {}\n", msg, err);
                }
                msg = format!("{}Backtrace: {:?}\n", msg, err.backtrace());
                libflo.errln(&msg)?;
            }
        }

        Ok(())
    }
}
