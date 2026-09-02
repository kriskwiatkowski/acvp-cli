use anyhow::{Context, Result};
use clap::Parser;
use log::{error, info};
use std::path::{Path, PathBuf};

mod acvp;
mod config;
mod subprocess;
mod utils;

use crate::acvp::ACVPClient;
use crate::config::Config;
use crate::subprocess::Subprocess;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Print module capabilities JSON to stdout
    #[arg(long)]
    regcap: bool,

    /// Location of the configuration JSON file
    #[arg(short, long, default_value = "config.json")]
    config: PathBuf,

    /// Location of a vector-set input file
    #[arg(long)]
    r#in: Option<PathBuf>,

    /// Location of the response file
    #[arg(long)]
    out: Option<PathBuf>,

    /// Directory with request vectors (requires -outdir)
    #[arg(long)]
    indir: Option<PathBuf>,

    /// Directory for storing response files (requires -indir)
    #[arg(long)]
    outdir: Option<PathBuf>,

    /// Name of primitive to run tests for
    #[arg(long)]
    run: Option<String>,

    /// Name of primitive to fetch vectors for
    #[arg(long)]
    fetch: Option<String>,

    /// Path to the wrapper binary
    #[arg(short, long)]
    wrapper: PathBuf,

    /// Optional parameter that's passed to the wrapper
    #[arg(long)]
    param: Option<String>,

    /// Path to expected results JSON for verification (used with --in/--out)
    #[arg(long)]
    expected: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();

    // Handle regcap flag
    if args.regcap {
        let mut subprocess = Subprocess::new(&args.wrapper, args.param.as_deref())?;
        let regcap = subprocess.get_config()?;
        println!("{}", serde_json::to_string_pretty(&regcap)?);
        return Ok(());
    }

    // Handle file-based vector processing
    if let (Some(input), Some(output)) = (&args.r#in, &args.out) {
        process_vectors_from_file(
            &args.wrapper,
            args.param.as_deref(),
            input,
            output,
            args.expected.as_deref(),
        )?;
        return Ok(());
    }

    // Handle directory-based processing
    if let (Some(indir), Some(outdir)) = (&args.indir, &args.outdir) {
        process_vectors_from_directory(&args.wrapper, args.param.as_deref(), indir, outdir)?;
        return Ok(());
    }

    // Handle interactive mode with ACVP server
    if args.run.is_some() || args.fetch.is_some() {
        let config = Config::from_file(&args.config)?;
        let mut client = ACVPClient::new(config, &args.wrapper, args.param.as_deref()).await?;

        if let Some(primitive) = args.fetch {
            client.fetch_vectors(&primitive).await?;
        }

        if let Some(primitive) = args.run {
            client.run_tests(&primitive).await?;
        }

        return Ok(());
    }

    error!("No operation specified. Use --regcap, --in/--out, --indir/--outdir, or --run/--fetch");
    std::process::exit(1);
}

fn process_vectors_from_file(
    wrapper_path: &Path,
    param: Option<&str>,
    input: &PathBuf,
    output: &PathBuf,
    expected_path: Option<&Path>,
) -> Result<()> {
    info!("Processing vectors from file: {:?}", input);

    let input_data = if input.extension().and_then(|s| s.to_str()) == Some("zip") {
        utils::read_vectors_from_zip(input)?
    } else {
        std::fs::read_to_string(input).context("Failed to read input file")?
    };

    let test_vectors: serde_json::Value =
        serde_json::from_str(&input_data).context("Failed to parse input JSON")?;

    let mut subprocess = Subprocess::new(wrapper_path, param)?;
    let responses = subprocess.process_vectors(&test_vectors)?;

    let output_json = serde_json::to_string_pretty(&responses)?;
    std::fs::write(output, &output_json).context("Failed to write output file")?;
    info!("Responses written to: {:?}", output);

    if let Some(path) = expected_path {
        let expected_data =
            std::fs::read_to_string(path).context("Failed to read expected results file")?;
        let expected: serde_json::Value =
            serde_json::from_str(&expected_data).context("Failed to parse expected JSON")?;
        check_expected(&responses, &expected)?;
        println!("PASS");
    }

    Ok(())
}

fn check_expected(actual: &serde_json::Value, expected: &serde_json::Value) -> Result<()> {
    // Actual may be a single response object or an array of them (one per vsId).
    // Expected results only carry testGroups; we compare group-by-group.
    let actuals = match actual {
        serde_json::Value::Array(arr) => arr.clone(),
        obj => vec![obj.clone()],
    };
    let expecteds = match expected {
        serde_json::Value::Array(arr) => arr.clone(),
        obj => vec![obj.clone()],
    };

    let mut total = 0usize;
    let mut failures = 0usize;

    for (act, exp) in actuals.iter().zip(expecteds.iter()) {
        let act_groups = act["testGroups"]
            .as_array()
            .context("actual response missing testGroups")?;
        let exp_groups = exp["testGroups"]
            .as_array()
            .context("expected results missing testGroups")?;

        for exp_group in exp_groups {
            let tg_id = exp_group["tgId"]
                .as_u64()
                .context("expected group missing tgId")?;

            let act_group = act_groups
                .iter()
                .find(|g| g["tgId"].as_u64() == Some(tg_id))
                .with_context(|| format!("actual response missing tgId {tg_id}"))?;

            let exp_tests = exp_group["tests"]
                .as_array()
                .with_context(|| format!("expected group {tg_id} missing tests"))?;
            let act_tests = act_group["tests"]
                .as_array()
                .with_context(|| format!("actual group {tg_id} missing tests"))?;

            for exp_test in exp_tests {
                let tc_id = exp_test["tcId"]
                    .as_u64()
                    .with_context(|| format!("expected test in group {tg_id} missing tcId"))?;

                let act_test = act_tests
                    .iter()
                    .find(|t| t["tcId"].as_u64() == Some(tc_id))
                    .with_context(|| {
                        format!("actual response missing tcId {tc_id} in group {tg_id}")
                    })?;

                if let Some(fields) = exp_test.as_object() {
                    for (key, exp_val) in fields {
                        if key == "tcId" {
                            continue;
                        }
                        total += 1;
                        let act_val = &act_test[key];
                        let matches = match (exp_val, act_val) {
                            (serde_json::Value::String(e), serde_json::Value::String(a)) => {
                                e.to_lowercase() == a.to_lowercase()
                            }
                            _ => exp_val == act_val,
                        };
                        if !matches {
                            eprintln!(
                                "FAIL tgId={tg_id} tcId={tc_id} field={key}: \
                                 expected={exp_val} actual={act_val}"
                            );
                            failures += 1;
                        }
                    }
                }
            }
        }
    }

    if failures > 0 {
        anyhow::bail!("{failures}/{total} field(s) did not match");
    }
    info!("{total} field(s) verified");
    Ok(())
}

fn process_vectors_from_directory(
    wrapper_path: &Path,
    param: Option<&str>,
    indir: &PathBuf,
    outdir: &PathBuf,
) -> Result<()> {
    info!("Processing vectors from directory: {:?}", indir);

    std::fs::create_dir_all(outdir).context("Failed to create output directory")?;

    let mut subprocess = Subprocess::new(wrapper_path, param)?;

    for entry in std::fs::read_dir(indir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file()
            && path
                .extension()
                .and_then(|s| s.to_str())
                .is_some_and(|ext| ext == "json" || ext == "zip")
        {
            let input_data = if path.extension().and_then(|s| s.to_str()) == Some("zip") {
                utils::read_vectors_from_zip(&path)?
            } else {
                std::fs::read_to_string(&path)?
            };

            let test_vectors: serde_json::Value = serde_json::from_str(&input_data)?;
            let responses = subprocess.process_vectors(&test_vectors)?;

            let output_path = outdir.join(
                path.file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .replace(".zip", ".json"),
            );

            std::fs::write(&output_path, serde_json::to_string_pretty(&responses)?)?;
            info!("Processed: {:?} -> {:?}", path, output_path);
        }
    }

    Ok(())
}
