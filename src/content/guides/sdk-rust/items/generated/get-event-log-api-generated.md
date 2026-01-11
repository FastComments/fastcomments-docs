
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
async fn example() -> Result<(), Error> {
    let params = GetEventLogParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article-123".to_string(),
        user_id_ws: "user-98765".to_string(),
        start_time: 1_697_000_000i64,
        end_time: 1_697_086_400i64,
        limit: Some(100),
        event_types: Some(vec!["comment_created".to_string(), "vote".to_string()]),
    };
    let response: GetEventLog200Response = get_event_log(configuration, params).await?;
    Ok(())
}
[inline-code-end]
