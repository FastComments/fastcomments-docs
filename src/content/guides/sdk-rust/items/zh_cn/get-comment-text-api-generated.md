## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| comment_id | String | 是 |  |
| edit_key | String | 否 |  |
| sso | String | 否 |  |

## 响应

返回: [`GetCommentText200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comment_text_200_response.rs)

## 示例

[inline-code-attrs-start title = 'get_comment_text 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
pub async fn run() -> Result<GetCommentText200Response, Error> {
    let params: GetCommentTextParams = GetCommentTextParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article-2026-03-25-98765".to_string(),
        edit_key: Some("edit_4f3d2b9a".to_string()),
        sso: Some("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9".to_string()),
    };
    let comment: GetCommentText200Response = get_comment_text(&configuration, params).await?;
    Ok(comment)
}
[inline-code-end]