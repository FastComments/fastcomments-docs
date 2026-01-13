
req
tenantId
urlId
userIdWS

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| url_id | String | Yes |  |
| user_id_ws | String | Yes |  |
| start_time | i64 | Yes |  |
| end_time | i64 | Yes |  |

## Response

Returns: [`GetEventLog200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_event_log_200_response.rs)

## Example

[inline-code-attrs-start title = 'get_global_event_log Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<GetEventLog200Response, Error> {
    let params: GetGlobalEventLogParams = GetGlobalEventLogParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/world/article-123".to_string(),
        user_id_ws: "user-987".to_string(),
        start_time: 1672531200_i64,
        end_time: 1672617600_i64,
        cursor: Some("cursor-0001".to_string()),
        limit: Some(500),
    };
    let response: GetEventLog200Response = get_global_event_log(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
