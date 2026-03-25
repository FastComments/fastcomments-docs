
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
async fn run() -> Result<(), Error> {
    let include_deleted: Option<bool> = Some(false);
    let params: GetGlobalEventLogParams = GetGlobalEventLogParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article-123".to_string(),
        user_id_ws: "user-789-ws".to_string(),
        start_time: 1711411200i64,
        end_time: 1711497599i64,
    };
    let response: GetEventLog200Response = get_global_event_log(&configuration, params).await?;
    let _include_deleted = include_deleted;
    Ok(())
}
[inline-code-end]
