页面上当前不在线的历史评论者。按 displayName 排序。
在用尽 /users/online 之后，使用此接口渲染“成员”部分。
基于 commenterName 的游标分页：服务器通过 $gt 从 afterName 开始向前遍历部分索引 {tenantId, urlId, commenterName}，没有 $skip 的开销。

## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| url_id | String | 是 |  |
| after_name | String | 否 |  |
| after_user_id | String | 否 |  |

## 响应

返回：[`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_offline_response.rs)

## 示例

[inline-code-attrs-start title = 'get_offline_users 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_offline(config: &configuration::Configuration) -> Result<(), Error> {
    let params = GetOfflineUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
        after_name: Some("alice".to_string()),
        after_user_id: Some("user-42".to_string()),
    };
    let _response = get_offline_users(config, params).await?;
    Ok(())
}
[inline-code-end]