## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenant_id | String | Ja |  |
| after_id | String | Nein |  |
| after_created_at | i64 | Nein |  |
| unread_only | bool | Nein |  |
| dm_only | bool | Nein |  |
| no_dm | bool | Nein |  |
| sso | String | Nein |  |

## Antwort

Rückgabe: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/reset_user_notifications_response.rs)

## Beispiel

[inline-code-attrs-start title = 'reset_user_notifications Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = ResetUserNotificationsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        after_id: Some("notif-12345".to_string()),
        after_created_at: Some(1_640_995_200),
        unread_only: Some(true),
        dm_only: Some(false),
        no_dm: Some(true),
        sso: Some("sso-provider".to_string()),
    };
    let _response: ResetUserNotificationsResponse =
        reset_user_notifications(&configuration, params).await?;
    Ok(())
}
[inline-code-end]