Omogućite ili onemogućite obaveštenja za stranicu. Kada su korisnici pretplaćeni na stranicu, obaveštenja se kreiraju za nove korenske komentare, i takođe

## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenant_id | String | Yes |  |
| url_id | String | Yes |  |
| url | String | Yes |  |
| page_title | String | Yes |  |
| subscribed_or_unsubscribed | String | Yes |  |
| sso | String | No |  |

## Odgovor

Vraća: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_page_subscription_status_response.rs)

## Primer

[inline-code-attrs-start title = 'update_user_notification_page_subscription_status Primer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<UpdateUserNotificationPageSubscriptionStatusResponse, Error> {
    let params = UpdateUserNotificationPageSubscriptionStatusParams {
        tenant_id: "acme-corp-tenant".to_owned(),
        url_id: "news-article-2024".to_owned(),
        url: "https://news.example.com/articles/rust".to_owned(),
        page_title: "Rust Dominates the Programming World".to_owned(),
        subscribed_or_unsubscribed: "subscribed".to_owned(),
        sso: Some("sso-token-abc".to_owned()),
    };
    update_user_notification_page_subscription_status(&configuration, params).await
}
[inline-code-end]