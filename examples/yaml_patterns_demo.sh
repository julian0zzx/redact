#!/bin/bash
# Demo script for YAML pattern loading feature

set -e

echo "=========================================="
echo "Redact YAML Pattern Loading Demo"
echo "=========================================="
echo ""

# Build the CLI if needed
echo "Building redact-cli..."
cargo build --release --package redact-cli --quiet
echo "✓ Build complete"
echo ""

REDACT="./target/release/redact"

echo "=========================================="
echo "Test 1: Global PII Detection"
echo "=========================================="
echo ""
echo "Input: \"Email: user@example.com, Phone: +44 20 7946 0958\""
echo ""
$REDACT --load-yaml analyze "Email: user@example.com, Phone: +44 20 7946 0958" 2>/dev/null
echo ""

echo "=========================================="
echo "Test 2: Healthcare Data (HIPAA)"
echo "=========================================="
echo ""
echo "Input: \"Patient MRN: MED123456, SSN: 123-45-6789\""
echo ""
$REDACT --load-yaml analyze "Patient MRN: MED123456, SSN: 123-45-6789" 2>/dev/null
echo ""

echo "=========================================="
echo "Test 3: UK GDPR Data"
echo "=========================================="
echo ""
echo "Input: \"NHS Number: 401 023 2137, Postcode: SW1A 1AA\""
echo ""
$REDACT --load-yaml analyze "NHS Number: 401 023 2137, Postcode: SW1A 1AA" 2>/dev/null
echo ""

echo "=========================================="
echo "Test 4: Security Credentials"
echo "=========================================="
echo ""
echo "Input: \"GitHub token: ghp_1234567890abcdef1234567890abcdef12345678\""
echo ""
$REDACT --load-yaml analyze "GitHub token: ghp_1234567890abcdef1234567890abcdef12345678" 2>/dev/null
echo ""

echo "=========================================="
echo "Test 5: Financial Data"
echo "=========================================="
echo ""
echo "Input: \"Card: 4532015112830366, IBAN: GB82WEST12345698765432\""
echo ""
$REDACT --load-yaml analyze "Card: 4532015112830366, IBAN: GB82WEST12345698765432" 2>/dev/null
echo ""

echo "=========================================="
echo "Test 6: Anonymization with YAML Patterns"
echo "=========================================="
echo ""
echo "Input: \"Contact: john@example.com, NHS: 401 023 2137\""
echo ""
echo "Anonymized:"
$REDACT --load-yaml anonymize "Contact: john@example.com, NHS: 401 023 2137" 2>/dev/null
echo ""

echo "=========================================="
echo "Test 7: Mask Strategy"
echo "=========================================="
echo ""
echo "Input: \"Email: sensitive@company.com, Card: 4532015112830366\""
echo ""
echo "Masked:"
$REDACT --load-yaml anonymize --strategy mask "Email: sensitive@company.com, Card: 4532015112830366" 2>/dev/null
echo ""

echo "=========================================="
echo "Test 8: JSON Output"
echo "=========================================="
echo ""
echo "Input: \"test@example.com, +44 7700 900123\""
echo ""
$REDACT --load-yaml --format json analyze "test@example.com, +44 7700 900123" 2>/dev/null | head -20
echo "  ..."
echo ""

echo "=========================================="
echo "Demo Complete!"
echo "=========================================="
echo ""
echo "Pattern Statistics:"
echo "  - Total patterns loaded: 64+"
echo "  - Pattern files: 5 (global_pii, us_hipaa, us_ccpa, uk_gdpr, credentials)"
echo "  - Entity types: 70+"
echo ""
echo "For more information:"
echo "  - patterns/README.md - Pattern library documentation"
echo "  - docs/yaml-patterns.md - YAML pattern format and usage"
echo "  - $REDACT --help - CLI help"
