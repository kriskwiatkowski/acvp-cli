// ACVP binary-protocol wrapper around the mlkem-ref library.
// See acvp_cli::modulewrapper for the shared stdin/stdout framing.

use acvp_cli::modulewrapper;
use mlkem_ref::{check_dk, check_ek, ml_kem_decaps, ml_kem_encaps, ml_kem_keygen, MLKEMParameters};

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
    modulewrapper::run("mlkem_wrapper", dispatch);
}
