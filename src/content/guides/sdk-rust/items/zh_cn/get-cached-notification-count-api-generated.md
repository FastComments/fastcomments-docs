## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |

## 响应

返回: [`GetCachedNotificationCountResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_cached_notification_count_response.rs)

## 示例

[inline-code-attrs-start title = 'get_cached_notification_count 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_notification_count() -> Result<(), Error> {
    let params = GetCachedNotificationCountParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article".to_string(),
    };
    let response = get_cached_notification_count(&configuration, params).await?;
    let _ = response.user_notification_count;
    Ok(())
}
[inline-code-end]

---