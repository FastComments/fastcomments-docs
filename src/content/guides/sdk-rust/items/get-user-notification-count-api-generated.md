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
async fn run() -> Result<GetUserNotificationCount200Response, Error> {
    let params: GetUserNotificationCountParams = GetUserNotificationCountParams {
        tenant_id: String::from("acme-corp-tenant"),
        sso: Some(String::from("user-12345-sso-token")),
    };
    let notifications: GetUserNotificationCount200Response =
        get_user_notification_count(&configuration, params).await?;
    Ok(notifications)
}
[inline-code-end]
