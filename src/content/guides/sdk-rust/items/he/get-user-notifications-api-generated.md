## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| url_id | String | לא |  |
| page_size | i32 | לא |  |
| after_id | String | לא |  |
| include_context | bool | לא |  |
| after_created_at | i64 | לא |  |
| unread_only | bool | לא |  |
| dm_only | bool | לא |  |
| no_dm | bool | לא |  |
| include_translations | bool | לא |  |
| include_tenant_notifications | bool | לא |  |
| sso | String | לא |  |

## תגובה

מחזיר: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_my_notifications_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_user_notifications'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_notifications() -> Result<GetMyNotificationsResponse, Error> {
    let params: GetUserNotificationsParams = GetUserNotificationsParams {
        tenant_id: String::from("acme-corp-tenant"),
        url_id: Some(String::from("news/product-launch")),
        page_size: Some(25),
        after_id: Some(String::from("notif_1024")),
        include_context: Some(true),
        after_created_at: Some(1_676_000_000i64),
        unread_only: Some(true),
        dm_only: Some(false),
        no_dm: Some(false),
        include_translations: Some(true),
        include_tenant_notifications: Some(false),
        sso: Some(String::from("sso_token_abc123")),
    };
    let notifications: GetMyNotificationsResponse = get_user_notifications(&configuration, params).await?;
    Ok(notifications)
}
[inline-code-end]

---