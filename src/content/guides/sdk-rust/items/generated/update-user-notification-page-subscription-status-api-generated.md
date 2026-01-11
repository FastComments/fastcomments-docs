Enable or disable notifications for a page. When users are subscribed to a page, notifications are created
for new root comments, and also

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| url_id | String | Yes |  |
| url | String | Yes |  |
| page_title | String | Yes |  |
| subscribed_or_unsubscribed | String | Yes |  |
| sso | String | No |  |

## Response

Returns: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_status_200_response.rs)

## Example

[inline-code-attrs-start title = 'update_user_notification_page_subscription_status Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: UpdateUserNotificationPageSubscriptionStatusParams = UpdateUserNotificationPageSubscriptionStatusParams {
    tenant_id: "acme-corp-tenant".to_string(),
    url_id: "news-article-12345".to_string(),
    url: "https://acme.example.com/news/2025/11/21/new-product".to_string(),
    page_title: "Acme Launches New Product".to_string(),
    subscribed_or_unsubscribed: "subscribed".to_string(),
    sso: Some("sso_user_jwt_eyJhbGciOiJIUzI1Ni...".to_string()),
};
let response: UpdateUserNotificationStatus200Response = update_user_notification_page_subscription_status(&configuration, params).await?;
[inline-code-end]
