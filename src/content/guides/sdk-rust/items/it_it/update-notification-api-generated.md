## Parameters

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenant_id | String | Sì |  |
| id | String | Sì |  |
| update_notification_body | models::UpdateNotificationBody | Sì |  |
| user_id | String | No |  |

## Risposta

Restituisce: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Esempio

[inline-code-attrs-start title = 'Esempio di update_notification'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let update_notification_body: models::UpdateNotificationBody = models::UpdateNotificationBody {
        enabled: true,
        event: "comment.posted".into(),
        channels: vec!["email".into(), "webhook".into()],
        template_id: "tmpl-new-comment".into(),
    };
    let params: UpdateNotificationParams = UpdateNotificationParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "notif-12345".to_string(),
        update_notification_body,
        user_id: Some("admin-user-99".to_string()),
    };
    let response: ApiEmptyResponse = update_notification(&configuration, params).await?;
    Ok(())
}
[inline-code-end]