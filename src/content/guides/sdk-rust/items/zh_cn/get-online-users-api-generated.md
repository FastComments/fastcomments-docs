当前页面在线的观众：当前其 websocket 会话已订阅该页面的人。
返回 anonCount + totalCount（房间范围的订阅者，包括我们不枚举的匿名观众）。

## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| url_id | String | 是 |  |
| after_name | String | 否 |  |
| after_user_id | String | 否 |  |

## 响应

返回：[`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_online_response.rs)

## 示例

[inline-code-attrs-start title = 'get_online_users 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_online_users() -> Result<PageUsersOnlineResponse, Error> {
    let params: GetOnlineUsersParams = GetOnlineUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/world/article-2026".to_string(),
        after_name: Some("jane.doe".to_string()),
        after_user_id: Some("user_98765".to_string()),
    };
    let response: PageUsersOnlineResponse = get_online_users(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---