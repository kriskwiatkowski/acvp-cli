//! Binary framing shared by the ACVP modulewrapper binaries (mlkem_wrapper,
//! hqckem_wrapper, ...). Speaks the same stdin/stdout framing as the C++
//! modulewrapper so that acvp-cli's Subprocess layer can drive it transparently.
//!
//! Frame format (little-endian u32 throughout):
//!   Request:  [num_args][cmd_len][arg1_len]...[cmd_bytes][arg1_bytes]...
//!   Response: [num_results][res1_len]...[res1_bytes]...

use std::io::{self, Read, Write};

pub fn read_u32(r: &mut impl Read) -> io::Result<u32> {
    let mut buf = [0u8; 4];
    r.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

pub fn write_u32(w: &mut impl Write, v: u32) -> io::Result<()> {
    w.write_all(&v.to_le_bytes())
}

pub fn read_frame(r: &mut impl Read) -> io::Result<Vec<Vec<u8>>> {
    let num_args = read_u32(r)? as usize;
    let mut lengths = Vec::with_capacity(num_args);
    for _ in 0..num_args {
        lengths.push(read_u32(r)? as usize);
    }
    let mut args = Vec::with_capacity(num_args);
    for len in lengths {
        let mut buf = vec![0u8; len];
        r.read_exact(&mut buf)?;
        args.push(buf);
    }
    Ok(args)
}

pub fn write_frame(w: &mut impl Write, results: &[Vec<u8>]) -> io::Result<()> {
    write_u32(w, results.len() as u32)?;
    for r in results {
        write_u32(w, r.len() as u32)?;
    }
    for r in results {
        w.write_all(r)?;
    }
    w.flush()
}

/// Runs the read-dispatch-write loop until stdin is closed.
///
/// `name` is used to prefix error messages (e.g. "hqckem_wrapper").
pub fn run(name: &str, dispatch: impl Fn(&[Vec<u8>]) -> Result<Vec<Vec<u8>>, String>) {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();

    loop {
        let args = match read_frame(&mut stdin) {
            Ok(a) => a,
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
            Err(e) => {
                eprintln!("{name}: read error: {e}");
                break;
            }
        };

        match dispatch(&args) {
            Ok(results) => {
                if let Err(e) = write_frame(&mut stdout, &results) {
                    eprintln!("{name}: write error: {e}");
                    break;
                }
            }
            Err(e) => {
                eprintln!("{name}: handler error: {e}");
                break;
            }
        }
    }
}
