הפעל או השבת התראות עבור דף. כאשר משתמשים מנויים לדף, נוצרות התראות
עבור הערות שורש חדשות, וגם

## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| url_id | String | כן |  |
| url | String | כן |  |
| page_title | String | כן |  |
| subscribed_or_unsubscribed | String | כן |  |
| sso | String | לא |  |

## תגובה

מחזיר: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_status_200_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-update_user_notification_page_subscription_status'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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

---