## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| user_id | String | No |  |
| url_id | String | No |  |
| from_comment_id | String | No |  |
| viewed | bool | No |  |
| skip | f64 | No |  |

## Response

Returns: [`GetNotifications200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_notifications_200_response.rs)

## Example

[inline-code-attrs-start title = 'get_notifications Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_notifications() -> Result<(), Error> {
    let params: GetNotificationsParams = GetNotificationsParams {
        tenant_id: "acme-news-tenant".to_string(),
        user_id: Some("user-98765".to_string()),
        url_id: Some("news/politics/2026-election".to_string()),
        from_comment_id: Some("cmt-7890".to_string()),
        viewed: Some(false),
        skip: Some(0.0),
    };
    let response: GetNotifications200Response = get_notifications(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
