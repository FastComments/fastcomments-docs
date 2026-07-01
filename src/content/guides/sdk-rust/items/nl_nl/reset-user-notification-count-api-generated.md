## Parameters

| Naam | Type | Verplicht | Omschrijving |
|------|------|-----------|--------------|
| tenant_id | String | Ja |  |
| sso | String | Nee |  |

## Response

Retourneert: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/reset_user_notifications_response.rs)

## Example

[inline-code-attrs-start title = 'reset_user_notification_count Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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