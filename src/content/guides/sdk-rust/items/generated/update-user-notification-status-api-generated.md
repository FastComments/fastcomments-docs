## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| notification_id | String | Yes |  |
| new_status | String | Yes |  |
| sso | String | No |  |

## Response

Returns: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_status_200_response.rs)

## Example

[inline-code-attrs-start title = 'update_user_notification_status Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_update_user_notification_status() -> Result<UpdateUserNotificationStatus200Response, Error> {
    let params: UpdateUserNotificationStatusParams = UpdateUserNotificationStatusParams {
        tenant_id: String::from("acme-corp-tenant"),
        notification_id: String::from("notification-8b7f3"),
        new_status: String::from("read"),
        sso: Some(String::from("sso-jwt-1a2b3c")),
    };
    let response: UpdateUserNotificationStatus200Response = update_user_notification_status(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
