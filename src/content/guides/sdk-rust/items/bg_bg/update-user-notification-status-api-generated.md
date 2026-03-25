## Параметри

| Име | Type | Задължително | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| notification_id | String | Да |  |
| new_status | String | Да |  |
| sso | String | Не |  |

## Отговор

Връща: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_status_200_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример за update_user_notification_status'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<UpdateUserNotificationStatus200Response, Error> {
    let params: UpdateUserNotificationStatusParams = UpdateUserNotificationStatusParams {
        tenant_id: "acme-corp-tenant".to_string(),
        notification_id: "notif-2026-04-01-7f3b".to_string(),
        new_status: "read".to_string(),
        sso: Some("sso-session-abcdef123456".to_string()),
    };
    let resp: UpdateUserNotificationStatus200Response =
        update_user_notification_status(&configuration, params).await?;
    Ok(resp)
}
[inline-code-end]

---