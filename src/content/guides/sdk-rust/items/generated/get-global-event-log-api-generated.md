
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
    let tenant_id: String = "acme-corp-tenant".to_string();
    let url_id: String = "news/article/2025-election".to_string();
    let user_id_ws: String = "ws-user-42".to_string();
    let start_time: i64 = 1_693_000_000;
    let end_time: i64 = 1_693_086_400;
    let params = GetGlobalEventLogParams {
        tenant_id,
        url_id,
        user_id_ws,
        start_time,
        end_time,
    };
    let response: GetEventLog200Response = get_global_event_log(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
