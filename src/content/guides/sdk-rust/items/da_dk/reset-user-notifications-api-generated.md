## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| after_id | String | Nej |  |
| after_created_at | i64 | Nej |  |
| unread_only | bool | Nej |  |
| dm_only | bool | Nej |  |
| no_dm | bool | Nej |  |
| sso | String | Nej |  |

## Svar

Returnerer: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/reset_user_notifications_response.rs)

## Eksempel

[inline-code-attrs-start title = 'reset_user_notifications Eksempel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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

---