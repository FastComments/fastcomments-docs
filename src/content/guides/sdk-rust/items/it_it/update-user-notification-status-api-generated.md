## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenant_id | String | Sì |  |
| notification_id | String | Sì |  |
| new_status | String | Sì |  |
| sso | String | No |  |

## Risposta

Restituisce: [`UpdateUserNotificationStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_status_response.rs)

## Esempio

[inline-code-attrs-start title = 'Esempio update_user_notification_status'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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

---