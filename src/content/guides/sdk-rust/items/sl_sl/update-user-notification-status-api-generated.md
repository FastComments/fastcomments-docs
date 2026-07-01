## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| notification_id | String | Da |  |
| new_status | String | Da |  |
| sso | String | Ne |  |

## Odgovor

Vrne: [`UpdateUserNotificationStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_status_response.rs)

## Primer

[inline-code-attrs-start title = 'update_user_notification_status Primer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_update() -> Result<(), Error> {
    let params = UpdateUserNotificationStatusParams {
        tenant_id: "acme-corp-tenant".to_string(),
        notification_id: "news/article".to_string(),
        new_status: "read".to_string(),
        sso: Some("sso-token-123".to_string()),
    };
    let _response: UpdateUserNotificationStatusResponse =
        update_user_notification_status(&configuration, params).await?;
    Ok(())
}
[inline-code-end]