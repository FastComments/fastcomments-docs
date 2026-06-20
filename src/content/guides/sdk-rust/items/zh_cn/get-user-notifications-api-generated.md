## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| url_id | String | 否 |  |
| page_size | i32 | 否 |  |
| after_id | String | 否 |  |
| include_context | bool | 否 |  |
| after_created_at | i64 | 否 |  |
| unread_only | bool | 否 |  |
| dm_only | bool | 否 |  |
| no_dm | bool | 否 |  |
| include_translations | bool | 否 |  |
| include_tenant_notifications | bool | 否 |  |
| sso | String | 否 |  |

## 响应

返回：[`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_my_notifications_response.rs)

## 示例

[inline-code-attrs-start title = 'get_user_notifications 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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