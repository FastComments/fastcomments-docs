## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| after_id | String | Hayır |  |
| after_created_at | i64 | Hayır |  |
| unread_only | bool | Hayır |  |
| dm_only | bool | Hayır |  |
| no_dm | bool | Hayır |  |
| sso | String | Hayır |  |

## Yanıt

Döndürür: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/reset_user_notifications_response.rs)

## Örnek

[inline-code-attrs-start title = 'reset_user_notifications Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_reset() -> Result<(), Error> {
    let params: ResetUserNotificationsParams = ResetUserNotificationsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        after_id: Some("notif-20260619-0001".to_string()),
        after_created_at: Some(1_787_400_000i64),
        unread_only: Some(true),
        dm_only: Some(false),
        no_dm: Some(false),
        sso: Some("saml".to_string()),
    };
    let response: ResetUserNotificationsResponse =
        reset_user_notifications(&configuration, params).await?;
    Ok(())
}
[inline-code-end]