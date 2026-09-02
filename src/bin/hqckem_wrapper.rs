// ACVP binary-protocol wrapper around the hqckem-ref library.
// See acvp_cli::modulewrapper for the shared stdin/stdout framing.

use acvp_cli::modulewrapper;
use hqc::{decaps, encaps, keygen, HqcParams};
use sha3::{
    digest::{ExtendableOutput, Update, XofReader},
    Shake256,
};

/// Derives a 32-byte seed from arbitrary-length input via SHAKE256.
fn generate_kem_seed(input: &[u8]) -> [u8; 32] {
    let mut h = Shake256::default();
    Update::update(&mut h, input);
    Update::update(&mut h, &[0u8]);
    let mut out = [0u8; 32];
    XofReader::read(&mut h.finalize_xof(), &mut out);
    out
}

fn params(name: &[u8]) -> Result<HqcParams, String> {
    let s = std::str::from_utf8(name).map_err(|e| e.to_string())?;
    HqcParams::new(s).map_err(|e| e.to_string())
}

fn dispatch(args: &[Vec<u8>]) -> Result<Vec<Vec<u8>>, String> {
    let cmd = std::str::from_utf8(args.first().ok_or("empty frame")?).map_err(|e| e.to_string())?;

    match cmd {
        "getConfig" => Ok(vec![
            br#"[{"algorithm":"HQC-KEM","revision":"FIPS207","mode":"keyGen"},{"algorithm":"HQC-KEM","revision":"FIPS207","mode":"encapDecap"}]"#
                .to_vec(),
        ]),

        // args: [cmd, param_set, seed(32 bytes)]
        // Returns: [ek, dk]
        "HQC-KEM/keyGen" => {
            let p = params(&args[1])?;
            let seed = &args[2];
            let mut ek = vec![0u8; p.public_key_size()];
            let mut dk = vec![0u8; p.secret_key_size()];

            if seed.len() != 48 {
                return Err(format!("keyGen seed must be 48 bytes, got {}", seed.len()));
            }

            let s = generate_kem_seed(seed);
            keygen(&p, &s, &mut ek, &mut dk);
            Ok(vec![ek, dk])
        }

        // args: [cmd, param_set, ek, m(32 bytes)]
        // Returns: [c, k]
        "HQC-KEM/encaps" => {
            let p = params(&args[1])?;
            let ek = &args[2];
            let m: &[u8; 32] = args[3]
                .as_slice()
                .try_into()
                .map_err(|_| format!("encaps m must be 32 bytes, got {}", args[3].len()))?;
            let mut k = [0u8; 32];
            let mut c = vec![0u8; p.ciphertext_size()];
            encaps(&p, m, ek, &mut k, &mut c);
            Ok(vec![c, k.to_vec()])
        }

        // args: [cmd, param_set, dk, ct]
        // Returns: [k]
        "HQC-KEM/decaps" => {
            let p = params(&args[1])?;
            let dk = &args[2];
            let ct = &args[3];
            let mut k = [0u8; 32];

            if dk.len() != p.secret_key_size() {
                return Err(format!(
                    "decaps dk must be {} bytes, got {}",
                    p.secret_key_size(),
                    dk.len()
                ));
            }

            if ct.len() != p.ciphertext_size() {
                return Err(format!(
                    "decaps ct must be {} bytes, got {}",
                    p.ciphertext_size(),
                    ct.len()
                ));
            }

            decaps(&p, dk, ct, &mut k);
            Ok(vec![k.to_vec()])
        }
        _ => Err(format!("unknown command: {cmd}")),
    }
}

fn main() {
    modulewrapper::run("hqckem_wrapper", dispatch);
}
