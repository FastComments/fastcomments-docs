Enable of uitschakelen van meldingen voor een specifieke opmerking.

## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|---------------|
| tenant_id | String | Ja |  |
| notification_id | String | Ja |  |
| opted_in_or_out | String | Ja |  |
| comment_id | String | Ja |  |
| sso | String | Nee |  |

## Response

Retourneert: [`UpdateUserNotificationCommentSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_comment_subscription_status_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'update_user_notification_comment_subscription_status Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = UpdateUserNotificationCommentSubscriptionStatusParams {
        tenant_id: "acme-corp-tenant".to_string(),
        notification_id: "comment-reply".to_string(),
        opted_in_or_out: "opted_in".to_string(),
        comment_id: "12345".to_string(),
        sso: Some("user-sso-token".to_string()),
    };
    let _response = update_user_notification_comment_subscription_status(configuration, params).await?;
    Ok(())
}
[inline-code-end]