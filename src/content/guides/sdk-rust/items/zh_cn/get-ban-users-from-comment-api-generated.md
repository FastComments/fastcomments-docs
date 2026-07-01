## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|------|
| tenant_id | String | 是 |  |
| comment_id | String | 是 |  |
| sso | String | 否 |  |

## 响应

返回: [`GetBannedUsersFromCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_banned_users_from_comment_response.rs)

## 示例

[inline-code-attrs-start title = 'get_ban_users_from_comment 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetBanUsersFromCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article/12345".to_string(),
        sso: Some("sso-unique-id".to_string()),
    };
    let _response = get_ban_users_from_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]