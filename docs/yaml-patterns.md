# YAML Pattern Loading

Starting from version 0.8.3, Redact supports loading PII detection patterns from YAML configuration files. This allows you to:

- **Extend detection capabilities** without modifying code
- **Customize patterns** for your specific use case
- **Share pattern libraries** across teams
- **Version control** your detection rules

## YAML Pattern File Format

Pattern files follow this structure:

```yaml
version: "1.0"
framework: "Framework-Name"
jurisdiction: "Geographic scope"
description: "Description of pattern set"
last_updated: "YYYY-MM-DD"

patterns:
  - id: "unique_pattern_id"
    name: "Human-readable name"
    category: "category_name"
    regex: 'Regular expression pattern'
    confidence: 0.85  # Score from 0.0 to 1.0
    description: "What this pattern detects"
    examples:
      - "example1"
      - "example2"
    replacement: "[REDACTED_TEXT]"
    enabled: true
```

### Fields

- **id**: Unique identifier for the pattern (used for entity type mapping)
- **name**: Human-readable name
- **category**: Category for organization (metadata only)
- **regex**: Regular expression pattern (Rust regex syntax)
- **confidence**: Detection confidence score (0.0 - 1.0)
- **description**: Explanation of what the pattern detects
- **examples**: Example strings that should match
- **replacement**: Default replacement text (not currently used)
- **enabled**: Set to `false` to disable a pattern

## Entity Type Mapping

The `id` field is automatically mapped to `EntityType` variants:

| ID Pattern | EntityType |
|------------|------------|
| Contains "EMAIL" | `EmailAddress` |
| Contains "PHONE" or "MOBILE" | `PhoneNumber` |
| Contains "CREDIT_CARD" | `CreditCard` |
| Contains "SSN" | `UsSsn` |
| Contains "IP" and "ADDRESS" | `IpAddress` |
| Contains "UK_NHS" | `UkNhs` |
| Contains "UK_NINO" | `UkNino` |
| Contains "IBAN" | `IbanCode` |
| Contains "UUID" or "GUID" | `Guid` |
| ... (see pattern.rs for full list) | ... |
| No match | `Custom(id)` |

## Usage

### CLI Usage

```bash
# Analyze text with YAML patterns
cargo run -p redact-cli -- --load-yaml analyze "test@example.com"

# Specify custom patterns directory
cargo run -p redact-cli -- --load-yaml --patterns-dir ./my-patterns analyze "text"

# Anonymize with YAML patterns
cargo run -p redact-cli -- --load-yaml anonymize "SSN: 123-45-6789"
```

### API Server Usage

Set environment variables:

```bash
# Enable YAML pattern loading
export LOAD_YAML_PATTERNS=true

# Optional: specify custom patterns directory (default: "patterns")
export PATTERNS_DIR=/path/to/patterns

# Start the server
cargo run --release -p redact-api
```

### Programmatic Usage

```rust
use redact_core::recognizers::pattern::PatternRecognizer;

let mut recognizer = PatternRecognizer::new();

// Load patterns from directory
let count = recognizer.load_patterns_from_yaml("patterns")?;
println!("Loaded {} patterns", count);

// Use the recognizer
let results = recognizer.analyze("test@example.com", "en")?;
```

## Example Pattern Files

The project includes several pattern libraries in the `patterns/` directory:

### Global PII (`patterns/pii/global_pii.yaml`)
- Email addresses
- Phone numbers (international)
- Credit cards (Visa, MC, Amex, Discover)
- IP addresses (v4 and v6)
- MAC addresses
- URLs and domain names
- GPS coordinates
- UUIDs
- API keys
- Cryptographic hashes (MD5, SHA-256)

### US HIPAA (`patterns/compliance/us_hipaa.yaml`)
- Patient names
- Social Security Numbers
- Medical Record Numbers (MRN)
- Health plan beneficiary numbers
- Account numbers
- Certificate/license numbers
- Device identifiers
- Biometric identifiers
- ICD-10 diagnosis codes
- CPT procedure codes

### US CCPA (`patterns/compliance/us_ccpa.yaml`)
- California-specific identifiers
- Driver's license numbers
- Financial information
- Employment information
- Network identifiers

### UK GDPR (`patterns/compliance/uk_gdpr.yaml`)
- National Insurance Numbers (NINO)
- NHS Numbers
- UK Passport Numbers
- UK Driving Licence Numbers
- UK Bank Account Numbers and Sort Codes
- UK Postcodes
- UK VAT Numbers

### Security Credentials (`patterns/security/credentials.yaml`)
- AWS Access Keys and Secret Keys
- GitHub Tokens
- Slack Tokens
- Stripe API Keys
- Google API Keys
- Azure Subscription Keys
- Database Connection Strings
- JWT Tokens
- SSH Private/Public Keys
- Bearer Tokens
- Basic Authentication
- X.509 Certificates

## Creating Custom Patterns

1. Create a new YAML file in the `patterns/` directory (or subdirectory):

```yaml
version: "1.0"
framework: "Custom-Patterns"
jurisdiction: "Global"
description: "Custom patterns for my organization"

patterns:
  - id: "custom_employee_id"
    name: "Employee ID"
    category: "identifier"
    regex: '\bEMP-\d{6}\b'
    confidence: 0.9
    description: "Detects employee IDs in format EMP-123456"
    examples:
      - "EMP-123456"
      - "EMP-999999"
    replacement: "[EMPLOYEE_ID]"
    enabled: true

  - id: "custom_project_code"
    name: "Project Code"
    category: "identifier"
    regex: '\bPROJ-[A-Z]{3}-\d{4}\b'
    confidence: 0.85
    description: "Project codes like PROJ-ABC-1234"
    examples:
      - "PROJ-ABC-1234"
      - "PROJ-XYZ-9999"
    replacement: "[PROJECT_CODE]"
    enabled: true
```

2. Load the patterns:

```bash
cargo run -p redact-cli -- --load-yaml analyze "Employee EMP-123456 works on PROJ-ABC-1234"
```

## Performance Considerations

- YAML patterns are loaded once at startup
- Patterns are compiled into Regex objects for fast matching
- No runtime overhead compared to hardcoded patterns
- Large pattern sets (100+) may increase startup time slightly

## Pattern Development Tips

1. **Test your regex** using online tools like regex101.com (use Rust flavor)
2. **Start with high confidence** values (0.8+) and adjust based on false positives
3. **Use word boundaries** (`\b`) to avoid matching inside words
4. **Avoid overly broad patterns** that could match non-PII text
5. **Set enabled: false** to temporarily disable patterns during testing
6. **Organize patterns** into files by jurisdiction or category

## Troubleshooting

### Patterns not loading

Check logs for error messages:
```bash
RUST_LOG=redact_core=debug cargo run -p redact-api
```

Common issues:
- Invalid YAML syntax
- Invalid regex pattern
- Wrong file extension (must be `.yaml` or `.yml`)
- Patterns directory doesn't exist

### Patterns detected incorrectly

- Adjust the `confidence` value
- Make regex more specific
- Check for conflicting patterns
- Review false positives in test data

## Roadmap

Future enhancements:
- [ ] Context words support in YAML patterns
- [ ] Pattern validation on load
- [ ] Hot reloading of pattern files
- [ ] Pattern conflict detection
- [ ] Performance profiling per pattern
- [ ] Pattern testing framework
