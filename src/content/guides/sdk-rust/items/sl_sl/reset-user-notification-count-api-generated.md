## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| sso | String | Ne |  |

## Odgovor

Vrne: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/reset_user_notifications_response.rs)

## Primer

[inline-code-attrs-start title = 'reset_user_notification_count Primer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_example(config: &configuration::Configuration) -> Result<(), Error> {
    let params = ResetUserNotificationCountParams {
        tenant_id: "acme-corp-tenant".to_string(),
        sso: Some("john.doe".to_string()),
    };
    let _response: ResetUserNotificationsResponse = reset_user_notification_count(config, params).await?;
    Ok(())
}
[inline-code-end]

---