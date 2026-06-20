Omogućite ili onemogućite obavijesti za stranicu. Kada su korisnici pretplaćeni na stranicu, obavijesti se stvaraju za nove glavne komentare, i također

## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| url_id | String | Yes |  |
| url | String | Yes |  |
| page_title | String | Yes |  |
| subscribed_or_unsubscribed | String | Yes |  |
| sso | String | No |  |

## Odgovor

Vraća: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_page_subscription_status_response.rs)

## Primjer

[inline-code-attrs-start title = 'update_user_notification_page_subscription_status Primjer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<UpdateUserNotificationPageSubscriptionStatusResponse, Error> {
    let params: UpdateUserNotificationPageSubscriptionStatusParams = UpdateUserNotificationPageSubscriptionStatusParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/rocket-launch-2026".to_string(),
        url: "https://acme.example.com/news/rocket-launch-2026".to_string(),
        page_title: "Acme Rocket Launch — June 2026".to_string(),
        subscribed_or_unsubscribed: "subscribed".to_string(),
        sso: Some("user:alice@acme.com".to_string()),
    };
    let response: UpdateUserNotificationPageSubscriptionStatusResponse =
        update_user_notification_page_subscription_status(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]