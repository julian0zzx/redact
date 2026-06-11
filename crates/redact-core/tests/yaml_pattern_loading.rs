// Copyright 2026 Censgate LLC.
// Licensed under the Apache License, Version 2.0. See the LICENSE file
// in the project root for license information.

//! Integration tests for YAML pattern loading

use redact_core::recognizers::{pattern::PatternRecognizer, Recognizer};
use std::fs;
use std::path::PathBuf;

fn get_patterns_dir() -> PathBuf {
    // Patterns directory is at workspace root
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop(); // Go up from crates/redact-core
    path.pop(); // Go up to workspace root
    path.push("patterns");
    path
}

#[test]
fn test_load_patterns_from_yaml_directory() {
    let patterns_dir = get_patterns_dir();
    
    if !patterns_dir.exists() {
        eprintln!("Skipping test: patterns directory not found at {:?}", patterns_dir);
        return;
    }

    let mut recognizer = PatternRecognizer::new();
    let result = recognizer.load_patterns_from_yaml(&patterns_dir);
    
    assert!(result.is_ok(), "Failed to load patterns: {:?}", result.err());
    
    let count = result.unwrap();
    assert!(count > 0, "Should load at least one pattern");
    
    // We expect around 60-80 patterns from the default pattern files
    assert!(count >= 50, "Expected at least 50 patterns, got {}", count);
    
    println!("Successfully loaded {} patterns from YAML files", count);
}

#[test]
fn test_yaml_pattern_detection_email() {
    let patterns_dir = get_patterns_dir();
    
    if !patterns_dir.exists() {
        eprintln!("Skipping test: patterns directory not found");
        return;
    }

    let mut recognizer = PatternRecognizer::new();
    recognizer.load_patterns_from_yaml(&patterns_dir).unwrap();
    
    let text = "Contact: user@example.com";
    let results = recognizer.analyze(text, "en").unwrap();
    
    assert!(!results.is_empty(), "Should detect email address");
    
    let email_result = results.iter()
        .find(|r| matches!(r.entity_type, redact_core::EntityType::EmailAddress));
    
    assert!(email_result.is_some(), "Should detect EmailAddress entity type");
}

#[test]
fn test_yaml_pattern_detection_credit_card() {
    let patterns_dir = get_patterns_dir();
    
    if !patterns_dir.exists() {
        eprintln!("Skipping test: patterns directory not found");
        return;
    }

    let mut recognizer = PatternRecognizer::new();
    recognizer.load_patterns_from_yaml(&patterns_dir).unwrap();
    
    let text = "Card number: 4532015112830366";
    let results = recognizer.analyze(text, "en").unwrap();
    
    assert!(!results.is_empty(), "Should detect credit card");
    
    let cc_result = results.iter()
        .find(|r| matches!(r.entity_type, redact_core::EntityType::CreditCard));
    
    assert!(cc_result.is_some(), "Should detect CreditCard entity type");
}

#[test]
fn test_yaml_pattern_detection_uk_nhs() {
    let patterns_dir = get_patterns_dir();
    
    if !patterns_dir.exists() {
        eprintln!("Skipping test: patterns directory not found");
        return;
    }

    let mut recognizer = PatternRecognizer::new();
    recognizer.load_patterns_from_yaml(&patterns_dir).unwrap();
    
    // Valid NHS number with mod-11 checksum
    let text = "NHS number: 401 023 2137";
    let results = recognizer.analyze(text, "en").unwrap();
    
    assert!(!results.is_empty(), "Should detect NHS number");
    
    let nhs_result = results.iter()
        .find(|r| matches!(r.entity_type, redact_core::EntityType::UkNhs));
    
    assert!(nhs_result.is_some(), "Should detect UkNhs entity type");
}

#[test]
fn test_yaml_pattern_no_false_positives() {
    let patterns_dir = get_patterns_dir();
    
    if !patterns_dir.exists() {
        eprintln!("Skipping test: patterns directory not found");
        return;
    }

    let mut recognizer = PatternRecognizer::new();
    recognizer.load_patterns_from_yaml(&patterns_dir).unwrap();
    
    // Text with no PII
    let text = "The quick brown fox jumps over the lazy dog";
    let results = recognizer.analyze(text, "en").unwrap();
    
    assert!(results.is_empty(), "Should not detect any PII in plain text");
}

#[test]
fn test_load_nonexistent_directory() {
    let mut recognizer = PatternRecognizer::new();
    let result = recognizer.load_patterns_from_yaml("/nonexistent/directory");
    
    assert!(result.is_err(), "Should fail to load from nonexistent directory");
}

#[test]
fn test_yaml_patterns_with_default_patterns() {
    let patterns_dir = get_patterns_dir();
    
    if !patterns_dir.exists() {
        eprintln!("Skipping test: patterns directory not found");
        return;
    }

    // Recognizer already has default patterns loaded
    let mut recognizer = PatternRecognizer::new();
    
    // Load YAML patterns on top
    let yaml_count = recognizer.load_patterns_from_yaml(&patterns_dir).unwrap();
    
    assert!(yaml_count > 0, "Should load YAML patterns");
    
    // Both default and YAML patterns should work
    let text = "Email: test@example.com, SSN: 123-45-6789";
    let results = recognizer.analyze(text, "en").unwrap();
    
    assert!(results.len() >= 2, "Should detect both email and SSN");
}

#[test]
fn test_yaml_pattern_file_structure() {
    let patterns_dir = get_patterns_dir();
    
    if !patterns_dir.exists() {
        eprintln!("Skipping test: patterns directory not found");
        return;
    }

    // Check that expected pattern files exist
    let expected_files = vec![
        "pii/global_pii.yaml",
        "compliance/us_hipaa.yaml",
        "compliance/us_ccpa.yaml",
        "compliance/uk_gdpr.yaml",
        "security/credentials.yaml",
    ];
    
    for file in expected_files {
        let path = patterns_dir.join(file);
        assert!(
            path.exists(),
            "Expected pattern file not found: {}",
            path.display()
        );
        
        // Verify it's a valid YAML file
        let content = fs::read_to_string(&path)
            .expect(&format!("Failed to read {}", path.display()));
        
        assert!(
            content.contains("patterns:"),
            "File {} should contain 'patterns:' key",
            file
        );
    }
}
