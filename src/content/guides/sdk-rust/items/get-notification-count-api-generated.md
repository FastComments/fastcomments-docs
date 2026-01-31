## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| user_id | String | No |  |
| url_id | String | No |  |
| from_comment_id | String | No |  |
| viewed | bool | No |  |

## Response

Returns: [`GetNotificationCount200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_notification_count_200_response.rs)

## Example

[inline-code-attrs-start title = 'get_notification_count Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<GetNotificationCount200Response, Error> {
    let params: GetNotificationCountParams = GetNotificationCountParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: Some("user-1234".to_string()),
        url_id: Some("news/article/2026/01/12/breaking".to_string()),
        from_comment_id: Some("cmt-98765".to_string()),
        viewed: Some(false),
    };
    let notification_count: GetNotificationCount200Response = get_notification_count(&configuration, params).await?;
    Ok(notification_count)
}
[inline-code-end]
