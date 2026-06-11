# Pattern Library

This directory contains YAML pattern files for PII detection. Patterns are organized by category:

## Directory Structure

```
patterns/
├── pii/              # General PII patterns (global)
│   └── global_pii.yaml
├── compliance/       # Compliance-specific patterns
│   ├── us_hipaa.yaml
│   ├── us_ccpa.yaml
│   └── uk_gdpr.yaml
└── security/         # Security credentials and secrets
    └── credentials.yaml
```

## Pattern Files

### `pii/global_pii.yaml`
**17 patterns** covering globally applicable PII:
- Email addresses
- Phone numbers (international)
- Credit cards (Visa, MasterCard, Amex, Discover)
- IBAN
- Dates (ISO and common formats)
- IP addresses (IPv4 and IPv6)
- MAC addresses
- URLs and domains
- GPS coordinates
- UUIDs
- API keys
- Cryptographic hashes (MD5, SHA-256)

### `compliance/us_hipaa.yaml`
**19 patterns** for HIPAA Safe Harbor compliance:
- Patient names
- Addresses and ZIP codes
- Dates of birth
- Telephone numbers
- Email addresses
- Social Security Numbers
- Medical Record Numbers (MRN)
- Health plan beneficiary numbers
- Account numbers
- Certificate/license numbers
- Vehicle identifiers
- Device identifiers
- Web URLs
- IP addresses
- Biometric identifiers
- Full face photographs (references)
- ICD-10 diagnosis codes
- CPT procedure codes

### `compliance/us_ccpa.yaml`
**11 patterns** for California Consumer Privacy Act:
- Social Security Numbers
- California driver's licenses
- Email addresses
- Phone numbers
- Credit cards
- Bank account numbers
- Medical record numbers
- ZIP codes
- Street addresses
- IP addresses
- MAC addresses
- Employment information

### `compliance/uk_gdpr.yaml`
**14 patterns** for UK GDPR / Data Protection Act 2018:
- National Insurance Numbers (NINO)
- NHS Numbers
- UK Passport Numbers
- UK Driving Licence Numbers
- Email addresses
- UK Phone Numbers
- Sort Codes
- Bank Account Numbers
- UK IBANs
- UK Postcodes
- UK Addresses
- VAT Numbers
- Company Registration Numbers
- Network identifiers (IP, MAC)

### `security/credentials.yaml`
**17 patterns** for detecting security credentials:
- Generic API keys
- AWS Access Keys and Secret Keys
- GitHub Personal Access Tokens
- Slack API Tokens
- Stripe API Keys
- Google API Keys
- Azure Subscription Keys
- Database Connection Strings
- JDBC URLs
- JWT Tokens
- SSH Private Keys
- SSH Public Keys
- Password Fields
- Bearer Tokens
- Basic Authentication
- X.509 Certificates
- Webhook URLs

## Usage

### Command Line

```bash
# Enable YAML pattern loading
cargo run -p redact-cli -- --load-yaml analyze "text to analyze"

# Custom patterns directory
cargo run -p redact-cli -- --load-yaml --patterns-dir /path/to/patterns analyze "text"

# Anonymize with YAML patterns
cargo run -p redact-cli -- --load-yaml anonymize "SSN: 123-45-6789"
```

### API Server

```bash
# Enable YAML patterns via environment variable
export LOAD_YAML_PATTERNS=true
export PATTERNS_DIR=patterns  # optional, defaults to "patterns"

# Start server
cargo run --release -p redact-api
```

### Programmatic

```rust
use redact_core::recognizers::pattern::PatternRecognizer;

let mut recognizer = PatternRecognizer::new();
recognizer.load_patterns_from_yaml("patterns")?;
```

## Pattern Statistics

| File | Patterns | Entity Types | Coverage |
|------|----------|--------------|----------|
| global_pii.yaml | 17 | 13 | Global |
| us_hipaa.yaml | 19 | 18 | US Healthcare |
| us_ccpa.yaml | 11 | 11 | California |
| uk_gdpr.yaml | 14 | 14 | United Kingdom |
| credentials.yaml | 17 | 17 | Global |
| **Total** | **78** | **73** | - |

Note: Some patterns map to the same EntityType (e.g., multiple email patterns all map to `EmailAddress`).

## Creating Custom Patterns

1. Create a new YAML file in the appropriate subdirectory
2. Follow the pattern format (see `docs/yaml-patterns.md`)
3. Test your patterns:

```bash
cargo run -p redact-cli -- --load-yaml analyze "your test text"
```

## Pattern Development Guidelines

1. **Be specific**: Avoid overly broad patterns that match non-PII
2. **Use word boundaries**: Add `\b` to prevent matching inside words
3. **Set appropriate confidence**: Higher confidence = fewer false positives
4. **Add context**: For ambiguous patterns, consider adding context words
5. **Test thoroughly**: Verify with both positive and negative test cases
6. **Document examples**: Include realistic examples in the YAML

## Regex Syntax

Patterns use **Rust regex syntax**. Key differences from other flavors:

- ✅ Supports: `\b`, `\d`, `\w`, `\s`, character classes, quantifiers
- ✅ Supports: Non-capturing groups `(?:...)`
- ❌ Not supported: Lookahead `(?=...)` or lookbehind `(?<=...)`
- ❌ Not supported: Backreferences `\1`

See: https://docs.rs/regex/latest/regex/#syntax

## Performance

- Patterns are compiled once at startup
- No runtime overhead compared to hardcoded patterns
- All 78 patterns load in ~10-20ms
- Regex matching is highly optimized

## License

All pattern files in this directory are public domain and free to use, modify, and distribute.

## References

- **HIPAA Safe Harbor**: [HHS.gov](https://www.hhs.gov/hipaa/for-professionals/privacy/special-topics/de-identification/index.html)
- **CCPA**: [California AG](https://oag.ca.gov/privacy/ccpa)
- **UK GDPR**: [ICO.org.uk](https://ico.org.uk/for-organisations/guide-to-data-protection/)
- **Regex Reference**: [Rust regex crate](https://docs.rs/regex/)
