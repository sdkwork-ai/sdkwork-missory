//! Response envelope helpers (`API_SPEC.md` §15.1).

use axum::http::{HeaderName, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use sdkwork_missory_contract::error::MissoryServiceError;
use sdkwork_missory_contract::MissoryPage;
use sdkwork_utils_rust::{
    SdkWorkApiResponse, SdkWorkPageData, SdkWorkResourceData, SDKWORK_SUCCESS_CODE,
};

use crate::error::ApiProblem;

/// Response header carrying the server-minted trace id.
pub const TRACE_ID_HEADER: &str = "x-sdkwork-trace-id";

/// Mints a fresh server-side trace id (never echoes client request ids).
pub fn new_trace_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

fn envelope_response<T: serde::Serialize>(
    status: StatusCode,
    envelope: SdkWorkApiResponse<T>,
) -> Response {
    let trace_id = envelope.trace_id.clone();
    let value = serde_json::to_value(&envelope).unwrap_or(serde_json::Value::Null);
    let mut response = (status, Json(value)).into_response();
    if let Ok(header_value) = HeaderValue::from_str(&trace_id) {
        response
            .headers_mut()
            .insert(HeaderName::from_static(TRACE_ID_HEADER), header_value);
    }
    response
}

/// Single-resource success (`data.item`) with HTTP 200.
pub fn ok_item<T: serde::Serialize>(item: T) -> Result<Response, ApiProblem> {
    Ok(envelope_response(
        StatusCode::OK,
        SdkWorkApiResponse {
            code: SDKWORK_SUCCESS_CODE,
            data: SdkWorkResourceData { item },
            trace_id: new_trace_id(),
        },
    ))
}

/// Single-resource success with HTTP 201 (create).
pub fn created_item<T: serde::Serialize>(item: T) -> Result<Response, ApiProblem> {
    Ok(envelope_response(
        StatusCode::CREATED,
        SdkWorkApiResponse {
            code: SDKWORK_SUCCESS_CODE,
            data: SdkWorkResourceData { item },
            trace_id: new_trace_id(),
        },
    ))
}

/// Page success (`data.items` + `data.pageInfo`) with HTTP 200.
pub fn ok_page<T: serde::Serialize>(page: MissoryPage<T>) -> Result<Response, ApiProblem> {
    Ok(envelope_response(
        StatusCode::OK,
        SdkWorkApiResponse {
            code: SDKWORK_SUCCESS_CODE,
            data: SdkWorkPageData {
                items: page.items,
                page_info: page.page_info,
            },
            trace_id: new_trace_id(),
        },
    ))
}

/// Command success (`data.accepted` plus optional fields) with HTTP 200.
pub fn ok_command(
    accepted: bool,
    resource_id: String,
    status: String,
) -> Result<Response, ApiProblem> {
    Ok(envelope_response(
        StatusCode::OK,
        SdkWorkApiResponse {
            code: SDKWORK_SUCCESS_CODE,
            data: serde_json::json!({
                "accepted": accepted,
                "resourceId": resource_id,
                "status": status,
            }),
            trace_id: new_trace_id(),
        },
    ))
}

/// 204 delete success: no JSON body, only headers.
pub fn no_content() -> Result<Response, ApiProblem> {
    Ok(StatusCode::NO_CONTENT.into_response())
}

/// Maps a service error into the problem envelope (handler tail helper).
pub fn map_error(error: MissoryServiceError) -> ApiProblem {
    error.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use sdkwork_utils_rust::PageInfo;

    #[tokio::test]
    async fn given_ok_item_when_serialized_then_envelope_has_code_zero_and_item() {
        let response = ok_item(serde_json::json!({"a": 1})).expect("ok");
        assert_eq!(response.status(), StatusCode::OK);
        assert!(response
            .headers()
            .get(HeaderName::from_static(TRACE_ID_HEADER))
            .is_some());
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("body");
        let json: serde_json::Value = serde_json::from_slice(&bytes).expect("json");
        assert_eq!(json["code"], 0);
        assert_eq!(json["data"]["item"]["a"], 1);
        assert!(json["traceId"].as_str().is_some());
    }

    #[tokio::test]
    async fn given_page_when_serialized_then_items_and_page_info_are_present() {
        let page = MissoryPage::<serde_json::Value> {
            items: vec![],
            page_info: PageInfo {
                mode: sdkwork_utils_rust::PageMode::Offset,
                page: Some(1),
                page_size: Some(20),
                total_items: Some("0".to_string()),
                total_pages: Some(0),
                next_cursor: None,
                has_more: Some(false),
            },
        };
        let response = ok_page(page).expect("ok");
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("body");
        let json: serde_json::Value = serde_json::from_slice(&bytes).expect("json");
        assert!(json["data"]["items"].is_array());
        assert_eq!(json["data"]["pageInfo"]["mode"], "offset");
    }

    #[tokio::test]
    async fn given_no_content_when_serialized_then_status_is_204_without_body() {
        let response = no_content().expect("ok");
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }
}
