Past commenters on the page who are NOT currently online. Sorted by displayName.  
使用页面上过去的评论者，且当前不在线。按 displayName 排序。

Use this after exhausting /users/online to render a "Members" section.  
在用完 /users/online 后使用它来渲染“Members”部分。

Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.  
在 commenterName 上进行游标分页：服务器从 afterName 向前遍历部分 {tenantId, urlId, commenterName} 索引，使用 $gt，没有 $skip 成本。

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| url_id | String | Yes |  |
| after_name | String | No |  |
| after_user_id | String | No |  |

## Response

Returns: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_offline_response.rs)

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