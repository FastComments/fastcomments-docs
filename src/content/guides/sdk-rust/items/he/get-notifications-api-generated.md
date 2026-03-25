## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| user_id | String | לא |  |
| url_id | String | לא |  |
| from_comment_id | String | לא |  |
| viewed | bool | לא |  |
| skip | f64 | לא |  |

## תגובה

מחזיר: [`GetNotifications200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_notifications_200_response.rs)

## דוגמה

[inline-code-attrs-start title = 'get_notifications דוגמה'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_notifications() -> Result<(), Error> {
    let params: GetNotificationsParams = GetNotificationsParams {
        tenant_id: "acme-corp-tenant".to_owned(),
        user_id: Some("user-12345".to_owned()),
        url_id: Some("news/article/2026/03/25/major-update".to_owned()),
        from_comment_id: Some("cmt-98765".to_owned()),
        viewed: Some(false),
        skip: Some(0.0),
    };
    let notifications: GetNotifications200Response = get_notifications(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---