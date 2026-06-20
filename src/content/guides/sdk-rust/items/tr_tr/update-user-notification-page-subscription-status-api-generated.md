Bir sayfa için bildirimleri etkinleştirir veya devre dışı bırakır. Kullanıcılar bir sayfaya abone olduğunda, bildirimler oluşturulur
yeni kök yorumlar için ve ayrıca

## Parametreler

| Ad | Tür | Gerekli | Açıklama |
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

---