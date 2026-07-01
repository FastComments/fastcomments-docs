当前在线的页面查看者：其 websocket 会话当前已订阅该页面的用户。返回 anonCount + totalCount（整个房间的订阅者，包括我们未枚举的匿名查看者）。

## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|------|------|
| tenant_id | String | Yes |  |
| url_id | String | Yes |  |
| after_name | String | No |  |
| after_user_id | String | No |  |

## 响应

返回：[`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_online_response.rs)

## 示例

[inline-code-attrs-start title = '获取在线用户 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetOnlineUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
        after_name: Some("john_doe".to_string()),
        after_user_id: Some("user-123".to_string()),
    };
    let _response: PageUsersOnlineResponse = get_online_users(&config, params).await?;
    Ok(())
}
[inline-code-end]