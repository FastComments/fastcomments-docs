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
async fn run_update() -> Result<UpdateUserNotificationStatus200Response, Error> {
    let params: UpdateUserNotificationPageSubscriptionStatusParams = UpdateUserNotificationPageSubscriptionStatusParams {
        tenant_id: String::from("acme-corp-tenant"),
        url_id: String::from("news-article-2026-01-12"),
        url: String::from("https://acme.example.com/news/rust-2-0"),
        page_title: String::from("Acme News — Rust 2.0 Released"),
        subscribed_or_unsubscribed: String::from("subscribed"),
        sso: Some(String::from("sso-jwt-abc123")),
    };
    let resp: UpdateUserNotificationStatus200Response = update_user_notification_page_subscription_status(&configuration, params).await?;
    Ok(resp)
}
[inline-code-end]
