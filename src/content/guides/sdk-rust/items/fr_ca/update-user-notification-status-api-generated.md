## Paramètres

| Nom | Type | Requis | Description |
|------|------|--------|-------------|
| tenant_id | String | Oui |  |
| notification_id | String | Oui |  |
| new_status | String | Oui |  |
| sso | String | Non |  |

## Réponse

Retourne : [`UpdateUserNotificationStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_status_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de update_user_notification_status'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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