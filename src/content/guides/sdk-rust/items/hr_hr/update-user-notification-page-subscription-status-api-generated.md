Omogućite ili onemogućite obavijesti za stranicu. Kada su korisnici pretplaćeni na stranicu, obavijesti se stvaraju
za nove glavne komentare, i također

## Parameters

| Naziv | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| url_id | String | Da |  |
| url | String | Da |  |
| page_title | String | Da |  |
| subscribed_or_unsubscribed | String | Da |  |
| sso | String | Ne |  |

## Odgovor

Vraća: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_status_200_response.rs)

## Primjer

[inline-code-attrs-start title = 'update_user_notification_page_subscription_status Primjer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<UpdateUserNotificationStatus200Response, Error> {
    let params = UpdateUserNotificationPageSubscriptionStatusParams {
        tenant_id: String::from("acme-corp-tenant"),
        url_id: String::from("article-12345"),
        url: String::from("https://news.acme.com/articles/2026/03/25/advances-in-ai"),
        page_title: String::from("Advances in AI: What to Expect in 2026"),
        subscribed_or_unsubscribed: String::from("subscribed"),
        sso: Some(String::from("user-jwt-xyz123")),
    };
    let response = update_user_notification_page_subscription_status(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]