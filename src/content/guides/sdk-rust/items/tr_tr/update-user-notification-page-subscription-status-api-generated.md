Bir sayfa için bildirimleri etkinleştirin veya devre dışı bırakın. Kullanıcılar bir sayfaya abone olduğunda, yeni kök yorumlar için bildirimler oluşturulur ve ayrıca

## Parametreler

| Ad | Tür | Zorunlu | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| url_id | String | Evet |  |
| url | String | Evet |  |
| page_title | String | Evet |  |
| subscribed_or_unsubscribed | String | Evet |  |
| sso | String | Hayır |  |

## Yanıt

Döndürür: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_page_subscription_status_response.rs)

## Örnek

[inline-code-attrs-start title = 'update_user_notification_page_subscription_status Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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