Aktiver eller deaktiver notifikationer for en side. Når brugere er tilmeldt en side, oprettes notifikationer for nye topniveau-kommentarer, og også

## Parametre

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| url_id | String | Ja |  |
| url | String | Ja |  |
| page_title | String | Ja |  |
| subscribed_or_unsubscribed | String | Ja |  |
| sso | String | Nej |  |

## Respons

Returnerer: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_page_subscription_status_response.rs)

## Eksempel

[inline-code-attrs-start title = 'update_user_notification_page_subscription_status Eksempel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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