## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |
| user_id | String | 否 |  |
| anon_user_id | String | 否 |  |

## 响应

返回： [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_response.rs)

## 示例

[inline-code-attrs-start title = 'flag_comment 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: FlagCommentParams = FlagCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article-2026-06-19/comment-98765".to_string(),
        user_id: Some("user-42".to_string()),
        anon_user_id: None,
    };
    let response: FlagCommentResponse = flag_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---