// Copyright 2026 Censgate LLC.
// Licensed under the Apache License, Version 2.0. See the LICENSE file
// in the project root for license information.

use crate::handlers::{analyze_api, anonymize_api, health, AppState};
use axum::{
    routing::{get, post},
    Router,
};

/// Create the application router with all routes
pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/healthz", get(health))
        .route("/api/v1/analyze", post(analyze_api))
        .route("/api/v1/anonymize", post(anonymize_api))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::{to_bytes, Body},
        http::{Request, StatusCode},
    };
    use redact_core::AnalyzerEngine;
    use serde_json::Value;
    use std::sync::Arc;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_health_route() {
        let state = AppState {
            engine: Arc::new(AnalyzerEngine::new()),
        };
        let app = create_router(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_healthz_route() {
        let state = AppState {
            engine: Arc::new(AnalyzerEngine::new()),
        };
        let app = create_router(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/healthz")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_analyze_route() {
        let state = AppState {
            engine: Arc::new(AnalyzerEngine::new()),
        };
        let app = create_router(state);

        let body = r#"{"text":"john@example.com","language":"en"}"#;

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/analyze")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["results"][0]["text"], "john@example.com");
    }

    #[tokio::test]
    async fn test_analyze_route_omits_entity_text_when_include_text_false() {
        let state = AppState {
            engine: Arc::new(AnalyzerEngine::new()),
        };
        let app = create_router(state);

        let body = r#"{"text":"Email: john@example.com","language":"en","include_text":false}"#;

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/analyze")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();
        let result = &json["results"][0];

        assert_eq!(result["entity_type"], "EMAIL_ADDRESS");
        assert!(result["start"].is_number());
        assert!(result["end"].is_number());
        assert!(result["score"].is_number());
        assert!(result["recognizer_name"].is_string());
        assert!(result.get("text").is_none());
    }

    #[tokio::test]
    async fn test_anonymize_route_omits_entity_text_when_include_text_false() {
        let state = AppState {
            engine: Arc::new(AnalyzerEngine::new()),
        };
        let app = create_router(state);

        let body = r#"{"text":"Email: john@example.com","language":"en","include_text":false}"#;

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/anonymize")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();
        let result = &json["results"][0];

        assert!(json["text"].as_str().unwrap().contains("[EMAIL_ADDRESS]"));
        assert_eq!(result["entity_type"], "EMAIL_ADDRESS");
        assert!(result["start"].is_number());
        assert!(result["end"].is_number());
        assert!(result["score"].is_number());
        assert!(result["recognizer_name"].is_string());
        assert!(result.get("text").is_none());
    }

    #[tokio::test]
    async fn test_anonymize_route_includes_entity_text_when_include_text_omitted() {
        let state = AppState {
            engine: Arc::new(AnalyzerEngine::new()),
        };
        let app = create_router(state);

        let body = r#"{"text":"Email: john@example.com","language":"en"}"#;

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/anonymize")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();

        assert!(json["text"].as_str().unwrap().contains("[EMAIL_ADDRESS]"));
        assert_eq!(json["results"][0]["text"], "john@example.com");
    }
}
