啟用或停用某個頁面的通知。當使用者訂閱了某個頁面時，系統會為新的根留言建立通知，同時也會

## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| url_id | String | 是 |  |
| url | String | 是 |  |
| page_title | String | 是 |  |
| subscribed_or_unsubscribed | String | 是 |  |
| sso | String | 否 |  |

## 回應

回傳：[`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_page_subscription_status_response.rs)

## 範例

[inline-code-attrs-start title = 'update_user_notification_page_subscription_status 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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