
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

[inline-code-attrs-start title = 'get_event_log Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_event_log() -> Result<(), Error> {
    let params: GetEventLogParams = GetEventLogParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/2026/01/investigation-into-qa".to_string(),
        user_id_ws: "user-54213-ws".to_string(),
        start_time: 1705123200i64,
        end_time: 1705126800i64,
        limit: Some(200),
    };
    let response: GetEventLog200Response = get_event_log(&configuration, params).await?;
    let _ = response;
    Ok(())
}
[inline-code-end]
