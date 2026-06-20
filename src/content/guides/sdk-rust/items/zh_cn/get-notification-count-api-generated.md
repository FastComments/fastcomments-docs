## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| user_id | String | 否 |  |
| url_id | String | 否 |  |
| from_comment_id | String | 否 |  |
| viewed | bool | 否 |  |

## 响应

返回: [`GetNotificationCountResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_notification_count_response.rs)

## 示例

[inline-code-attrs-start title = 'get_notification_count 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetNotificationCountParams = GetNotificationCountParams {
    tenant_id: "acme-corp-tenant".to_string(),
    user_id: Some("user-123".to_string()),
    url_id: Some("news/article/2026/06/19".to_string()),
    from_comment_id: Some("cmt-98765".to_string()),
    viewed: Some(false),
};
let notification_count: GetNotificationCountResponse = get_notification_count(configuration, params).await?;
[inline-code-end]

---