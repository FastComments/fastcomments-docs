## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| sso | String | Не |  |

## Одговор

Враћа: [`GetUserNotificationCountResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_notification_count_response.rs)

## Пример

[inline-code-attrs-start title = 'get_user_notification_count Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params: GetUserNotificationCountParams = GetUserNotificationCountParams {
        tenant_id: "acme-corp-tenant".to_owned(),
        sso: Some("user-42.sso.example".to_owned()),
    };
    let response: GetUserNotificationCountResponse = get_user_notification_count(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---