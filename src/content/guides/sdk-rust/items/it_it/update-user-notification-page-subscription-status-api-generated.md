Abilita o disabilita le notifiche per una pagina. Quando gli utenti sono iscritti a una pagina, le notifiche vengono create
per i nuovi commenti principali, e anche

## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|----------|-------------|
| tenant_id | String | Sì |  |
| url_id | String | Sì |  |
| url | String | Sì |  |
| page_title | String | Sì |  |
| subscribed_or_unsubscribed | String | Sì |  |
| sso | String | No |  |

## Risposta

Restituisce: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_page_subscription_status_response.rs)

## Esempio

[inline-code-attrs-start title = 'Esempio di update_user_notification_page_subscription_status'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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