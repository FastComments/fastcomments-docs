## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| sso | String | לא |  |

## תגובה

מחזיר: [`GetUserNotificationCount200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_notification_count_200_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_user_notification_count'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_user_notification_count() -> Result<(), Error> {
    let params: GetUserNotificationCountParams = GetUserNotificationCountParams {
        tenant_id: String::from("acme-corp-tenant"),
        sso: Some(String::from("sso-jwt-abc123")),
    };
    let _response: GetUserNotificationCount200Response =
        get_user_notification_count(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---