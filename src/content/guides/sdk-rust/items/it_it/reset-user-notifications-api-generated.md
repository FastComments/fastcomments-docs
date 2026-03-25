## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Sì |  |
| after_id | String | No |  |
| after_created_at | i64 | No |  |
| unread_only | bool | No |  |
| dm_only | bool | No |  |
| no_dm | bool | No |  |
| sso | String | No |  |

## Risposta

Restituisce: [`ResetUserNotifications200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/reset_user_notifications_200_response.rs)

## Esempio

[inline-code-attrs-start title = 'reset_user_notifications Esempio'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_reset() -> Result<(), Error> {
    let params: ResetUserNotificationsParams = ResetUserNotificationsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        after_id: Some("notif_987654321".to_string()),
        after_created_at: Some(1672531200),
        unread_only: Some(true),
        dm_only: Some(false),
        no_dm: Some(false),
        sso: Some("sso-enterprise".to_string()),
    };
    let resp: ResetUserNotifications200Response = reset_user_notifications(&configuration, params).await?;
    let _ = resp;
    Ok(())
}
[inline-code-end]