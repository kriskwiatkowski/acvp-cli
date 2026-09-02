# Configuration

Configure acvp-cli for your environment.

## Configuration File

acvp-cli uses a JSON configuration file for ACVP server settings.

### Location

Default: `config.json` in the current directory

Specify custom location with `-c` or `--config`:
```bash
acvp-cli --config /path/to/config.json --run SHA2-256
```

### Format

```json
{
  "certPEMFile": "cert.pem",
  "privateKeyDERFile": "key.der",
  "totpSecret": "BASE64_ENCODED_SECRET",
  "acvpServer": "https://demo.acvts.nist.gov/",
  "sessionTokensCache": "session_tokens.json",
  "logFile": "acvp.log"
}
```

### Fields

#### certPEMFile (optional)
Path to client certificate in PEM format.

```json
"certPEMFile": "path/to/cert.pem"
```

#### privateKeyFile (optional)
Path to private key in PEM format.

```json
"privateKeyFile": "path/to/key.pem"
```

#### privateKeyDERFile (optional)
Path to private key in DER format.

```json
"privateKeyDERFile": "path/to/key.der"
```

#### totpSecret (optional)
Base64-encoded TOTP secret for 2FA.

```json
"totpSecret": "JBSWY3DPEHPK3PXP"
```

#### acvpServer (required)
URL of the ACVP server.

```json
"acvpServer": "https://demo.acvts.nist.gov/"
```

Common servers:
- Demo: `https://demo.acvts.nist.gov/`
- Production: `https://acvts.nist.gov/`

#### sessionTokensCache (optional)
File to cache session tokens.

```json
"sessionTokensCache": "session_tokens.json"
```

#### logFile (optional)
Path to log file.

```json
"logFile": "acvp.log"
```

### Comments

JSON comments are supported (lines starting with `//`):

```json
{
  // Server configuration
  "acvpServer": "https://demo.acvts.nist.gov/",
  
  // Authentication
  "certPEMFile": "cert.pem",
  "totpSecret": "SECRET"
}
```

## Command-Line Options

### Required

#### --wrapper

Path to the modulewrapper binary:

```bash
acvp-cli --wrapper /path/to/modulewrapper --regcap
```

### Optional

#### --config, -c

Configuration file location:

```bash
acvp-cli -c /path/to/config.json --wrapper modulewrapper --run SHA2-256
```

Default: `config.json`

#### --param

Optional parameter passed to modulewrapper:

```bash
acvp-cli --wrapper modulewrapper --param "custom-param" --regcap
```

### File Processing

#### --in

Input test vector file (JSON or ZIP):

```bash
acvp-cli --wrapper modulewrapper --in test.json --out response.json
```

#### --out

Output response file:

```bash
acvp-cli --wrapper modulewrapper --in test.json --out response.json
```

### Directory Processing

#### --indir

Input directory containing test vectors:

```bash
acvp-cli --wrapper modulewrapper --indir vectors/ --outdir responses/
```

#### --outdir

Output directory for responses:

```bash
acvp-cli --wrapper modulewrapper --indir vectors/ --outdir responses/
```

Both `--indir` and `--outdir` must be specified together.

### Server Interaction

#### --fetch

Fetch test vectors for a primitive:

```bash
acvp-cli -c config.json --wrapper modulewrapper --fetch SHA2-256
```

#### --run

Run tests for a primitive:

```bash
acvp-cli -c config.json --wrapper modulewrapper --run SHA2-256
```

### Information

#### --regcap

Print module capabilities:

```bash
acvp-cli --wrapper modulewrapper --regcap
```

#### --help, -h

Show help message:

```bash
acvp-cli --help
```

#### --version, -V

Show version:

```bash
acvp-cli --version
```

## Environment Variables

### RUST_LOG

Control logging level:

```bash
RUST_LOG=debug acvp-cli --wrapper modulewrapper --regcap
```

Levels:
- `error` - Errors only
- `warn` - Warnings and errors
- `info` - General information
- `debug` - Detailed debug output
- `trace` - Very verbose output

Example:
```bash
RUST_LOG=info acvp-cli --wrapper modulewrapper --in test.json --out response.json
```

### RUST_BACKTRACE

Enable backtrace on panic:

```bash
RUST_BACKTRACE=1 acvp-cli --wrapper modulewrapper --regcap
```

## Examples

### Minimal Configuration

File-based processing only:

```bash
acvp-cli --wrapper modulewrapper --in test.json --out response.json
```

No configuration file needed!

### Full Configuration

For ACVP server interaction:

**config.json**:
```json
{
  "acvpServer": "https://demo.acvts.nist.gov/",
  "certPEMFile": "client-cert.pem",
  "privateKeyDERFile": "client-key.der",
  "totpSecret": "JBSWY3DPEHPK3PXP",
  "sessionTokensCache": ".session_tokens.json",
  "logFile": "acvp.log"
}
```

**Usage**:
```bash
acvp-cli -c config.json --wrapper modulewrapper --fetch SHA2-256
```

### Debug Configuration

With verbose logging:

```bash
RUST_LOG=debug RUST_BACKTRACE=1 acvp-cli \
  --wrapper modulewrapper \
  --in test.json \
  --out response.json
```

## Configuration Tips

### Security

1. **Protect secrets**: Set appropriate file permissions
   ```bash
   chmod 600 config.json
   chmod 600 *.pem *.der
   ```

2. **Use environment variables** for sensitive data:
   ```bash
   export TOTP_SECRET="..."
   # Reference in config or pass via command line
   ```

3. **Don't commit secrets** to version control:
   ```bash
   echo "config.json" >> .gitignore
   echo "*.pem" >> .gitignore
   echo "*.der" >> .gitignore
   ```

### Performance

1. **Cache session tokens**: Reduces authentication overhead
   ```json
   "sessionTokensCache": ".session_tokens.json"
   ```

2. **Use local files**: Faster than server interaction
   ```bash
   acvp-cli --wrapper modulewrapper --indir vectors/ --outdir responses/
   ```

### Organization

1. **Separate configs** for different environments:
   ```
   config.dev.json
   config.staging.json
   config.prod.json
   ```

2. **Use descriptive paths**:
   ```json
   {
     "logFile": "logs/acvp-$(date +%Y%m%d).log"
   }
   ```

## Next Steps

- See [Usage Examples](examples.md) for practical scenarios
- Check [Technical Architecture](../reference/technical.md) for implementation details
