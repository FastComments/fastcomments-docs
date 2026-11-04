페이지에 대한 알림을 활성화하거나 비활성화합니다. 사용자가 페이지를 구독하면 새로운 루트 댓글에 대한 알림이 생성되며, 또한

## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| url_id | String | 예 |  |
| url | String | 예 |  |
| page_title | String | 예 |  |
| subscribed_or_unsubscribed | String | 예 |  |
| sso | String | 아니요 |  |

## 응답

반환값: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_page_subscription_status_response.rs)

## 예제

[inline-code-attrs-start title = 'update_user_notification_page_subscription_status 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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