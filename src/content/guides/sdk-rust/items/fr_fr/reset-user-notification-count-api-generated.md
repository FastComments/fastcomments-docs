## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| sso | String | Non |  |

## Réponse

Renvoie : [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/reset_user_notifications_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de reset_user_notification_count'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_reset() -> Result<ResetUserNotificationsResponse, Error> {
    let params: ResetUserNotificationCountParams = ResetUserNotificationCountParams {
        tenant_id: "acme-corp-tenant".to_string(),
        sso: Some("https://sso.acme.com/session/abc123".to_string()),
    };
    let response: ResetUserNotificationsResponse = reset_user_notification_count(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---