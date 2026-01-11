## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| sso | String | No |  |

## Response

Returns: [`GetUserNotificationCount200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_notification_count_200_response.rs)

## Example

[inline-code-attrs-start title = 'get_user_notification_count Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_notification_count() -> Result<(), Error> {
    let params: GetUserNotificationCountParams = GetUserNotificationCountParams {
        tenant_id: "acme-corp-tenant".to_string(),
        sso: Some("user-9876-sso-token".to_string()),
    };
    let response: GetUserNotificationCount200Response =
        get_user_notification_count(&configuration, params).await?;
    let _ = response;
    Ok(())
}
[inline-code-end]
