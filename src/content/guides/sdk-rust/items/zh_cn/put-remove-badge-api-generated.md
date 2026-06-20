## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| badge_id | String | 是 |  |
| user_id | String | 否 |  |
| comment_id | String | 否 |  |
| broadcast_id | String | 否 |  |
| sso | String | 否 |  |

## 响应

返回：[`RemoveUserBadgeResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/remove_user_badge_response.rs)

## 示例

[inline-code-attrs-start title = 'put_remove_badge 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: PutRemoveBadgeParams = PutRemoveBadgeParams {
    badge_id: "trusted-moderator-1".to_string(),
    user_id: Some("user-82f9".to_string()),
    comment_id: Some("comment-000123".to_string()),
    broadcast_id: Some("live-broadcast-nyc-2026".to_string()),
    sso: Some("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9".to_string()),
};

let response: RemoveUserBadgeResponse = put_remove_badge(&configuration, params).await?;
[inline-code-end]

---