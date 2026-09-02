// ACVP binary-protocol wrapper around the mlkem-edu library.
// Speaks the same stdin/stdout framing as the C++ modulewrapper so that
// acvp-cli's Subprocess layer can drive it transparently.
//
// Frame format (little-endian u32 throughout):
//   Request:  [num_args][cmd_len][arg1_len]...[cmd_bytes][arg1_bytes]...
//   Response: [num_results][res1_len]...[res1_bytes]...

use mlkem_ref::{check_dk, check_ek, ml_kem_decaps, ml_kem_encaps, ml_kem_keygen, MLKEMParameters};
use std::io::{self, Read, Write};

fn read_u32(r: &mut impl Read) -> io::Result<u32> {
    let mut buf = [0u8; 4];
    r.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

fn write_u32(w: &mut impl Write, v: u32) -> io::Result<()> {
    w.write_all(&v.to_le_bytes())
}

fn read_frame(r: &mut impl Read) -> io::Result<Vec<Vec<u8>>> {
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

fn write_frame(w: &mut impl Write, results: &[Vec<u8>]) -> io::Result<()> {
    write_u32(w, results.len() as u32)?;
    for r in results {
        write_u32(w, r.len() as u32)?;
    }
    for r in results {
        w.write_all(r)?;
    }
    w.flush()
}

fn params(name: &[u8]) -> Result<MLKEMParameters, String> {
    let s = std::str::from_utf8(name).map_err(|e| e.to_string())?;
    MLKEMParameters::new(s).map_err(|e| e.to_string())
}

fn dispatch(args: &[Vec<u8>]) -> Result<Vec<Vec<u8>>, String> {
    let cmd = std::str::from_utf8(args.first().ok_or("empty frame")?).map_err(|e| e.to_string())?;

    match cmd {
        "getConfig" => Ok(vec![
            br#"[{"algorithm":"ML-KEM","revision":"FIPS203","mode":"keyGen"},{"algorithm":"ML-KEM","revision":"FIPS203","mode":"encapDecap"}]"#
                .to_vec(),
        ]),

        // args: [cmd, param_set, seed(64 bytes = z‖d)]
        // Returns: [ek, dk]
        "ML-KEM/keyGen" => {
            let p = params(&args[1])?;
            let seed = &args[2];
            if seed.len() != 64 {
                return Err(format!("keyGen seed must be 64 bytes, got {}", seed.len()));
            }
            // ACVP seed layout: z (bytes 0..32) ‖ d (bytes 32..64)
            // ml_kem_keygen signature: ml_kem_keygen(d, z, ...)
            let z: &[u8; 32] = seed[0..32].try_into().unwrap();
            let d: &[u8; 32] = seed[32..64].try_into().unwrap();
            let mut ek = vec![0u8; p.public_key_length];
            let mut dk = vec![0u8; p.secret_key_length];
            ml_kem_keygen(d, z, &p, &mut ek, &mut dk);
            Ok(vec![ek, dk])
        }

        // args: [cmd, param_set, ek, m(32 bytes)]
        // Returns: [c, k]
        "ML-KEM/encaps" => {
            let p = params(&args[1])?;
            let ek = &args[2];
            let m: &[u8; 32] = args[3]
                .as_slice()
                .try_into()
                .map_err(|_| format!("encaps m must be 32 bytes, got {}", args[3].len()))?;
            let mut k = [0u8; 32];
            let mut c = vec![0u8; p.ciphertext_length];
            ml_kem_encaps(ek, m, &p, &mut k, &mut c);
            Ok(vec![c, k.to_vec()])
        }

        // args: [cmd, param_set, dk, ct]
        // Returns: [k]
        "ML-KEM/decaps" => {
            let p = params(&args[1])?;
            let dk = &args[2];
            let ct = &args[3];
            let mut k = [0u8; 32];
            ml_kem_decaps(dk, ct, &p, &mut k);
            Ok(vec![k.to_vec()])
        }

        // args: [cmd, param_set, ek]
        // Returns: [0x01] if valid, [0x00] otherwise
        "ML-KEM/encapsulationKeyCheck" => {
            let p = params(&args[1])?;
            Ok(vec![if check_ek(&args[2], &p) {
                vec![0x01]
            } else {
                vec![0x00]
            }])
        }

        // args: [cmd, param_set, dk]
        // Returns: [0x01] if valid, [0x00] otherwise
        "ML-KEM/decapsulationKeyCheck" => {
            let p = params(&args[1])?;
            Ok(vec![if check_dk(&args[2], &p) {
                vec![0x01]
            } else {
                vec![0x00]
            }])
        }

        _ => Err(format!("unknown command: {cmd}")),
    }
}

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();

    loop {
        let args = match read_frame(&mut stdin) {
            Ok(a) => a,
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
            Err(e) => {
                eprintln!("mlkem_wrapper: read error: {e}");
                break;
            }
        };

        match dispatch(&args) {
            Ok(results) => {
                if let Err(e) = write_frame(&mut stdout, &results) {
                    eprintln!("mlkem_wrapper: write error: {e}");
                    break;
                }
            }
            Err(e) => {
                eprintln!("mlkem_wrapper: handler error: {e}");
                break;
            }
        }
    }
}
